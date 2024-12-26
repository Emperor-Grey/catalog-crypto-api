use chrono::{DateTime, Duration, Utc};
use sqlx::MySqlPool;
use tokio::time;
use tracing::{error, info};

use crate::services::jobs::cron::{
    depth_history_cron::DepthHistoryCron, earnings_history_cron::EarningsHistoryCron,
    runepool_units_history_cron::RunepoolUnitsHistoryCron, swap_history_cron::SwapHistoryCron,
};

pub struct HourlyFetcher {
    pool: MySqlPool,
    last_run: DateTime<Utc>,
}

impl HourlyFetcher {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            pool,
            last_run: Utc::now(),
        }
    }

    pub async fn start(&mut self) -> Result<(), anyhow::Error> {
        info!("Starting hourly fetcher...");

        loop {
            let now = Utc::now();
            let duration_since_last = now - self.last_run;

            if duration_since_last >= Duration::hours(1) {
                info!("Starting hourly data fetch cycle...");
                self.last_run = now;

                // Fetch depth history
                let depth_pool = self.pool.clone();
                let mut depth_cron = DepthHistoryCron::new(depth_pool);
                if let Err(e) = depth_cron.fetch_latest_hour().await {
                    error!("Failed to fetch depth history: {}", e);
                }
                time::sleep(Duration::seconds(3).to_std().unwrap()).await;

                // Fetch earnings history
                let earnings_pool = self.pool.clone();
                let mut earnings_cron = EarningsHistoryCron::new(earnings_pool);
                if let Err(e) = earnings_cron.fetch_latest_hour().await {
                    error!("Failed to fetch earnings history: {}", e);
                }
                time::sleep(Duration::seconds(3).to_std().unwrap()).await;

                // Fetch swap history
                let swap_pool = self.pool.clone();
                let mut swap_cron = SwapHistoryCron::new(swap_pool);
                if let Err(e) = swap_cron.fetch_latest_hour().await {
                    error!("Failed to fetch swap history: {}", e);
                }
                time::sleep(Duration::seconds(3).to_std().unwrap()).await;

                // Fetch runepool units history
                let runepool_pool = self.pool.clone();
                let mut runepool_cron = RunepoolUnitsHistoryCron::new(runepool_pool);
                if let Err(e) = runepool_cron.fetch_latest_hour().await {
                    error!("Failed to fetch runepool units history: {}", e);
                }

                info!("Completed hourly data fetch cycle");
            }

            // Sleep for a minute before checking again
            time::sleep(Duration::minutes(1).to_std().unwrap()).await;
        }
    }
}
