//! Helper functions for creating common diagnostics

use codespan_reporting::diagnostic::Severity;

use crate::diagnostic::TJLangDiagnostic;
use crate::error_codes::ErrorCode;
use crate::source_span::SourceSpan;

/// Helper functions for creating common diagnostics
pub mod helpers {
    use super::*;
    
    pub fn type_mismatch(
        expected: &str,
        found: &str,
        span: SourceSpan,
    ) -> TJLangDiagnostic {
        TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            format!("expected `{}`, found `{}`", expected, found),
            span,
        )
    }
    
    pub fn undefined_variable(
        name: &str,
        span: SourceSpan,
    ) -> TJLangDiagnostic {
        TJLangDiagnostic::new(
            ErrorCode::AnalyzerUndefinedVariable,
            Severity::Error,
            format!("undefined variable `{}`", name),
            span,
        )
    }
    
    pub fn undefined_function(
        name: &str,
        span: SourceSpan,
    ) -> TJLangDiagnostic {
        TJLangDiagnostic::new(
            ErrorCode::AnalyzerUndefinedFunction,
            Severity::Error,
            format!("undefined function `{}`", name),
            span,
        )
    }
    
    pub fn non_exhaustive_match(
        missing_patterns: Vec<String>,
        span: SourceSpan,
    ) -> TJLangDiagnostic {
        TJLangDiagnostic::new(
            ErrorCode::AnalyzerNonExhaustiveMatch,
            Severity::Error,
            format!("non-exhaustive match: missing patterns: {}", missing_patterns.join(", ")),
            span,
        )
    }
    
    pub fn unexpected_token(
        expected: &str,
        found: &str,
        span: SourceSpan,
    ) -> TJLangDiagnostic {
        TJLangDiagnostic::new(
            ErrorCode::ParserUnexpectedToken,
            Severity::Error,
            format!("expected `{}`, found `{}`", expected, found),
            span,
        )
    }
    
    pub fn unterminated_string(span: SourceSpan) -> TJLangDiagnostic {
        TJLangDiagnostic::new(
            ErrorCode::LexerUnterminatedString,
            Severity::Error,
            "unterminated string literal".to_string(),
            span,
        )
    }
}
