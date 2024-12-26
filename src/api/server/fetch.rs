use super::{
    depth_history::fetch_initial_depth_history, earning_history::fetch_initial_earnings_history,
    runepool_units_history::fetch_initial_runepool_units_history,
    swap_history::fetch_initial_swap_history,
};
use crate::services::repository::{depth, earnings, runepool, swap};

pub async fn fetch_and_store_depth_history(pool: &sqlx::MySqlPool) {
    tracing::info!("Fetching initial depth history...");
    match fetch_initial_depth_history().await {
        Ok(initial_data) => {
            tracing::info!("Successfully fetched initial depth history");
            match depth::store_intervals(pool, &initial_data.intervals).await {
                Ok(_) => tracing::info!(
                    "Successfully stored {} intervals",
                    initial_data.intervals.len()
                ),
                Err(e) => tracing::error!("Failed to store intervals: {}", e),
            }
        }
        Err(e) => tracing::error!("Failed to fetch initial depth history: {}", e),
    }
}

pub async fn fetch_and_store_earnings_history(pool: &sqlx::MySqlPool) {
    tracing::info!("Fetching initial earnings history...");
    match fetch_initial_earnings_history().await {
        Ok(initial_data) => {
            tracing::info!("Successfully fetched initial earnings history");
            match earnings::store_intervals(pool, &initial_data.intervals).await {
                Ok(_) => tracing::info!(
                    "Successfully stored {} intervals",
                    initial_data.intervals.len()
                ),
                Err(e) => tracing::error!("Failed to store intervals: {}", e),
            }
        }
        Err(e) => tracing::error!("Failed to fetch initial earnings history: {}", e),
    }
}

pub async fn fetch_and_store_swap_history(pool: &sqlx::MySqlPool) {
    tracing::info!("Fetching initial swap history...");
    match fetch_initial_swap_history().await {
        Ok(initial_data) => {
            tracing::info!("Successfully fetched initial swap history");
            match swap::store_intervals(pool, &initial_data.intervals).await {
                Ok(_) => tracing::info!(
                    "Successfully stored {} intervals",
                    initial_data.intervals.len()
                ),
                Err(e) => tracing::error!("Failed to store intervals: {}", e),
            }
        }
        Err(e) => tracing::error!("Failed to fetch initial swap history: {}", e),
    }
}

pub async fn fetch_and_store_runepool_units_history(pool: &sqlx::MySqlPool) {
    tracing::info!("Fetching initial runepool units history...");
    match fetch_initial_runepool_units_history().await {
        Ok(initial_data) => {
            tracing::info!("Successfully fetched initial runepool units history");
            match runepool::store_intervals(pool, &initial_data.intervals).await {
                Ok(_) => tracing::info!(
                    "Successfully stored {} intervals",
                    initial_data.intervals.len()
                ),
                Err(e) => tracing::error!("Failed to store intervals: {}", e),
            }
        }
        Err(e) => tracing::error!("Failed to fetch initial runepool units history: {}", e),
    }
}
