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

pub mod io;
pub mod file;
pub mod math;
pub mod string;
pub mod collections;
pub mod time;
pub mod error;
pub mod testing;

// Re-export commonly used modules
pub use io::*;
pub use file::*;
pub use math::*;
pub use string::*;
pub use collections::*;
pub use time::*;
pub use error::*;
pub use testing::*;

#[cfg(test)]
mod tests;
