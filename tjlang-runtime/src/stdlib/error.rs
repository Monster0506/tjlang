//! ERROR Module - Error handling operations
//!
//! Provides comprehensive error handling functionality including:
//! - Error types and creation
//! - Error propagation
//! - Error formatting
//! - Error logging
//! - Error recovery

use crate::values::Value;

/// ERROR module for error handling operations
pub struct ERROR;

impl ERROR {
    /// Create a new error
    pub fn new(message: &str) -> String {
        message.to_string()
    }
    
    /// Create a formatted error
    pub fn format(message: &str, args: &[String]) -> String {
        let mut result = message.to_string();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        result
    }
    
    /// Log an error
    pub fn log(message: &str) {
        eprintln!("ERROR: {}", message);
    }
    
    /// Log a warning
    pub fn warn(message: &str) {
        eprintln!("WARNING: {}", message);
    }
    
    /// Log an info message
    pub fn info(message: &str) {
        println!("INFO: {}", message);
    }
    
    /// Log a debug message
    pub fn debug(message: &str) {
        println!("DEBUG: {}", message);
    }
}








