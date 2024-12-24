#![allow(unused)]

use axum::{routing::get, Router};
use chrono::Utc;
use config::connect;
use dotenv::dotenv;
use handlers::{depth_history::get_depth_history, earning_history::get_earnings_history};
use http::Method;
use model::{
    common::Interval,
    depth_history::{DepthHistoryParams, DepthHistoryResponse},
    earnings_history::{EarningsHistoryParams, EarningsHistoryResponse},
    runepool_units_history::{RunepoolUnitsHistoryParams, RunepoolUnitsHistoryResponse},
    swap_history::{SwapHistoryParams, SwapHistoryResponse},
};
use reqwest::Client;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod handlers;
mod model;
mod services;

/* ************************************************************ */
/* ************************************************************ */
// !NOTE: PLEASE FETCH THINGS ONE BY ONE BECAUSE OF RATE LIMITS
/* ************************************************************ */
/* ************************************************************ */
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Database url issue");
    let pool = connect::connect_database(&database_url)
        .await
        .expect("Failed to connect to database");

    setup_tracing();

    tracing::info!("Connected to database...");
    println!("Current Utc TimeStamp: {:?}", Utc::now().timestamp());

    // !NOTE: Uncomment this if you want to fetch initial data and read the comment above the main
    // spawn_cron_jobs(pool.clone());
    // fetch_initial_data(pool.clone()).await;

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

fn spawn_cron_jobs(pool: sqlx::MySqlPool) {
    let depth_pool = pool.clone();
    tokio::spawn(async move {
        let mut depth_cron = services::cron::DepthHistoryCron::new(depth_pool);
        if let Err(e) = depth_cron.start().await {
            tracing::error!("Depth history cron failed: {}", e);
        }
    });

    let earnings_pool = pool.clone();
    tokio::spawn(async move {
        let mut earnings_cron = services::cron::EarningsHistoryCron::new(earnings_pool);
        if let Err(e) = earnings_cron.start().await {
            tracing::error!("Earnings history cron failed: {}", e);
        }
    });

    let swap_pool = pool.clone();
    tokio::spawn(async move {
        let mut swap_cron = services::cron::SwapHistoryCron::new(swap_pool);
        if let Err(e) = swap_cron.start().await {
            tracing::error!("Swap history cron failed: {}", e);
        }
    });

    let runepool_pool = pool.clone();
    tokio::spawn(async move {
        let mut runepool_cron = services::cron::RunepoolUnitsHistoryCron::new(runepool_pool);
        if let Err(e) = runepool_cron.start().await {
            tracing::error!("Runepool units history cron failed: {}", e);
        }
    });
}

async fn fetch_initial_data(pool: sqlx::MySqlPool) {
    tracing::info!("Starting initial data fetch...");

    fetch_and_store_depth_history(&pool).await;
    fetch_and_store_earnings_history(&pool).await;
    fetch_and_store_swap_history(&pool).await;
    fetch_and_store_runepool_units_history(&pool).await;
}

async fn fetch_and_store_depth_history(pool: &sqlx::MySqlPool) {
    tracing::info!("Fetching initial depth history...");
    match fetch_initial_depth_history().await {
        Ok(initial_data) => {
            tracing::info!("Successfully fetched initial depth history");
            match services::depth_history::store_intervals(pool, &initial_data.intervals).await {
                Ok(_) => tracing::info!(
                    "Successfully stored {} intervals",
                    initial_data.intervals.len()
                ),
                Err(e) => tracing::error!("Failed to store intervals: {}", e),
            }
        }
        Err(e) => tracing::error!("Failed to fetch initial depth history: {}", e),
    }
}

async fn fetch_and_store_earnings_history(pool: &sqlx::MySqlPool) {
    tracing::info!("Fetching initial earnings history...");
    match fetch_initial_earnings_history().await {
        Ok(initial_data) => {
            tracing::info!("Successfully fetched initial earnings history");
            match services::earnings_history::store_intervals(pool, &initial_data.intervals).await {
                Ok(_) => tracing::info!(
                    "Successfully stored {} intervals",
                    initial_data.intervals.len()
                ),
                Err(e) => tracing::error!("Failed to store intervals: {}", e),
            }
        }
        Err(e) => tracing::error!("Failed to fetch initial earnings history: {}", e),
    }
}

async fn fetch_and_store_swap_history(pool: &sqlx::MySqlPool) {
    tracing::info!("Fetching initial swap history...");
    match fetch_initial_swap_history().await {
        Ok(initial_data) => {
            tracing::info!("Successfully fetched initial swap history");
            match services::swap_history::store_intervals(pool, &initial_data.intervals).await {
                Ok(_) => tracing::info!(
                    "Successfully stored {} intervals",
                    initial_data.intervals.len()
                ),
                Err(e) => tracing::error!("Failed to store intervals: {}", e),
            }
        }
        Err(e) => tracing::error!("Failed to fetch initial swap history: {}", e),
    }
}

async fn fetch_and_store_runepool_units_history(pool: &sqlx::MySqlPool) {
    tracing::info!("Fetching initial runepool units history...");
    match fetch_initial_runepool_units_history().await {
        Ok(initial_data) => {
            tracing::info!("Successfully fetched initial runepool units history");
            match services::runepool_units_history::store_intervals(pool, &initial_data.intervals)
                .await
            {
                Ok(_) => tracing::info!(
                    "Successfully stored {} intervals",
                    initial_data.intervals.len()
                ),
                Err(e) => tracing::error!("Failed to store intervals: {}", e),
            }
        }
        Err(e) => tracing::error!("Failed to fetch initial runepool units history: {}", e),
    }
}

async fn start_server(pool: sqlx::MySqlPool) {
    let app = Router::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
        ]))
        .route("/", get(root))
        .route("/depth_history", get(get_depth_history))
        .route("/earning_history", get(get_earnings_history))
        .with_state(pool);

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

async fn fetch_initial_depth_history() -> Result<DepthHistoryResponse, reqwest::Error> {
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

async fn fetch_initial_earnings_history() -> Result<EarningsHistoryResponse, reqwest::Error> {
    let client = Client::new();

    let params = EarningsHistoryParams {
        interval: Some(Interval::Hour),
        count: Some(400),
        from: None,
        to: Some(Utc::now()),
    };

    let mut url = reqwest::Url::parse("https://midgard.ninerealms.com/v2/history/earnings")
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

    let earnings_history = response.json::<EarningsHistoryResponse>().await?;
    Ok(earnings_history)
}

async fn fetch_initial_swap_history() -> Result<SwapHistoryResponse, reqwest::Error> {
    let client = Client::new();

    let params = SwapHistoryParams {
        interval: Some(Interval::Hour),
        count: Some(400),
        from: None,
        to: Some(Utc::now()),
    };

    let mut url = reqwest::Url::parse("https://midgard.ninerealms.com/v2/history/swaps")
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

    let swap_history = response.json::<SwapHistoryResponse>().await?;
    Ok(swap_history)
}

async fn fetch_initial_runepool_units_history(
) -> Result<RunepoolUnitsHistoryResponse, reqwest::Error> {
    let client = Client::new();

    let params = RunepoolUnitsHistoryParams {
        interval: Some(Interval::Hour),
        count: Some(400),
        from: None,
        to: Some(Utc::now()),
    };

    let mut url = reqwest::Url::parse("https://midgard.ninerealms.com/v2/history/runepool")
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

    let runepool_units_history = response.json::<RunepoolUnitsHistoryResponse>().await?;
    Ok(runepool_units_history)
}
