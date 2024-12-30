use crate::core::models::common::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use crate::core::models::swap_history::SwapHistoryQueryParams;
use crate::core::models::swap_history::SwapHistoryResponse;
use crate::core::models::swap_history::SwapInterval;
use crate::core::models::swap_history::SwapMeta;
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
    path = "/swap_history",
    operation_id = "get_swap_history",
    tag = "swap",
    params(
        ("date_range" = Option<String>, Query, description = "Date range in format YYYY-MM-DD,YYYY-MM-DD"),
        ("page" = Option<u32>, Query, description = "Page number. Default is `0`"),
        ("limit" = Option<u32>, Query, description = "Items per page. Default is `100`"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by. Default is `start_time`"),
        ("order" = Option<String>, Query, description = "Sort order (asc/desc). Default is `desc`"),
        ("volume_gt" = Option<u64>, Query, description = "Filter by minimum volume. Default is `0`"),
        ("fees_gt" = Option<u64>, Query, description = "Filter by minimum fees. Default is `0`")
    ),
    responses(
        (status = 200, description = "List of swap history intervals", body = SwapHistoryResponse),
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

            let meta_stats =
                if let (Some(first), Some(last)) = (intervals.first(), intervals.last()) {
                    SwapMeta {
                        average_slip: last.average_slip,
                        end_time: last.end_time,
                        from_trade_average_slip: last.from_trade_average_slip,
                        from_trade_count: last.from_trade_count,
                        from_trade_fees: last.from_trade_fees,
                        from_trade_volume: last.from_trade_volume,
                        from_trade_volume_usd: last.from_trade_volume_usd,
                        rune_price_usd: last.rune_price_usd,
                        start_time: first.start_time,
                        synth_mint_average_slip: last.synth_mint_average_slip,
                        synth_mint_count: last.synth_mint_count,
                        synth_mint_fees: last.synth_mint_fees,
                        synth_mint_volume: last.synth_mint_volume,
                        synth_mint_volume_usd: last.synth_mint_volume_usd,
                        synth_redeem_average_slip: last.synth_redeem_average_slip,
                        synth_redeem_count: last.synth_redeem_count,
                        synth_redeem_fees: last.synth_redeem_fees,
                        synth_redeem_volume: last.synth_redeem_volume,
                        synth_redeem_volume_usd: last.synth_redeem_volume_usd,
                        to_asset_average_slip: last.to_asset_average_slip,
                        to_asset_count: last.to_asset_count,
                        to_asset_fees: last.to_asset_fees,
                        to_asset_volume: last.to_asset_volume,
                        to_asset_volume_usd: last.to_asset_volume_usd,
                        to_rune_average_slip: last.to_rune_average_slip,
                        to_rune_count: last.to_rune_count,
                        to_rune_fees: last.to_rune_fees,
                        to_rune_volume: last.to_rune_volume,
                        to_rune_volume_usd: last.to_rune_volume_usd,
                        to_trade_average_slip: last.to_trade_average_slip,
                        to_trade_count: last.to_trade_count,
                        to_trade_fees: last.to_trade_fees,
                        to_trade_volume: last.to_trade_volume,
                        to_trade_volume_usd: last.to_trade_volume_usd,
                        total_count: last.total_count,
                        total_fees: last.total_fees,
                        total_volume: last.total_volume,
                        total_volume_usd: last.total_volume_usd,
                    }
                } else {
                    return Json(json!({
                        "success": true,
                        "data": "no data found"
                    }))
                    .into_response();
                };

            let response = SwapHistoryResponse {
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
