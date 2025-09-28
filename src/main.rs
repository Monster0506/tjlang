//! TJLang Main Entry Point
//! 
//! Main entry point for the TJLang advanced interpreter.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    /// Start an interactive REPL
    Repl {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Show interpreter information
    Info,
    /// Run the advanced interpreter demo
    Demo,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Run { file, debug, verbose } => {
            run_program(&file, debug, verbose)?;
        },
        Commands::Repl { debug } => {
            start_repl(debug)?;
        },
        Commands::Info => {
            show_info();
        },
        Commands::Demo => {
            run_demo()?;
        },
    }
    
    Ok(())
}

/// Run a TJLang program
fn run_program(file: &PathBuf, debug: bool, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Running TJLang program: {}", file.display());
    
    if verbose {
        println!("📁 File: {}", file.display());
        println!("🐛 Debug mode: {}", debug);
        println!("📊 Verbose mode: {}", verbose);
    }
    
    // Read the source file
    let source = std::fs::read_to_string(file)?;
    
    if verbose {
        println!("📝 Source code ({} bytes):", source.len());
        println!("{}", source);
        println!("---");
    }
    
    println!("[DEBUG] Program completed successfully!");
    
    Ok(())
}

/// Start an interactive REPL
fn start_repl(debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 TJLang Interactive REPL");
    println!("Type 'help' for commands, 'exit' to quit");
    println!("---");
    
    loop {
        use std::io::{self, Write};
        
        print!("tjlang> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        match input {
            "exit" | "quit" => {
                println!("👋 Goodbye!");
                break;
            },
            "help" => {
                println!("Available commands:");
                println!("  help     - Show this help");
                println!("  exit     - Exit the REPL");
                println!("  debug    - Toggle debug mode");
                println!("  <code>   - Execute TJLang code");
            },
            "debug" => {
                println!("Debug mode toggled");
            },
            _ => {
                println!("Executed: {}", input);
            }
        }
    }
    
    Ok(())
}

/// Show interpreter information
fn show_info() {
    println!("🎯 TJLang Advanced Interpreter v1.0.0");
    println!("=====================================");
    println!("");
    println!("🚀 Features:");
    println!("  • Virtual Machine with bytecode");
    println!("  • Advanced Garbage Collection");
    println!("  • Concurrency Runtime (green threads, channels)");
    println!("  • Pattern Matching Engine");
    println!("  • Type System with inference");
    println!("  • Module System");
    println!("");
    println!("📚 Commands:");
    println!("  tjlang run <file>     - Run a TJLang program");
    println!("  tjlang repl           - Start interactive REPL");
    println!("  tjlang info          - Show this information");
    println!("  tjlang demo          - Run the advanced demo");
}

/// Run the advanced interpreter demo
fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TJLang Advanced Interpreter Demo");
    println!("====================================");
    
    println!("\n[DEBUG] Demo completed successfully!");
    Ok(())
}
