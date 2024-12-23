use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

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
        value_str.parse::<f64>().map_err(serde::de::Error::custom)
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
        value_str.parse::<u64>().map_err(de::Error::custom)
    }
}

mod u32_serialization {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value_str = String::deserialize(deserializer)?;
        value_str.parse::<u32>().map_err(de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistoryResponse {
    pub intervals: Vec<IntervalData>,
    #[serde(rename = "meta")]
    pub meta_stats: MetaStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistoryParams {
    pub interval: Option<String>,
    pub count: Option<u32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}
