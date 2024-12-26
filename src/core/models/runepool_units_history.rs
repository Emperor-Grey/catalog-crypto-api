use chrono::{DateTime, TimeZone, Utc};
use prkorm::Table;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::common::Interval;

mod timestamp_serialization {
    use super::*;
    use serde::{Deserializer, Serializer};

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
        value_str
            .trim()
            .replace(",", "")
            .parse::<u64>()
            .map_err(de::Error::custom)
    }
}

#[derive(Table, Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
#[table_name("`runepool_unit_intervals`")]
pub struct RunepoolUnitsInterval {
    #[serde(rename = "count", with = "u64_serialization")]
    pub count: u64,
    #[serde(rename = "endTime", with = "timestamp_serialization")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "startTime", with = "timestamp_serialization")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "units", with = "u64_serialization")]
    pub units: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MetaStats {
    #[serde(rename = "endCount", with = "u64_serialization")]
    pub end_count: u64,
    #[serde(rename = "endTime", with = "timestamp_serialization")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "endUnits", with = "u64_serialization")]
    pub end_units: u64,
    #[serde(rename = "startCount", with = "u64_serialization")]
    pub start_count: u64,
    #[serde(rename = "startTime", with = "timestamp_serialization")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "startUnits", with = "u64_serialization")]
    pub start_units: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RunepoolUnitsHistoryResponse {
    pub intervals: Vec<RunepoolUnitsInterval>,
    #[serde(rename = "meta")]
    pub meta_stats: MetaStats,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RunepoolUnitsHistoryParams {
    pub interval: Option<Interval>,
    pub count: Option<u32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RunepoolUnitsHistoryQueryParams {
    pub date_range: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub units_gt: Option<u64>,
}
