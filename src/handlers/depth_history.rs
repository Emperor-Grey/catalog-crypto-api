use crate::model::{
    common::{DepthHistoryQueryParams, DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE},
    depth_history::DepthInterval,
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use sqlx::MySqlPool;
use tracing::{debug, error, info};

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

    // Handle interval
    if let Some(interval) = &params.interval {
        debug!("Interval filter: {}", interval);
        query.push(" AND interval = ");
        query.push_bind(interval.to_string());
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
            Json(intervals).into_response()
        }
        Err(e) => {
            error!("Database error when fetching depth intervals: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}
