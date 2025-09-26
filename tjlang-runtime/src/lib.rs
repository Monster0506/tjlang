//! TJLang Runtime
//!
//! A real interpreter that works with the TJLang AST.

pub mod values;
pub mod interpreter;
pub mod stdlib;
pub mod stdlib_integration;

// Re-export main types
pub use values::Value;
pub use interpreter::Interpreter;
