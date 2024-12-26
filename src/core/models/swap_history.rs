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
        value_str
            .trim()
            .replace(",", "")
            .parse::<u64>()
            .map_err(de::Error::custom)
    }
}

#[derive(Table, Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
#[table_name("`swap_intervals`")]
#[serde(rename_all = "camelCase")]
pub struct SwapInterval {
    #[serde(rename = "averageSlip", with = "float_serialization")]
    pub average_slip: f64,
    #[serde(rename = "endTime", with = "timestamp_serialization")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "fromTradeAverageSlip", with = "float_serialization")]
    pub from_trade_average_slip: f64,
    #[serde(rename = "fromTradeCount", with = "u64_serialization")]
    pub from_trade_count: u64,
    #[serde(rename = "fromTradeFees", with = "u64_serialization")]
    pub from_trade_fees: u64,
    #[serde(rename = "fromTradeVolume", with = "u64_serialization")]
    pub from_trade_volume: u64,
    #[serde(rename = "fromTradeVolumeUSD", with = "u64_serialization")]
    pub from_trade_volume_usd: u64,
    #[serde(rename = "runePriceUSD", with = "float_serialization")]
    pub rune_price_usd: f64,
    #[serde(rename = "startTime", with = "timestamp_serialization")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "synthMintAverageSlip", with = "float_serialization")]
    pub synth_mint_average_slip: f64,
    #[serde(rename = "synthMintCount", with = "u64_serialization")]
    pub synth_mint_count: u64,
    #[serde(rename = "synthMintFees", with = "u64_serialization")]
    pub synth_mint_fees: u64,
    #[serde(rename = "synthMintVolume", with = "u64_serialization")]
    pub synth_mint_volume: u64,
    #[serde(rename = "synthMintVolumeUSD", with = "u64_serialization")]
    pub synth_mint_volume_usd: u64,
    #[serde(rename = "synthRedeemAverageSlip", with = "float_serialization")]
    pub synth_redeem_average_slip: f64,
    #[serde(rename = "synthRedeemCount", with = "u64_serialization")]
    pub synth_redeem_count: u64,
    #[serde(rename = "synthRedeemFees", with = "u64_serialization")]
    pub synth_redeem_fees: u64,
    #[serde(rename = "synthRedeemVolume", with = "u64_serialization")]
    pub synth_redeem_volume: u64,
    #[serde(rename = "synthRedeemVolumeUSD", with = "u64_serialization")]
    pub synth_redeem_volume_usd: u64,
    #[serde(rename = "toAssetAverageSlip", with = "float_serialization")]
    pub to_asset_average_slip: f64,
    #[serde(rename = "toAssetCount", with = "u64_serialization")]
    pub to_asset_count: u64,
    #[serde(rename = "toAssetFees", with = "u64_serialization")]
    pub to_asset_fees: u64,
    #[serde(rename = "toAssetVolume", with = "u64_serialization")]
    pub to_asset_volume: u64,
    #[serde(rename = "toAssetVolumeUSD", with = "u64_serialization")]
    pub to_asset_volume_usd: u64,
    #[serde(rename = "toRuneAverageSlip", with = "float_serialization")]
    pub to_rune_average_slip: f64,
    #[serde(rename = "toRuneCount", with = "u64_serialization")]
    pub to_rune_count: u64,
    #[serde(rename = "toRuneFees", with = "u64_serialization")]
    pub to_rune_fees: u64,
    #[serde(rename = "toRuneVolume", with = "u64_serialization")]
    pub to_rune_volume: u64,
    #[serde(rename = "toRuneVolumeUSD", with = "u64_serialization")]
    pub to_rune_volume_usd: u64,
    #[serde(rename = "toTradeAverageSlip", with = "float_serialization")]
    pub to_trade_average_slip: f64,
    #[serde(rename = "toTradeCount", with = "u64_serialization")]
    pub to_trade_count: u64,
    #[serde(rename = "toTradeFees", with = "u64_serialization")]
    pub to_trade_fees: u64,
    #[serde(rename = "toTradeVolume", with = "u64_serialization")]
    pub to_trade_volume: u64,
    #[serde(rename = "toTradeVolumeUSD", with = "u64_serialization")]
    pub to_trade_volume_usd: u64,
    #[serde(rename = "totalCount", with = "u64_serialization")]
    pub total_count: u64,
    #[serde(rename = "totalFees", with = "u64_serialization")]
    pub total_fees: u64,
    #[serde(rename = "totalVolume", with = "u64_serialization")]
    pub total_volume: u64,
    #[serde(rename = "totalVolumeUSD", with = "u64_serialization")]
    pub total_volume_usd: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SwapMeta {
    #[serde(rename = "averageSlip", with = "float_serialization")]
    pub average_slip: f64,
    #[serde(rename = "endTime", with = "timestamp_serialization")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "fromTradeAverageSlip", with = "float_serialization")]
    pub from_trade_average_slip: f64,
    #[serde(rename = "fromTradeCount", with = "u64_serialization")]
    pub from_trade_count: u64,
    #[serde(rename = "fromTradeFees", with = "u64_serialization")]
    pub from_trade_fees: u64,
    #[serde(rename = "fromTradeVolume", with = "u64_serialization")]
    pub from_trade_volume: u64,
    #[serde(rename = "fromTradeVolumeUSD", with = "u64_serialization")]
    pub from_trade_volume_usd: u64,
    #[serde(rename = "runePriceUSD", with = "float_serialization")]
    pub rune_price_usd: f64,
    #[serde(rename = "startTime", with = "timestamp_serialization")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "synthMintAverageSlip", with = "float_serialization")]
    pub synth_mint_average_slip: f64,
    #[serde(rename = "synthMintCount", with = "u64_serialization")]
    pub synth_mint_count: u64,
    #[serde(rename = "synthMintFees", with = "u64_serialization")]
    pub synth_mint_fees: u64,
    #[serde(rename = "synthMintVolume", with = "u64_serialization")]
    pub synth_mint_volume: u64,
    #[serde(rename = "synthMintVolumeUSD", with = "u64_serialization")]
    pub synth_mint_volume_usd: u64,
    #[serde(rename = "synthRedeemAverageSlip", with = "float_serialization")]
    pub synth_redeem_average_slip: f64,
    #[serde(rename = "synthRedeemCount", with = "u64_serialization")]
    pub synth_redeem_count: u64,
    #[serde(rename = "synthRedeemFees", with = "u64_serialization")]
    pub synth_redeem_fees: u64,
    #[serde(rename = "synthRedeemVolume", with = "u64_serialization")]
    pub synth_redeem_volume: u64,
    #[serde(rename = "synthRedeemVolumeUSD", with = "u64_serialization")]
    pub synth_redeem_volume_usd: u64,
    #[serde(rename = "toAssetAverageSlip", with = "float_serialization")]
    pub to_asset_average_slip: f64,
    #[serde(rename = "toAssetCount", with = "u64_serialization")]
    pub to_asset_count: u64,
    #[serde(rename = "toAssetFees", with = "u64_serialization")]
    pub to_asset_fees: u64,
    #[serde(rename = "toAssetVolume", with = "u64_serialization")]
    pub to_asset_volume: u64,
    #[serde(rename = "toAssetVolumeUSD", with = "u64_serialization")]
    pub to_asset_volume_usd: u64,
    #[serde(rename = "toRuneAverageSlip", with = "float_serialization")]
    pub to_rune_average_slip: f64,
    #[serde(rename = "toRuneCount", with = "u64_serialization")]
    pub to_rune_count: u64,
    #[serde(rename = "toRuneFees", with = "u64_serialization")]
    pub to_rune_fees: u64,
    #[serde(rename = "toRuneVolume", with = "u64_serialization")]
    pub to_rune_volume: u64,
    #[serde(rename = "toRuneVolumeUSD", with = "u64_serialization")]
    pub to_rune_volume_usd: u64,
    #[serde(rename = "toTradeAverageSlip", with = "float_serialization")]
    pub to_trade_average_slip: f64,
    #[serde(rename = "toTradeCount", with = "u64_serialization")]
    pub to_trade_count: u64,
    #[serde(rename = "toTradeFees", with = "u64_serialization")]
    pub to_trade_fees: u64,
    #[serde(rename = "toTradeVolume", with = "u64_serialization")]
    pub to_trade_volume: u64,
    #[serde(rename = "toTradeVolumeUSD", with = "u64_serialization")]
    pub to_trade_volume_usd: u64,
    #[serde(rename = "totalCount", with = "u64_serialization")]
    pub total_count: u64,
    #[serde(rename = "totalFees", with = "u64_serialization")]
    pub total_fees: u64,
    #[serde(rename = "totalVolume", with = "u64_serialization")]
    pub total_volume: u64,
    #[serde(rename = "totalVolumeUSD", with = "u64_serialization")]
    pub total_volume_usd: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SwapHistoryResponse {
    pub intervals: Vec<SwapInterval>,
    #[serde(rename = "meta")]
    pub meta_stats: SwapMeta,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SwapHistoryParams {
    pub interval: Option<Interval>,
    pub count: Option<u32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SwapHistoryQueryParams {
    pub date_range: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub volume_gt: Option<u64>,
    pub fees_gt: Option<u64>,
}
