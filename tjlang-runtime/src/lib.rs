//! TJLang Runtime
//!
//! A real interpreter that works with the TJLang AST.

pub mod interpreter;
pub mod primitive_methods;
pub mod stdlib;
pub mod stdlib_integration;
pub mod values;

#[cfg(test)]
mod tests;

// Re-export main types
pub use interpreter::Interpreter;
pub use values::Value;
