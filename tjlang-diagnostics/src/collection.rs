//! Diagnostic collection and management

use codespan_reporting::diagnostic::Severity;

use crate::diagnostic::TJLangDiagnostic;
use crate::error_codes::ErrorCode;
use crate::source_span::SourceSpan;

/// A collection of diagnostics
#[derive(Debug, Clone, Default)]
pub struct DiagnosticCollection {
    diagnostics: Vec<TJLangDiagnostic>,
}

impl DiagnosticCollection {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: TJLangDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn add_error(&mut self, code: ErrorCode, message: String, primary_span: SourceSpan) {
        self.add(TJLangDiagnostic::new(
            code,
            Severity::Error,
            message,
            primary_span,
        ));
    }

    pub fn add_warning(&mut self, code: ErrorCode, message: String, primary_span: SourceSpan) {
        self.add(TJLangDiagnostic::new(
            code,
            Severity::Warning,
            message,
            primary_span,
        ));
    }

    pub fn add_info(&mut self, code: ErrorCode, message: String, primary_span: SourceSpan) {
        self.add(TJLangDiagnostic::new(
            code,
            Severity::Note,
            message,
            primary_span,
        ));
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Warning)
    }

    pub fn iter(&self) -> std::slice::Iter<TJLangDiagnostic> {
        self.diagnostics.iter()
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    /// Merge another diagnostic collection into this one
    pub fn merge(&mut self, other: DiagnosticCollection) {
        self.diagnostics.extend(other.diagnostics);
    }

    /// Get the count of diagnostics
    pub fn count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Get diagnostics by severity
    pub fn get_diagnostics_by_severity(&self, severity: Severity) -> Vec<&TJLangDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == severity)
            .collect()
    }
}

impl IntoIterator for DiagnosticCollection {
    type Item = TJLangDiagnostic;
    type IntoIter = std::vec::IntoIter<TJLangDiagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}
