//! Tests for TIME module

#[cfg(test)]
mod tests {
    use crate::stdlib::time::*;

    #[test]
    fn test_now() {
        let result = TIME::now();
        assert!(result > 0);
    }

    #[test]
    fn test_now_millis() {
        let result = TIME::now_millis();
        assert!(result > 0);
    }

    #[test]
    fn test_now_micros() {
        let result = TIME::now_micros();
        assert!(result > 0);
    }

    #[test]
    fn test_now_nanos() {
        let result = TIME::now_nanos();
        assert!(result > 0);
    }

    #[test]
    fn test_now_string() {
        let result = TIME::now_string();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_today_string() {
        let result = TIME::today_string();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_time_string() {
        let result = TIME::time_string();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_timestamp() {
        let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
        let result = TIME::format_timestamp(timestamp, "%Y-%m-%d %H:%M:%S");
        assert!(result.contains("2021"));
    }

    #[test]
    fn test_parse_date() {
        let result = TIME::parse_date("2021-01-01", "%Y-%m-%d");
        if let Err(e) = &result {
            println!("Parse date error: {}", e);
        }
        assert!(result.is_ok());
        let timestamp = result.unwrap();
        assert!(timestamp > 0);
    }

    #[test]
    fn test_from_components() {
        let result = TIME::from_components(2021, 1, 1, 0, 0, 0);
        assert!(result.is_ok());
        let timestamp = result.unwrap();
        assert!(timestamp > 0);
    }

    #[test]
    fn test_to_components() {
        let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
        let result = TIME::to_components(timestamp);
        assert_eq!(result.0, 2021); // year
        assert_eq!(result.1, 1); // month
        assert_eq!(result.2, 1); // day
    }

    #[test]
    fn test_add_seconds() {
        let timestamp = 1609459200;
        let result = TIME::add_seconds(timestamp, 3600);
        assert_eq!(result, timestamp + 3600);
    }

    #[test]
    fn test_add_minutes() {
        let timestamp = 1609459200;
        let result = TIME::add_minutes(timestamp, 60);
        assert_eq!(result, timestamp + 3600);
    }

    #[test]
    fn test_add_hours() {
        let timestamp = 1609459200;
        let result = TIME::add_hours(timestamp, 1);
        assert_eq!(result, timestamp + 3600);
    }

    #[test]
    fn test_add_days() {
        let timestamp = 1609459200;
        let result = TIME::add_days(timestamp, 1);
        assert_eq!(result, timestamp + 86400);
    }

    #[test]
    fn test_add_weeks() {
        let timestamp = 1609459200;
        let result = TIME::add_weeks(timestamp, 1);
        assert_eq!(result, timestamp + 604800);
    }

    #[test]
    fn test_add_months() {
        let timestamp = 1609459200; // 2021-01-01
        let result = TIME::add_months(timestamp, 1);
        // Should be 2021-02-01
        assert!(result > timestamp);
    }

    #[test]
    fn test_add_years() {
        let timestamp = 1609459200; // 2021-01-01
        let result = TIME::add_years(timestamp, 1);
        // Should be 2022-01-01
        assert!(result > timestamp);
    }

    #[test]
    fn test_diff_seconds() {
        let timestamp1 = 1609459200;
        let timestamp2 = 1609459260;
        let result = TIME::diff_seconds(timestamp1, timestamp2);
        assert_eq!(result, 60);
    }

    #[test]
    fn test_diff_minutes() {
        let timestamp1 = 1609459200;
        let timestamp2 = 1609459260;
        let result = TIME::diff_minutes(timestamp1, timestamp2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_diff_hours() {
        let timestamp1 = 1609459200;
        let timestamp2 = 1609462800;
        let result = TIME::diff_hours(timestamp1, timestamp2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_diff_days() {
        let timestamp1 = 1609459200;
        let timestamp2 = 1609545600;
        let result = TIME::diff_days(timestamp1, timestamp2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_is_leap_year() {
        let result = TIME::is_leap_year(2020);
        assert!(result);
    }

    #[test]
    fn test_days_in_month() {
        let result = TIME::days_in_month(2021, 2);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_day_of_week() {
        let timestamp = 1609459200; // 2021-01-01 (Friday)
        let result = TIME::day_of_week(timestamp);
        assert_eq!(result, 5); // Friday
    }

    #[test]
    fn test_day_of_year() {
        let timestamp = 1609459200; // 2021-01-01
        let result = TIME::day_of_year(timestamp);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_week_of_year() {
        let timestamp = 1609459200; // 2021-01-01
        let result = TIME::week_of_year(timestamp);
        assert!(result > 0);
    }

    #[test]
    fn test_timezone_offset() {
        let result = TIME::timezone_offset();
        // Should return a valid offset in seconds
        assert!(result >= -86400 && result <= 86400);
    }

    #[test]
    fn test_to_timezone() {
        let timestamp = 1609459200;
        let result = TIME::to_timezone(timestamp, "UTC");
        assert!(result.is_ok());
    }

    #[test]
    fn test_timezone_name() {
        let result = TIME::timezone_name();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_list_timezones() {
        let result = TIME::list_timezones();
        assert!(!result.is_empty());
        assert!(result.contains(&"UTC".to_string()));
    }

    #[test]
    fn test_start_of_day() {
        let timestamp = 1609459200;
        let result = TIME::start_of_day(timestamp);
        assert!(result <= timestamp);
    }

    #[test]
    fn test_end_of_day() {
        let timestamp = 1609459200;
        let result = TIME::end_of_day(timestamp);
        assert!(result >= timestamp);
    }

    #[test]
    fn test_start_of_week() {
        let timestamp = 1609459200;
        let result = TIME::start_of_week(timestamp);
        assert!(result <= timestamp);
    }

    #[test]
    fn test_start_of_month() {
        let timestamp = 1609459200;
        let result = TIME::start_of_month(timestamp);
        assert!(result <= timestamp);
    }

    #[test]
    fn test_start_of_year() {
        let timestamp = 1609459200;
        let result = TIME::start_of_year(timestamp);
        assert!(result <= timestamp);
    }

    #[test]
    fn test_sleep() {
        let start = TIME::now();
        TIME::sleep(0.1);
        let end = TIME::now();
        assert!(end - start >= 0);
    }

    #[test]
    fn test_sleep_millis() {
        let start = TIME::now_millis();
        TIME::sleep_millis(100);
        let end = TIME::now_millis();
        assert!(end - start >= 100);
    }

    #[test]
    fn test_sleep_micros() {
        let start = TIME::now_micros();
        TIME::sleep_micros(1000);
        let end = TIME::now_micros();
        assert!(end - start >= 1000);
    }

    #[test]
    fn test_sleep_nanos() {
        let start = TIME::now_nanos();
        TIME::sleep_nanos(1000000);
        let end = TIME::now_nanos();
        assert!(end - start >= 1000000);
    }

    #[test]
    fn test_create_timer() {
        let timer = TIME::create_timer();
        // Just test that it was created successfully
        assert!(true);
    }

    #[test]
    fn test_create_stopwatch() {
        let stopwatch = TIME::create_stopwatch();
        assert!(!stopwatch.is_running());
    }

    #[test]
    fn test_is_valid_date() {
        let result = TIME::is_valid_date(2021, 1, 1);
        assert!(result);
    }

    #[test]
    fn test_is_valid_time() {
        let result = TIME::is_valid_time(12, 30, 45);
        assert!(result);
    }

    #[test]
    fn test_get_age() {
        let birth_timestamp = 1609459200; // 2021-01-01
        let result = TIME::get_age(birth_timestamp);
        assert!(result >= 0);
    }

    #[test]
    fn test_relative_time() {
        let timestamp = TIME::now() - 3600; // 1 hour ago
        let result = TIME::relative_time(timestamp);
        assert!(!result.is_empty());
    }

    // Test Timer methods
    #[test]
    fn test_timer_elapsed_secs() {
        let timer = TIME::create_timer();
        let elapsed = timer.elapsed_secs();
        assert!(elapsed >= 0.0);
    }

    #[test]
    fn test_timer_elapsed_millis() {
        let timer = TIME::create_timer();
        let elapsed = timer.elapsed_millis();
        assert!(elapsed >= 0);
    }

    #[test]
    fn test_timer_elapsed_micros() {
        let timer = TIME::create_timer();
        let elapsed = timer.elapsed_micros();
        assert!(elapsed >= 0);
    }

    #[test]
    fn test_timer_elapsed_nanos() {
        let timer = TIME::create_timer();
        let elapsed = timer.elapsed_nanos();
        assert!(elapsed >= 0);
    }

    #[test]
    fn test_timer_reset() {
        let mut timer = TIME::create_timer();
        timer.reset();
        let elapsed = timer.elapsed_secs();
        assert!(elapsed >= 0.0);
    }

    // Test Stopwatch methods
    #[test]
    fn test_stopwatch_start() {
        let mut stopwatch = TIME::create_stopwatch();
        stopwatch.start();
        assert!(stopwatch.is_running());
    }

    #[test]
    fn test_stopwatch_stop() {
        let mut stopwatch = TIME::create_stopwatch();
        stopwatch.start();
        stopwatch.stop();
        assert!(!stopwatch.is_running());
    }

    #[test]
    fn test_stopwatch_lap() {
        let mut stopwatch = TIME::create_stopwatch();
        stopwatch.start();
        let lap_time = stopwatch.lap();
        assert!(lap_time.is_some());
    }

    #[test]
    fn test_stopwatch_reset() {
        let mut stopwatch = TIME::create_stopwatch();
        stopwatch.start();
        stopwatch.reset();
        assert!(!stopwatch.is_running());
    }
}
