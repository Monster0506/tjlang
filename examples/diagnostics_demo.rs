//! TJLang Diagnostics System Demo
//! 
//! This demonstrates the rich error reporting capabilities of the TJLang compiler.

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
    println!("🔍 TJLang Diagnostics System Demo\n");
    
    // Create a file map for our demo source code
    let mut files = Files::new();
    let file_id = files.add("demo.tj", include_str!("demo.tj"));
    
    // Create a collection of diagnostics
    let mut diagnostics = DiagnosticCollection::new();
    
    // Demo 1: Type Mismatch Error with Suggestion
    println!("📝 Demo 1: Type Mismatch Error");
    println!("Source: x: int = \"hello\"");
    println!();
    
    let span = SourceSpan::new(file_id, Span::new(120, 127)); // "hello" in demo.tj
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
    
    // Demo 2: Undefined Variable Error
    println!("📝 Demo 2: Undefined Variable Error");
    println!("Source: print(unknown_var)");
    println!();
    
    let span = SourceSpan::new(file_id, Span::new(160, 172)); // "unknown_var"
    let undefined_error = helpers::undefined_variable("unknown_var", span);
    diagnostics.add(undefined_error);
    
    // Demo 3: Non-exhaustive Match Error
    println!("📝 Demo 3: Non-exhaustive Match Error");
    println!("Source: match option {{ Some(x) => x }}");
    println!();
    
    let span = SourceSpan::new(file_id, Span::new(205, 217)); // "match option"
    let missing_patterns = vec!["None".to_string()];
    let match_error = helpers::non_exhaustive_match(missing_patterns, span);
    diagnostics.add(match_error);
    
    // Demo 4: Parser Error with Multiple Spans
    println!("📝 Demo 4: Parser Error with Multiple Spans");
    println!("Source: def func( -> int {{ return 42 }}");
    println!();
    
    let primary_span = SourceSpan::new(file_id, Span::new(285, 287)); // "->"
    let secondary_span = SourceSpan::new(file_id, Span::new(284, 285)); // "("
    let parser_error = TJLangDiagnostic::new(
        ErrorCode::ParserExpectedToken,
        Severity::Error,
        "expected parameter name, found `->`".to_string(),
        primary_span,
    )
    .with_secondary_span(secondary_span)
    .with_note("function parameters must have names".to_string());
    
    diagnostics.add(parser_error);
    
    // Demo 5: Warning with Multiple Suggestions
    println!("📝 Demo 5: Warning with Multiple Suggestions");
    println!("Source: unused_var: int = 42");
    println!();
    
    let span = SourceSpan::new(file_id, Span::new(350, 361)); // "unused_var"
    let suggestion1 = tjlang_diagnostics::Suggestion::new(
        "prefix with underscore".to_string(),
        "_unused_var".to_string(),
        span,
    );
    let suggestion2 = tjlang_diagnostics::Suggestion::new(
        "remove unused variable".to_string(),
        "".to_string(),
        span,
    );
    
    let warning = TJLangDiagnostic::new(
        ErrorCode::AnalyzerUndefinedVariable, // Using as warning
        Severity::Warning,
        "variable `unused_var` is never used".to_string(),
        span,
    )
    .with_suggestion(suggestion1)
    .with_suggestion(suggestion2)
    .with_note("consider removing or using the variable".to_string());
    
    diagnostics.add(warning);
    
    // Demo 6: Lexer Error
    println!("📝 Demo 6: Lexer Error");
    println!("Source: str = \"unterminated string");
    println!();
    
    let span = SourceSpan::new(file_id, Span::new(411, 424)); // "unterminated"
    let lexer_error = helpers::unterminated_string(span);
    diagnostics.add(lexer_error);
    
    // Display all diagnostics using codespan-reporting
    println!("🎨 Formatted Output:\n");
    println!("{}", "=".repeat(80));
    
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    
    for diagnostic in diagnostics.iter() {
        let codespan_diagnostic = diagnostic.to_codespan_diagnostic();
        term::emit(&mut writer.lock(), &config, &files, &codespan_diagnostic)?;
        println!();
    }
    
    println!("{}", "=".repeat(80));
    
    // Summary
    println!("\n📊 Summary:");
    println!("Total diagnostics: {}", diagnostics.len());
    println!("Errors: {}", diagnostics.iter().filter(|d| d.severity == Severity::Error).count());
    println!("Warnings: {}", diagnostics.iter().filter(|d| d.severity == Severity::Warning).count());
    println!("Notes: {}", diagnostics.iter().filter(|d| d.severity == Severity::Note).count());
    
    Ok(())
}
