//! TJLang Standard Library
//!
//! Comprehensive standard library with modules for:
//! - IO operations (print, input, formatting)
//! - File operations (read, write, copy, move, delete, rename)
//! - Math functions (trigonometry, logarithms, statistics, etc.)
//! - String operations (manipulation, regex, encoding)
//! - Collections (arrays, maps, sets, iterators)
//! - Time and date operations
//! - Error handling
//! - Testing framework

pub mod collections;
pub mod error;
pub mod file;
pub mod io;
pub mod math;
pub mod string;
pub mod testing;
pub mod time;

// Re-export commonly used modules
pub use collections::*;
pub use error::*;
pub use file::*;
pub use io::*;
pub use math::*;
pub use string::*;
pub use testing::*;
pub use time::*;

#[cfg(test)]
mod tests;
