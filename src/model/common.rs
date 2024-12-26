use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{
    depth_history::DepthHistoryQueryParams, earnings_history::EarningsHistoryQueryParams,
    runepool_units_history::RunepoolUnitsHistoryQueryParams, swap_history::SwapHistoryQueryParams,
};

pub const DEFAULT_PAGE_SIZE: u32 = 30;
pub const MAX_PAGE_SIZE: u32 = 400;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
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
        let interval_str = match self {
            Interval::FiveMin => "5min",
            Interval::Hour => "hour",
            Interval::Day => "day",
            Interval::Week => "week",
            Interval::Month => "month",
            Interval::Quarter => "quarter",
            Interval::Year => "year",
        };
        write!(f, "{}", interval_str)
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

impl SwapHistoryQueryParams {
    pub fn get_sort_field(&self) -> &str {
        match self.sort_by.as_deref() {
            Some("volume") => "total_volume",
            Some("fees") => "total_fees",
            Some("count") => "total_count",
            Some("timestamp") => "start_time",
            _ => "start_time",
        }
    }

    pub fn parse_date_range(&self) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        parse_date_range(&self.date_range)
    }
}

impl RunepoolUnitsHistoryQueryParams {
    pub fn get_sort_field(&self) -> &str {
        match self.sort_by.as_deref() {
            Some("units") => "units",
            Some("count") => "count",
            Some("timestamp") => "start_time",
            _ => "start_time",
        }
    }

    pub fn parse_date_range(&self) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        parse_date_range(&self.date_range)
    }
}

fn parse_date_range(date_range: &Option<String>) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    date_range.as_ref().and_then(|range| {
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
