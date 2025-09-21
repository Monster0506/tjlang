//! Diagnostic representation and reporting

use codespan::FileId;
use codespan_reporting::diagnostic::{Diagnostic, Label, Severity};
use std::fmt;

use crate::error_codes::ErrorCode;
use crate::source_span::SourceSpan;

/// A diagnostic suggestion
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Suggestion {
    pub message: String,
    pub replacement: String,
    pub span: SourceSpan,
}

impl Suggestion {
    pub fn new(message: String, replacement: String, span: SourceSpan) -> Self {
        Self {
            message,
            replacement,
            span,
        }
    }
}

/// A TJLang diagnostic
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TJLangDiagnostic {
    pub code: ErrorCode,
    pub severity: Severity,
    pub message: String,
    pub primary_span: SourceSpan,
    pub secondary_spans: Vec<SourceSpan>,
    pub suggestions: Vec<Suggestion>,
    pub notes: Vec<String>,
}

impl TJLangDiagnostic {
    pub fn new(
        code: ErrorCode,
        severity: Severity,
        message: String,
        primary_span: SourceSpan,
    ) -> Self {
        Self {
            code,
            severity,
            message,
            primary_span,
            secondary_spans: Vec::new(),
            suggestions: Vec::new(),
            notes: Vec::new(),
        }
    }
    
    pub fn with_secondary_span(mut self, span: SourceSpan) -> Self {
        self.secondary_spans.push(span);
        self
    }
    
    pub fn with_suggestion(mut self, suggestion: Suggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }
    
    pub fn with_note(mut self, note: String) -> Self {
        self.notes.push(note);
        self
    }
    
    /// Convert to a codespan Diagnostic for reporting
    pub fn to_codespan_diagnostic(&self) -> Diagnostic<FileId> {
        let mut diagnostic = Diagnostic::new(self.severity)
            .with_code(self.code.as_str())
            .with_message(&self.message)
            .with_labels(vec![Label::primary(
                self.primary_span.file_id,
                self.primary_span.span,
            )]);
        
        // Add secondary labels
        for span in &self.secondary_spans {
            diagnostic = diagnostic.with_labels(vec![Label::secondary(
                span.file_id,
                span.span,
            )]);
        }
        
        // Add suggestions as notes
        for suggestion in &self.suggestions {
            diagnostic = diagnostic.with_notes(vec![format!(
                "suggestion: {}",
                suggestion.message
            )]);
        }
        
        // Add additional notes
        for note in &self.notes {
            diagnostic = diagnostic.with_notes(vec![note.clone()]);
        }
        
        diagnostic
    }
}

impl fmt::Display for TJLangDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error[{}]: {}",
            self.code.as_str(),
            self.message
        )
    }
}

impl std::error::Error for TJLangDiagnostic {}
