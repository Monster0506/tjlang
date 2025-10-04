//! Comprehensive tests for the TJLang diagnostics system

use codespan::{Files, Span};
use codespan_reporting::diagnostic::Severity;

use crate::collection::DiagnosticCollection;
use crate::diagnostic::{Suggestion, TJLangDiagnostic};
use crate::error_codes::ErrorCode;
use crate::helpers::helpers;
use crate::source_span::SourceSpan;

// Helper function to create test spans
fn create_test_span() -> SourceSpan {
    let mut files = Files::new();
    let file_id = files.add("test.tj", "test content");
    SourceSpan::new(file_id, Span::new(10, 20))
}

fn create_test_span_at(start: usize, end: usize) -> SourceSpan {
    let mut files = Files::new();
    let file_id = files.add("test.tj", "test content");
    SourceSpan::new(file_id, Span::new(start as u32, end as u32))
}

#[cfg(test)]
mod error_code_tests {
    use super::*;

    #[test]
    fn test_error_code_strings() {
        // Test lexer error codes
        assert_eq!(ErrorCode::LexerInvalidCharacter.as_str(), "L0001");
        assert_eq!(ErrorCode::LexerUnterminatedString.as_str(), "L0002");
        assert_eq!(ErrorCode::LexerUnterminatedComment.as_str(), "L0003");
        assert_eq!(ErrorCode::LexerInvalidNumber.as_str(), "L0004");
        assert_eq!(ErrorCode::LexerInvalidEscape.as_str(), "L0005");

        // Test parser error codes
        assert_eq!(ErrorCode::ParserUnexpectedToken.as_str(), "P1000");
        assert_eq!(ErrorCode::ParserExpectedToken.as_str(), "P1001");
        assert_eq!(ErrorCode::ParserUnexpectedEof.as_str(), "P1002");
        assert_eq!(ErrorCode::ParserInvalidExpression.as_str(), "P1003");
        assert_eq!(ErrorCode::ParserInvalidStatement.as_str(), "P1004");
        assert_eq!(ErrorCode::ParserInvalidType.as_str(), "P1005");
        assert_eq!(ErrorCode::ParserInvalidPattern.as_str(), "P1006");
        assert_eq!(ErrorCode::ParserInvalidFunction.as_str(), "P1007");
        assert_eq!(ErrorCode::ParserInvalidStruct.as_str(), "P1008");
        assert_eq!(ErrorCode::ParserInvalidEnum.as_str(), "P1009");
        assert_eq!(ErrorCode::ParserInvalidInterface.as_str(), "P1010");
        assert_eq!(ErrorCode::ParserInvalidModule.as_str(), "P1011");
        assert_eq!(ErrorCode::ParserInvalidImport.as_str(), "P1012");
        assert_eq!(ErrorCode::ParserInvalidExport.as_str(), "P1013");

        // Test analyzer error codes
        assert_eq!(ErrorCode::AnalyzerUndefinedVariable.as_str(), "A2000");
        assert_eq!(ErrorCode::AnalyzerUndefinedFunction.as_str(), "A2001");
        assert_eq!(ErrorCode::AnalyzerUndefinedType.as_str(), "A2002");
        assert_eq!(ErrorCode::AnalyzerTypeMismatch.as_str(), "A2003");
        assert_eq!(ErrorCode::AnalyzerTraitNotImplemented.as_str(), "A2004");
        assert_eq!(ErrorCode::AnalyzerNonExhaustiveMatch.as_str(), "A2005");
        assert_eq!(ErrorCode::AnalyzerDuplicateDefinition.as_str(), "A2006");
        assert_eq!(ErrorCode::AnalyzerCircularDependency.as_str(), "A2007");
        assert_eq!(ErrorCode::AnalyzerInvalidGeneric.as_str(), "A2008");
        assert_eq!(ErrorCode::AnalyzerInvalidTraitBound.as_str(), "A2009");
        assert_eq!(ErrorCode::AnalyzerInvalidInterface.as_str(), "A2010");
        assert_eq!(ErrorCode::AnalyzerInvalidImplementation.as_str(), "A2011");
        assert_eq!(ErrorCode::AnalyzerInvalidModule.as_str(), "A2012");
        assert_eq!(ErrorCode::AnalyzerInvalidImport.as_str(), "A2013");
        assert_eq!(ErrorCode::AnalyzerInvalidExport.as_str(), "A2014");

        // Test codegen error codes
        assert_eq!(ErrorCode::CodegenInvalidType.as_str(), "C3000");
        assert_eq!(ErrorCode::CodegenInvalidExpression.as_str(), "C3001");
        assert_eq!(ErrorCode::CodegenInvalidFunction.as_str(), "C3002");
        assert_eq!(ErrorCode::CodegenInvalidStruct.as_str(), "C3003");
        assert_eq!(ErrorCode::CodegenInvalidEnum.as_str(), "C3004");
        assert_eq!(ErrorCode::CodegenInvalidInterface.as_str(), "C3005");
        assert_eq!(ErrorCode::CodegenInvalidModule.as_str(), "C3006");
        assert_eq!(ErrorCode::CodegenInvalidImport.as_str(), "C3007");
        assert_eq!(ErrorCode::CodegenInvalidExport.as_str(), "C3008");

        // Test runtime error codes
        assert_eq!(ErrorCode::RuntimePanic.as_str(), "R4000");
        assert_eq!(ErrorCode::RuntimeTaskError.as_str(), "R4001");
        assert_eq!(ErrorCode::RuntimeMemoryError.as_str(), "R4002");
        assert_eq!(ErrorCode::RuntimeTypeError.as_str(), "R4003");
        assert_eq!(ErrorCode::RuntimeValueError.as_str(), "R4004");
    }

    #[test]
    fn test_error_code_categories() {
        // Test lexer category
        assert_eq!(ErrorCode::LexerInvalidCharacter.category(), "Lexer");
        assert_eq!(ErrorCode::LexerUnterminatedString.category(), "Lexer");
        assert_eq!(ErrorCode::LexerUnterminatedComment.category(), "Lexer");
        assert_eq!(ErrorCode::LexerInvalidNumber.category(), "Lexer");
        assert_eq!(ErrorCode::LexerInvalidEscape.category(), "Lexer");

        // Test parser category
        assert_eq!(ErrorCode::ParserUnexpectedToken.category(), "Parser");
        assert_eq!(ErrorCode::ParserExpectedToken.category(), "Parser");
        assert_eq!(ErrorCode::ParserUnexpectedEof.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidExpression.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidStatement.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidType.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidPattern.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidFunction.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidStruct.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidEnum.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidInterface.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidModule.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidImport.category(), "Parser");
        assert_eq!(ErrorCode::ParserInvalidExport.category(), "Parser");

        // Test analyzer category
        assert_eq!(ErrorCode::AnalyzerUndefinedVariable.category(), "Analyzer");
        assert_eq!(ErrorCode::AnalyzerUndefinedFunction.category(), "Analyzer");
        assert_eq!(ErrorCode::AnalyzerUndefinedType.category(), "Analyzer");
        assert_eq!(ErrorCode::AnalyzerTypeMismatch.category(), "Analyzer");
        assert_eq!(
            ErrorCode::AnalyzerTraitNotImplemented.category(),
            "Analyzer"
        );
        assert_eq!(ErrorCode::AnalyzerNonExhaustiveMatch.category(), "Analyzer");
        assert_eq!(
            ErrorCode::AnalyzerDuplicateDefinition.category(),
            "Analyzer"
        );
        assert_eq!(ErrorCode::AnalyzerCircularDependency.category(), "Analyzer");
        assert_eq!(ErrorCode::AnalyzerInvalidGeneric.category(), "Analyzer");
        assert_eq!(ErrorCode::AnalyzerInvalidTraitBound.category(), "Analyzer");
        assert_eq!(ErrorCode::AnalyzerInvalidInterface.category(), "Analyzer");
        assert_eq!(
            ErrorCode::AnalyzerInvalidImplementation.category(),
            "Analyzer"
        );
        assert_eq!(ErrorCode::AnalyzerInvalidModule.category(), "Analyzer");
        assert_eq!(ErrorCode::AnalyzerInvalidImport.category(), "Analyzer");
        assert_eq!(ErrorCode::AnalyzerInvalidExport.category(), "Analyzer");

        // Test codegen category
        assert_eq!(ErrorCode::CodegenInvalidType.category(), "Codegen");
        assert_eq!(ErrorCode::CodegenInvalidExpression.category(), "Codegen");
        assert_eq!(ErrorCode::CodegenInvalidFunction.category(), "Codegen");
        assert_eq!(ErrorCode::CodegenInvalidStruct.category(), "Codegen");
        assert_eq!(ErrorCode::CodegenInvalidEnum.category(), "Codegen");
        assert_eq!(ErrorCode::CodegenInvalidInterface.category(), "Codegen");
        assert_eq!(ErrorCode::CodegenInvalidModule.category(), "Codegen");
        assert_eq!(ErrorCode::CodegenInvalidImport.category(), "Codegen");
        assert_eq!(ErrorCode::CodegenInvalidExport.category(), "Codegen");

        // Test runtime category
        assert_eq!(ErrorCode::RuntimePanic.category(), "Runtime");
        assert_eq!(ErrorCode::RuntimeTaskError.category(), "Runtime");
        assert_eq!(ErrorCode::RuntimeMemoryError.category(), "Runtime");
        assert_eq!(ErrorCode::RuntimeTypeError.category(), "Runtime");
        assert_eq!(ErrorCode::RuntimeValueError.category(), "Runtime");
    }

    #[test]
    fn test_error_code_equality() {
        assert_eq!(
            ErrorCode::LexerInvalidCharacter,
            ErrorCode::LexerInvalidCharacter
        );
        assert_ne!(
            ErrorCode::LexerInvalidCharacter,
            ErrorCode::LexerUnterminatedString
        );
        assert_ne!(
            ErrorCode::ParserUnexpectedToken,
            ErrorCode::AnalyzerTypeMismatch
        );
    }

    #[test]
    fn test_error_code_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(ErrorCode::LexerInvalidCharacter, "invalid character");
        map.insert(ErrorCode::ParserUnexpectedToken, "unexpected token");
        map.insert(ErrorCode::AnalyzerTypeMismatch, "type mismatch");

        assert_eq!(
            map.get(&ErrorCode::LexerInvalidCharacter),
            Some(&"invalid character")
        );
        assert_eq!(
            map.get(&ErrorCode::ParserUnexpectedToken),
            Some(&"unexpected token")
        );
        assert_eq!(
            map.get(&ErrorCode::AnalyzerTypeMismatch),
            Some(&"type mismatch")
        );
        assert_eq!(map.get(&ErrorCode::RuntimePanic), None);
    }
}

#[cfg(test)]
mod source_span_tests {
    use super::*;

    #[test]
    fn test_source_span_creation() {
        let mut files = Files::new();
        let file_id = files.add("test.tj", "test content");
        let span = Span::new(10, 20);
        let source_span = SourceSpan::new(file_id, span);

        assert_eq!(source_span.file_id, file_id);
        assert_eq!(source_span.span, span);
    }

    #[test]
    fn test_source_span_methods() {
        let mut files = Files::new();
        let file_id = files.add("test.tj", "test content");
        let span = Span::new(10, 20);
        let source_span = SourceSpan::new(file_id, span);

        assert_eq!(source_span.start(), 10);
        assert_eq!(source_span.end(), 20);
        assert_eq!(source_span.len(), 10);
    }

    #[test]
    fn test_source_span_zero_length() {
        let mut files = Files::new();
        let file_id = files.add("test.tj", "test content");
        let span = Span::new(15, 15);
        let source_span = SourceSpan::new(file_id, span);

        assert_eq!(source_span.start(), 15);
        assert_eq!(source_span.end(), 15);
        assert_eq!(source_span.len(), 0);
    }

    #[test]
    fn test_source_span_equality() {
        let mut files = Files::new();
        let file_id1 = files.add("test1.tj", "test content 1");
        let file_id2 = files.add("test2.tj", "test content 2");
        let span1 = Span::new(10, 20);
        let span2 = Span::new(10, 20);
        let span3 = Span::new(15, 25);

        let source_span1 = SourceSpan::new(file_id1, span1);
        let source_span2 = SourceSpan::new(file_id1, span2);
        let source_span3 = SourceSpan::new(file_id1, span3);
        let source_span4 = SourceSpan::new(file_id2, span1);

        assert_eq!(source_span1, source_span2);
        assert_ne!(source_span1, source_span3);
        assert_ne!(source_span1, source_span4);
    }

    #[test]
    fn test_source_span_clone() {
        let mut files = Files::new();
        let file_id = files.add("test.tj", "test content");
        let span = Span::new(10, 20);
        let source_span = SourceSpan::new(file_id, span);
        let cloned = source_span.clone();

        assert_eq!(source_span, cloned);
    }
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let span = create_test_span();
        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            span,
        );

        assert_eq!(diagnostic.code, ErrorCode::AnalyzerTypeMismatch);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.message, "expected int, found str");
        assert_eq!(diagnostic.primary_span, span);
        assert!(diagnostic.secondary_spans.is_empty());
        assert!(diagnostic.suggestions.is_empty());
        assert!(diagnostic.notes.is_empty());
    }

    #[test]
    fn test_diagnostic_with_secondary_span() {
        let primary_span = create_test_span();
        let secondary_span = create_test_span_at(30, 40);

        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            primary_span,
        )
        .with_secondary_span(secondary_span);

        assert_eq!(diagnostic.secondary_spans.len(), 1);
        assert_eq!(diagnostic.secondary_spans[0], secondary_span);
    }

    #[test]
    fn test_diagnostic_with_suggestion() {
        let span = create_test_span();
        let suggestion = Suggestion::new("change to int".to_string(), "int".to_string(), span);

        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            span,
        )
        .with_suggestion(suggestion.clone());

        assert_eq!(diagnostic.suggestions.len(), 1);
        assert_eq!(diagnostic.suggestions[0], suggestion);
    }

    #[test]
    fn test_diagnostic_with_note() {
        let span = create_test_span();
        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            span,
        )
        .with_note("consider using a type conversion".to_string());

        assert_eq!(diagnostic.notes.len(), 1);
        assert_eq!(diagnostic.notes[0], "consider using a type conversion");
    }

    #[test]
    fn test_diagnostic_chaining() {
        let primary_span = create_test_span();
        let secondary_span = create_test_span_at(30, 40);
        let suggestion =
            Suggestion::new("change to int".to_string(), "int".to_string(), primary_span);

        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            primary_span,
        )
        .with_secondary_span(secondary_span)
        .with_suggestion(suggestion.clone())
        .with_note("consider using a type conversion".to_string());

        assert_eq!(diagnostic.secondary_spans.len(), 1);
        assert_eq!(diagnostic.suggestions.len(), 1);
        assert_eq!(diagnostic.notes.len(), 1);
        assert_eq!(diagnostic.secondary_spans[0], secondary_span);
        assert_eq!(diagnostic.suggestions[0], suggestion);
        assert_eq!(diagnostic.notes[0], "consider using a type conversion");
    }

    #[test]
    fn test_diagnostic_display() {
        let span = create_test_span();
        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            span,
        );

        assert_eq!(
            format!("{}", diagnostic),
            "error[A2003]: expected int, found str"
        );
    }

    #[test]
    fn test_diagnostic_codespan_conversion() {
        let span = create_test_span();
        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            span,
        );

        let codespan_diagnostic = diagnostic.to_codespan_diagnostic();
        assert_eq!(codespan_diagnostic.severity, Severity::Error);
        assert_eq!(codespan_diagnostic.code, Some("A2003".to_string()));
        assert_eq!(codespan_diagnostic.message, "expected int, found str");
    }
}

#[cfg(test)]
mod suggestion_tests {
    use super::*;

    #[test]
    fn test_suggestion_creation() {
        let span = create_test_span();
        let suggestion = Suggestion::new("change to int".to_string(), "int".to_string(), span);

        assert_eq!(suggestion.message, "change to int");
        assert_eq!(suggestion.replacement, "int");
        assert_eq!(suggestion.span, span);
    }

    #[test]
    fn test_suggestion_equality() {
        let span1 = create_test_span();
        let span2 = create_test_span();
        let span3 = create_test_span_at(30, 40);

        let suggestion1 = Suggestion::new("change to int".to_string(), "int".to_string(), span1);
        let suggestion2 = Suggestion::new("change to int".to_string(), "int".to_string(), span2);
        let suggestion3 = Suggestion::new("change to str".to_string(), "str".to_string(), span1);
        let suggestion4 = Suggestion::new("change to int".to_string(), "int".to_string(), span3);

        assert_eq!(suggestion1, suggestion2);
        assert_ne!(suggestion1, suggestion3);
        assert_ne!(suggestion1, suggestion4);
    }
}

#[cfg(test)]
mod collection_tests {
    use super::*;

    #[test]
    fn test_empty_collection() {
        let collection = DiagnosticCollection::new();
        assert!(collection.is_empty());
        assert!(!collection.has_errors());
        assert!(!collection.has_warnings());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_add_diagnostic() {
        let mut collection = DiagnosticCollection::new();
        let span = create_test_span();
        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            span,
        );

        collection.add(diagnostic);

        assert!(!collection.is_empty());
        assert!(collection.has_errors());
        assert!(!collection.has_warnings());
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn test_add_error() {
        let mut collection = DiagnosticCollection::new();
        let span = create_test_span();

        collection.add_error(
            ErrorCode::AnalyzerTypeMismatch,
            "expected int, found str".to_string(),
            span,
        );

        assert!(!collection.is_empty());
        assert!(collection.has_errors());
        assert!(!collection.has_warnings());
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn test_add_warning() {
        let mut collection = DiagnosticCollection::new();
        let span = create_test_span();

        collection.add_warning(
            ErrorCode::AnalyzerTypeMismatch,
            "expected int, found str".to_string(),
            span,
        );

        assert!(!collection.is_empty());
        assert!(!collection.has_errors());
        assert!(collection.has_warnings());
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn test_add_info() {
        let mut collection = DiagnosticCollection::new();
        let span = create_test_span();

        collection.add_info(
            ErrorCode::AnalyzerTypeMismatch,
            "expected int, found str".to_string(),
            span,
        );

        assert!(!collection.is_empty());
        assert!(!collection.has_errors());
        assert!(!collection.has_warnings());
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn test_multiple_diagnostics() {
        let mut collection = DiagnosticCollection::new();
        let span1 = create_test_span();
        let span2 = create_test_span_at(30, 40);

        collection.add_error(
            ErrorCode::AnalyzerTypeMismatch,
            "expected int, found str".to_string(),
            span1,
        );
        collection.add_warning(
            ErrorCode::AnalyzerUndefinedVariable,
            "undefined variable 'x'".to_string(),
            span2,
        );

        assert!(!collection.is_empty());
        assert!(collection.has_errors());
        assert!(collection.has_warnings());
        assert_eq!(collection.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut collection = DiagnosticCollection::new();
        let span = create_test_span();

        collection.add_error(
            ErrorCode::AnalyzerTypeMismatch,
            "expected int, found str".to_string(),
            span,
        );

        assert!(!collection.is_empty());
        collection.clear();
        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_iteration() {
        let mut collection = DiagnosticCollection::new();
        let span1 = create_test_span();
        let span2 = create_test_span_at(30, 40);

        collection.add_error(
            ErrorCode::AnalyzerTypeMismatch,
            "expected int, found str".to_string(),
            span1,
        );
        collection.add_warning(
            ErrorCode::AnalyzerUndefinedVariable,
            "undefined variable 'x'".to_string(),
            span2,
        );

        let diagnostics: Vec<_> = collection.iter().collect();
        assert_eq!(diagnostics.len(), 2);

        let error_count = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count();
        let warning_count = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .count();

        assert_eq!(error_count, 1);
        assert_eq!(warning_count, 1);
    }

    #[test]
    fn test_into_iter() {
        let mut collection = DiagnosticCollection::new();
        let span = create_test_span();

        collection.add_error(
            ErrorCode::AnalyzerTypeMismatch,
            "expected int, found str".to_string(),
            span,
        );

        let diagnostics: Vec<_> = collection.into_iter().collect();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, ErrorCode::AnalyzerTypeMismatch);
    }
}

#[cfg(test)]
mod helpers_tests {
    use super::*;

    #[test]
    fn test_type_mismatch_helper() {
        let span = create_test_span();
        let diagnostic = helpers::type_mismatch("int", "str", span);

        assert_eq!(diagnostic.code, ErrorCode::AnalyzerTypeMismatch);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.message, "expected `int`, found `str`");
        assert_eq!(diagnostic.primary_span, span);
    }

    #[test]
    fn test_undefined_variable_helper() {
        let span = create_test_span();
        let diagnostic = helpers::undefined_variable("x", span);

        assert_eq!(diagnostic.code, ErrorCode::AnalyzerUndefinedVariable);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.message, "undefined variable `x`");
        assert_eq!(diagnostic.primary_span, span);
    }

    #[test]
    fn test_undefined_function_helper() {
        let span = create_test_span();
        let diagnostic = helpers::undefined_function("foo", span);

        assert_eq!(diagnostic.code, ErrorCode::AnalyzerUndefinedFunction);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.message, "undefined function `foo`");
        assert_eq!(diagnostic.primary_span, span);
    }

    #[test]
    fn test_non_exhaustive_match_helper() {
        let span = create_test_span();
        let missing_patterns = vec!["Some(_)".to_string(), "None".to_string()];
        let diagnostic = helpers::non_exhaustive_match(missing_patterns, span);

        assert_eq!(diagnostic.code, ErrorCode::AnalyzerNonExhaustiveMatch);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(
            diagnostic.message,
            "non-exhaustive match: missing patterns: Some(_), None"
        );
        assert_eq!(diagnostic.primary_span, span);
    }

    #[test]
    fn test_unexpected_token_helper() {
        let span = create_test_span();
        let diagnostic = helpers::unexpected_token("int", "str", span);

        assert_eq!(diagnostic.code, ErrorCode::ParserUnexpectedToken);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.message, "expected `int`, found `str`");
        assert_eq!(diagnostic.primary_span, span);
    }

    #[test]
    fn test_unterminated_string_helper() {
        let span = create_test_span();
        let diagnostic = helpers::unterminated_string(span);

        assert_eq!(diagnostic.code, ErrorCode::LexerUnterminatedString);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.message, "unterminated string literal");
        assert_eq!(diagnostic.primary_span, span);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_diagnostic_workflow() {
        // Create a diagnostic collection
        let mut collection = DiagnosticCollection::new();

        // Add various types of diagnostics
        let span1 = create_test_span();
        let span2 = create_test_span_at(30, 40);
        let span3 = create_test_span_at(50, 60);

        // Add a type mismatch error with suggestion
        let suggestion = Suggestion::new("change to int".to_string(), "int".to_string(), span1);
        let diagnostic1 = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            span1,
        )
        .with_suggestion(suggestion)
        .with_note("consider using a type conversion".to_string());

        collection.add(diagnostic1);

        // Add an undefined variable warning
        collection.add_warning(
            ErrorCode::AnalyzerUndefinedVariable,
            "undefined variable 'x'".to_string(),
            span2,
        );

        // Add a parser error
        collection.add_error(
            ErrorCode::ParserUnexpectedToken,
            "expected ';', found '}'".to_string(),
            span3,
        );

        // Verify the collection
        assert!(!collection.is_empty());
        assert!(collection.has_errors());
        assert!(collection.has_warnings());
        assert_eq!(collection.len(), 3);

        // Test iteration
        let diagnostics: Vec<_> = collection.iter().collect();
        assert_eq!(diagnostics.len(), 3);

        // Count by severity
        let error_count = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count();
        let warning_count = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .count();

        assert_eq!(error_count, 2);
        assert_eq!(warning_count, 1);

        // Test specific diagnostics
        let type_mismatch = diagnostics
            .iter()
            .find(|d| d.code == ErrorCode::AnalyzerTypeMismatch)
            .unwrap();
        assert_eq!(type_mismatch.suggestions.len(), 1);
        assert_eq!(type_mismatch.notes.len(), 1);
        assert_eq!(type_mismatch.suggestions[0].message, "change to int");
        assert_eq!(type_mismatch.notes[0], "consider using a type conversion");
    }

    #[test]
    fn test_diagnostic_display_formatting() {
        let span = create_test_span();
        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::AnalyzerTypeMismatch,
            Severity::Error,
            "expected int, found str".to_string(),
            span,
        );

        let formatted = format!("{}", diagnostic);
        assert_eq!(formatted, "error[A2003]: expected int, found str");
    }

    #[test]
    fn test_error_code_consistency() {
        // Ensure all error codes have unique strings
        let mut codes = Vec::new();

        // Collect all error code strings
        let error_codes = [
            ErrorCode::LexerInvalidCharacter,
            ErrorCode::LexerUnterminatedString,
            ErrorCode::LexerUnterminatedComment,
            ErrorCode::LexerInvalidNumber,
            ErrorCode::LexerInvalidEscape,
            ErrorCode::ParserUnexpectedToken,
            ErrorCode::ParserExpectedToken,
            ErrorCode::ParserUnexpectedEof,
            ErrorCode::ParserInvalidExpression,
            ErrorCode::ParserInvalidStatement,
            ErrorCode::ParserInvalidType,
            ErrorCode::ParserInvalidPattern,
            ErrorCode::ParserInvalidFunction,
            ErrorCode::ParserInvalidStruct,
            ErrorCode::ParserInvalidEnum,
            ErrorCode::ParserInvalidInterface,
            ErrorCode::ParserInvalidModule,
            ErrorCode::ParserInvalidImport,
            ErrorCode::ParserInvalidExport,
            ErrorCode::AnalyzerUndefinedVariable,
            ErrorCode::AnalyzerUndefinedFunction,
            ErrorCode::AnalyzerUndefinedType,
            ErrorCode::AnalyzerTypeMismatch,
            ErrorCode::AnalyzerTraitNotImplemented,
            ErrorCode::AnalyzerNonExhaustiveMatch,
            ErrorCode::AnalyzerDuplicateDefinition,
            ErrorCode::AnalyzerCircularDependency,
            ErrorCode::AnalyzerInvalidGeneric,
            ErrorCode::AnalyzerInvalidTraitBound,
            ErrorCode::AnalyzerInvalidInterface,
            ErrorCode::AnalyzerInvalidImplementation,
            ErrorCode::AnalyzerInvalidModule,
            ErrorCode::AnalyzerInvalidImport,
            ErrorCode::AnalyzerInvalidExport,
            ErrorCode::CodegenInvalidType,
            ErrorCode::CodegenInvalidExpression,
            ErrorCode::CodegenInvalidFunction,
            ErrorCode::CodegenInvalidStruct,
            ErrorCode::CodegenInvalidEnum,
            ErrorCode::CodegenInvalidInterface,
            ErrorCode::CodegenInvalidModule,
            ErrorCode::CodegenInvalidImport,
            ErrorCode::CodegenInvalidExport,
            ErrorCode::RuntimePanic,
            ErrorCode::RuntimeTaskError,
            ErrorCode::RuntimeMemoryError,
            ErrorCode::RuntimeTypeError,
            ErrorCode::RuntimeValueError,
        ];

        for code in &error_codes {
            codes.push(code.as_str());
        }

        // Check for duplicates
        let mut sorted_codes = codes.clone();
        sorted_codes.sort();
        sorted_codes.dedup();

        assert_eq!(
            codes.len(),
            sorted_codes.len(),
            "Duplicate error codes found"
        );
    }
}
