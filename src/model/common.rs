use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::FiveMin => write!(f, "5min"),
            Interval::Hour => write!(f, "hour"),
            Interval::Day => write!(f, "day"),
            Interval::Week => write!(f, "week"),
            Interval::Month => write!(f, "month"),
            Interval::Quarter => write!(f, "quarter"),
            Interval::Year => write!(f, "year"),
        }
    }
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

#[derive(Debug, Deserialize)]
pub struct DepthHistoryQueryParams {
    pub date_range: Option<String>,
    pub liquidity_gt: Option<u64>,
    pub interval: Option<Interval>,
    #[serde(rename = "sort_by")]
    pub sort_field: Option<String>, // Do you know you can also pass this timestamp, (this gets mapped to start_time internally)
    pub order: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
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

pub const DEFAULT_PAGE_SIZE: u32 = 30;
pub const MAX_PAGE_SIZE: u32 = 400;

impl DepthHistoryQueryParams {
    // Helper method to parse date range
    pub fn parse_date_range(&self) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.date_range.as_ref().and_then(|range| {
            let parts: Vec<&str> = range.split(',').collect();
            if parts.len() == 2 {
                let start = NaiveDateTime::parse_from_str(
                    &format!("{} 00:00:00", parts[0]),
                    "%Y-%m-%d %H:%M:%S",
                )
                .ok()?;
                let end = NaiveDateTime::parse_from_str(
                    &format!("{} 23:59:59", parts[1]),
                    "%Y-%m-%d %H:%M:%S",
                )
                .ok()?;
                Some((
                    DateTime::from_naive_utc_and_offset(start, Utc),
                    DateTime::from_naive_utc_and_offset(end, Utc),
                ))
            } else {
                None
            }
        })
    }

    // Helper method to map timestamp to actual db field
    pub fn get_sort_field(&self) -> &str {
        match self.sort_field.as_deref() {
            Some("timestamp") => "start_time", // Map timestamp to start_time
            Some(field) => field,
            None => "start_time", // Default sort field
        }
    }
}

impl EarningsHistoryQueryParams {
    // Helper method to parse date range
    pub fn parse_date_range(&self) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.date_range.as_ref().and_then(|range| {
            let parts: Vec<&str> = range.split(',').collect();
            if parts.len() == 2 {
                let start = NaiveDateTime::parse_from_str(
                    &format!("{} 00:00:00", parts[0]),
                    "%Y-%m-%d %H:%M:%S",
                )
                .ok()?;
                let end = NaiveDateTime::parse_from_str(
                    &format!("{} 23:59:59", parts[1]),
                    "%Y-%m-%d %H:%M:%S",
                )
                .ok()?;
                Some((
                    DateTime::from_naive_utc_and_offset(start, Utc),
                    DateTime::from_naive_utc_and_offset(end, Utc),
                ))
            } else {
                None
            }
        })
    }

    // Helper method to map timestamp to actual db field
    pub fn get_sort_field(&self) -> &str {
        match self.sort_by.as_deref() {
            Some("timestamp") => "start_time", // Map timestamp to start_time
            Some(field) => field,
            None => "start_time", // Default sort field
        }
    }
}
