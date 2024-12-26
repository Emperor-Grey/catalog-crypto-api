use api::routes::depth::get_depth_history;
use api::routes::earnings::get_earnings_history;
use api::routes::runepool::get_runepool_units_history;
use api::routes::swap::get_swap_history;
use api::server::fetch::{
    fetch_and_store_depth_history, fetch_and_store_earnings_history,
    fetch_and_store_runepool_units_history, fetch_and_store_swap_history,
};
use axum::{routing::get, Router};
use chrono::Utc;
use config::connect;
use dotenv::dotenv;
use http::Method;
use services::{
    client::get_midgard_api_url, jobs::cron::hourly_fetcher::HourlyFetcher, spawn::spawn_cron_jobs,
};
use std::env;
use std::net::SocketAddr;
use swagger::SwaggerApiDoc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod config;
mod core;
mod services;

mod swagger;

/* ************************************************************ */
/* ************************************************************ */
/* !NOTE: PLEASE FETCH THINGS ONE BY ONE BECAUSE OF RATE LIMITS */
/* ************************************************************ */
/* ************************************************************ */
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Database url issue");

    tracing::info!(
        "Env variables are \n{}\n{}",
        get_midgard_api_url(),
        database_url
    );

    let pool = connect::connect_database(&database_url)
        .await
        .expect("Failed to connect to database");

    setup_tracing();

    tracing::info!("Connected to database...");
    println!("Current Utc TimeStamp: {:?}", Utc::now().timestamp());

    // !NOTE: Uncomment this if you want to fetch initial data and read the comment above the main
    // spawn_cron_jobs(pool.clone());
    // fetch_initial_data(pool.clone()).await;

    let hourly_pool = pool.clone();
    tokio::spawn(async move {
        let mut hourly_fetcher = HourlyFetcher::new(hourly_pool);
        if let Err(e) = hourly_fetcher.start().await {
            tracing::error!("Hourly fetcher failed: {}", e);
        }
    });

    start_server(pool).await;
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn fetch_initial_data(pool: sqlx::MySqlPool) {
    tracing::info!("Starting initial data fetch...");

    fetch_and_store_depth_history(&pool).await;
    fetch_and_store_earnings_history(&pool).await;
    fetch_and_store_swap_history(&pool).await;
    fetch_and_store_runepool_units_history(&pool).await;
}

async fn start_server(pool: sqlx::MySqlPool) {
    let app = Router::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
        ]))
        .route("/depth_history", get(get_depth_history))
        .route("/earning_history", get(get_earnings_history))
        .route("/swap_history", get(get_swap_history))
        .route("/runepool_units_history", get(get_runepool_units_history))
        .with_state(pool)
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", SwaggerApiDoc::openapi()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
