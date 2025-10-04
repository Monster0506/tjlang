//! TJLang Unified Type System
//! 
//! This crate provides a unified type system that works for both:
//! - Runtime type checking and inference
//! - Static analysis and type safety
//! 
//! Based on the formal algebraic specification with:
//! - Algebraic types (sum, product, function types)
//! - Interface system (traits/type classes)
//! - Constraint solving and type inference
//! - Subtyping and compatibility rules

pub mod types;
pub mod checker;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use types::*;
pub use checker::*;