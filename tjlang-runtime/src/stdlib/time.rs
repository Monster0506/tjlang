//! TIME Module - Time and date operations
//!
//! Provides comprehensive time functionality including:
//! - Current time operations
//! - Date formatting and parsing
//! - Time arithmetic
//! - Timezone operations
//! - Timer and stopwatch functionality
//! - Date/time validation
//! - Calendar operations

use crate::values::Value;
use std::time::{SystemTime, UNIX_EPOCH, Duration, Instant};
use chrono::{Datelike, Timelike};

/// TIME module for time and date operations
pub struct TIME;

impl TIME {
    /// Get current timestamp in seconds since Unix epoch
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
    
    /// Get current timestamp in milliseconds since Unix epoch
    pub fn now_millis() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    }
    
    /// Get current timestamp in microseconds since Unix epoch
    pub fn now_micros() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros()
    }
    
    /// Get current timestamp in nanoseconds since Unix epoch
    pub fn now_nanos() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    }
    
    /// Get current date and time as string
    pub fn now_string() -> String {
        let now = SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
    
    /// Get current date as string
    pub fn today_string() -> String {
        let now = SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        datetime.format("%Y-%m-%d").to_string()
    }
    
    /// Get current time as string
    pub fn time_string() -> String {
        let now = SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        datetime.format("%H:%M:%S").to_string()
    }
    
    /// Format timestamp as string
    pub fn format_timestamp(timestamp: u64, format: &str) -> String {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        datetime.format(format).to_string()
    }
    
    /// Parse date string to timestamp
    pub fn parse_date(date_str: &str, format: &str) -> Result<u64, String> {
        let date = chrono::NaiveDate::parse_from_str(date_str, format)
            .map_err(|e| e.to_string())?;
        let datetime = date.and_hms_opt(0, 0, 0).ok_or("Invalid time")?;
        Ok(datetime.and_utc().timestamp() as u64)
    }
    
    /// Get Unix timestamp from date components
    pub fn from_components(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Result<u64, String> {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or("Invalid date")?;
        let time = chrono::NaiveTime::from_hms_opt(hour, minute, second)
            .ok_or("Invalid time")?;
        let datetime = chrono::NaiveDateTime::new(date, time);
        Ok(datetime.and_utc().timestamp() as u64)
    }
    
    /// Get date components from timestamp
    pub fn to_components(timestamp: u64) -> (i32, u32, u32, u32, u32, u32) {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        let date = datetime.date_naive();
        let time = datetime.time();
        (
            date.year(),
            date.month(),
            date.day(),
            time.hour(),
            time.minute(),
            time.second(),
        )
    }
    
    /// Add seconds to timestamp
    pub fn add_seconds(timestamp: u64, seconds: i64) -> u64 {
        (timestamp as i64 + seconds) as u64
    }
    
    /// Add minutes to timestamp
    pub fn add_minutes(timestamp: u64, minutes: i64) -> u64 {
        Self::add_seconds(timestamp, minutes * 60)
    }
    
    /// Add hours to timestamp
    pub fn add_hours(timestamp: u64, hours: i64) -> u64 {
        Self::add_minutes(timestamp, hours * 60)
    }
    
    /// Add days to timestamp
    pub fn add_days(timestamp: u64, days: i64) -> u64 {
        Self::add_hours(timestamp, days * 24)
    }
    
    /// Add weeks to timestamp
    pub fn add_weeks(timestamp: u64, weeks: i64) -> u64 {
        Self::add_days(timestamp, weeks * 7)
    }
    
    /// Add months to timestamp
    pub fn add_months(timestamp: u64, months: i32) -> u64 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        let new_datetime = datetime + chrono::Months::new(months as u32);
        new_datetime.timestamp() as u64
    }
    
    /// Add years to timestamp
    pub fn add_years(timestamp: u64, years: i32) -> u64 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        let new_datetime = datetime + chrono::Months::new(years as u32 * 12);
        new_datetime.timestamp() as u64
    }
    
    /// Get difference between timestamps in seconds
    pub fn diff_seconds(timestamp1: u64, timestamp2: u64) -> i64 {
        timestamp2 as i64 - timestamp1 as i64
    }
    
    /// Get difference between timestamps in minutes
    pub fn diff_minutes(timestamp1: u64, timestamp2: u64) -> i64 {
        Self::diff_seconds(timestamp1, timestamp2) / 60
    }
    
    /// Get difference between timestamps in hours
    pub fn diff_hours(timestamp1: u64, timestamp2: u64) -> i64 {
        Self::diff_minutes(timestamp1, timestamp2) / 60
    }
    
    /// Get difference between timestamps in days
    pub fn diff_days(timestamp1: u64, timestamp2: u64) -> i64 {
        Self::diff_hours(timestamp1, timestamp2) / 24
    }
    
    /// Check if year is leap year
    pub fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
    
    /// Get number of days in month
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap_year(year) { 29 } else { 28 },
            _ => 0,
        }
    }
    
    /// Get day of week (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
    pub fn day_of_week(timestamp: u64) -> u32 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        datetime.weekday().num_days_from_sunday()
    }
    
    /// Get day of year (1-366)
    pub fn day_of_year(timestamp: u64) -> u32 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        datetime.ordinal()
    }
    
    /// Get week number of year
    pub fn week_of_year(timestamp: u64) -> u32 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        datetime.iso_week().week()
    }
    
    /// Get timezone offset in seconds
    pub fn timezone_offset() -> i32 {
        // TODO: Implement proper timezone detection
        0
    }
    
    /// Convert timestamp to timezone
    pub fn to_timezone(timestamp: u64, timezone: &str) -> Result<u64, String> {
        // TODO: Implement timezone conversion
        Ok(timestamp)
    }
    
    /// Get start of day timestamp
    pub fn start_of_day(timestamp: u64) -> u64 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        let start_of_day = datetime.date_naive().and_hms_opt(0, 0, 0).unwrap_or_default();
        start_of_day.and_utc().timestamp() as u64
    }
    
    /// Get end of day timestamp
    pub fn end_of_day(timestamp: u64) -> u64 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        let end_of_day = datetime.date_naive().and_hms_opt(23, 59, 59).unwrap_or_default();
        end_of_day.and_utc().timestamp() as u64
    }
    
    /// Get start of week timestamp
    pub fn start_of_week(timestamp: u64) -> u64 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        let days_from_monday = datetime.weekday().num_days_from_monday();
        let start_of_week = datetime.date_naive() - chrono::Duration::days(days_from_monday as i64);
        start_of_week.and_hms_opt(0, 0, 0).unwrap_or_default().and_utc().timestamp() as u64
    }
    
    /// Get start of month timestamp
    pub fn start_of_month(timestamp: u64) -> u64 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        let start_of_month = datetime.date_naive().with_day(1).unwrap_or_default().and_hms_opt(0, 0, 0).unwrap_or_default();
        start_of_month.and_utc().timestamp() as u64
    }
    
    /// Get start of year timestamp
    pub fn start_of_year(timestamp: u64) -> u64 {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
        let start_of_year = datetime.date_naive().with_month(1).unwrap_or_default().with_day(1).unwrap_or_default().and_hms_opt(0, 0, 0).unwrap_or_default();
        start_of_year.and_utc().timestamp() as u64
    }
    
    /// Sleep for specified duration in seconds
    pub fn sleep(seconds: f64) {
        std::thread::sleep(Duration::from_secs_f64(seconds));
    }
    
    /// Sleep for specified duration in milliseconds
    pub fn sleep_millis(millis: u64) {
        std::thread::sleep(Duration::from_millis(millis));
    }
    
    /// Sleep for specified duration in microseconds
    pub fn sleep_micros(micros: u64) {
        std::thread::sleep(Duration::from_micros(micros));
    }
    
    /// Sleep for specified duration in nanoseconds
    pub fn sleep_nanos(nanos: u64) {
        std::thread::sleep(Duration::from_nanos(nanos));
    }
    
    /// Create a timer
    pub fn create_timer() -> Timer {
        Timer::new()
    }
    
    /// Create a stopwatch
    pub fn create_stopwatch() -> Stopwatch {
        Stopwatch::new()
    }
    
    /// Get current timezone name
    pub fn timezone_name() -> String {
        // TODO: Implement timezone name detection
        "UTC".to_string()
    }
    
    /// List available timezones
    pub fn list_timezones() -> Vec<String> {
        // TODO: Implement timezone listing
        vec!["UTC".to_string()]
    }
    
    /// Validate date components
    pub fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
        if month < 1 || month > 12 { return false; }
        if day < 1 || day > Self::days_in_month(year, month) { return false; }
        true
    }
    
    /// Validate time components
    pub fn is_valid_time(hour: u32, minute: u32, second: u32) -> bool {
        hour < 24 && minute < 60 && second < 60
    }
    
    /// Get age in years
    pub fn get_age(birth_timestamp: u64) -> u32 {
        let now = Self::now();
        let birth_date = chrono::DateTime::from_timestamp(birth_timestamp as i64, 0).unwrap_or_default();
        let current_date = chrono::DateTime::from_timestamp(now as i64, 0).unwrap_or_default();
        (current_date.year() - birth_date.year()) as u32
    }
    
    /// Get relative time string (e.g., "2 hours ago")
    pub fn relative_time(timestamp: u64) -> String {
        let now = Self::now();
        let diff = now as i64 - timestamp as i64;
        
        if diff < 60 {
            "just now".to_string()
        } else if diff < 3600 {
            format!("{} minutes ago", diff / 60)
        } else if diff < 86400 {
            format!("{} hours ago", diff / 3600)
        } else if diff < 2592000 {
            format!("{} days ago", diff / 86400)
        } else if diff < 31536000 {
            format!("{} months ago", diff / 2592000)
        } else {
            format!("{} years ago", diff / 31536000)
        }
    }
}

/// Timer for measuring elapsed time
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
    
    pub fn elapsed_millis(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
    
    pub fn elapsed_micros(&self) -> u128 {
        self.start.elapsed().as_micros()
    }
    
    pub fn elapsed_nanos(&self) -> u128 {
        self.start.elapsed().as_nanos()
    }
    
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

/// Stopwatch for measuring intervals
pub struct Stopwatch {
    start: Option<Instant>,
    laps: Vec<Duration>,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            start: None,
            laps: Vec::new(),
        }
    }
    
    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }
    
    pub fn stop(&mut self) -> Option<Duration> {
        if let Some(start) = self.start {
            let elapsed = start.elapsed();
            self.start = None;
            Some(elapsed)
        } else {
            None
        }
    }
    
    pub fn lap(&mut self) -> Option<Duration> {
        if let Some(start) = self.start {
            let elapsed = start.elapsed();
            self.laps.push(elapsed);
            Some(elapsed)
        } else {
            None
        }
    }
    
    pub fn reset(&mut self) {
        self.start = None;
        self.laps.clear();
    }
    
    pub fn is_running(&self) -> bool {
        self.start.is_some()
    }
    
    pub fn get_laps(&self) -> &[Duration] {
        &self.laps
    }
    
    pub fn get_total_time(&self) -> Duration {
        self.laps.iter().sum()
    }
}
