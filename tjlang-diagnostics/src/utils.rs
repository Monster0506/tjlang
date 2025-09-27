/// Utility functions for TJLang CLI
pub mod debug {
    use std::sync::atomic::{AtomicBool, Ordering};
    
    static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);
    
    /// Enable debug output
    pub fn enable() {
        DEBUG_ENABLED.store(true, Ordering::Relaxed);
    }
    
    /// Disable debug output
    pub fn disable() {
        DEBUG_ENABLED.store(false, Ordering::Relaxed);
    }
    
    /// Check if debug is enabled
    pub fn is_enabled() -> bool {
        DEBUG_ENABLED.load(Ordering::Relaxed)
    }
    
    /// Debug print function that only prints when debug is enabled
    pub fn debug_print(message: &str) {
        if is_enabled() {
            println!("{}", message);
        }
    }
    
    /// Debug print function with format string - for now just calls println!
    pub fn debug_println(format: &str) {
        if is_enabled() {
            println!("{}", format);
        }
    }
    
    /// Debug print function with format string and arguments - for now just calls println!
    pub fn debug_printf(format: &str, _args: &[&dyn std::fmt::Display]) {
        if is_enabled() {
            // For now, just call println! with the format string
            // TODO: Implement proper formatting like printf
            println!("{}", format);
        }
    }
}

/// Macro to replace debug println! statements
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if $crate::utils::debug::is_enabled() {
            println!($($arg)*);
        }
    };
}