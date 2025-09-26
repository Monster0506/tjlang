//! Tests for ERROR module

#[cfg(test)]
mod tests {
    use crate::stdlib::error::*;

    #[test]
    fn test_new() {
        let error = ERROR::new("Test error message");
        assert_eq!(error, "Test error message");
    }

    #[test]
    fn test_format() {
        let args = vec!["test".to_string(), "error".to_string()];
        let error = ERROR::format("{0} {1}", &args);
        assert_eq!(error, "test error");
    }

    #[test]
    fn test_log() {
        // This test just ensures the function doesn't panic
        ERROR::log("Test log message");
        assert!(true);
    }

    #[test]
    fn test_warn() {
        // This test just ensures the function doesn't panic
        ERROR::warn("Test warning message");
        assert!(true);
    }

    #[test]
    fn test_info() {
        // This test just ensures the function doesn't panic
        ERROR::info("Test info message");
        assert!(true);
    }

    #[test]
    fn test_debug() {
        // This test just ensures the function doesn't panic
        ERROR::debug("Test debug message");
        assert!(true);
    }
}