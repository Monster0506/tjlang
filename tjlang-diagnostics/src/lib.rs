//! TJLang Diagnostics System
//!
//! Provides unified error reporting with error codes, source spans, and suggestions.

pub mod collection;
pub mod diagnostic;
pub mod error_codes;
pub mod helpers;
pub mod source_span;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use collection::DiagnosticCollection;
pub use diagnostic::{Suggestion, TJLangDiagnostic};
pub use error_codes::ErrorCode;
pub use helpers::helpers as diagnostic_helpers;
pub use source_span::SourceSpan;
