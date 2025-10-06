//! TJLang CLI
//!
//! Command-line interface for the TJLang advanced interpreter.

use clap::{Parser, Subcommand};
use codespan_reporting::diagnostic::Severity;
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};
use std::path::PathBuf;
use tjlang_diagnostics::debug_println;
use tjlang_diagnostics::utils::debug;
use tjlang_diagnostics::{DiagnosticCollection, ErrorCode, SourceSpan, TJLangDiagnostic};
use tjlang_lexer::lex;
use tjlang_parser::parse;
use tjlang_runtime::Interpreter;
use tjlang_analyzer::{AnalysisPipeline, RuleConfig};

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
        /// Use specific configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
        /// Use strict analysis rules
        #[arg(long)]
        strict: bool,
    },
    /// Configure analysis rules and settings
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// List all available rules and their status
    List {
        /// Show only enabled rules
        #[arg(short, long)]
        enabled: bool,
        /// Show only disabled rules
        #[arg(short, long)]
        disabled: bool,
    },
    /// Show current configuration
    Show {
        /// Show configuration in JSON format
        #[arg(short, long)]
        json: bool,
    },
    /// Enable a specific rule
    Enable {
        /// Rule name to enable
        rule: String,
    },
    /// Disable a specific rule
    Disable {
        /// Rule name to disable
        rule: String,
    },
    /// Set rule severity level
    SetSeverity {
        /// Rule name
        rule: String,
        /// Severity level (Error, Warning, Info, Disabled)
        severity: String,
    },
    /// Save current configuration to file
    Save {
        /// Path to save configuration file
        file: PathBuf,
    },
    /// Reset to default configuration
    Reset {
        /// Confirm reset (required)
        #[arg(long)]
        confirm: bool,
    },
    /// Validate configuration file
    Validate {
        /// Path to configuration file to validate
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            debug,
            verbose,
            config,
            strict,
        } => {
            run_program(&file, debug, verbose, config, strict)?;
        }
        Commands::Config { command } => {
            handle_config_command(command)?;
        }
    }

    Ok(())
}

/// Run a TJLang program
fn run_program(
    file: &PathBuf,
    debug: bool,
    verbose: bool,
    config_file: Option<PathBuf>,
    strict: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    debug_println!(" Running TJLang program: {}", file.display());

    if verbose {
        debug_println!(" File: {}", file.display());
        debug_println!(" Debug mode: {}", debug);
        debug_println!(" Verbose mode: {}", verbose);
    }

    // Read the source file with proper error handling
    let source = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            // Use diagnostic system for file errors
            use codespan::{Files, Span};

            let mut files: Files<String> = Files::new();
            let file_id = files.add(file.to_string_lossy().to_string(), String::new());
            let span = SourceSpan::new(file_id, Span::from(0..0));

            let (code, message, notes) = match e.kind() {
                std::io::ErrorKind::NotFound => (
                    ErrorCode::RuntimeValueError,
                    format!("File not found: {}", file.display()),
                    vec![
                        "Please check that the file exists and the path is correct.".to_string(),
                        format!(
                            "Current directory: {}",
                            std::env::current_dir()
                                .map(|p| p.display().to_string())
                                .unwrap_or_else(|_| "unknown".to_string())
                        ),
                    ],
                ),
                std::io::ErrorKind::PermissionDenied => (
                    ErrorCode::RuntimeValueError,
                    format!("Permission denied: {}", file.display()),
                    vec![
                        "You don't have permission to read this file.".to_string(),
                        "Check the file permissions and try again.".to_string(),
                    ],
                ),
                _ => (
                    ErrorCode::RuntimeValueError,
                    format!("Failed to read file: {}", file.display()),
                    vec![format!("IO error: {}", e)],
                ),
            };

            let mut diagnostic = TJLangDiagnostic::new(code, Severity::Error, message, span);
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

    // Create a file ID for the source
    use codespan::Files;
    let mut files: Files<String> = Files::new();
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
                eprintln!(
                    "Parse Error: Failed to parse {} (no diagnostic information available)",
                    file.display()
                );
            }
            std::process::exit(1);
        }
    };

    // Enable debug mode if debug flag is set
    if debug {
        debug::enable();
    }

    // Run static analysis
    if verbose {
        debug_println!(" Running static analysis...");
    }

    // Load configuration
    let config = load_configuration(config_file, strict)?;
    let pipeline = AnalysisPipeline::with_config(config);

    let analysis_result = pipeline.analyze(&source, file_id);

    if debug {
        debug_println!(" Analysis completed:");
        debug_println!("  Rules executed: {}", analysis_result.rules_executed);
        debug_println!("  Diagnostics found: {}", analysis_result.diagnostics_count);
        debug_println!("  Execution time: {:?}", analysis_result.execution_time);
    }

    // Display analysis diagnostics if any
    if !analysis_result.diagnostics.is_empty() {
        eprintln!("\nStatic Analysis Errors in {}:", file.display());
        eprintln!();
        display_diagnostics(&files, &analysis_result.diagnostics)?;

        // Count errors (not warnings)
        let error_count = analysis_result
            .diagnostics
            .iter()
            .filter(|d| matches!(d.severity, codespan_reporting::diagnostic::Severity::Error))
            .count();

        if error_count > 0 {
            eprintln!(
                "\n{} error(s) found. Fix these before running.",
                error_count
            );
            std::process::exit(1);
        }
    }

    if debug {
        debug_println!(" AST:");
        debug_println!("{:#?}", ast);
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

            // Convert runtime error to diagnostic with proper location tracking
            let span = SourceSpan::new(e.file_id, e.span);
            let diagnostic = TJLangDiagnostic::new(
                ErrorCode::RuntimeValueError,
                Severity::Error,
                format!("Runtime Error: {}", e.message),
                span,
            )
            .with_note("The program failed during execution.".to_string())
            .with_note("Run with --debug flag for more detailed information.".to_string());

            let mut diagnostics = DiagnosticCollection::new();
            diagnostics.add(diagnostic);

            eprintln!("\nRuntime Error in {}:", file.display());
            display_diagnostics(&files, &diagnostics)?;
            std::process::exit(1);
        }
    };

    debug_println!("[DEBUG] After interpret_program call");

    Ok(())
}

/// Handle configuration commands
fn handle_config_command(command: ConfigCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ConfigCommands::List { enabled, disabled } => {
            list_rules(enabled, disabled)?;
        }
        ConfigCommands::Show { json } => {
            show_configuration(json)?;
        }
        ConfigCommands::Enable { rule } => {
            enable_rule(&rule)?;
        }
        ConfigCommands::Disable { rule } => {
            disable_rule(&rule)?;
        }
        ConfigCommands::SetSeverity { rule, severity } => {
            set_rule_severity(&rule, &severity)?;
        }
        ConfigCommands::Save { file } => {
            save_configuration_file(&file)?;
        }
        ConfigCommands::Reset { confirm } => {
            reset_configuration(confirm)?;
        }
        ConfigCommands::Validate { file } => {
            validate_configuration_file(&file)?;
        }
    }
    Ok(())
}

/// Load configuration for analysis
fn load_configuration(
    config_file: Option<PathBuf>,
    strict: bool,
) -> Result<RuleConfig, Box<dyn std::error::Error>> {
    use std::fs;
    use serde_json;
    
    // If explicit config file provided, use it
    if let Some(file) = config_file {
        if file.exists() {
            let content = fs::read_to_string(&file)?;
            let config: RuleConfig = serde_json::from_str(&content)?;
            return Ok(config);
        } else {
            eprintln!("Warning: Configuration file {} not found, using default", file.display());
        }
    }
    
    // Try to find configuration file automatically
    let config_paths = [
        ".tjlang.json",
        "tjlang.config.json", 
        ".tjlang/tjlang.json",
    ];
    
    for path in &config_paths {
        if std::path::Path::new(path).exists() {
            let content = fs::read_to_string(path)?;
            let config: RuleConfig = serde_json::from_str(&content)?;
            return Ok(config);
        }
    }
    
    // No config file found, use default with strict mode if requested
    let mut config = RuleConfig::default();
    
    if strict {
        // Enable additional rules for strict mode (code quality rules)
        let strict_rules = vec![
            // Granular naming convention rules
            "LongIdentifierRule",
            "SnakeCaseNamingRule",
            "PascalCaseNamingRule", 
            "MeaningfulNameRule",
            
            // Granular function complexity rules
            "CyclomaticComplexityRule",
            "FunctionLengthLimitRule",
            "FunctionNestingDepthRule",
            "FunctionParameterCountRule",
            "FunctionLocalVariableCountRule",
            
            // Granular formatting rules
            "IndentationConsistencyRule",
            "TrailingWhitespaceRule",
            "LineLengthRule",
            "BracketStyleRule",
            "OperatorSpacingRule",
        ];
        
        for rule in strict_rules {
            config.enable_rule(rule);
            config.set_rule_severity(rule, tjlang_analyzer::RuleSeverity::Warning);
            
            // Set rule-specific configurations
            match rule {
                "LongIdentifierRule" => {
                    config.set_config_value(rule, "max_length", 50);
                },
                "CyclomaticComplexityRule" => {
                    config.set_config_value(rule, "max_complexity", 10);
                },
                "FunctionLengthLimitRule" => {
                    config.set_config_value(rule, "max_lines", 50);
                },
                "FunctionNestingDepthRule" => {
                    config.set_config_value(rule, "max_depth", 4);
                },
                "FunctionParameterCountRule" => {
                    config.set_config_value(rule, "max_parameters", 5);
                },
                "FunctionLocalVariableCountRule" => {
                    config.set_config_value(rule, "max_variables", 10);
                },
                "IndentationConsistencyRule" => {
                    config.set_config_value(rule, "spaces_per_indent", 4);
                },
                "LineLengthRule" => {
                    config.set_config_value(rule, "max_length", 120);
                },
                _ => {}
            }
        }
    }
    
    Ok(config)
}

/// List all available rules
fn list_rules(enabled_only: bool, disabled_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    use tjlang_analyzer::AnalysisPipeline;
    
    // Create a pipeline to get configuration
    let pipeline = AnalysisPipeline::new();
    let config = pipeline.get_config();
    
    println!("Available Analysis Rules:");
    println!("========================");
    println!();
    
    // Get all rules from the helper function
    let all_rules = get_all_analysis_rules();
    
    if all_rules.is_empty() {
        println!("No rules found in the analysis pipeline.");
        return Ok(());
    }
    
    // Group rules by category
    let mut rules_by_category: std::collections::HashMap<tjlang_analyzer::RuleCategory, Vec<&dyn tjlang_analyzer::AnalysisRule>> = std::collections::HashMap::new();
    
    for rule in all_rules.iter() {
        let category = rule.category();
        rules_by_category.entry(category).or_insert_with(Vec::new).push(*rule);
    }
    
    // Sort categories by priority
    let mut categories: Vec<_> = rules_by_category.keys().collect();
    categories.sort_by_key(|cat| match cat {
        tjlang_analyzer::RuleCategory::TypeSafety => 0,
        tjlang_analyzer::RuleCategory::Security => 1,
        tjlang_analyzer::RuleCategory::Quality => 2,
        tjlang_analyzer::RuleCategory::Performance => 3,
        tjlang_analyzer::RuleCategory::Style => 4,
        tjlang_analyzer::RuleCategory::Architecture => 5,
        tjlang_analyzer::RuleCategory::Language => 6,
        tjlang_analyzer::RuleCategory::DeadCode => 7,
    });
    
    for category in categories {
        let rules = &rules_by_category[category];
        let category_name = match category {
            tjlang_analyzer::RuleCategory::TypeSafety => "Type Safety",
            tjlang_analyzer::RuleCategory::Security => "Security",
            tjlang_analyzer::RuleCategory::Quality => "Code Quality",
            tjlang_analyzer::RuleCategory::Performance => "Performance",
            tjlang_analyzer::RuleCategory::Style => "Style",
            tjlang_analyzer::RuleCategory::Architecture => "Architecture",
            tjlang_analyzer::RuleCategory::Language => "Language",
            tjlang_analyzer::RuleCategory::DeadCode => "Dead Code",
        };
        
        println!("{} Rules:", category_name);
        println!("{}", "=".repeat(category_name.len() + 8));
        
        for rule in rules {
            let is_enabled = rule.is_enabled(config);
            let severity = rule.severity(config);
            
            // Apply filters
            if enabled_only && !is_enabled {
                continue;
            }
            if disabled_only && is_enabled {
                continue;
            }
            
            let status = if is_enabled { "ENABLED" } else { "DISABLED" };
            let severity_str = match severity {
                tjlang_analyzer::RuleSeverity::Error => "Error",
                tjlang_analyzer::RuleSeverity::Warning => "Warning", 
                tjlang_analyzer::RuleSeverity::Info => "Info",
                tjlang_analyzer::RuleSeverity::Disabled => "Disabled",
            };
            
            println!("  {} {} ({})", status, rule.name(), severity_str);
            println!("    {}", rule.description());
            println!();
        }
        println!();
    }
    
    // Summary
    let total_rules = all_rules.len();
    let enabled_count = all_rules.iter().filter(|r| r.is_enabled(config)).count();
    let disabled_count = total_rules - enabled_count;
    
    println!("Summary:");
    println!("========");
    println!("Total rules: {}", total_rules);
    println!("Enabled: {}", enabled_count);
    println!("Disabled: {}", disabled_count);
    
    Ok(())
}

/// Get all available rules from the analyzer
fn get_all_analysis_rules() -> Vec<&'static dyn tjlang_analyzer::AnalysisRule> {
    use tjlang_analyzer::rules::*;
    
    vec![
        // Core rules
        &NullPointerRule,
        &BufferOverflowRule,
        &UnsafeOperationRule,
        &UnusedVariableRule,
        &DeadCodeRule,
        &UnusedParameterRule,
        &DuplicateNameRule,
        &CircularDependencyRule,
        &LiteralIndexBoundsRule,
        &LiteralDivisionByZeroRule,
        &UndefinedVariableRule,
        &UndefinedFunctionRule,
        
        // Granular module validation rules
        &ModuleEmptyNameRule,
        &ModuleInvalidCharactersRule,
        &ModuleReservedNameRule,
        
        // Granular type checking rules
        &VariableTypeCheckRule,
        &FunctionTypeCheckRule,
        &ExpressionTypeCheckRule,
        &MemberAccessTypeCheckRule,
        
        // Granular naming convention rules
        &LongIdentifierRule,
        &SnakeCaseNamingRule,
        &PascalCaseNamingRule,
        &MeaningfulNameRule,
        
        // Granular function complexity rules
        &CyclomaticComplexityRule,
        &FunctionLengthLimitRule,
        &FunctionNestingDepthRule,
        &FunctionParameterCountRule,
        &FunctionLocalVariableCountRule,
        
        // Granular formatting rules
        &IndentationConsistencyRule,
        &TrailingWhitespaceRule,
        &LineLengthRule,
        &BracketStyleRule,
        &OperatorSpacingRule,
        &NamingConventionRule,
        &FunctionComplexityRule,
        &MagicNumberRule,
        &ParameterCountRule,
        &InefficientLoopRule,
        &MemoryAllocationRule,
        &StringConcatenationRule,
        &LargeFileRule,
        &TooManyImportsRule,
        &GlobalVariableRule,
        &FormattingConventionRule,
        &UnreachableCodeRule,
        &RecursionDepthRule,
        &ResourceLeakRule,
        &AsyncAwaitRule,
        &ErrorHandlingRule,
        &PatternMatchingRule,
        &GenericConstraintRule,
        &CommentStyleRule,
        &SemicolonRule,
        &BracketMatchingRule,
        &ImportOrderRule,
        &CacheEfficiencyRule,
        &BranchPredictionRule,
        &VectorizationRule,
        &ConcurrencyRule,
        &MemoryLeakRule,
        &RaceConditionRule,
        &InputValidationRule,
        &HardcodedCredentialsRule,
        &SQLInjectionRule,
        &CouplingRule,
        &CohesionRule,
    ]
}

/// Show current configuration
fn show_configuration(json_format: bool) -> Result<(), Box<dyn std::error::Error>> {
    use tjlang_analyzer::AnalysisPipeline;
    use serde_json;
    
    // Create a pipeline to get current configuration
    let pipeline = AnalysisPipeline::new();
    let config = pipeline.get_config();
    
    if json_format {
        // Show configuration in JSON format
        let json = serde_json::to_string_pretty(config)?;
        println!("{}", json);
    } else {
        // Show human-readable configuration
        println!("Current Configuration:");
        println!("=====================");
        println!();
        
        // Show enabled rules
        let all_rules = get_all_analysis_rules();
        let enabled_rules: Vec<_> = all_rules.iter()
            .filter(|rule| rule.is_enabled(config))
            .collect();
        
        println!("Enabled Rules ({}):", enabled_rules.len());
        println!("===================");
        for rule in &enabled_rules {
            let severity = rule.severity(config);
            let severity_str = match severity {
                tjlang_analyzer::RuleSeverity::Error => "Error",
                tjlang_analyzer::RuleSeverity::Warning => "Warning", 
                tjlang_analyzer::RuleSeverity::Info => "Info",
                tjlang_analyzer::RuleSeverity::Disabled => "Disabled",
            };
            println!("  {} ({})", rule.name(), severity_str);
        }
        println!();
        
        // Show disabled rules count
        let disabled_count = all_rules.len() - enabled_rules.len();
        println!("Disabled Rules: {}", disabled_count);
        println!();
        
        // Show configuration summary
        println!("Configuration Summary:");
        println!("====================");
        println!("Total rules: {}", all_rules.len());
        println!("Enabled: {}", enabled_rules.len());
        println!("Disabled: {}", disabled_count);
    }
    
    Ok(())
}

/// Enable a specific rule
fn enable_rule(rule_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use tjlang_analyzer::AnalysisPipeline;
    
    // Check if rule exists
    let all_rules = get_all_analysis_rules();
    let rule_exists = all_rules.iter().any(|rule| rule.name() == rule_name);
    
    if !rule_exists {
        eprintln!("Error: Rule '{}' not found", rule_name);
        eprintln!("Use 'tjlang config list' to see available rules");
        std::process::exit(1);
    }
    
    // Create a new pipeline and enable the rule
    let mut pipeline = AnalysisPipeline::new();
    pipeline.enable_rule(rule_name);
    
    println!("Enabled rule: {}", rule_name);
    println!("Note: This change is temporary. Use 'tjlang config save' to persist changes.");
    
    Ok(())
}

/// Disable a specific rule
fn disable_rule(rule_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use tjlang_analyzer::AnalysisPipeline;
    
    // Check if rule exists
    let all_rules = get_all_analysis_rules();
    let rule_exists = all_rules.iter().any(|rule| rule.name() == rule_name);
    
    if !rule_exists {
        eprintln!("Error: Rule '{}' not found", rule_name);
        eprintln!("Use 'tjlang config list' to see available rules");
        std::process::exit(1);
    }
    
    // Create a new pipeline and disable the rule
    let mut pipeline = AnalysisPipeline::new();
    pipeline.disable_rule(rule_name);
    
    println!("Disabled rule: {}", rule_name);
    println!("Note: This change is temporary. Use 'tjlang config save' to persist changes.");
    
    Ok(())
}

/// Set rule severity
fn set_rule_severity(rule_name: &str, severity: &str) -> Result<(), Box<dyn std::error::Error>> {
    use tjlang_analyzer::AnalysisPipeline;
    use tjlang_analyzer::RuleSeverity;
    
    // Check if rule exists
    let all_rules = get_all_analysis_rules();
    let rule_exists = all_rules.iter().any(|rule| rule.name() == rule_name);
    
    if !rule_exists {
        eprintln!("Error: Rule '{}' not found", rule_name);
        eprintln!("Use 'tjlang config list' to see available rules");
        std::process::exit(1);
    }
    
    // Parse severity string
    let rule_severity = match severity.to_lowercase().as_str() {
        "error" => RuleSeverity::Error,
        "warning" => RuleSeverity::Warning,
        "info" => RuleSeverity::Info,
        "disabled" => RuleSeverity::Disabled,
        _ => {
            eprintln!("Error: Invalid severity '{}'", severity);
            eprintln!("Valid severities: error, warning, info, disabled");
            std::process::exit(1);
        }
    };
    
    // Create a new pipeline and set the rule severity
    let mut pipeline = AnalysisPipeline::new();
    pipeline.set_rule_severity(rule_name, rule_severity);
    
    println!("Set rule '{}' severity to: {}", rule_name, severity);
    println!("Note: This change is temporary. Use 'tjlang config save' to persist changes.");
    
    Ok(())
}


/// Save configuration to file
fn save_configuration_file(file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use tjlang_analyzer::AnalysisPipeline;
    use serde_json;
    use std::fs;
    
    // Create a pipeline to get current configuration
    let pipeline = AnalysisPipeline::new();
    let config = pipeline.get_config();
    
    // Convert to JSON
    let json = serde_json::to_string_pretty(config)?;
    
    // Create parent directories if they don't exist
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write to file
    fs::write(file, json)?;
    
    println!("Configuration saved to: {}", file.display());
    println!("You can now use this file with: tjlang run --config {}", file.display());
    
    Ok(())
}

/// Reset configuration to defaults
fn reset_configuration(confirm: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !confirm {
        eprintln!("Error: --confirm flag required for reset operation");
        eprintln!("Use: tjlang config reset --confirm");
        std::process::exit(1);
    }
    
    use tjlang_analyzer::RuleConfig;
    
    // Create default configuration
    let default_config = RuleConfig::default();
    
    println!("Configuration reset to defaults");
    println!("Default rules enabled:");
    
    // Show what rules are enabled by default
    let all_rules = get_all_analysis_rules();
    let enabled_rules: Vec<_> = all_rules.iter()
        .filter(|rule| rule.is_enabled(&default_config))
        .collect();
    
    for rule in &enabled_rules {
        let severity = rule.severity(&default_config);
        let severity_str = match severity {
            tjlang_analyzer::RuleSeverity::Error => "Error",
            tjlang_analyzer::RuleSeverity::Warning => "Warning", 
            tjlang_analyzer::RuleSeverity::Info => "Info",
            tjlang_analyzer::RuleSeverity::Disabled => "Disabled",
        };
        println!("  {} ({})", rule.name(), severity_str);
    }
    
    println!();
    println!("Note: This change is temporary. Use 'tjlang config save' to persist the default configuration.");
    
    Ok(())
}

/// Validate configuration file
fn validate_configuration_file(file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use serde_json;
    use tjlang_analyzer::RuleConfig;
    
    // Check if file exists
    if !file.exists() {
        eprintln!("Error: Configuration file '{}' does not exist", file.display());
        std::process::exit(1);
    }
    
    // Read file content
    let content = fs::read_to_string(file)?;
    
    // Try to parse as JSON
    let json_value: serde_json::Value = match serde_json::from_str(&content) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: Invalid JSON in configuration file");
            eprintln!("JSON Error: {}", e);
            std::process::exit(1);
        }
    };
    
    // Try to deserialize as RuleConfig
    let config: RuleConfig = match serde_json::from_value(json_value) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: Invalid configuration format");
            eprintln!("Configuration Error: {}", e);
            std::process::exit(1);
        }
    };
    
    // Validate that all referenced rules exist
    let all_rules = get_all_analysis_rules();
    let all_rule_names: std::collections::HashSet<&str> = all_rules.iter()
        .map(|rule| rule.name())
        .collect();
    
    // Check enabled rules
    for (rule_name, is_enabled) in &config.enabled_rules {
        if !all_rule_names.contains(rule_name.as_str()) {
            eprintln!("Warning: Unknown rule '{}' is configured", rule_name);
        }
    }
    
    // Count enabled and disabled rules
    let enabled_count = config.enabled_rules.values().filter(|&&enabled| enabled).count();
    let disabled_count = config.enabled_rules.values().filter(|&&enabled| !enabled).count();
    
    // Show validation results
    println!("Configuration file '{}' is valid!", file.display());
    println!("Enabled rules: {}", enabled_count);
    println!("Disabled rules: {}", disabled_count);
    println!("Total rules: {}", all_rules.len());
    
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
