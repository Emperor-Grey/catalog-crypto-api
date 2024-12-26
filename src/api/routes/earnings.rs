use crate::core::models::common::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use crate::core::models::earnings_history::{EarningsHistoryQueryParams, Pool};
use crate::core::models::earnings_history::{EarningsHistoryResponse, IntervalData, MetaStats};
use axum::http::StatusCode;
use axum::Json;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde_json::json;
use serde_json::Value as JsonValue;
use sqlx::prelude::FromRow;
use sqlx::MySqlPool;
use tracing::{debug, error, info};
use utoipa::ToSchema;

// !Just cuz in the models, we have intervalData which contains Vec<Pool> and rust don't know how to deserialize it
// !So we need to create a new struct to deserialize it (Only solution i found)
#[derive(Debug, FromRow, ToSchema, Clone)]
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

// #[derive(Debug, Serialize, ToSchema)]
// struct IntervalResponse {
//     #[serde(rename = "avgNodeCount")]
//     avg_node_count: f64,
//     #[serde(rename = "blockRewards")]
//     block_rewards: String,
//     #[serde(rename = "bondingEarnings")]
//     bonding_earnings: String,
//     earnings: String,
//     #[serde(rename = "endTime")]
//     end_time: i64,
//     #[serde(rename = "liquidityEarnings")]
//     liquidity_earnings: String,
//     #[serde(rename = "liquidityFees")]
//     liquidity_fees: String,
//     #[serde(rename = "runePriceUSD")]
//     rune_price_usd: f64,
//     #[serde(rename = "startTime")]
//     start_time: i64,
//     pools: Vec<Pool>,
// }

#[utoipa::path(
    get,
    operation_id = "get_earnings_history",
    path = "/earning_history",
    tag = "earnings",
    params(
        ("date_range" = Option<String>, Query, description = "Date range in format YYYY-MM-DD,YYYY-MM-DD"),
        ("earnings_gt" = Option<u64>, Query, description = "Filter by minimum earnings. Default is `0`"),
        ("block_rewards_gt" = Option<u64>, Query, description = "Filter by minimum block rewards. Default is `0`"),
        ("node_count_gt" = Option<u64>, Query, description = "Filter by minimum node count. Default is `0`"),
        ("pool" = Option<String>, Query, description = "Filter by pool,(only returns data that contain the given pool name in the pools array)"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by. Default is `start_time`"),
        ("order" = Option<String>, Query, description = "Sort order (asc/desc). Default is `desc`"),
        ("page" = Option<u32>, Query, description = "Page number. Default is `0`"),
        ("limit" = Option<u32>, Query, description = "Items per page. Default is `100`")
    ),
    responses(
        (status = 200, description = "List of earnings history intervals", body = EarningsHistoryResponse),
        (status = 500, description = "Internal server error")
    )
)]
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
        query.push(" AND JSON_CONTAINS(pools, JSON_ARRAY(JSON_OBJECT('pool', ");
        query.push_bind(pool_name);
        query.push(")))");
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
            if db_intervals.is_empty() {
                return Json(json!({
                    "success": true,
                    "data": "no data found in the database for the given params"
                }))
                .into_response();
            }

            let intervals: Result<Vec<IntervalData>, serde_json::Error> = db_intervals
                .iter()
                .map(|db| {
                    let pools: Vec<Pool> = serde_json::from_value(db.pools.clone())?;

                    Ok(IntervalData {
                        start_time: db.start_time,
                        end_time: db.end_time,
                        avg_node_count: db.avg_node_count,
                        block_rewards: db.block_rewards,
                        bonding_earnings: db.bonding_earnings,
                        earnings: db.earnings,
                        liquidity_earnings: db.liquidity_earnings,
                        liquidity_fees: db.liquidity_fees,
                        rune_price_usd: db.rune_price_usd,
                        pools,
                    })
                })
                .collect();

            match intervals {
                Ok(intervals) => {
                    // Calculate meta statistics
                    let meta_stats = if let (Some(_first), Some(last)) =
                        (db_intervals.first(), db_intervals.last())
                    {
                        MetaStats {
                            avg_node_count: last.avg_node_count,
                            block_rewards: last.block_rewards,
                            bonding_earnings: last.bonding_earnings,
                            earnings: last.earnings,
                            end_time: last.end_time,
                            liquidity_earnings: last.liquidity_earnings,
                            liquidity_fees: last.liquidity_fees,
                            pools: serde_json::from_value(last.pools.clone()).unwrap_or_default(),
                        }
                    } else {
                        return Json(json!({
                            "success": true,
                            "data": "no data found"
                        }))
                        .into_response();
                    };

                    let response = EarningsHistoryResponse {
                        intervals,
                        meta_stats,
                    };

                    Json(response).into_response()
                }
                Err(e) => {
                    error!("Error parsing intervals: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "success": false,
                            "error": format!("Error parsing JSON pools data: {}", e)
                        })),
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
