// #![allow(unused, dead_code)]

use axum::{routing::get, Router};
use chrono::Utc;
use database::connect;
use dotenv::dotenv;
use http::Method;
use model::depth_history::{DepthHistoryParams, DepthHistoryResponse, Interval};
use reqwest::Client;
use services::depth_history::store_intervals;
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

    let database_url = std::env::var("DATABASE_URL").expect("Database url issue");
    let pool = connect::connect_database(&database_url)
        .await
        .expect("Failed to connect to database");

    // Start the depth history cron job
    // let cron_pool = pool.clone();
    // tokio::spawn(async move {
    //     let mut cron = services::cron::DepthHistoryCron::new(cron_pool);
    //     cron.start().await;
    // });

    // Set up tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Connected to database...");
    println!("{:?}", Utc::now().timestamp());

    // Fetch and store initial data
    let initial_data = fetch_initial_data().await.unwrap();
    match store_intervals(&pool, &initial_data.intervals).await {
        Ok(_) => tracing::info!(
            "Successfully stored {} intervals",
            initial_data.intervals.len()
        ),
        Err(e) => tracing::error!("Failed to store intervals: {}", e),
    }

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

    let params = DepthHistoryParams {
        interval: Some(Interval::Hour),
        count: Some(400),
        from: None,
        to: Some(Utc::now()),
    };

    let mut url = reqwest::Url::parse("https://midgard.ninerealms.com/v2/history/depths/ETH.ETH")
        .expect("Failed to parse URL");

    if let Some(interval) = &params.interval {
        url.query_pairs_mut().append_pair(
            "interval",
            match interval {
                Interval::FiveMin => "5min",
                Interval::Hour => "hour",
                Interval::Day => "day",
                Interval::Week => "week",
                Interval::Month => "month",
                Interval::Quarter => "quarter",
                Interval::Year => "year",
            },
        );
    }

    if let Some(count) = params.count {
        url.query_pairs_mut()
            .append_pair("count", &count.to_string());
    }

    if let Some(from) = params.from {
        url.query_pairs_mut()
            .append_pair("from", &from.timestamp().to_string());
    }

    if let Some(to) = params.to {
        url.query_pairs_mut()
            .append_pair("to", &to.timestamp().to_string());
    }

    let response = client.get(url).send().await?;

    let depth_history = response.json::<DepthHistoryResponse>().await?;
    Ok(depth_history)
}
