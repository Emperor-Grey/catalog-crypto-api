use crate::core::models::common::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use crate::core::models::depth_history::DepthHistoryQueryParams;
use crate::core::models::depth_history::DepthInterval;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::MySqlPool;
use tracing::{debug, error, info};

#[utoipa::path(
    get,
    path = "/depth_history",
    operation_id = "get_depth_history",
    tag = "depth",
    params(
        ("date_range" = Option<String>, Query, description = "Date range in format YYYY-MM-DD,YYYY-MM-DD"),
        ("liquidity_gt" = Option<u64>, Query, description = "Filter by minimum liquidity"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by"),
        ("order" = Option<String>, Query, description = "Sort order (asc/desc)"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of depth history intervals", body = Vec<DepthInterval>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_depth_history(
    State(pool): State<MySqlPool>,
    Query(params): Query<DepthHistoryQueryParams>,
) -> impl IntoResponse {
    info!("Received depth history request with params: {:#?}", params);

    let limit = params.limit.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = params.page.unwrap_or(0) * limit;
    debug!("Using limit: {}, offset: {}", limit, offset);

    let mut query = sqlx::QueryBuilder::new("SELECT * FROM `depth_intervals` WHERE 1=1");

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

    // Handle liquidity filter
    if let Some(min_liquidity) = params.liquidity_gt {
        debug!("Liquidity filter: > {}", min_liquidity);
        query.push(" AND liquidity_units > ");
        query.push_bind(min_liquidity);
    }

    // TODO // Handle interval next time if you can
    // if let Some(interval) = &params.interval {
    //     debug!("Interval filter: {}", interval);
    //     query.push(" AND `interval` = ");
    //     query.push_bind(interval.to_string());
    // }

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
    query.push(if params.order.as_deref() == Some("desc") {
        " DESC"
    } else {
        " ASC"
    });

    // Add pagination
    query.push(" LIMIT ");
    query.push_bind(limit);
    query.push(" OFFSET ");
    query.push_bind(offset);

    let query_sql = query.sql();
    debug!("Executing SQL query: {}", query_sql);

    match query
        .build_query_as::<DepthInterval>()
        .fetch_all(&pool)
        .await
    {
        Ok(intervals) => {
            info!("Successfully retrieved {} depth intervals", intervals.len());

            if intervals.is_empty() {
                return Json(json!({
                    "success": true,
                    "data": "no data found in the database for the given params"
                }))
                .into_response();
            }

            Json(intervals).into_response()
        }
        Err(e) => {
            error!("Database error when fetching depth intervals: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Database error: {}", e)
                })),
            )
                .into_response()
        }
    }
}
