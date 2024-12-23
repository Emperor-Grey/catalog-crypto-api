use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::time::Duration;

pub async fn connect_database(url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(url)
        .await
}
