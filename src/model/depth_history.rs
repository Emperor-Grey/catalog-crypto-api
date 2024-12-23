use chrono::{DateTime, TimeZone, Utc};
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

mod float_serialization {
    use super::*;

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
    use serde::de;

    use super::*;

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
    use serde::de;

    use super::*;

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
pub struct DepthInterval {
    #[serde(rename = "assetDepth", with = "u64_serialization")]
    pub asset_depth: u64,
    #[serde(rename = "assetPrice", with = "float_serialization")]
    pub asset_price: f64,
    #[serde(rename = "assetPriceUSD", with = "float_serialization")]
    pub asset_price_usd: f64,
    #[serde(rename = "endTime", with = "timestamp_serialization")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "liquidityUnits", with = "u64_serialization")]
    pub liquidity_units: u64,
    #[serde(with = "float_serialization")]
    pub luvi: f64,
    #[serde(rename = "membersCount", with = "u32_serialization")]
    pub members_count: u32,
    #[serde(rename = "runeDepth", with = "u64_serialization")]
    pub rune_depth: u64,
    #[serde(rename = "startTime", with = "timestamp_serialization")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "synthSupply", with = "u64_serialization")]
    pub synth_supply: u64,
    #[serde(rename = "synthUnits", with = "u64_serialization")]
    pub synth_units: u64,
    #[serde(with = "u64_serialization")]
    pub units: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaStats {
    #[serde(rename = "endAssetDepth", with = "u64_serialization")]
    pub end_asset_depth: u64,
    #[serde(rename = "endLPUnits", with = "u64_serialization")]
    pub end_lp_units: u64,
    #[serde(rename = "endMemberCount", with = "u32_serialization")]
    pub end_member_count: u32,
    #[serde(rename = "endRuneDepth", with = "u64_serialization")]
    pub end_rune_depth: u64,
    #[serde(rename = "endSynthUnits", with = "u64_serialization")]
    pub end_synth_units: u64,
    #[serde(rename = "endTime", with = "timestamp_serialization")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "luviIncrease", with = "float_serialization")]
    pub luvi_increase: f64,
    #[serde(rename = "priceShiftLoss", with = "float_serialization")]
    pub price_shift_loss: f64,
    #[serde(rename = "startAssetDepth", with = "u64_serialization")]
    pub start_asset_depth: u64,
    #[serde(rename = "startLPUnits", with = "u64_serialization")]
    pub start_lp_units: u64,
    #[serde(rename = "startMemberCount", with = "u32_serialization")]
    pub start_member_count: u32,
    #[serde(rename = "startRuneDepth", with = "u64_serialization")]
    pub start_rune_depth: u64,
    #[serde(rename = "startSynthUnits", with = "u64_serialization")]
    pub start_synth_units: u64,
    #[serde(rename = "startTime", with = "timestamp_serialization")]
    pub start_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistoryParams {
    pub interval: Option<Interval>,
    pub count: Option<u32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistoryResponse {
    pub intervals: Vec<DepthInterval>,
    #[serde(rename = "meta")]
    pub meta_stats: MetaStats,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Interval {
    #[serde(rename = "5min")]
    FiveMin,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl TryFrom<String> for Interval {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "five_min" => Ok(Interval::FiveMin),
            "hour" => Ok(Interval::Hour),
            "day" => Ok(Interval::Day),
            "week" => Ok(Interval::Week),
            "month" => Ok(Interval::Month),
            "quarter" => Ok(Interval::Quarter),
            "year" => Ok(Interval::Year),
            _ => Err("Invalid interval".to_string()),
        }
    }
}
