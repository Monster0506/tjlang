//! TJLang Lexer Demo
//! 
//! This demonstrates the lexer working with the diagnostics system.

use codespan::{Files, Span};
use tjlang_lexer::lex;
use tjlang_diagnostics::{
    ErrorCode, SourceSpan, TJLangDiagnostic, DiagnosticCollection,
    diagnostic_helpers as helpers,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 TJLang Lexer Demo\n");
    
    // Create a file map for our demo source code
    let mut files = Files::new();
    let file_id = files.add("demo.tj", include_str!("demo.tj"));
    
    // Lex the source code
    let (tokens, diagnostics) = lex(include_str!("demo.tj"), file_id);
    
    println!("📝 Tokens found:");
    println!("{}", "=".repeat(50));
    
    for (i, token) in tokens.iter().enumerate() {
        println!("{:3}: {:20} | {:?}", i, token.text, token.kind);
    }
    
    println!("{}", "=".repeat(50));
    println!("\n📊 Summary:");
    println!("Total tokens: {}", tokens.len());
    println!("Diagnostics: {}", diagnostics.len());
    
    if !diagnostics.is_empty() {
        println!("\n🚨 Diagnostics:");
        for diagnostic in diagnostics.iter() {
            println!("  - {}", diagnostic);
        }
    }
    
    Ok(())
}
