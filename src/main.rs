// #![allow(unused, dead_code)]

use axum::{routing::get, Router};
use database::connect;
use dotenv::dotenv;
use http::Method;
use model::depth_history::DepthHistoryResponse;
use reqwest::Client;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod database;
pub mod handlers;
pub mod model;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let initial_data = fetch_initial_data().await.unwrap();
    println!("{:?}", initial_data);

    let database_url = std::env::var("DATABASE_URL").expect("Database url issue");
    let pool = connect::connect_database(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Connected to database...");

    let app = Router::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
        ]))
        .route("/", get(root))
        .with_state(pool);

    // Host and Port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn fetch_initial_data() -> Result<DepthHistoryResponse, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://midgard.ninerealms.com/v2/history/depths/ETH.ETH")
        .send()
        .await?;

    let depth_history = response.json::<DepthHistoryResponse>().await?;
    Ok(depth_history)
}
