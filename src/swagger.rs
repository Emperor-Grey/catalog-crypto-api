use utoipa::OpenApi;

// !I don't know why but the this is working but i need to import the __path_ to make it work wise words from the compiler
use crate::handlers::depth_history::{__path_get_depth_history, get_depth_history};
use crate::handlers::earning_history::{__path_get_earnings_history, get_earnings_history};
use crate::handlers::runepool_unit_history::{
    __path_get_runepool_units_history, get_runepool_units_history,
};
use crate::handlers::swap_history::{__path_get_swap_history, get_swap_history};
use crate::model::{
    depth_history::DepthHistoryResponse, earnings_history::EarningsHistoryResponse,
    runepool_units_history::RunepoolUnitsHistoryResponse, swap_history::SwapHistoryResponse,
};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Crypto History API",
        version = "1.0.0",
        description = "API for retrieving historical cryptocurrency data including depth, swaps, earnings, and runepool units",
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
        (url = "http://localhost:3000", description = "Local development server"),
        // (url = "https://api.production.example.com", description = "Production server")
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
