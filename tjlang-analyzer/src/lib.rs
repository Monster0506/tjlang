//! TJLang Rule-Based Analysis System
//!
//! A comprehensive, extensible analysis pipeline with rule-based architecture.

pub mod config;
pub mod context;
pub mod pipeline;
pub mod rules;
pub mod type_checker;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod type_checker_tests;

// Re-export commonly used types
pub use config::{RuleConfig, RuleSeverity};
pub use context::AnalysisContext;
pub use pipeline::{AnalysisPhase, AnalysisPipeline, AnalysisResult};
pub use rules::*;
