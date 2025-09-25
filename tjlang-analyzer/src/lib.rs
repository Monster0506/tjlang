//! TJLang Rule-Based Analysis System
//! 
//! A comprehensive, extensible analysis pipeline with rule-based architecture.

pub mod rules;
pub mod pipeline;
pub mod context;
pub mod config;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use rules::*;
pub use pipeline::{AnalysisPipeline, AnalysisResult, AnalysisPhase};
pub use context::AnalysisContext;
pub use config::{RuleConfig, RuleSeverity};