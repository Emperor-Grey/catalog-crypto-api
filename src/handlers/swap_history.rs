use axum::http::StatusCode;
use axum::Json;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;
use sqlx::MySqlPool;
use tracing::{debug, error, info};

use crate::model::common::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use crate::model::swap_history::{SwapHistoryQueryParams, SwapInterval};

#[utoipa::path(
    get,
    path = "/swap_history",
    operation_id = "get_swap_history",
    tag = "swap",
    params(
        ("date_range" = Option<String>, Query, description = "Date range in format YYYY-MM-DD,YYYY-MM-DD"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by"),
        ("order" = Option<String>, Query, description = "Sort order (asc/desc)"),
        ("volume_gt" = Option<u64>, Query, description = "Filter by minimum volume"),
        ("fees_gt" = Option<u64>, Query, description = "Filter by minimum fees")
    ),
    responses(
        (status = 200, description = "List of swap history intervals", body = Vec<SwapInterval>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_swap_history(
    State(pool): State<MySqlPool>,
    Query(params): Query<SwapHistoryQueryParams>,
) -> impl IntoResponse {
    info!("Received swap history request with params: {:#?}", params);

    let limit = params.limit.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = params.page.unwrap_or(0) * limit;
    debug!("Using limit: {}, offset: {}", limit, offset);

    let mut query = sqlx::QueryBuilder::new("SELECT * FROM `swap_intervals` WHERE 1=1");

    if let Some((start, end)) = params.parse_date_range() {
        debug!("Date range filter: start={}, end={}", start, end);
        query
            .push(" AND start_time >= ")
            .push_bind(start)
            .push(" AND end_time <= ")
            .push_bind(end);
    }

    if let Some(min_volume) = params.volume_gt {
        debug!("Volume filter: > {}", min_volume);
        query.push(" AND total_volume > ").push_bind(min_volume);
    }

    if let Some(min_fees) = params.fees_gt {
        debug!("Fees filter: > {}", min_fees);
        query.push(" AND total_fees > ").push_bind(min_fees);
    }

    let sort_field = params.get_sort_field();
    let sort_order = if params.order.as_deref() == Some("desc") {
        "DESC"
    } else {
        "ASC"
    };

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
        .build_query_as::<SwapInterval>()
        .fetch_all(&pool)
        .await
    {
        Ok(intervals) => {
            info!("Successfully retrieved {} swap intervals", intervals.len());

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
            error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Database error: {}", e)
                })),
            )
                .into_response()
        }
    }
}
