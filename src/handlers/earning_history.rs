use crate::model::common::{EarningsHistoryQueryParams, DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use tracing::{debug, error, info};

// !Did this because umm... I'm not sure how to do this with the structs containing the Vec<Pool> struct
// !So I broken it down into two structs storing pool in another table but referencing the interval id
#[derive(FromRow, Serialize, Deserialize)]
struct EarningIntervalDB {
    id: i64,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    avg_node_count: f64,
    block_rewards: i64,
    bonding_earnings: i64,
    earnings: i64,
    liquidity_earnings: i64,
    liquidity_fees: i64,
    rune_price_usd: f64,
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

    let mut query = sqlx::QueryBuilder::new(
        "SELECT earning_intervals.*, earning_pool.*
         FROM earning_intervals
         LEFT JOIN pool_earnings ON earning_intervals.id = earning_pool.interval_id
         WHERE 1=1",
    );

    // Handle date range
    if let Some((start, end)) = params.parse_date_range() {
        debug!("Date range filter: start={}, end={}", start, end);
        query.push(" AND start_time >= ");
        query.push_bind(start.naive_utc());
        query.push(" AND end_time <= ");
        query.push_bind(end.naive_utc());
    } else {
        debug!("No date range provided or invalid format");
    }

    // Handle earnings filter
    if let Some(min_earnings) = params.earnings_gt {
        debug!("Earnings filter: > {}", min_earnings);
        query.push(" AND earnings > ");
        query.push_bind(min_earnings);
    }

    // Handle block rewards filter
    if let Some(min_rewards) = params.block_rewards_gt {
        debug!("Block rewards filter: > {}", min_rewards);
        query.push(" AND block_rewards > ");
        query.push_bind(min_rewards);
    }

    // Handle node count filter
    if let Some(min_nodes) = params.node_count_gt {
        debug!("Node count filter: > {}", min_nodes);
        query.push(" AND avg_node_count > ");
        query.push_bind(min_nodes);
    }

    // Handle pool filter
    if let Some(pool_name) = &params.pool {
        debug!("Pool filter: {}", pool_name);
        query.push(" AND EXISTS (SELECT 1 FROM JSON_TABLE(pools, '$[*]' COLUMNS (pool VARCHAR(255) PATH '$.pool')) as p WHERE p.pool = ");
        query.push_bind(pool_name);
        query.push(")");
    }

    // Handle sorting
    let sort_field = params.get_sort_field();
    let sort_order = if params.order.as_deref() == Some("desc") {
        "DESC"
    } else {
        "ASC"
    };
    debug!("Sorting by {} {}", sort_field, sort_order);
    query.push(" ORDER BY ");
    query.push(sort_field);
    query.push(" ");
    query.push(sort_order);

    // Add pagination
    query.push(" LIMIT ");
    query.push_bind(limit);
    query.push(" OFFSET ");
    query.push_bind(offset);

    // Execute query and return results
    match query
        .build_query_as::<EarningIntervalDB>()
        .fetch_all(&pool)
        .await
    {
        Ok(intervals) => {
            info!(
                "Successfully retrieved {} earnings intervals",
                intervals.len()
            );
            Json(intervals).into_response()
        }
        Err(e) => {
            error!("Database error when fetching earnings intervals: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}
