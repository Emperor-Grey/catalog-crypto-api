use crate::core::models::common::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use crate::core::models::runepool_units_history::{
    MetaStats, RunepoolUnitsHistoryQueryParams, RunepoolUnitsHistoryResponse, RunepoolUnitsInterval,
};
use axum::http::StatusCode;

use axum::Json;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::MySqlPool;
use tracing::{debug, error, info};

#[utoipa::path(
    get,
    operation_id = "get_runepool_units_history",
    path = "/runepool_units_history",
    tag = "runepool",
    params(
        ("date_range" = Option<String>, Query, description = "Date range in format YYYY-MM-DD,YYYY-MM-DD"),
        ("units_gt" = Option<u64>, Query, description = "Filter by minimum units. Default is `0`"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by. Default is `start_time`"),
        ("order" = Option<String>, Query, description = "Sort order (asc/desc). Default is `desc`"),
        ("page" = Option<u32>, Query, description = "Page number. Default is `0`"),
        ("limit" = Option<u32>, Query, description = "Items per page. Default is `100`")
    ),
    responses(
        (status = 200, description = "List of runepool units history intervals", body = RunepoolUnitsHistoryResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_runepool_units_history(
    State(pool): State<MySqlPool>,
    Query(params): Query<RunepoolUnitsHistoryQueryParams>,
) -> impl IntoResponse {
    info!(
        "Received runepool units history request with params: {:#?}",
        params
    );

    let limit = params.limit.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = params.page.unwrap_or(0) * limit;
    debug!("Using limit: {}, offset: {}", limit, offset);

    let mut query = sqlx::QueryBuilder::new("SELECT * FROM `runepool_unit_intervals` WHERE 1=1");

    if let Some((start, end)) = params.parse_date_range() {
        debug!("Date range filter: start={}, end={}", start, end);
        query
            .push(" AND start_time >= ")
            .push_bind(start)
            .push(" AND end_time <= ")
            .push_bind(end);
    }

    if let Some(min_units) = params.units_gt {
        debug!("Units filter: > {}", min_units);
        query.push(" AND units > ").push_bind(min_units);
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
        .build_query_as::<RunepoolUnitsInterval>()
        .fetch_all(&pool)
        .await
    {
        Ok(intervals) => {
            info!(
                "Successfully retrieved {} runepool unit intervals",
                intervals.len()
            );

            if intervals.is_empty() {
                return Json(json!({
                    "success": true,
                    "data": "no data found in the database for the given params"
                }))
                .into_response();
            }

            let meta_stats =
                if let (Some(first), Some(last)) = (intervals.first(), intervals.last()) {
                    MetaStats {
                        start_time: first.start_time,
                        end_time: last.end_time,
                        start_count: first.count,
                        end_count: last.count,
                        start_units: first.units,
                        end_units: last.units,
                    }
                } else {
                    return Json(json!({
                        "success": true,
                        "data": "no data found in the database for the given params"
                    }))
                    .into_response();
                };

            let response = RunepoolUnitsHistoryResponse {
                intervals,
                meta_stats,
            };

            Json(response).into_response()
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
