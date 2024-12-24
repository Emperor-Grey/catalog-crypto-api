use axum::http::StatusCode;
use axum::Json;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::prelude::FromRow;
use sqlx::MySqlPool;
use tracing::{debug, error, info};

use crate::model::{
    common::{EarningsHistoryQueryParams, DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE},
    earnings_history::Pool,
};

#[derive(Debug, FromRow)]
struct EarningIntervalDB {
    pub avg_node_count: f64,
    pub block_rewards: u64,
    pub bonding_earnings: u64,
    pub earnings: u64,
    pub end_time: DateTime<Utc>,
    pub liquidity_earnings: u64,
    pub liquidity_fees: u64,
    pub rune_price_usd: f64,
    pub start_time: DateTime<Utc>,
    pub pools: JsonValue,
}

#[derive(Debug, Serialize)]
struct IntervalResponse {
    #[serde(rename = "avgNodeCount")]
    avg_node_count: f64,
    #[serde(rename = "blockRewards")]
    block_rewards: String,
    #[serde(rename = "bondingEarnings")]
    bonding_earnings: String,
    earnings: String,
    #[serde(rename = "endTime")]
    end_time: i64,
    #[serde(rename = "liquidityEarnings")]
    liquidity_earnings: String,
    #[serde(rename = "liquidityFees")]
    liquidity_fees: String,
    #[serde(rename = "runePriceUSD")]
    rune_price_usd: f64,
    #[serde(rename = "startTime")]
    start_time: i64,
    pools: Vec<Pool>,
}

pub async fn get_earnings_history(
    State(pool): State<MySqlPool>,
    Query(params): Query<EarningsHistoryQueryParams>,
) -> impl IntoResponse {
    info!(
        "Received earnings history request with params: {:#?}",
        params
    );

    let limit = params.limit.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = params.page.unwrap_or(0) * limit;
    debug!("Using limit: {}, offset: {}", limit, offset);

    let mut query = sqlx::QueryBuilder::new("SELECT * FROM `earning_intervals` WHERE 1=1");

    // Add filters
    if let Some((start, end)) = params.parse_date_range() {
        debug!("Date range filter: start={}, end={}", start, end);
        query
            .push(" AND start_time >= ")
            .push_bind(start)
            .push(" AND end_time <= ")
            .push_bind(end);
    }

    if let Some(min_earnings) = params.earnings_gt {
        debug!("Earnings filter: > {}", min_earnings);
        query.push(" AND earnings > ").push_bind(min_earnings);
    }

    if let Some(min_rewards) = params.block_rewards_gt {
        debug!("Block rewards filter: > {}", min_rewards);
        query.push(" AND block_rewards > ").push_bind(min_rewards);
    }

    if let Some(min_nodes) = params.node_count_gt {
        debug!("Node count filter: > {}", min_nodes);
        query.push(" AND avg_node_count > ").push_bind(min_nodes);
    }

    if let Some(pool_name) = &params.pool {
        debug!("Pool filter: {}", pool_name);
        query.push(" AND JSON_SEARCH(pools, 'one', ?)");
        query.push_bind(format!("%{}%", pool_name));
    }

    // Add sorting
    let sort_field = params.get_sort_field();
    let sort_order = if params.order.as_deref() == Some("desc") {
        "DESC"
    } else {
        "ASC"
    };
    debug!("Sorting by {} {}", sort_field, sort_order);
    query
        .push(" ORDER BY ")
        .push(sort_field)
        .push(" ")
        .push(sort_order);

    query.push(" LIMIT ").push_bind(limit as i64);
    query.push(" OFFSET ").push_bind(offset as i64);

    let query_string = query.sql();
    debug!("Executing query: {}", query_string);

    match query
        .build_query_as::<EarningIntervalDB>()
        .fetch_all(&pool)
        .await
    {
        Ok(db_intervals) => {
            let intervals: Result<Vec<IntervalResponse>, serde_json::Error> = db_intervals
                .into_iter()
                .map(|db| {
                    // Parse the JSON value into Vec<Pool>
                    let pools: Vec<Pool> = serde_json::from_value(db.pools)?;

                    Ok(IntervalResponse {
                        start_time: db.start_time.timestamp(),
                        end_time: db.end_time.timestamp(),
                        avg_node_count: db.avg_node_count,
                        block_rewards: db.block_rewards.to_string(),
                        bonding_earnings: db.bonding_earnings.to_string(),
                        earnings: db.earnings.to_string(),
                        liquidity_earnings: db.liquidity_earnings.to_string(),
                        liquidity_fees: db.liquidity_fees.to_string(),
                        rune_price_usd: db.rune_price_usd,
                        pools,
                    })
                })
                .collect();

            match intervals {
                Ok(intervals) => {
                    info!(
                        "Successfully retrieved {} earnings intervals",
                        intervals.len()
                    );
                    Json(intervals).into_response()
                }
                Err(e) => {
                    error!("Error parsing intervals: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Error parsing JSON pools data: {}", e),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            error!("Database error when fetching earnings intervals: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}
