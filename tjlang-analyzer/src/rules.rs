//! Rule-based analysis system

use crate::config::{RuleConfig, RuleSeverity};
use crate::context::{AnalysisContext, RuleResult};
use codespan_reporting::diagnostic::Severity;
use std::collections::HashMap;
use std::time::Instant;
use tjlang_ast::*;
use tjlang_diagnostics::debug_println;
use tjlang_diagnostics::DiagnosticCollection;
use tjlang_lexer::Token;

/// Base trait for all analysis rules
pub trait AnalysisRule {
    /// Name of the rule
    fn name(&self) -> &str;

    /// Description of what the rule does
    fn description(&self) -> &str;

    /// Category of the rule
    fn category(&self) -> RuleCategory;

    /// Priority of the rule (higher = more important)
    fn priority(&self) -> u32;

    /// Check if the rule is enabled in the configuration
    fn is_enabled(&self, config: &RuleConfig) -> bool {
        config.is_rule_enabled(self.name())
    }

    /// Get the severity level for this rule
    fn severity(&self, config: &RuleConfig) -> RuleSeverity {
        config
            .get_rule_settings(self.name())
            .map(|s| s.severity)
            .unwrap_or(RuleSeverity::Warning)
    }
}

/// Trait for rules that analyze tokens (pre-AST)
pub trait PreASTRule: AnalysisRule {
    /// Analyze tokens and return diagnostics
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection;
}

/// Trait for rules that analyze AST (post-parse)
pub trait ASTRule: AnalysisRule {
    /// Analyze AST and return diagnostics
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection;
}

/// Trait for rules that perform semantic analysis (post-AST)
pub trait PostASTRule: AnalysisRule {
    /// Perform semantic analysis and return diagnostics
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection;
}

/// Category of analysis rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleCategory {
    /// Code quality and style
    Quality,
    /// Dead code and usage analysis
    DeadCode,
    /// Performance analysis
    Performance,
    /// Architecture and design
    Architecture,
    /// Style and formatting
    Style,
    /// Security and safety
    Security,
    /// Language-specific features
    Language,
    /// Type safety and correctness
    TypeSafety,
}

/// Helper function to run a rule and collect results
pub fn run_rule<R: AnalysisRule + ?Sized>(
    rule: &R,
    context: &AnalysisContext,
    analyzer: fn(&R, &AnalysisContext) -> DiagnosticCollection,
) -> RuleResult {
    let start_time = Instant::now();

    let diagnostics = if rule.is_enabled(&context.config) {
        analyzer(rule, context)
    } else {
        DiagnosticCollection::new()
    };

    let execution_time = start_time.elapsed();

    RuleResult {
        rule_name: rule.name().to_string(),
        diagnostics,
        metadata: HashMap::new(),
        execution_time,
    }
}

// ============================================================================
// CRITICAL RULES (Phase 1) - Must Have
// ============================================================================

/// Type safety analysis rule
pub struct TypeSafetyRule;

impl AnalysisRule for TypeSafetyRule {
    fn name(&self) -> &str {
        "TypeSafetyRule"
    }
    fn description(&self) -> &str {
        "Type checking, type mismatches, invalid operations"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::TypeSafety
    }
    fn priority(&self) -> u32 {
        1000
    }
}

impl PostASTRule for TypeSafetyRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement type checking logic
        // - Type inference
        // - Type mismatch detection
        // - Invalid operation detection

        diagnostics
    }
}

/// Null pointer safety analysis rule
pub struct NullPointerRule;

impl AnalysisRule for NullPointerRule {
    fn name(&self) -> &str {
        "NullPointerRule"
    }
    fn description(&self) -> &str {
        "Null safety analysis, prevent null dereferences"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        950
    }
}

impl PostASTRule for NullPointerRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement null pointer analysis
        // - Track nullable types
        // - Detect potential null dereferences
        // - Suggest null checks

        diagnostics
    }
}

/// Buffer overflow detection rule
pub struct BufferOverflowRule;

impl AnalysisRule for BufferOverflowRule {
    fn name(&self) -> &str {
        "BufferOverflowRule"
    }
    fn description(&self) -> &str {
        "Array bounds checking, prevent buffer overflows"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        900
    }
}

impl PostASTRule for BufferOverflowRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement buffer overflow detection
        // - Array bounds analysis
        // - String length validation
        // - Memory access patterns

        diagnostics
    }
}

/// Unsafe operations detection rule
pub struct UnsafeOperationRule;

impl AnalysisRule for UnsafeOperationRule {
    fn name(&self) -> &str {
        "UnsafeOperationRule"
    }
    fn description(&self) -> &str {
        "Dangerous operations, memory safety"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        850
    }
}

impl PostASTRule for UnsafeOperationRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement unsafe operation detection
        // - Raw pointer operations
        // - Memory manipulation
        // - System calls

        diagnostics
    }
}

/// Unused variable detection rule
pub struct UnusedVariableRule;

impl AnalysisRule for UnusedVariableRule {
    fn name(&self) -> &str {
        "UnusedVariableRule"
    }
    fn description(&self) -> &str {
        "Variables declared but never used"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::DeadCode
    }
    fn priority(&self) -> u32 {
        800
    }
}

impl PostASTRule for UnusedVariableRule {
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection {
        let mut diagnostics = DiagnosticCollection::new();

        // Get the AST - if it's not available, return empty diagnostics
        let ast = match &context.ast {
            Some(ast) => ast,
            None => return diagnostics,
        };

        // Track variable declarations and usage
        let mut variable_declarations = std::collections::HashMap::new();
        let mut variable_usage = std::collections::HashSet::new();

        // Analyze the AST to find variable declarations and usage
        self.analyze_program(
            ast,
            &mut variable_declarations,
            &mut variable_usage,
            context.file_id,
        );

        // Find unused variables
        for (var_name, (span, var_type)) in variable_declarations {
            if !variable_usage.contains(&var_name) {
                // This variable is declared but never used
                let source_span = tjlang_diagnostics::SourceSpan::new(context.file_id, span);

                let suggestions = vec![tjlang_diagnostics::Suggestion::new(
                    format!("Remove unused variable '{}'", var_name),
                    "".to_string(),
                    source_span,
                )];

                let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                    tjlang_diagnostics::ErrorCode::AnalyzerUnusedVariable,
                    codespan_reporting::diagnostic::Severity::Warning,
                    format!("Variable '{}' is declared but never used", var_name),
                    source_span,
                );

                diagnostic.notes = vec![
                    "Unused variables clutter the code and should be removed".to_string(),
                    "Consider removing the variable declaration if it's not needed".to_string(),
                ];
                diagnostic.suggestions = suggestions;

                diagnostics.add(diagnostic);
            }
        }

        diagnostics
    }
}

impl UnusedVariableRule {
    /// Recursively analyze AST nodes for variable declarations and usage
    fn analyze_program(
        &self,
        program: &Program,
        declarations: &mut std::collections::HashMap<String, (codespan::Span, String)>,
        usage: &mut std::collections::HashSet<String>,
        file_id: codespan::FileId,
    ) {
        for unit in &program.units {
            match unit {
                ProgramUnit::Declaration(decl) => {
                    self.analyze_declaration(decl, declarations, usage, file_id)
                }
                _ => {} // Handle other program units as needed
            }
        }
    }

    fn analyze_declaration(
        &self,
        decl: &Declaration,
        declarations: &mut std::collections::HashMap<String, (codespan::Span, String)>,
        usage: &mut std::collections::HashSet<String>,
        file_id: codespan::FileId,
    ) {
        match decl {
            Declaration::Variable(var_decl) => {
                // Track variable declaration
                let var_name = var_decl.name.clone();
                let var_type = format!("{:?}", var_decl.var_type);
                declarations.insert(var_name.clone(), (var_decl.span.span, var_type));

                // Analyze the initial value for variable usage
                self.analyze_expression(&var_decl.value, usage, file_id);
            }
            Declaration::Function(func_decl) => {
                // Analyze function parameters (they are declared variables)
                for param in &func_decl.params {
                    let param_name = param.name.clone();
                    let param_type = format!("{:?}", param.param_type);
                    declarations.insert(param_name.clone(), (param.span.span, param_type));
                }

                // Analyze function body for variable usage
                self.analyze_block(&func_decl.body, declarations, usage, file_id);
            }
            _ => {} // Handle other declaration types
        }
    }

    fn analyze_statement(
        &self,
        stmt: &Statement,
        declarations: &mut std::collections::HashMap<String, (codespan::Span, String)>,
        usage: &mut std::collections::HashSet<String>,
        file_id: codespan::FileId,
    ) {
        match stmt {
            Statement::Expression(expr) => {
                self.analyze_expression(expr, usage, file_id);
            }
            Statement::Return(return_stmt) => {
                if let Some(expr) = &return_stmt.value {
                    self.analyze_expression(expr, usage, file_id);
                }
            }
            Statement::Block(block) => {
                self.analyze_block(block, declarations, usage, file_id);
            }
            _ => {} // Handle other statement types
        }
    }

    fn analyze_block(
        &self,
        block: &Block,
        declarations: &mut std::collections::HashMap<String, (codespan::Span, String)>,
        usage: &mut std::collections::HashSet<String>,
        file_id: codespan::FileId,
    ) {
        for stmt in &block.statements {
            self.analyze_statement(stmt, declarations, usage, file_id);
        }
    }

    fn analyze_expression(
        &self,
        expr: &Expression,
        usage: &mut std::collections::HashSet<String>,
        file_id: codespan::FileId,
    ) {
        match expr {
            Expression::Variable(var_name) => {
                // This is a variable usage
                usage.insert(var_name.clone());
            }
            Expression::Binary { left, right, .. } => {
                self.analyze_expression(left, usage, file_id);
                self.analyze_expression(right, usage, file_id);
            }
            Expression::Unary { operand, .. } => {
                self.analyze_expression(operand, usage, file_id);
            }
            Expression::Call { args, .. } => {
                for arg in args {
                    self.analyze_expression(arg, usage, file_id);
                }
            }
            _ => {} // Handle other expression types
        }
    }
}

/// Dead code detection rule
pub struct DeadCodeRule;

impl AnalysisRule for DeadCodeRule {
    fn name(&self) -> &str {
        "DeadCodeRule"
    }
    fn description(&self) -> &str {
        "Unreachable statements after returns/throws"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::DeadCode
    }
    fn priority(&self) -> u32 {
        750
    }
}

impl PostASTRule for DeadCodeRule {
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection {
        let mut diagnostics = DiagnosticCollection::new();

        // Get the AST - if it's not available, return empty diagnostics
        let ast = match &context.ast {
            Some(ast) => ast,
            None => return diagnostics,
        };

        // Analyze the AST for dead code
        self.analyze_program(ast, &mut diagnostics, context.file_id);

        diagnostics
    }
}

impl DeadCodeRule {
    /// Recursively analyze AST nodes for dead code
    fn analyze_program(
        &self,
        program: &Program,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        for unit in &program.units {
            match unit {
                ProgramUnit::Declaration(decl) => {
                    self.analyze_declaration(decl, diagnostics, file_id)
                }
                _ => {} // Handle other program units as needed
            }
        }
    }

    fn analyze_declaration(
        &self,
        decl: &Declaration,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        match decl {
            Declaration::Function(func_decl) => {
                // Analyze function body for dead code
                self.analyze_block(&func_decl.body, diagnostics, file_id, false);
            }
            _ => {} // Handle other declaration types
        }
    }

    fn analyze_statement(
        &self,
        stmt: &Statement,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
        is_unreachable: bool,
    ) {
        match stmt {
            Statement::Return(return_stmt) => {
                // Return statements make subsequent code unreachable
                if is_unreachable {
                    // This return statement itself is unreachable
                    let source_span =
                        tjlang_diagnostics::SourceSpan::new(file_id, return_stmt.span.span);

                    let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                        tjlang_diagnostics::ErrorCode::AnalyzerDeadCode,
                        codespan_reporting::diagnostic::Severity::Warning,
                        "Unreachable return statement".to_string(),
                        source_span,
                    );

                    diagnostic.notes = vec![
                        "This return statement is unreachable due to previous control flow"
                            .to_string(),
                        "Consider removing this statement or restructuring the code".to_string(),
                    ];

                    diagnostics.add(diagnostic);
                }
            }
            Statement::Break(break_stmt) => {
                // Break statements terminate execution
                if is_unreachable {
                    let source_span =
                        tjlang_diagnostics::SourceSpan::new(file_id, break_stmt.span.span);

                    let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                        tjlang_diagnostics::ErrorCode::AnalyzerDeadCode,
                        codespan_reporting::diagnostic::Severity::Warning,
                        "Unreachable break statement".to_string(),
                        source_span,
                    );

                    diagnostic.notes = vec![
                        "This break statement is unreachable due to previous control flow"
                            .to_string(),
                        "Consider removing this statement or restructuring the code".to_string(),
                    ];

                    diagnostics.add(diagnostic);
                }
            }
            Statement::Continue(continue_stmt) => {
                // Continue statements terminate execution
                if is_unreachable {
                    let source_span =
                        tjlang_diagnostics::SourceSpan::new(file_id, continue_stmt.span.span);

                    let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                        tjlang_diagnostics::ErrorCode::AnalyzerDeadCode,
                        codespan_reporting::diagnostic::Severity::Warning,
                        "Unreachable continue statement".to_string(),
                        source_span,
                    );

                    diagnostic.notes = vec![
                        "This continue statement is unreachable due to previous control flow"
                            .to_string(),
                        "Consider removing this statement or restructuring the code".to_string(),
                    ];

                    diagnostics.add(diagnostic);
                }
            }
            Statement::Raise(raise_stmt) => {
                // Raise statements terminate execution
                if is_unreachable {
                    let source_span =
                        tjlang_diagnostics::SourceSpan::new(file_id, raise_stmt.span.span);

                    let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                        tjlang_diagnostics::ErrorCode::AnalyzerDeadCode,
                        codespan_reporting::diagnostic::Severity::Warning,
                        "Unreachable raise statement".to_string(),
                        source_span,
                    );

                    diagnostic.notes = vec![
                        "This raise statement is unreachable due to previous control flow"
                            .to_string(),
                        "Consider removing this statement or restructuring the code".to_string(),
                    ];

                    diagnostics.add(diagnostic);
                }
            }
            Statement::Block(block) => {
                self.analyze_block(block, diagnostics, file_id, is_unreachable);
            }
            Statement::If(if_stmt) => {
                // Analyze if statement
                self.analyze_expression(&if_stmt.condition, diagnostics, file_id);
                self.analyze_block(&if_stmt.then_block, diagnostics, file_id, is_unreachable);

                // Analyze elif branches
                for elif in &if_stmt.elif_branches {
                    self.analyze_expression(&elif.condition, diagnostics, file_id);
                    self.analyze_block(&elif.block, diagnostics, file_id, is_unreachable);
                }

                // Analyze else branch
                if let Some(else_block) = &if_stmt.else_block {
                    self.analyze_block(else_block, diagnostics, file_id, is_unreachable);
                }
            }
            Statement::While(while_stmt) => {
                self.analyze_expression(&while_stmt.condition, diagnostics, file_id);
                self.analyze_block(&while_stmt.body, diagnostics, file_id, is_unreachable);
            }
            Statement::For(for_stmt) => match for_stmt {
                ForStatement::ForEach { body, .. } => {
                    self.analyze_block(body, diagnostics, file_id, is_unreachable);
                }
                ForStatement::CStyle { body, .. } => {
                    self.analyze_block(body, diagnostics, file_id, is_unreachable);
                }
            },
            Statement::Match(match_stmt) => {
                self.analyze_expression(&match_stmt.expression, diagnostics, file_id);
                for arm in &match_stmt.arms {
                    if let Some(guard) = &arm.guard {
                        self.analyze_expression(guard, diagnostics, file_id);
                    }
                    self.analyze_block(&arm.body, diagnostics, file_id, is_unreachable);
                }
            }
            _ => {
                // For other statements, check if they're unreachable
                if is_unreachable {
                    // Get span from the statement - we need to handle different statement types
                    let span = match stmt {
                        Statement::Variable(var_decl) => var_decl.span.span,
                        Statement::Expression(expr) => {
                            match expr {
                                Expression::Binary { span, .. } => span.span,
                                Expression::Unary { span, .. } => span.span,
                                Expression::Call { span, .. } => span.span,
                                Expression::Index { span, .. } => span.span,
                                Expression::Member { span, .. } => span.span,
                                Expression::Lambda { span, .. } => span.span,
                                Expression::Range { span, .. } => span.span,
                                Expression::Spawn { span, .. } => span.span,
                                Expression::Literal(_lit) => {
                                    // Literals don't have spans in this AST structure
                                    codespan::Span::new(0, 0)
                                }
                                _ => codespan::Span::new(0, 0), // Fallback for expressions without span
                            }
                        }
                        Statement::If(if_stmt) => if_stmt.span.span,
                        Statement::While(while_stmt) => while_stmt.span.span,
                        Statement::For(for_stmt) => match for_stmt {
                            ForStatement::ForEach { span, .. } => span.span,
                            ForStatement::CStyle { span, .. } => span.span,
                        },
                        Statement::Match(match_stmt) => match_stmt.span.span,
                        Statement::Block(block) => block.span.span,
                        _ => codespan::Span::new(0, 0), // Fallback
                    };

                    let source_span = tjlang_diagnostics::SourceSpan::new(file_id, span);

                    let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                        tjlang_diagnostics::ErrorCode::AnalyzerDeadCode,
                        codespan_reporting::diagnostic::Severity::Warning,
                        "Unreachable code detected".to_string(),
                        source_span,
                    );

                    diagnostic.notes = vec![
                        "This code is unreachable due to previous control flow".to_string(),
                        "Consider removing this code or restructuring the logic".to_string(),
                    ];

                    diagnostics.add(diagnostic);
                }
            }
        }
    }

    fn analyze_block(
        &self,
        block: &Block,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
        is_unreachable: bool,
    ) {
        let mut current_unreachable = is_unreachable;

        for (i, stmt) in block.statements.iter().enumerate() {
            // Check if this statement is unreachable
            if current_unreachable {
                self.analyze_statement(stmt, diagnostics, file_id, true);
            } else {
                self.analyze_statement(stmt, diagnostics, file_id, false);

                // Check if this statement makes subsequent statements unreachable
                match stmt {
                    Statement::Return(_)
                    | Statement::Break(_)
                    | Statement::Continue(_)
                    | Statement::Raise(_) => {
                        current_unreachable = true;
                    }
                    _ => {}
                }
            }
        }
    }

    fn analyze_expression(
        &self,
        expr: &Expression,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        match expr {
            Expression::Binary { left, right, .. } => {
                self.analyze_expression(left, diagnostics, file_id);
                self.analyze_expression(right, diagnostics, file_id);
            }
            Expression::Unary { operand, .. } => {
                self.analyze_expression(operand, diagnostics, file_id);
            }
            Expression::Call { args, .. } => {
                for arg in args {
                    self.analyze_expression(arg, diagnostics, file_id);
                }
            }
            _ => {} // Handle other expression types
        }
    }
}

/// Unused parameter detection rule
pub struct UnusedParameterRule;

impl AnalysisRule for UnusedParameterRule {
    fn name(&self) -> &str {
        "UnusedParameterRule"
    }
    fn description(&self) -> &str {
        "Function parameters never referenced"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::DeadCode
    }
    fn priority(&self) -> u32 {
        700
    }
}

impl PostASTRule for UnusedParameterRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement unused parameter detection
        // - Track parameter declarations
        // - Track parameter usage in function body
        // - Report unused parameters

        diagnostics
    }
}

/// Duplicate name detection rule
pub struct DuplicateNameRule;

impl AnalysisRule for DuplicateNameRule {
    fn name(&self) -> &str {
        "DuplicateNameRule"
    }
    fn description(&self) -> &str {
        "Non-overloaded function name conflicts"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::TypeSafety
    }
    fn priority(&self) -> u32 {
        650
    }
}

impl PostASTRule for DuplicateNameRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement duplicate name detection
        // - Symbol table analysis
        // - Name conflict detection
        // - Overload resolution

        diagnostics
    }
}

/// Undefined variable detection rule
pub struct UndefinedVariableRule;

impl AnalysisRule for UndefinedVariableRule {
    fn name(&self) -> &str {
        "UndefinedVariableRule"
    }
    fn description(&self) -> &str {
        "Variables used but not declared"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::TypeSafety
    }
    fn priority(&self) -> u32 {
        600
    }
}

impl PostASTRule for UndefinedVariableRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement undefined variable detection
        // - Symbol resolution
        // - Scope analysis
        // - Undefined reference detection

        diagnostics
    }
}

/// Circular dependency detection rule
pub struct CircularDependencyRule;

impl AnalysisRule for CircularDependencyRule {
    fn name(&self) -> &str {
        "CircularDependencyRule"
    }
    fn description(&self) -> &str {
        "Module dependency cycles"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Architecture
    }
    fn priority(&self) -> u32 {
        550
    }
}

impl PostASTRule for CircularDependencyRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement circular dependency detection
        // - Import graph analysis
        // - Cycle detection algorithm
        // - Dependency resolution

        diagnostics
    }
}

// ============================================================================
// HIGH PRIORITY RULES (Phase 2) - Important
// ============================================================================

/// Naming convention analysis rule
pub struct NamingConventionRule;

impl AnalysisRule for NamingConventionRule {
    fn name(&self) -> &str {
        "NamingConventionRule"
    }
    fn description(&self) -> &str {
        "Variable/function/type naming standards"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Quality
    }
    fn priority(&self) -> u32 {
        500
    }
}

impl PostASTRule for NamingConventionRule {
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection {
        let mut diagnostics = DiagnosticCollection::new();

        // Get the AST - if it's not available, return empty diagnostics
        let ast = match &context.ast {
            Some(ast) => ast,
            None => return diagnostics,
        };

        // Maximum reasonable length for identifiers
        const MAX_IDENTIFIER_LENGTH: usize = 50;

        // Analyze the AST for long identifiers
        self.analyze_program(
            ast,
            &mut diagnostics,
            context.file_id,
            MAX_IDENTIFIER_LENGTH,
        );

        diagnostics
    }
}

impl NamingConventionRule {
    /// Recursively analyze AST nodes for long identifiers
    fn analyze_program(
        &self,
        program: &Program,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
        max_length: usize,
    ) {
        for unit in &program.units {
            match unit {
                ProgramUnit::Declaration(decl) => {
                    self.analyze_declaration(decl, diagnostics, file_id, max_length)
                }
                _ => {} // Handle other program units as needed
            }
        }
    }

    fn analyze_declaration(
        &self,
        decl: &Declaration,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
        max_length: usize,
    ) {
        match decl {
            Declaration::Variable(var_decl) => {
                // Check variable name length
                self.check_identifier_length(
                    &var_decl.name,
                    diagnostics,
                    file_id,
                    max_length,
                    "variable",
                );
            }
            Declaration::Function(func_decl) => {
                // Check function name length
                self.check_identifier_length(
                    &func_decl.name,
                    diagnostics,
                    file_id,
                    max_length,
                    "function",
                );

                // Check parameter names
                for param in &func_decl.params {
                    self.check_identifier_length(
                        &param.name,
                        diagnostics,
                        file_id,
                        max_length,
                        "parameter",
                    );
                }

                // Analyze function body for variable declarations
                self.analyze_block(&func_decl.body, diagnostics, file_id, max_length);
            }
            Declaration::Type(type_decl) => {
                // Check type name length
                self.check_identifier_length(
                    &type_decl.name,
                    diagnostics,
                    file_id,
                    max_length,
                    "type",
                );
            }
            Declaration::Struct(struct_decl) => {
                // Check struct name length
                self.check_identifier_length(
                    &struct_decl.name,
                    diagnostics,
                    file_id,
                    max_length,
                    "struct",
                );

                // Check field names
                for field in &struct_decl.fields {
                    self.check_identifier_length(
                        &field.name,
                        diagnostics,
                        file_id,
                        max_length,
                        "field",
                    );
                }
            }
            Declaration::Enum(enum_decl) => {
                // Check enum name length
                self.check_identifier_length(
                    &enum_decl.name,
                    diagnostics,
                    file_id,
                    max_length,
                    "enum",
                );

                // Check variant names
                for variant in &enum_decl.variants {
                    self.check_identifier_length(
                        &variant.name,
                        diagnostics,
                        file_id,
                        max_length,
                        "enum variant",
                    );
                }
            }
            Declaration::Interface(interface_decl) => {
                // Check interface name length
                self.check_identifier_length(
                    &interface_decl.name,
                    diagnostics,
                    file_id,
                    max_length,
                    "interface",
                );

                // Check method names
                for method in &interface_decl.methods {
                    self.check_identifier_length(
                        &method.name,
                        diagnostics,
                        file_id,
                        max_length,
                        "method",
                    );
                }
            }
            _ => {} // Handle other declaration types
        }
    }

    fn analyze_statement(
        &self,
        stmt: &Statement,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
        max_length: usize,
    ) {
        match stmt {
            Statement::Variable(var_decl) => {
                self.check_identifier_length(
                    &var_decl.name,
                    diagnostics,
                    file_id,
                    max_length,
                    "variable",
                );
            }
            Statement::Block(block) => {
                self.analyze_block(block, diagnostics, file_id, max_length);
            }
            _ => {} // Handle other statement types
        }
    }

    fn analyze_block(
        &self,
        block: &Block,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
        max_length: usize,
    ) {
        for stmt in &block.statements {
            self.analyze_statement(stmt, diagnostics, file_id, max_length);
        }
    }

    fn check_identifier_length(
        &self,
        name: &str,
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
        max_length: usize,
        identifier_type: &str,
    ) {
        if name.len() > max_length {
            // Create a span for the identifier (we'll use a fallback since we don't have exact position)
            let source_span = tjlang_diagnostics::SourceSpan::new(
                file_id,
                codespan::Span::new(0, name.len() as u32),
            );

            let suggestions = vec![tjlang_diagnostics::Suggestion::new(
                format!("Consider shortening '{}' to a more concise name", name),
                format!("{}_short", &name[..max_length.min(name.len())]),
                source_span,
            )];

            let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                tjlang_diagnostics::ErrorCode::AnalyzerNamingConvention,
                codespan_reporting::diagnostic::Severity::Warning,
                format!(
                    "{} name '{}' is too long ({} characters, max recommended: {})",
                    identifier_type,
                    name,
                    name.len(),
                    max_length
                ),
                source_span,
            );

            diagnostic.notes = vec![
                "Long identifiers can make code harder to read and maintain".to_string(),
                "Consider using a shorter, more descriptive name".to_string(),
            ];
            diagnostic.suggestions = suggestions;

            diagnostics.add(diagnostic);
        }
    }
}

/// Function complexity analysis rule
pub struct FunctionComplexityRule;

impl AnalysisRule for FunctionComplexityRule {
    fn name(&self) -> &str {
        "FunctionComplexityRule"
    }
    fn description(&self) -> &str {
        "Cyclomatic complexity, function length limits"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Quality
    }
    fn priority(&self) -> u32 {
        450
    }
}

impl PostASTRule for FunctionComplexityRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement complexity analysis
        // - Cyclomatic complexity calculation
        // - Function length analysis
        // - Nesting depth analysis

        diagnostics
    }
}

/// Magic number detection rule
pub struct MagicNumberRule;

impl AnalysisRule for MagicNumberRule {
    fn name(&self) -> &str {
        "MagicNumberRule"
    }
    fn description(&self) -> &str {
        "Replace literals with named constants"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Quality
    }
    fn priority(&self) -> u32 {
        400
    }
}

impl PostASTRule for MagicNumberRule {
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection {
        let mut diagnostics = DiagnosticCollection::new();

        // Get the AST - if it's not available, return empty diagnostics
        let ast = match &context.ast {
            Some(ast) => {
                println!("[DEBUG]: MagicNumberRule - AST is available");
                ast
            }
            None => {
                println!("[DEBUG]: MagicNumberRule - No AST available");
                return diagnostics;
            }
        };

        // Common magic numbers that should be constants
        let magic_numbers = vec![
            (0, "ZERO"),
            (1, "ONE"),
            (-1, "NEGATIVE_ONE"),
            (2, "TWO"),
            (3, "THREE"),
            (4, "FOUR"),
            (5, "FIVE"),
            (10, "TEN"),
            (42, "FORTY_TWO"),
            (100, "HUNDRED"),
            (1000, "THOUSAND"),
            (1024, "KILOBYTE"),
            (2048, "TWO_KILOBYTES"),
            (4096, "FOUR_KILOBYTES"),
            (3600, "SECONDS_PER_HOUR"),
            (86400, "SECONDS_PER_DAY"),
            (7, "DAYS_PER_WEEK"),
            (30, "DAYS_PER_MONTH"),
            (365, "DAYS_PER_YEAR"),
            (24, "HOURS_PER_DAY"),
            (60, "MINUTES_PER_HOUR"),
            (80, "DEFAULT_PORT"),
            (443, "HTTPS_PORT"),
            (22, "SSH_PORT"),
            (21, "FTP_PORT"),
            (25, "SMTP_PORT"),
            (53, "DNS_PORT"),
            (3306, "MYSQL_PORT"),
            (5432, "POSTGRES_PORT"),
            (6379, "REDIS_PORT"),
            (27017, "MONGODB_PORT"),
        ];

        // Analyze the AST to find magic numbers
        self.analyze_program(ast, &magic_numbers, &mut diagnostics, context.file_id);

        diagnostics
    }
}

impl MagicNumberRule {
    /// Recursively analyze AST nodes for magic numbers
    fn analyze_program(
        &self,
        program: &Program,
        magic_numbers: &[(i32, &str)],
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        for unit in &program.units {
            match unit {
                ProgramUnit::Declaration(decl) => {
                    self.analyze_declaration(decl, magic_numbers, diagnostics, file_id)
                }
                _ => {} // Handle other program units as needed
            }
        }
    }

    fn analyze_declaration(
        &self,
        decl: &Declaration,
        magic_numbers: &[(i32, &str)],
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        match decl {
            Declaration::Variable(var_decl) => {
                self.analyze_expression(&var_decl.value, magic_numbers, diagnostics, file_id);
            }
            Declaration::Function(func_decl) => {
                // Analyze function body
                self.analyze_block(&func_decl.body, magic_numbers, diagnostics, file_id);
            }
            _ => {} // Handle other declaration types
        }
    }

    fn analyze_statement(
        &self,
        stmt: &Statement,
        magic_numbers: &[(i32, &str)],
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        match stmt {
            Statement::Expression(expr) => {
                self.analyze_expression(expr, magic_numbers, diagnostics, file_id);
            }
            Statement::Return(return_stmt) => {
                if let Some(expr) = &return_stmt.value {
                    self.analyze_expression(expr, magic_numbers, diagnostics, file_id);
                }
            }
            Statement::Block(block) => {
                self.analyze_block(block, magic_numbers, diagnostics, file_id);
            }
            _ => {} // Handle other statement types
        }
    }

    fn analyze_block(
        &self,
        block: &Block,
        magic_numbers: &[(i32, &str)],
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        for stmt in &block.statements {
            self.analyze_statement(stmt, magic_numbers, diagnostics, file_id);
        }
    }

    fn analyze_expression(
        &self,
        expr: &Expression,
        magic_numbers: &[(i32, &str)],
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        match expr {
            Expression::Literal(lit) => {
                self.analyze_literal(lit, magic_numbers, diagnostics, file_id);
            }
            Expression::Binary { left, right, .. } => {
                self.analyze_expression(left, magic_numbers, diagnostics, file_id);
                self.analyze_expression(right, magic_numbers, diagnostics, file_id);
            }
            Expression::Unary { operand, .. } => {
                self.analyze_expression(operand, magic_numbers, diagnostics, file_id);
            }
            _ => {} // Handle other expression types
        }
    }

    fn analyze_literal(
        &self,
        lit: &Literal,
        magic_numbers: &[(i32, &str)],
        diagnostics: &mut DiagnosticCollection,
        file_id: codespan::FileId,
    ) {
        match lit {
            Literal::Int(value) => {
                // Check if this integer is a magic number
                for (magic_value, suggested_name) in magic_numbers {
                    if *value as i32 == *magic_value {
                        // Create diagnostic for magic number
                        let source_span = tjlang_diagnostics::SourceSpan::new(
                            file_id,
                            codespan::Span::new(
                                codespan::ByteIndex::from(0), // TODO: Get actual span from AST
                                codespan::ByteIndex::from(0),
                            ),
                        );

                        let suggestions = vec![tjlang_diagnostics::Suggestion::new(
                            format!(
                                "Replace with named constant: const {}: int = {};",
                                suggested_name, value
                            ),
                            suggested_name.to_string(),
                            source_span,
                        )];

                        let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                            tjlang_diagnostics::ErrorCode::AnalyzerMagicNumber,
                            codespan_reporting::diagnostic::Severity::Warning,
                            format!(
                                "Magic number {} found. Consider replacing with a named constant.",
                                value
                            ),
                            source_span,
                        );

                        diagnostic.notes = vec![
                            "Magic numbers make code harder to understand and maintain".to_string(),
                            "Consider defining a named constant with a descriptive name"
                                .to_string(),
                        ];
                        diagnostic.suggestions = suggestions;

                        diagnostics.add(diagnostic);
                        break; // Only report the first match
                    }
                }
            }
            _ => {} // Handle other literal types
        }
    }
}

/// Parameter count analysis rule
pub struct ParameterCountRule;

impl AnalysisRule for ParameterCountRule {
    fn name(&self) -> &str {
        "ParameterCountRule"
    }
    fn description(&self) -> &str {
        "Function parameter limits (too many parameters)"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Quality
    }
    fn priority(&self) -> u32 {
        350
    }
}

impl PostASTRule for ParameterCountRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement parameter count analysis
        // - Function parameter counting
        // - Threshold checking
        // - Refactoring suggestions

        diagnostics
    }
}

/// Performance analysis rules
pub struct InefficientLoopRule;
pub struct MemoryAllocationRule;
pub struct StringConcatenationRule;

impl AnalysisRule for InefficientLoopRule {
    fn name(&self) -> &str {
        "InefficientLoopRule"
    }
    fn description(&self) -> &str {
        "O(n²) patterns, unnecessary iterations"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Performance
    }
    fn priority(&self) -> u32 {
        300
    }
}

impl PostASTRule for InefficientLoopRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement inefficient loop detection
        // - Nested loop analysis
        // - Algorithm complexity detection
        // - Optimization suggestions

        diagnostics
    }
}

impl AnalysisRule for MemoryAllocationRule {
    fn name(&self) -> &str {
        "MemoryAllocationRule"
    }
    fn description(&self) -> &str {
        "Excessive allocations in loops"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Performance
    }
    fn priority(&self) -> u32 {
        250
    }
}

impl PostASTRule for MemoryAllocationRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement memory allocation analysis
        // - Allocation pattern detection
        // - Loop allocation analysis
        // - Memory optimization suggestions

        diagnostics
    }
}

impl AnalysisRule for StringConcatenationRule {
    fn name(&self) -> &str {
        "StringConcatenationRule"
    }
    fn description(&self) -> &str {
        "String building inefficiencies"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Performance
    }
    fn priority(&self) -> u32 {
        200
    }
}

impl PostASTRule for StringConcatenationRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement string concatenation analysis
        // - String building pattern detection
        // - Performance impact analysis
        // - StringBuilder suggestions

        diagnostics
    }
}

/// Architecture analysis rules
pub struct LargeFileRule;
pub struct TooManyImportsRule;
pub struct GlobalVariableRule;

impl AnalysisRule for LargeFileRule {
    fn name(&self) -> &str {
        "LargeFileRule"
    }
    fn description(&self) -> &str {
        "File size limits"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Architecture
    }
    fn priority(&self) -> u32 {
        150
    }
}

impl PreASTRule for LargeFileRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement file size analysis
        // - Source code length checking
        // - Line count analysis
        // - File splitting suggestions

        diagnostics
    }
}

impl AnalysisRule for TooManyImportsRule {
    fn name(&self) -> &str {
        "TooManyImportsRule"
    }
    fn description(&self) -> &str {
        "Import statement limits"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Architecture
    }
    fn priority(&self) -> u32 {
        100
    }
}

impl ASTRule for TooManyImportsRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement import count analysis
        // - Import statement counting
        // - Threshold checking
        // - Module organization suggestions

        diagnostics
    }
}

impl AnalysisRule for GlobalVariableRule {
    fn name(&self) -> &str {
        "GlobalVariableRule"
    }
    fn description(&self) -> &str {
        "Discourage global state"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Architecture
    }
    fn priority(&self) -> u32 {
        50
    }
}

impl PostASTRule for GlobalVariableRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement global variable detection
        // - Global scope analysis
        // - State management detection
        // - Encapsulation suggestions

        diagnostics
    }
}

// ============================================================================
// MEDIUM PRIORITY RULES (Phase 3) - Valuable
// ============================================================================

/// Style and formatting rules
pub struct FormattingConventionRule;
pub struct IndentationRule;
pub struct TrailingWhitespaceRule;
pub struct LineLengthRule;

impl AnalysisRule for FormattingConventionRule {
    fn name(&self) -> &str {
        "FormattingConventionRule"
    }
    fn description(&self) -> &str {
        "Indentation, spacing, bracket style"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }
    fn priority(&self) -> u32 {
        40
    }
}

impl PreASTRule for FormattingConventionRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement formatting analysis
        // - Indentation checking
        // - Spacing validation
        // - Bracket style checking

        diagnostics
    }
}

impl AnalysisRule for IndentationRule {
    fn name(&self) -> &str {
        "IndentationRule"
    }
    fn description(&self) -> &str {
        "Consistent indentation levels"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }
    fn priority(&self) -> u32 {
        35
    }
}

impl PreASTRule for IndentationRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement indentation analysis
        // - Indentation level checking
        // - Tab vs space detection
        // - Consistency validation

        diagnostics
    }
}

impl AnalysisRule for TrailingWhitespaceRule {
    fn name(&self) -> &str {
        "TrailingWhitespaceRule"
    }
    fn description(&self) -> &str {
        "Remove trailing whitespace"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }
    fn priority(&self) -> u32 {
        30
    }
}

impl PreASTRule for TrailingWhitespaceRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement trailing whitespace detection
        // - Line ending analysis
        // - Whitespace detection
        // - Cleanup suggestions

        diagnostics
    }
}

impl AnalysisRule for LineLengthRule {
    fn name(&self) -> &str {
        "LineLengthRule"
    }
    fn description(&self) -> &str {
        "Maximum line length limits"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }
    fn priority(&self) -> u32 {
        25
    }
}

impl PreASTRule for LineLengthRule {
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection {
        let mut diagnostics = DiagnosticCollection::new();

        // Default line length threshold (configurable)
        let max_line_length = 100;

        // Split source into lines and check each line
        let lines: Vec<&str> = context.source.lines().collect();

        for (line_number, line) in lines.iter().enumerate() {
            let line_length = line.len();

            if line_length > max_line_length {
                // Create diagnostic for this line
                let start_byte = lines[..line_number]
                    .iter()
                    .map(|l| l.len() + 1) // +1 for newline
                    .sum::<usize>();
                let end_byte = start_byte + line.len();

                // Create a span using codespan::Span
                let span = codespan::Span::new(
                    codespan::ByteIndex::from(start_byte as u32),
                    codespan::ByteIndex::from(end_byte as u32),
                );

                let source_span = tjlang_diagnostics::SourceSpan::new(context.file_id, span);

                // Create suggestions
                let suggestions = vec![tjlang_diagnostics::Suggestion::new(
                    "Break the line at a logical point (e.g., after a comma, operator)".to_string(),
                    "".to_string(), // No replacement for now
                    source_span,
                )];

                let mut diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                    tjlang_diagnostics::ErrorCode::AnalyzerLineLength,
                    codespan_reporting::diagnostic::Severity::Warning,
                    format!(
                        "Line {} is {} characters long, exceeding the limit of {} characters",
                        line_number + 1,
                        line_length,
                        max_line_length
                    ),
                    source_span,
                );

                diagnostic.notes = vec![
                    "Consider breaking this line into multiple lines".to_string(),
                    "Use line continuation or split at logical boundaries".to_string(),
                ];
                diagnostic.suggestions = suggestions;

                diagnostics.add(diagnostic);
            }
        }

        diagnostics
    }
}

/// Maintainability rules
pub struct CommentCoverageRule;
pub struct FunctionLengthRule;
pub struct NestingDepthRule;
pub struct EmptyFunctionRule;

impl AnalysisRule for CommentCoverageRule {
    fn name(&self) -> &str {
        "CommentCoverageRule"
    }
    fn description(&self) -> &str {
        "Documentation requirements"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Quality
    }
    fn priority(&self) -> u32 {
        20
    }
}

impl ASTRule for CommentCoverageRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement comment coverage analysis
        // - Comment density calculation
        // - Function documentation checking
        // - Documentation suggestions

        diagnostics
    }
}

impl AnalysisRule for FunctionLengthRule {
    fn name(&self) -> &str {
        "FunctionLengthRule"
    }
    fn description(&self) -> &str {
        "Function size limits"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Quality
    }
    fn priority(&self) -> u32 {
        15
    }
}

impl PostASTRule for FunctionLengthRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement function length analysis
        // - Function size calculation
        // - Threshold checking
        // - Refactoring suggestions

        diagnostics
    }
}

impl AnalysisRule for NestingDepthRule {
    fn name(&self) -> &str {
        "NestingDepthRule"
    }
    fn description(&self) -> &str {
        "Control flow nesting limits"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Quality
    }
    fn priority(&self) -> u32 {
        10
    }
}

impl PostASTRule for NestingDepthRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement nesting depth analysis
        // - Control flow depth calculation
        // - Nesting threshold checking
        // - Simplification suggestions

        diagnostics
    }
}

impl AnalysisRule for EmptyFunctionRule {
    fn name(&self) -> &str {
        "EmptyFunctionRule"
    }
    fn description(&self) -> &str {
        "Empty function bodies"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::DeadCode
    }
    fn priority(&self) -> u32 {
        5
    }
}

impl PostASTRule for EmptyFunctionRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement empty function detection
        // - Function body analysis
        // - Empty function detection
        // - Removal suggestions

        diagnostics
    }
}

// ============================================================================
// LOW PRIORITY RULES (Phase 4) - Nice to Have
// ============================================================================

/// Advanced analysis rules
pub struct UnreachableCodeRule;
pub struct RecursionDepthRule;
pub struct ResourceLeakRule;

impl AnalysisRule for UnreachableCodeRule {
    fn name(&self) -> &str {
        "UnreachableCodeRule"
    }
    fn description(&self) -> &str {
        "Code after returns/throws/breaks"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::DeadCode
    }
    fn priority(&self) -> u32 {
        4
    }
}

impl PostASTRule for UnreachableCodeRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement unreachable code detection
        // - Control flow analysis
        // - Unreachable statement detection
        // - Code removal suggestions

        diagnostics
    }
}

impl AnalysisRule for RecursionDepthRule {
    fn name(&self) -> &str {
        "RecursionDepthRule"
    }
    fn description(&self) -> &str {
        "Deep recursion warnings"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Performance
    }
    fn priority(&self) -> u32 {
        3
    }
}

impl PostASTRule for RecursionDepthRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement recursion depth analysis
        // - Recursive call detection
        // - Depth calculation
        // - Stack overflow prevention

        diagnostics
    }
}

impl AnalysisRule for ResourceLeakRule {
    fn name(&self) -> &str {
        "ResourceLeakRule"
    }
    fn description(&self) -> &str {
        "Unclosed files/handles"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        2
    }
}

impl PostASTRule for ResourceLeakRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement resource leak detection
        // - Resource allocation tracking
        // - Resource cleanup validation
        // - Leak prevention suggestions

        diagnostics
    }
}

// ============================================================================
// ADVANCED RULES (Phase 5) - Future Enhancements
// ============================================================================

/// Advanced performance rules
pub struct CacheEfficiencyRule;
pub struct BranchPredictionRule;
pub struct VectorizationRule;

impl AnalysisRule for CacheEfficiencyRule {
    fn name(&self) -> &str {
        "CacheEfficiencyRule"
    }
    fn description(&self) -> &str {
        "Cache-friendly code patterns"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Performance
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for CacheEfficiencyRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement cache efficiency analysis
        // - Memory access pattern analysis
        // - Cache-friendly optimization suggestions
        // - Performance impact assessment

        diagnostics
    }
}

impl AnalysisRule for BranchPredictionRule {
    fn name(&self) -> &str {
        "BranchPredictionRule"
    }
    fn description(&self) -> &str {
        "Branch prediction optimization"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Performance
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for BranchPredictionRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement branch prediction analysis
        // - Branch pattern analysis
        // - Prediction optimization suggestions
        // - Performance impact assessment

        diagnostics
    }
}

impl AnalysisRule for VectorizationRule {
    fn name(&self) -> &str {
        "VectorizationRule"
    }
    fn description(&self) -> &str {
        "SIMD optimization opportunities"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Performance
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for VectorizationRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement vectorization analysis
        // - SIMD opportunity detection
        // - Vectorization suggestions
        // - Performance impact assessment

        diagnostics
    }
}

/// Advanced semantic rules
pub struct ConcurrencyRule;
pub struct MemoryLeakRule;
pub struct RaceConditionRule;

impl AnalysisRule for ConcurrencyRule {
    fn name(&self) -> &str {
        "ConcurrencyRule"
    }
    fn description(&self) -> &str {
        "Thread safety analysis"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for ConcurrencyRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement concurrency analysis
        // - Thread safety analysis
        // - Race condition detection
        // - Synchronization suggestions

        diagnostics
    }
}

impl AnalysisRule for MemoryLeakRule {
    fn name(&self) -> &str {
        "MemoryLeakRule"
    }
    fn description(&self) -> &str {
        "Memory leak detection"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for MemoryLeakRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement memory leak detection
        // - Memory allocation tracking
        // - Leak pattern detection
        // - Cleanup suggestions

        diagnostics
    }
}

impl AnalysisRule for RaceConditionRule {
    fn name(&self) -> &str {
        "RaceConditionRule"
    }
    fn description(&self) -> &str {
        "Race condition detection"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for RaceConditionRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement race condition detection
        // - Shared state analysis
        // - Race condition detection
        // - Synchronization suggestions

        diagnostics
    }
}

/// Language-specific rules
pub struct AsyncAwaitRule;
pub struct ErrorHandlingRule;
pub struct PatternMatchingRule;
pub struct GenericConstraintRule;

impl AnalysisRule for AsyncAwaitRule {
    fn name(&self) -> &str {
        "AsyncAwaitRule"
    }
    fn description(&self) -> &str {
        "Proper async/await usage"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Language
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for AsyncAwaitRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement async/await analysis
        // - Async pattern validation
        // - Await usage checking
        // - Performance optimization suggestions

        diagnostics
    }
}

impl AnalysisRule for ErrorHandlingRule {
    fn name(&self) -> &str {
        "ErrorHandlingRule"
    }
    fn description(&self) -> &str {
        "Missing error handling"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Language
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for ErrorHandlingRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement error handling analysis
        // - Error handling pattern detection
        // - Missing error handling detection
        // - Error handling suggestions

        diagnostics
    }
}

impl AnalysisRule for PatternMatchingRule {
    fn name(&self) -> &str {
        "PatternMatchingRule"
    }
    fn description(&self) -> &str {
        "Exhaustive pattern matching"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Language
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for PatternMatchingRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement pattern matching analysis
        // - Pattern exhaustiveness checking
        // - Pattern completeness validation
        // - Pattern optimization suggestions

        diagnostics
    }
}

impl AnalysisRule for GenericConstraintRule {
    fn name(&self) -> &str {
        "GenericConstraintRule"
    }
    fn description(&self) -> &str {
        "Generic type constraints"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Language
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for GenericConstraintRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement generic constraint analysis
        // - Generic constraint validation
        // - Type bound checking
        // - Constraint optimization suggestions

        diagnostics
    }
}

/// Additional style rules
pub struct CommentStyleRule;
pub struct SemicolonRule;
pub struct BracketMatchingRule;
pub struct ImportOrderRule;

impl AnalysisRule for CommentStyleRule {
    fn name(&self) -> &str {
        "CommentStyleRule"
    }
    fn description(&self) -> &str {
        "Comment formatting and style"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl ASTRule for CommentStyleRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement comment style analysis
        // - Comment format validation
        // - Comment style checking
        // - Comment improvement suggestions

        diagnostics
    }
}

impl AnalysisRule for SemicolonRule {
    fn name(&self) -> &str {
        "SemicolonRule"
    }
    fn description(&self) -> &str {
        "Consistent semicolon usage"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl ASTRule for SemicolonRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement semicolon analysis
        // - Semicolon consistency checking
        // - Missing semicolon detection
        // - Style consistency validation

        diagnostics
    }
}

impl AnalysisRule for BracketMatchingRule {
    fn name(&self) -> &str {
        "BracketMatchingRule"
    }
    fn description(&self) -> &str {
        "Bracket style consistency"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl ASTRule for BracketMatchingRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement bracket matching analysis
        // - Bracket style validation
        // - Bracket consistency checking
        // - Style improvement suggestions

        diagnostics
    }
}

impl AnalysisRule for ImportOrderRule {
    fn name(&self) -> &str {
        "ImportOrderRule"
    }
    fn description(&self) -> &str {
        "Import statement ordering"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl ASTRule for ImportOrderRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement import order analysis
        // - Import ordering validation
        // - Import grouping checking
        // - Import organization suggestions

        diagnostics
    }
}

/// Additional security rules
pub struct InputValidationRule;
pub struct HardcodedCredentialsRule;
pub struct SQLInjectionRule;

impl AnalysisRule for InputValidationRule {
    fn name(&self) -> &str {
        "InputValidationRule"
    }
    fn description(&self) -> &str {
        "Missing input validation"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for InputValidationRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement input validation analysis
        // - Input validation pattern detection
        // - Missing validation detection
        // - Validation suggestion

        diagnostics
    }
}

impl AnalysisRule for HardcodedCredentialsRule {
    fn name(&self) -> &str {
        "HardcodedCredentialsRule"
    }
    fn description(&self) -> &str {
        "Hardcoded secrets/passwords"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for HardcodedCredentialsRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement hardcoded credentials detection
        // - Credential pattern detection
        // - Secret scanning
        // - Security improvement suggestions

        diagnostics
    }
}

impl AnalysisRule for SQLInjectionRule {
    fn name(&self) -> &str {
        "SQLInjectionRule"
    }
    fn description(&self) -> &str {
        "SQL injection vulnerabilities"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for SQLInjectionRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement SQL injection detection
        // - SQL query analysis
        // - Injection vulnerability detection
        // - Security improvement suggestions

        diagnostics
    }
}

/// Additional quality rules
pub struct CouplingRule;
pub struct CohesionRule;

impl AnalysisRule for CouplingRule {
    fn name(&self) -> &str {
        "CouplingRule"
    }
    fn description(&self) -> &str {
        "Module coupling analysis"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Architecture
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for CouplingRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement coupling analysis
        // - Module dependency analysis
        // - Coupling metric calculation
        // - Decoupling suggestions

        diagnostics
    }
}

impl AnalysisRule for CohesionRule {
    fn name(&self) -> &str {
        "CohesionRule"
    }
    fn description(&self) -> &str {
        "Module cohesion analysis"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Architecture
    }
    fn priority(&self) -> u32 {
        1
    }
}

impl PostASTRule for CohesionRule {
    fn analyze(&self, _context: &AnalysisContext) -> DiagnosticCollection {
        let diagnostics = DiagnosticCollection::new();

        // TODO: Implement cohesion analysis
        // - Module cohesion calculation
        // - Cohesion metric analysis
        // - Cohesion improvement suggestions

        diagnostics
    }
}

// ============================================================================
// STATIC SEMANTIC ANALYSIS RULES (A2800-A2899)
// ============================================================================

/// Rule to detect literal array index out of bounds at compile time
pub struct LiteralIndexBoundsRule;

impl AnalysisRule for LiteralIndexBoundsRule {
    fn name(&self) -> &str {
        "LiteralIndexBoundsRule"
    }
    fn description(&self) -> &str {
        "Detects array index out of bounds on literal arrays"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::TypeSafety
    }
    fn priority(&self) -> u32 {
        10
    } // High priority - prevents runtime crash
}

impl ASTRule for LiteralIndexBoundsRule {
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection {
        debug_println!("[DEBUG] [BOUNDS] LiteralIndexBoundsRule.analyze() called");
        let mut diagnostics = DiagnosticCollection::new();

        if let Some(ast) = &context.ast {
            debug_println!(
                "[DEBUG] [BOUNDS] AST is present with {} units",
                ast.units.len()
            );
            check_expressions_for_index_bounds(&ast.units, &mut diagnostics);
        } else {
            debug_println!("[DEBUG] [BOUNDS] No AST available");
        }

        debug_println!(
            "[DEBUG] [BOUNDS] Returning {} diagnostics",
            diagnostics.count()
        );
        diagnostics
    }
}

/// Recursively check expressions for literal index bounds violations
fn check_expressions_for_index_bounds(
    units: &[ProgramUnit],
    diagnostics: &mut DiagnosticCollection,
) {
    use tjlang_diagnostics::ErrorCode;

    debug_println!(
        "[DEBUG] [BOUNDS] check_expressions_for_index_bounds: {} units",
        units.len()
    );
    for (i, unit) in units.iter().enumerate() {
        debug_println!(
            "[DEBUG] [BOUNDS] Unit {}: {:?}",
            i,
            std::mem::discriminant(unit)
        );
        match unit {
            ProgramUnit::Statement(stmt) => {
                debug_println!("[DEBUG] [BOUNDS] Checking statement");
                check_statement_for_index_bounds(stmt, diagnostics);
            }
            ProgramUnit::Declaration(decl) => {
                debug_println!("[DEBUG] [BOUNDS] Checking declaration");
                if let Declaration::Function(func) = decl {
                    check_block_for_index_bounds(&func.body, diagnostics);
                }
            }
            ProgramUnit::Expression(expr) => {
                debug_println!("[DEBUG] [BOUNDS] Checking expression directly");
                check_expr_for_index_bounds(expr, diagnostics);
            }
            _ => {
                debug_println!("[DEBUG] [BOUNDS] Skipping unit type");
            }
        }
    }
}

fn check_statement_for_index_bounds(stmt: &Statement, diagnostics: &mut DiagnosticCollection) {
    match stmt {
        Statement::Expression(expr) => check_expr_for_index_bounds(expr, diagnostics),
        Statement::Variable(var_decl) => check_expr_for_index_bounds(&var_decl.value, diagnostics),
        Statement::If(if_stmt) => {
            check_expr_for_index_bounds(&if_stmt.condition, diagnostics);
            check_block_for_index_bounds(&if_stmt.then_block, diagnostics);
            if let Some(else_block) = &if_stmt.else_block {
                check_block_for_index_bounds(else_block, diagnostics);
            }
        }
        Statement::While(while_stmt) => {
            check_expr_for_index_bounds(&while_stmt.condition, diagnostics);
            check_block_for_index_bounds(&while_stmt.body, diagnostics);
        }
        Statement::DoWhile(do_while) => {
            check_expr_for_index_bounds(&do_while.condition, diagnostics);
            check_block_for_index_bounds(&do_while.body, diagnostics);
        }
        Statement::For(for_stmt) => match for_stmt {
            ForStatement::ForEach { body, .. } => check_block_for_index_bounds(body, diagnostics),
            ForStatement::CStyle { body, .. } => check_block_for_index_bounds(body, diagnostics),
        },
        Statement::Return(ret) => {
            if let Some(expr) = &ret.value {
                check_expr_for_index_bounds(expr, diagnostics);
            }
        }
        _ => {}
    }
}

fn check_block_for_index_bounds(block: &Block, diagnostics: &mut DiagnosticCollection) {
    for stmt in &block.statements {
        check_statement_for_index_bounds(stmt, diagnostics);
    }
}

fn check_expr_for_index_bounds(expr: &Expression, diagnostics: &mut DiagnosticCollection) {
    use tjlang_diagnostics::ErrorCode;

    match expr {
        // Check for method calls like [1,2,3].at(5) or [1,2,3].get(5)
        Expression::Call { callee, args, span } => {
            debug_println!("[DEBUG] [BOUNDS] Checking Call expression");
            if let Expression::Member { target, member, .. } = callee.as_ref() {
                debug_println!(
                    "[DEBUG] [BOUNDS] Call callee is Member: member={}, target={:?}",
                    member,
                    std::mem::discriminant(target.as_ref())
                );
                if matches!(member.as_str(), "at" | "get") {
                    debug_println!("[DEBUG] [BOUNDS] Method is 'at' or 'get'");
                    // Check if target is a literal array
                    if let Expression::VecLiteral { elements, .. } = target.as_ref() {
                        debug_println!(
                            "[DEBUG] [BOUNDS] Target is VecLiteral with {} elements",
                            elements.len()
                        );
                        let array_len = elements.len();

                        // Check if the index argument is a literal integer
                        if !args.is_empty() {
                            if let Expression::Literal(Literal::Int(index)) = &args[0] {
                                if *index < 0 || *index as usize >= array_len {
                                    let message = format!(
                                        "Array index {} is out of bounds for array of length {}",
                                        index, array_len
                                    );

                                    // Convert AST SourceSpan to diagnostics SourceSpan
                                    let diag_span = tjlang_diagnostics::SourceSpan::new(
                                        span.file_id,
                                        span.span,
                                    );

                                    let diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                                        ErrorCode::AnalyzerIndexOutOfBoundsStatic,
                                        codespan_reporting::diagnostic::Severity::Error,
                                        message,
                                        diag_span,
                                    )
                                    .with_note(format!("Valid indices are 0 $= {}", array_len - 1));

                                    diagnostics.add(diagnostic);
                                }
                            }
                        }
                    }
                }
            }

            // Recursively check nested expressions
            check_expr_for_index_bounds(callee, diagnostics);
            for arg in args {
                check_expr_for_index_bounds(arg, diagnostics);
            }
        }

        // Recursively check other expression types
        Expression::Binary { left, right, .. } => {
            check_expr_for_index_bounds(left, diagnostics);
            check_expr_for_index_bounds(right, diagnostics);
        }
        Expression::Unary { operand, .. } => {
            check_expr_for_index_bounds(operand, diagnostics);
        }
        Expression::Member { target, .. } => {
            check_expr_for_index_bounds(target, diagnostics);
        }
        Expression::Index { target, index, .. } => {
            check_expr_for_index_bounds(target, diagnostics);
            check_expr_for_index_bounds(index, diagnostics);
        }
        Expression::VecLiteral { elements, .. } => {
            for elem in elements {
                check_expr_for_index_bounds(elem, diagnostics);
            }
        }
        Expression::If {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            check_expr_for_index_bounds(condition, diagnostics);
            check_expr_for_index_bounds(then_expr, diagnostics);
            check_expr_for_index_bounds(else_expr, diagnostics);
        }
        _ => {}
    }
}

// ============================================================================
// DIVISION BY ZERO RULE (A2801)
// ============================================================================

/// Rule to detect division by zero with literal values at compile time
pub struct LiteralDivisionByZeroRule;

impl AnalysisRule for LiteralDivisionByZeroRule {
    fn name(&self) -> &str {
        "LiteralDivisionByZeroRule"
    }
    fn description(&self) -> &str {
        "Detects division by zero with literal values"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::TypeSafety
    }
    fn priority(&self) -> u32 {
        10
    } // High priority - prevents runtime crash
}

impl ASTRule for LiteralDivisionByZeroRule {
    fn analyze(&self, context: &AnalysisContext) -> DiagnosticCollection {
        debug_println!("[DEBUG] [DIVZERO] LiteralDivisionByZeroRule.analyze() called");
        let mut diagnostics = DiagnosticCollection::new();

        if let Some(ast) = &context.ast {
            debug_println!(
                "[DEBUG] [DIVZERO] AST is present with {} units",
                ast.units.len()
            );
            check_expressions_for_division_by_zero(&ast.units, &mut diagnostics);
        } else {
            debug_println!("[DEBUG] [DIVZERO] No AST available");
        }

        debug_println!(
            "[DEBUG] [DIVZERO] Returning {} diagnostics",
            diagnostics.count()
        );
        diagnostics
    }
}

/// Recursively check expressions for division by zero violations
fn check_expressions_for_division_by_zero(
    units: &[ProgramUnit],
    diagnostics: &mut DiagnosticCollection,
) {
    debug_println!(
        "[DEBUG] [DIVZERO] check_expressions_for_division_by_zero: {} units",
        units.len()
    );
    for (i, unit) in units.iter().enumerate() {
        debug_println!(
            "[DEBUG] [DIVZERO] Unit {}: {:?}",
            i,
            std::mem::discriminant(unit)
        );
        match unit {
            ProgramUnit::Statement(stmt) => {
                debug_println!(
                    "[DEBUG] [DIVZERO] Checking statement: {:?}",
                    std::mem::discriminant(stmt)
                );
                check_statement_for_div_zero(stmt, diagnostics);
            }
            ProgramUnit::Declaration(decl) => {
                debug_println!(
                    "[DEBUG] [DIVZERO] Checking declaration: {:?}",
                    std::mem::discriminant(decl)
                );
                match decl {
                    Declaration::Function(func) => {
                        check_block_for_div_zero(&func.body, diagnostics);
                    }
                    Declaration::Variable(var_decl) => {
                        debug_println!("[DEBUG] [DIVZERO] Found variable declaration, checking init expression");
                        check_expr_for_div_zero(&var_decl.value, diagnostics);
                    }
                    _ => {}
                }
            }
            ProgramUnit::Expression(expr) => {
                debug_println!("[DEBUG] [DIVZERO] Checking expression directly");
                check_expr_for_div_zero(expr, diagnostics);
            }
            _ => {
                debug_println!("[DEBUG] [DIVZERO] Skipping unit type");
            }
        }
    }
}

fn check_statement_for_div_zero(stmt: &Statement, diagnostics: &mut DiagnosticCollection) {
    match stmt {
        Statement::Expression(expr) => check_expr_for_div_zero(expr, diagnostics),
        Statement::Variable(var_decl) => check_expr_for_div_zero(&var_decl.value, diagnostics),
        Statement::If(if_stmt) => {
            check_expr_for_div_zero(&if_stmt.condition, diagnostics);
            check_block_for_div_zero(&if_stmt.then_block, diagnostics);
            if let Some(else_block) = &if_stmt.else_block {
                check_block_for_div_zero(else_block, diagnostics);
            }
        }
        Statement::While(while_stmt) => {
            check_expr_for_div_zero(&while_stmt.condition, diagnostics);
            check_block_for_div_zero(&while_stmt.body, diagnostics);
        }
        Statement::DoWhile(do_while) => {
            check_expr_for_div_zero(&do_while.condition, diagnostics);
            check_block_for_div_zero(&do_while.body, diagnostics);
        }
        Statement::For(for_stmt) => match for_stmt {
            ForStatement::ForEach { body, .. } => check_block_for_div_zero(body, diagnostics),
            ForStatement::CStyle {
                initializer,
                condition,
                increment,
                body,
                ..
            } => {
                if let Some(init_stmt) = initializer {
                    check_statement_for_div_zero(init_stmt, diagnostics);
                }
                if let Some(cond_expr) = condition {
                    check_expr_for_div_zero(cond_expr, diagnostics);
                }
                if let Some(inc_expr) = increment {
                    check_expr_for_div_zero(inc_expr, diagnostics);
                }
                check_block_for_div_zero(body, diagnostics);
            }
        },
        Statement::Return(ret) => {
            if let Some(expr) = &ret.value {
                check_expr_for_div_zero(expr, diagnostics);
            }
        }
        _ => {}
    }
}

fn check_block_for_div_zero(block: &Block, diagnostics: &mut DiagnosticCollection) {
    for stmt in &block.statements {
        check_statement_for_div_zero(stmt, diagnostics);
    }
}

fn check_expr_for_div_zero(expr: &Expression, diagnostics: &mut DiagnosticCollection) {
    use tjlang_diagnostics::ErrorCode;

    match expr {
        // Check for division operations: x / 0 or x % 0
        Expression::Binary {
            operator,
            left,
            right,
            span,
        } => {
            // Check if it's a division or modulo operation
            if matches!(operator, BinaryOperator::Divide | BinaryOperator::Modulo) {
                // Check if the right operand is a literal zero
                if is_literal_zero(right) {
                    let op_str = match operator {
                        BinaryOperator::Divide => "division",
                        BinaryOperator::Modulo => "modulo",
                        _ => unreachable!(),
                    };

                    let message = format!("Literal {} by zero detected", op_str);

                    // Convert AST SourceSpan to diagnostics SourceSpan
                    let diag_span = tjlang_diagnostics::SourceSpan::new(span.file_id, span.span);

                    let diagnostic = tjlang_diagnostics::TJLangDiagnostic::new(
                        ErrorCode::AnalyzerDivisionByZeroStatic,
                        codespan_reporting::diagnostic::Severity::Error,
                        message,
                        diag_span,
                    )
                    .with_note(format!(
                        "{} by zero will cause a runtime panic",
                        match operator {
                            BinaryOperator::Divide => "Division",
                            BinaryOperator::Modulo => "Modulo",
                            _ => unreachable!(),
                        }
                    ));

                    diagnostics.add(diagnostic);
                }
            }

            // Recursively check nested expressions
            check_expr_for_div_zero(left, diagnostics);
            check_expr_for_div_zero(right, diagnostics);
        }

        // Recursively check other expression types
        Expression::Unary { operand, .. } => {
            check_expr_for_div_zero(operand, diagnostics);
        }
        Expression::Call { callee, args, .. } => {
            check_expr_for_div_zero(callee, diagnostics);
            for arg in args {
                check_expr_for_div_zero(arg, diagnostics);
            }
        }
        Expression::Member { target, .. } => {
            check_expr_for_div_zero(target, diagnostics);
        }
        Expression::Index { target, index, .. } => {
            check_expr_for_div_zero(target, diagnostics);
            check_expr_for_div_zero(index, diagnostics);
        }
        Expression::VecLiteral { elements, .. } => {
            for elem in elements {
                check_expr_for_div_zero(elem, diagnostics);
            }
        }
        Expression::If {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            check_expr_for_div_zero(condition, diagnostics);
            check_expr_for_div_zero(then_expr, diagnostics);
            check_expr_for_div_zero(else_expr, diagnostics);
        }
        _ => {}
    }
}

/// Helper function to check if an expression is a literal zero
fn is_literal_zero(expr: &Expression) -> bool {
    match expr {
        Expression::Literal(Literal::Int(0)) => true,
        Expression::Literal(Literal::Float(f)) if *f == 0.0 => true,
        _ => false,
    }
}
