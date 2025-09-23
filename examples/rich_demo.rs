//! Rich TJLang Diagnostics Demo
//! 
//! Shows the full formatted output with colors and suggestions

use codespan::{Files, Span};
use codespan_reporting::{
    diagnostic::{Diagnostic, Label, Severity},
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

use tjlang_diagnostics::{
    ErrorCode, SourceSpan, TJLangDiagnostic, DiagnosticCollection,
    diagnostic_helpers as helpers,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 TJLang Rich Diagnostics Demo\n");
    
    // Create source code with various errors
    let source_code = r#"// TJLang source with errors
x: int = "hello"  // Type mismatch
print(unknown_var)  // Undefined variable
match option {  // Non-exhaustive match
    Some(x) => x
}
def func( -> int {  // Missing parameter
    return 42
}
str = "unterminated  // Unterminated string"#;
    
    let mut files = Files::new();
    let file_id = files.add("example.tj", source_code);
    
    let mut diagnostics = DiagnosticCollection::new();
    
    // 1. Type mismatch with suggestion - "hello" on line 2
    let span = SourceSpan::new(file_id, Span::new(38, 45)); // "hello"
    let suggestion = tjlang_diagnostics::Suggestion::new(
        "change to int literal".to_string(),
        "42".to_string(),
        span,
    );
    let type_error = TJLangDiagnostic::new(
        ErrorCode::AnalyzerTypeMismatch,
        Severity::Error,
        "expected `int`, found `str`".to_string(),
        span,
    )
    .with_suggestion(suggestion)
    .with_note("consider using a type conversion".to_string());
    diagnostics.add(type_error);
    
    // 2. Undefined variable - "unknown_var" on line 3
    let span = SourceSpan::new(file_id, Span::new(70, 82)); // "unknown_var"
    let undefined_error = helpers::undefined_variable("unknown_var", span);
    diagnostics.add(undefined_error);
    
    // 3. Non-exhaustive match - "match option" on line 4
    let span = SourceSpan::new(file_id, Span::new(106, 118)); // "match option"
    let missing_patterns = vec!["None".to_string()];
    let match_error = helpers::non_exhaustive_match(missing_patterns, span);
    diagnostics.add(match_error);
    
    // 4. Parser error with secondary span - "->" on line 7
    let primary_span = SourceSpan::new(file_id, Span::new(175, 177)); // "->"
    let secondary_span = SourceSpan::new(file_id, Span::new(174, 175)); // "("
    let parser_error = TJLangDiagnostic::new(
        ErrorCode::ParserExpectedToken,
        Severity::Error,
        "expected parameter name, found `->`".to_string(),
        primary_span,
    )
    .with_secondary_span(secondary_span)
    .with_note("function parameters must have names".to_string());
    diagnostics.add(parser_error);
    
    // 5. Lexer error - unterminated string on line 10
    let span = SourceSpan::new(file_id, Span::new(229, 242)); // "unterminated"
    let lexer_error = helpers::unterminated_string(span);
    diagnostics.add(lexer_error);
    
    // Display with rich formatting
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    
    for diagnostic in diagnostics.iter() {
        let codespan_diagnostic = diagnostic.to_codespan_diagnostic();
        term::emit(&mut writer.lock(), &config, &files, &codespan_diagnostic)?;
        println!();
    }
    
    Ok(())
}
