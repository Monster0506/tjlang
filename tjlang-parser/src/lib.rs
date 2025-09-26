//! TJLang Parser
//! 
//! Pest-based parser for TJLang source code.

pub mod parser;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod ast_conversion_tests;

/// Main pest parser
pub use parser::PestParser;

/// Parse TJLang source code into an AST using pest
pub fn parse(source: &str, file_id: codespan::FileId) -> Result<(tjlang_ast::Program, tjlang_diagnostics::DiagnosticCollection), tjlang_diagnostics::DiagnosticCollection> {
    let mut parser = PestParser::new();
    
    match parser.parse(source) {
        Ok(program) => {
            // Only return error if there are actual errors, not warnings
            let has_errors = parser.diagnostics.iter().any(|d| d.severity == codespan_reporting::diagnostic::Severity::Error);
            if has_errors {
                Err(parser.diagnostics)
            } else {
                Ok((program, parser.diagnostics))
            }
        },
        Err(e) => {
            // Add parse error to diagnostics
            let span = tjlang_diagnostics::SourceSpan::new(
                file_id,
                codespan::Span::new(0, source.len() as u32)
            );
            parser.diagnostics.add_error(
                tjlang_diagnostics::ErrorCode::ParserInvalidType,
                format!("Parse error: {}", e),
                span,
            );
            Err(parser.diagnostics)
        }
    }
}
