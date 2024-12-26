use super::jobs::cron::{
    depth_history_cron::DepthHistoryCron, earnings_history_cron::EarningsHistoryCron,
    runepool_units_history_cron::RunepoolUnitsHistoryCron, swap_history_cron::SwapHistoryCron,
};

pub fn spawn_cron_jobs(pool: sqlx::MySqlPool) {
    let depth_pool = pool.clone();
    tokio::spawn(async move {
        let mut depth_cron = DepthHistoryCron::new(depth_pool);
        if let Err(e) = depth_cron.start().await {
            tracing::error!("Depth history cron failed: {}", e);
        }
    });

    let earnings_pool = pool.clone();
    tokio::spawn(async move {
        let mut earnings_cron = EarningsHistoryCron::new(earnings_pool);
        if let Err(e) = earnings_cron.start().await {
            tracing::error!("Earnings history cron failed: {}", e);
        }
    });

    let swap_pool = pool.clone();
    tokio::spawn(async move {
        let mut swap_cron = SwapHistoryCron::new(swap_pool);
        if let Err(e) = swap_cron.start().await {
            tracing::error!("Swap history cron failed: {}", e);
        }
    });

    let runepool_pool = pool.clone();
    tokio::spawn(async move {
        let mut runepool_cron = RunepoolUnitsHistoryCron::new(runepool_pool);
        if let Err(e) = runepool_cron.start().await {
            tracing::error!("Runepool units history cron failed: {}", e);
        }
    });
}
