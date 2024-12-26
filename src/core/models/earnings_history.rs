use chrono::{DateTime, TimeZone, Utc};
use prkorm::Table;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::common::Interval;

mod float_serialization {
    use serde::{de::Deserializer, ser::Serializer, Deserialize};

    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value_str = String::deserialize(deserializer)?;
        if value_str == "NaN" {
            return Ok(f64::NAN);
        }
        match value_str.parse::<f64>() {
            Ok(num) => Ok(num),
            Err(_) => value_str
                .trim()
                .replace(",", "")
                .parse::<f64>()
                .map_err(serde::de::Error::custom),
        }
    }
}

mod timestamp_serialization {
    use serde::{Deserializer, Serializer};

    use super::*;

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.timestamp().to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp_str = String::deserialize(deserializer)?;
        let timestamp = timestamp_str
            .parse::<i64>()
            .map_err(serde::de::Error::custom)?;
        Ok(Utc.timestamp_opt(timestamp, 0).unwrap())
    }
}

mod u64_serialization {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value_str = String::deserialize(deserializer)?;
        if let Ok(float_val) = value_str.trim().replace(",", "").parse::<f64>() {
            return Ok(float_val as u64);
        }
        value_str
            .trim()
            .replace(",", "")
            .parse::<u64>()
            .map_err(de::Error::custom)
    }
}

#[derive(Table, Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
#[table_name("`earning_pool`")]
pub struct Pool {
    #[serde(rename = "assetLiquidityFees", with = "u64_serialization")]
    pub asset_liquidity_fees: u64,
    #[serde(rename = "earnings", with = "u64_serialization")]
    pub earnings: u64,
    pub pool: String,
    #[serde(rename = "rewards", with = "u64_serialization")]
    pub rewards: u64,
    #[serde(rename = "runeLiquidityFees", with = "u64_serialization")]
    pub rune_liquidity_fees: u64,
    #[serde(rename = "saverEarning", with = "u64_serialization")]
    pub saver_earning: u64,
    #[serde(rename = "totalLiquidityFeesRune", with = "u64_serialization")]
    pub total_liquidity_fees_rune: u64,
}

#[derive(Table, Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
#[table_name("`earning_intervals`")]
#[serde(rename_all = "camelCase")]
#[derive(sqlx::Type)]
pub struct IntervalData {
    #[serde(rename = "avgNodeCount", with = "float_serialization")]
    pub avg_node_count: f64,
    #[serde(rename = "blockRewards", with = "u64_serialization")]
    pub block_rewards: u64,
    #[serde(rename = "bondingEarnings", with = "u64_serialization")]
    pub bonding_earnings: u64,
    #[serde(rename = "earnings", with = "u64_serialization")]
    pub earnings: u64,
    #[serde(rename = "endTime", with = "timestamp_serialization")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "liquidityEarnings", with = "u64_serialization")]
    pub liquidity_earnings: u64,
    #[serde(rename = "liquidityFees", with = "u64_serialization")]
    pub liquidity_fees: u64,
    #[serde(rename = "pools")]
    pub pools: Vec<Pool>,
    #[serde(rename = "runePriceUSD", with = "float_serialization")]
    pub rune_price_usd: f64,
    #[serde(rename = "startTime", with = "timestamp_serialization")]
    pub start_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MetaStats {
    #[serde(rename = "avgNodeCount", with = "float_serialization")]
    pub avg_node_count: f64,
    #[serde(rename = "blockRewards", with = "u64_serialization")]
    pub block_rewards: u64,
    #[serde(rename = "bondingEarnings", with = "u64_serialization")]
    pub bonding_earnings: u64,
    #[serde(rename = "earnings", with = "u64_serialization")]
    pub earnings: u64,
    #[serde(rename = "endTime", with = "timestamp_serialization")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "liquidityEarnings", with = "u64_serialization")]
    pub liquidity_earnings: u64,
    #[serde(rename = "liquidityFees", with = "u64_serialization")]
    pub liquidity_fees: u64,
    #[serde(rename = "pools")]
    pub pools: Vec<Pool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EarningsHistoryResponse {
    pub intervals: Vec<IntervalData>,
    #[serde(rename = "meta")]
    pub meta_stats: MetaStats,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EarningsHistoryParams {
    pub interval: Option<Interval>,
    pub count: Option<u32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EarningsHistoryQueryParams {
    pub date_range: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub earnings_gt: Option<u64>,
    pub block_rewards_gt: Option<u64>,
    pub node_count_gt: Option<f64>,
    pub pool: Option<String>,
}
