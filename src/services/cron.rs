use chrono::{DateTime, Duration, Utc};
use sqlx::MySqlPool;
use tokio::time;
use tracing::{error, info};

use crate::{
    model::depth_history::{DepthHistoryParams, DepthHistoryResponse, Interval},
    services::depth_history::store_intervals,
};

pub struct DepthHistoryCron {
    pool: MySqlPool,
    interval: Interval,
    count: u32,
    last_fetch_time: Option<DateTime<Utc>>,
}

impl DepthHistoryCron {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            pool,
            interval: Interval::Hour,
            count: 400,
            last_fetch_time: Some(DateTime::from_timestamp(1648771200, 0).unwrap()),
        }
    }

    pub async fn start(&mut self) {
        loop {
            if let Err(e) = self.fetch_and_store().await {
                error!("Failed to fetch and store depth history: {}", e);
                time::sleep(Duration::seconds(3).to_std().unwrap()).await;
                continue;
            }

            time::sleep(Duration::seconds(3).to_std().unwrap()).await;
        }
    }

    async fn fetch_and_store(&mut self) -> Result<(), anyhow::Error> {
        let client = reqwest::Client::new();
        let current_time = Utc::now();

        let params = DepthHistoryParams {
            interval: Some(self.interval.clone()),
            count: Some(self.count),
            from: self.last_fetch_time,
            to: Some(current_time),
        };

        let mut url =
            reqwest::Url::parse("https://midgard.ninerealms.com/v2/history/depths/ETH.ETH")?;

        // Add query parameters
        if let Some(interval) = &params.interval {
            url.query_pairs_mut().append_pair(
                "interval",
                match interval {
                    Interval::FiveMin => "5min",
                    Interval::Hour => "hour",
                    Interval::Day => "day",
                    Interval::Week => "week",
                    Interval::Month => "month",
                    Interval::Quarter => "quarter",
                    Interval::Year => "year",
                },
            );
        }

        url.query_pairs_mut()
            .append_pair("count", &params.count.unwrap().to_string());

        if let Some(from) = params.from {
            url.query_pairs_mut()
                .append_pair("from", &from.timestamp().to_string());
        }

        let response = client.get(url.clone()).send().await?;
        let response_text = response.text().await?;

        let depth_history =
            serde_json::from_str::<DepthHistoryResponse>(&response_text).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to parse response: {}, response: {}",
                    e,
                    response_text
                )
            })?;

        // Store the intervals in the database
        store_intervals(&self.pool, &depth_history.intervals).await?;

        info!(
            "Successfully stored {} intervals",
            depth_history.intervals.len()
        );

        // Update last fetch time to the end time of the last interval
        if let Some(last_interval) = depth_history.intervals.last() {
            self.last_fetch_time = Some(last_interval.end_time);
            info!(
                "Successfully updated depth history.URL: {} Last fetch time: {}",
                url, last_interval.end_time
            );
        }

        Ok(())
    }
}
