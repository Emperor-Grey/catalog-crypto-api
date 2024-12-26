// !I don't know why but the this is working but i need to import the __path_ to make it work wise words from the compiler
use crate::api::routes::depth::__path_get_depth_history;
use crate::api::routes::earnings::__path_get_earnings_history;
use crate::api::routes::runepool::__path_get_runepool_units_history;
use crate::api::routes::swap::__path_get_swap_history;
use crate::core::models::{
    depth_history::DepthHistoryResponse, earnings_history::EarningsHistoryResponse,
    runepool_units_history::RunepoolUnitsHistoryResponse, swap_history::SwapHistoryResponse,
};

// ! Don't format the description it will break the swagger ui description it looks better this way
#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Crypto History API",
        version = "1.0.1",
        description = "A comprehensive REST API for fetching and managing historical cryptocurrency data from the THORChain network (midgard).

    Features include:
        - Depth history tracking for liquidity pools
        - Swap transaction analytics and metrics
        - Network earnings data across different pools
        - Runepool units historical data

'The API supports pagination, filtering, sorting, and date range queries. Data is continuously synchronized through background cron jobs with rate limiting protection. Built with Rust using Axum framework and MySQL for persistence.'
",
        contact(
            name = "API Support",
            email = "lohitsaidev@gmail.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "https://catalog-crypto-api.onrender.com/", description = "Production server"),
        (url = "http://localhost:3000/", description = "Local development server")
    ),
    tags(
        (name = "depth", description = "Depth history operations"),
        (name = "swap", description = "Swap history operations"),
        (name = "earnings", description = "Earnings history operations"),
        (name = "runepool", description = "Runepool units history operations")
    ),
    paths(
        get_depth_history,
        get_swap_history,
        get_runepool_units_history,
        get_earnings_history
    ),
    components(
        schemas(
            DepthHistoryResponse,
            SwapHistoryResponse,
            RunepoolUnitsHistoryResponse,
            EarningsHistoryResponse
        )
    ),
    // modifiers(&SecurityAddon)
)]
pub struct SwaggerApiDoc;

// struct SecurityAddon;

// impl utoipa::Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         if let Some(components) = openapi.components.as_mut() {
//             components.add_security_scheme(
//                 "api_key",
//                 utoipa::openapi::security::SecurityScheme::ApiKey(
//                     utoipa::openapi::security::ApiKey::Header(
//                         utoipa::openapi::security::ApiKeyValue::new("x-api-key"),
//                     ),
//                 ),
//             );
//         }
//     }
// }
