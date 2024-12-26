use crate::core::models::common::Interval;
use crate::core::models::earnings_history::{EarningsHistoryParams, EarningsHistoryResponse};
use crate::services::client::get_midgard_api_url;
use crate::services::repository::earnings::store_intervals;
use chrono::{DateTime, Duration, Utc};
use sqlx::MySqlPool;
use tokio::time;
use tracing::{error, info};

pub struct EarningsHistoryCron {
    pool: MySqlPool,
    interval: Interval,
    count: u32,
    last_fetch_time: Option<DateTime<Utc>>,
}

impl EarningsHistoryCron {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            pool,
            interval: Interval::Hour,
            count: 400,
            last_fetch_time: Some(DateTime::from_timestamp(1648771200, 0).unwrap()),
        }
    }

    pub async fn start(&mut self) -> Result<(), anyhow::Error> {
        loop {
            if let Err(e) = self.fetch_and_store().await {
                error!("Failed to fetch and store earnings history: {}", e);
                time::sleep(Duration::seconds(3).to_std().unwrap()).await;
                continue;
            }

            time::sleep(Duration::seconds(3).to_std().unwrap()).await;
        }
    }

    async fn fetch_and_store(&mut self) -> Result<(), anyhow::Error> {
        let client = reqwest::Client::new();

        loop {
            let params = EarningsHistoryParams {
                interval: Some(self.interval.clone()),
                count: Some(self.count),
                from: self.last_fetch_time,
                to: None,
            };

            let base_url = get_midgard_api_url();
            let mut url = reqwest::Url::parse(&format!("{}/history/earnings", base_url))?;

            if let Some(interval) = &params.interval {
                url.query_pairs_mut()
                    .append_pair("interval", &interval.to_string());
            }

            if let Some(count) = params.count {
                url.query_pairs_mut()
                    .append_pair("count", &count.to_string());
            }

            if let Some(from) = params.from {
                url.query_pairs_mut()
                    .append_pair("from", &from.timestamp().to_string());
            }

            match client.get(url.clone()).send().await {
                Ok(response) => {
                    let response_text = response.text().await?;

                    // Check if we got rate limited
                    if response_text.contains("slow down") {
                        tracing::warn!("Rate limited, waiting for 5 seconds before retry...");
                        time::sleep(Duration::seconds(5).to_std().unwrap()).await;
                        continue;
                    }

                    match serde_json::from_str::<EarningsHistoryResponse>(&response_text) {
                        Ok(earnings_history) => {
                            store_intervals(&self.pool, &earnings_history.intervals).await?;

                            info!(
                                "Successfully stored {} intervals",
                                earnings_history.intervals.len()
                            );

                            if let Some(last_interval) = earnings_history.intervals.last() {
                                self.last_fetch_time = Some(last_interval.end_time);
                                info!(
                                    "Successfully updated earnings history. URL: {} Last fetch time: {}",
                                    url, last_interval.end_time
                                );
                            }
                            break Ok(());
                        }
                        Err(e) => {
                            error!(
                                "Failed to parse response: {}, response text (first 500 chars): {}",
                                e,
                                response_text.chars().take(500).collect::<String>()
                            );
                            time::sleep(Duration::seconds(5).to_std().unwrap()).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    error!("Request failed: {}", e);
                    time::sleep(Duration::seconds(5).to_std().unwrap()).await;
                    continue;
                }
            }
        }
    }

    // Box Pin to avoid indefinite recursion
    pub async fn fetch_latest_hour(&mut self) -> Result<(), anyhow::Error> {
        let now = Utc::now();
        let one_hour_ago = now - Duration::hours(1);

        // let params = EarningsHistoryParams {
        //     interval: Some(Interval::Hour),
        //     count: Some(1),
        //     from: Some(one_hour_ago),
        //     to: Some(now),
        // };

        let client = reqwest::Client::new();
        let base_url = get_midgard_api_url();
        let mut url = reqwest::Url::parse(&format!("{}/history/earnings", base_url))?;

        url.query_pairs_mut()
            .append_pair("interval", "hour")
            .append_pair("count", "1")
            .append_pair("from", &one_hour_ago.timestamp().to_string())
            .append_pair("to", &now.timestamp().to_string());

        match client.get(url.clone()).send().await {
            Ok(response) => {
                let response_text = response.text().await?;

                if response_text.contains("slow down") {
                    tracing::warn!("Rate limited, waiting for 5 seconds before retry...");
                    time::sleep(Duration::seconds(5).to_std().unwrap()).await;
                    return Box::pin(self.fetch_latest_hour()).await;
                }

                match serde_json::from_str::<EarningsHistoryResponse>(&response_text) {
                    Ok(earnings_history) => {
                        store_intervals(&self.pool, &earnings_history.intervals).await?;
                        info!("Successfully stored latest hour earnings data");
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to parse response: {}", e);
                        Err(anyhow::anyhow!("Failed to parse response"))
                    }
                }
            }
            Err(e) => {
                error!("Request failed: {}", e);
                Err(anyhow::anyhow!("Request failed"))
            }
        }
    }
}
