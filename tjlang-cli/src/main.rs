//! TJLang CLI
//!
//! Command-line interface for the TJLang advanced interpreter.

use clap::{Parser, Subcommand};
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};
use codespan_reporting::diagnostic::Severity;
use std::path::PathBuf;
use tjlang_diagnostics::debug_println;
use tjlang_diagnostics::utils::debug;
use tjlang_diagnostics::{TJLangDiagnostic, ErrorCode, SourceSpan, DiagnosticCollection};
use tjlang_lexer::lex;
use tjlang_parser::parse;
use tjlang_runtime::Interpreter;

/// TJLang - Advanced Programming Language Interpreter
#[derive(Parser)]
#[command(name = "tjlang")]
#[command(about = "TJLang - Advanced Programming Language Interpreter")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a TJLang program
    Run {
        /// Path to the TJLang file
        file: PathBuf,
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            debug,
            verbose,
        } => {
            run_program(&file, debug, verbose)?;
        }
    }

    Ok(())
}

/// Run a TJLang program
fn run_program(
    file: &PathBuf,
    debug: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    debug_println!(" Running TJLang program: {}", file.display());

    if verbose {
        debug_println!(" File: {}", file.display());
        debug_println!(" Debug mode: {}", debug);
        debug_println!(" Verbose mode: {}", verbose);
    }

    // Create a file system for diagnostics
    use codespan::{Files, Span};
    let mut files: Files<String> = Files::new();
    
    // Read the source file
    let source = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            // Create a dummy file entry for error reporting
            let file_id = files.add(file.to_string_lossy().to_string(), String::new());
            let span = SourceSpan::new(file_id, Span::from(0..0));
            
            let (code, message, notes) = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    (
                        ErrorCode::FileNotFound,
                        format!("File not found: {}", file.display()),
                        vec![
                            "Please check that the file exists and the path is correct.".to_string(),
                            format!("Looked for file at: {}", file.canonicalize().unwrap_or(file.to_path_buf()).display()),
                        ]
                    )
                }
                std::io::ErrorKind::PermissionDenied => {
                    (
                        ErrorCode::FilePermissionDenied,
                        format!("Permission denied: {}", file.display()),
                        vec![
                            "You don't have permission to read this file.".to_string(),
                            "Check the file permissions and try again.".to_string(),
                        ]
                    )
                }
                _ => {
                    (
                        ErrorCode::FileReadError,
                        format!("Failed to read file: {}", file.display()),
                        vec![format!("IO error: {}", e)]
                    )
                }
            };
            
            let mut diagnostic = TJLangDiagnostic::new(
                code,
                Severity::Error,
                message,
                span,
            );
            
            for note in notes {
                diagnostic = diagnostic.with_note(note);
            }
            
            let mut diagnostics = DiagnosticCollection::new();
            diagnostics.add(diagnostic);
            
            display_diagnostics(&files, &diagnostics).ok();
            std::process::exit(1);
        }
    };

    if verbose {
        debug_println!(" Source code ({} bytes):", source.len());
        debug_println!("{}", source);
        debug_println!("---");
    }

    // Add the source to the files (already created above for error handling)
    let file_id = files.add(file.to_string_lossy().to_string(), source.clone());

    // Lex the source
    if verbose {
        debug_println!(" Lexing source...");
    }
    let (tokens, diagnostics) = lex(&source, file_id);

    if debug {
        debug_println!(" Tokens:");
        for token in &tokens {
            debug_println!("  {:?}", token);
        }
    }

    // Parse the source
    if verbose {
        debug_println!(" Parsing tokens...");
    }
    let (ast, parse_diagnostics) = match parse(&source, file_id) {
        Ok((ast, diagnostics)) => {
            debug_println!("[DEBUG] Parse successful! AST units: {}", ast.units.len());
            for (i, unit) in ast.units.iter().enumerate() {
                debug_println!("  Unit {}: {:?}", i, std::mem::discriminant(unit));
            }
            (ast, diagnostics)
        }
        Err(diagnostics) => {
            debug_println!(" Parse failed with {} diagnostics", diagnostics.len());

            // Display diagnostics using codespan-reporting
            if !diagnostics.is_empty() {
                eprintln!("Parse Error: Failed to parse {}", file.display());
                eprintln!();
                display_diagnostics(&files, &diagnostics)?;
            } else {
                eprintln!("Parse Error: Failed to parse {} (no diagnostic information available)", file.display());
            }
            std::process::exit(1);
        }
    };

    if debug {
        debug_println!(" AST:");
        debug_println!("{:#?}", ast);
    }

    // Enable debug mode if debug flag is set
    if debug {
        debug::enable();
    }

    // Interpret the program
    if verbose {
        debug_println!(" Running interpreter...");
    }
    debug_println!("[DEBUG] Creating interpreter...");
    let mut interpreter = Interpreter::new();
    debug_println!("[DEBUG] Interpreter created successfully");
    debug_println!("[DEBUG] Starting program interpretation...");
    debug_println!("[DEBUG] AST units: {}", ast.units.len());
    for (i, unit) in ast.units.iter().enumerate() {
        debug_println!("[DEBUG] Unit {}: {:?}", i, std::mem::discriminant(unit));
    }

    debug_println!("[DEBUG] About to call interpret_program...");

    let result = match interpreter.interpret_program(&ast) {
        Ok(result) => {
            debug_println!("[DEBUG] Program completed successfully!");
            debug_println!(" Result type: {:?}", std::mem::discriminant(&result));
            debug_println!(" Result: {}", result.to_string());
            result
        }
        Err(e) => {
            debug_println!(" Program execution failed: {}", e);
            eprintln!("Runtime Error: {}", e);
            eprintln!();
            eprintln!("The program failed during execution.");
            eprintln!("Run with --debug flag for more detailed information.");
            std::process::exit(1);
        }
    };

    debug_println!("[DEBUG] After interpret_program call");

    Ok(())
}

/// Display diagnostics using codespan-reporting
fn display_diagnostics(
    files: &codespan::Files<String>,
    diagnostics: &tjlang_diagnostics::DiagnosticCollection,
) -> Result<(), Box<dyn std::error::Error>> {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    for diagnostic in diagnostics.iter() {
        let codespan_diagnostic = diagnostic.to_codespan_diagnostic();
        term::emit(&mut writer.lock(), &config, files, &codespan_diagnostic)?;
        println!();
    }

    Ok(())
}
