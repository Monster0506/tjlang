//! Analysis pipeline orchestrator

use crate::config::{RuleConfig, RuleSeverity};
use crate::context::{AnalysisContext, RuleResult};
use crate::rules::*;
use codespan_reporting::diagnostic::Severity;
use std::collections::HashMap;
use std::time::Instant;
use tjlang_ast::Program;
use tjlang_diagnostics::{
    debug_println, DiagnosticCollection, ErrorCode, SourceSpan, TJLangDiagnostic,
};
use tjlang_lexer::Token;

/// Main analysis pipeline that orchestrates all rules
pub struct AnalysisPipeline {
    /// Configuration for the pipeline
    config: RuleConfig,

    /// All available rules
    rules: Vec<Box<dyn AnalysisRule>>,

    /// Pre-AST rules (analyze tokens)
    pre_ast_rules: Vec<Box<dyn PreASTRule>>,

    /// AST rules (analyze AST)
    ast_rules: Vec<Box<dyn ASTRule>>,

    /// Post-AST rules (semantic analysis)
    post_ast_rules: Vec<Box<dyn PostASTRule>>,
}

/// Result of running the analysis pipeline
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// All collected diagnostics
    pub diagnostics: DiagnosticCollection,

    /// Results from individual rules
    pub rule_results: Vec<RuleResult>,

    /// Total execution time
    pub execution_time: std::time::Duration,

    /// Number of rules executed
    pub rules_executed: usize,

    /// Number of diagnostics found
    pub diagnostics_count: usize,
}

impl AnalysisPipeline {
    /// Create a new analysis pipeline with default configuration
    pub fn new() -> Self {
        Self::with_config(RuleConfig::default())
    }

    /// Create a new analysis pipeline with custom configuration
    pub fn with_config(config: RuleConfig) -> Self {
        let mut pipeline = Self {
            config,
            rules: Vec::new(),
            pre_ast_rules: Vec::new(),
            ast_rules: Vec::new(),
            post_ast_rules: Vec::new(),
        };

        pipeline.initialize_rules();
        pipeline
    }

    /// Initialize rules based on configuration
    fn initialize_rules(&mut self) {
        // Only add rules that are enabled in the configuration (no umbrellas)
        if self.config.is_rule_enabled("NullPointerRule") {
            self.add_post_ast_rule(Box::new(NullPointerRule));
        }
        if self.config.is_rule_enabled("BufferOverflowRule") {
            self.add_post_ast_rule(Box::new(BufferOverflowRule));
        }
        if self.config.is_rule_enabled("UnsafeOperationRule") {
            self.add_post_ast_rule(Box::new(UnsafeOperationRule));
        }
        if self.config.is_rule_enabled("UnusedVariableRule") {
            self.add_post_ast_rule(Box::new(UnusedVariableRule));
        }
        if self.config.is_rule_enabled("DeadCodeRule") {
            self.add_post_ast_rule(Box::new(DeadCodeRule));
        }
        if self.config.is_rule_enabled("UnusedParameterRule") {
            self.add_post_ast_rule(Box::new(UnusedParameterRule));
        }
        if self.config.is_rule_enabled("DuplicateNameRule") {
            self.add_post_ast_rule(Box::new(DuplicateNameRule));
        }
        if self.config.is_rule_enabled("CircularDependencyRule") {
            self.add_post_ast_rule(Box::new(CircularDependencyRule));
        }

        // Granular module validation rules
        if self.config.is_rule_enabled("ModuleEmptyNameRule") {
            self.add_ast_rule(Box::new(ModuleEmptyNameRule));
        }
        if self.config.is_rule_enabled("ModuleInvalidCharactersRule") {
            self.add_ast_rule(Box::new(ModuleInvalidCharactersRule));
        }
        if self.config.is_rule_enabled("ModuleReservedNameRule") {
            println!("DEBUG: Adding ModuleReservedNameRule to pipeline");
            self.add_ast_rule(Box::new(ModuleReservedNameRule));
        } else {
            println!("DEBUG: ModuleReservedNameRule is disabled, not adding to pipeline");
        }

        // Granular type checking rules
        if self.config.is_rule_enabled("VariableTypeCheckRule") {
            self.add_ast_rule(Box::new(VariableTypeCheckRule));
        }
        if self.config.is_rule_enabled("FunctionTypeCheckRule") {
            self.add_ast_rule(Box::new(FunctionTypeCheckRule));
        }
        if self.config.is_rule_enabled("ExpressionTypeCheckRule") {
            self.add_ast_rule(Box::new(ExpressionTypeCheckRule));
        }
        if self.config.is_rule_enabled("MemberAccessTypeCheckRule") {
            self.add_ast_rule(Box::new(MemberAccessTypeCheckRule));
        }

        // Granular naming convention rules
        if self.config.is_rule_enabled("LongIdentifierRule") {
            self.add_post_ast_rule(Box::new(LongIdentifierRule));
        }
        if self.config.is_rule_enabled("SnakeCaseNamingRule") {
            self.add_post_ast_rule(Box::new(SnakeCaseNamingRule));
        }
        if self.config.is_rule_enabled("PascalCaseNamingRule") {
            self.add_post_ast_rule(Box::new(PascalCaseNamingRule));
        }
        if self.config.is_rule_enabled("MeaningfulNameRule") {
            self.add_post_ast_rule(Box::new(MeaningfulNameRule));
        }

        // Granular function complexity rules
        if self.config.is_rule_enabled("CyclomaticComplexityRule") {
            self.add_post_ast_rule(Box::new(CyclomaticComplexityRule));
        }
        if self.config.is_rule_enabled("FunctionLengthLimitRule") {
            self.add_post_ast_rule(Box::new(FunctionLengthLimitRule));
        }
        if self.config.is_rule_enabled("FunctionNestingDepthRule") {
            self.add_post_ast_rule(Box::new(FunctionNestingDepthRule));
        }
        if self.config.is_rule_enabled("FunctionParameterCountRule") {
            self.add_post_ast_rule(Box::new(FunctionParameterCountRule));
        }
        if self.config.is_rule_enabled("FunctionLocalVariableCountRule") {
            self.add_post_ast_rule(Box::new(FunctionLocalVariableCountRule));
        }

        // Granular formatting rules
        if self.config.is_rule_enabled("IndentationConsistencyRule") {
            self.add_pre_ast_rule(Box::new(IndentationConsistencyRule));
        }
        if self.config.is_rule_enabled("TrailingWhitespaceRule") {
            self.add_pre_ast_rule(Box::new(TrailingWhitespaceRule));
        }
        if self.config.is_rule_enabled("LineLengthRule") {
            self.add_pre_ast_rule(Box::new(LineLengthRule));
        }
        if self.config.is_rule_enabled("BracketStyleRule") {
            self.add_pre_ast_rule(Box::new(BracketStyleRule));
        }
        if self.config.is_rule_enabled("OperatorSpacingRule") {
            self.add_pre_ast_rule(Box::new(OperatorSpacingRule));
        }

        // Static semantic analysis rules (prevents runtime crashes)
        if self.config.is_rule_enabled("LiteralIndexBoundsRule") {
            self.add_ast_rule(Box::new(LiteralIndexBoundsRule));
        }
        if self.config.is_rule_enabled("LiteralDivisionByZeroRule") {
            self.add_ast_rule(Box::new(LiteralDivisionByZeroRule));
        }
        if self.config.is_rule_enabled("UndefinedVariableRule") {
            self.add_ast_rule(Box::new(UndefinedVariableRule));
        }
        if self.config.is_rule_enabled("UndefinedFunctionRule") {
            self.add_ast_rule(Box::new(UndefinedFunctionRule));
        }

        // Legacy rules (only add if enabled)
        if self.config.is_rule_enabled("NamingConventionRule") {
            self.add_post_ast_rule(Box::new(NamingConventionRule));
        }
        if self.config.is_rule_enabled("FunctionComplexityRule") {
            self.add_post_ast_rule(Box::new(FunctionComplexityRule));
        }
        if self.config.is_rule_enabled("MagicNumberRule") {
            self.add_post_ast_rule(Box::new(MagicNumberRule));
        }
        if self.config.is_rule_enabled("ParameterCountRule") {
            self.add_post_ast_rule(Box::new(ParameterCountRule));
        }
        if self.config.is_rule_enabled("FormattingConventionRule") {
            self.add_pre_ast_rule(Box::new(FormattingConventionRule));
        }
    }

    /// Add a rule to the pipeline and automatically categorize it
    fn add_rule(&mut self, rule: Box<dyn AnalysisRule>) {
        // Add to general rules list
        self.rules.push(rule);
    }

    /// Add a pre-AST rule to the pipeline
    fn add_pre_ast_rule(&mut self, rule: Box<dyn PreASTRule>) {
        // Add to pre-AST rules list
        self.pre_ast_rules.push(rule);
    }

    /// Add an AST rule to the pipeline
    fn add_ast_rule(&mut self, rule: Box<dyn ASTRule>) {
        // Add to AST rules list
        self.ast_rules.push(rule);
    }

    /// Add a post-AST rule to the pipeline
    fn add_post_ast_rule(&mut self, rule: Box<dyn PostASTRule>) {
        // Add to post-AST rules list
        self.post_ast_rules.push(rule);
    }

    /// Run the complete analysis pipeline
    pub fn analyze(&self, source: &str, file_id: codespan::FileId) -> AnalysisResult {
        let start_time = Instant::now();
        let mut all_diagnostics = DiagnosticCollection::new();
        let mut rule_results = Vec::new();

        // Create analysis context
        let mut context = AnalysisContext::new(source.to_string(), file_id, self.config.clone());

        // Phase 1: Pre-AST analysis (token-based rules)
        let pre_ast_result = self.run_pre_ast_analysis(&context);
        all_diagnostics.merge(pre_ast_result.diagnostics);
        rule_results.extend(pre_ast_result.rule_results);

        // Phase 2: AST analysis (if parsing succeeds)
        if let Some(ast) = self.parse_ast(source, file_id) {
            context = context.with_ast(ast);
            let ast_result = self.run_ast_analysis(&context);
            all_diagnostics.merge(ast_result.diagnostics);
            rule_results.extend(ast_result.rule_results);
        }

        // Phase 3: Post-AST analysis (semantic analysis)
        if context.ast.is_some() {
            let post_ast_result = self.run_post_ast_analysis(&context);
            all_diagnostics.merge(post_ast_result.diagnostics);
            rule_results.extend(post_ast_result.rule_results);
        }

        let execution_time = start_time.elapsed();

        AnalysisResult {
            diagnostics: all_diagnostics.clone(),
            rule_results: rule_results.clone(),
            execution_time,
            rules_executed: rule_results.len(),
            diagnostics_count: all_diagnostics.count(),
        }
    }

    /// Run pre-AST analysis (token-based rules)
    fn run_pre_ast_analysis(&self, context: &AnalysisContext) -> AnalysisResult {
        let start_time = Instant::now();
        let mut diagnostics = DiagnosticCollection::new();
        let mut rule_results = Vec::new();

        // Get tokens from lexer
        let tokens = self.lex_tokens(&context.source);
        let mut context = context.clone();
        context.tokens = tokens;

        // Run pre-AST rules
        for rule in &self.pre_ast_rules {
            if rule.is_enabled(&self.config) {
                let rule_start = Instant::now();
                let rule_diagnostics = rule.analyze(&context);
                let rule_time = rule_start.elapsed();

                diagnostics.merge(rule_diagnostics.clone());

                rule_results.push(RuleResult {
                    rule_name: rule.name().to_string(),
                    diagnostics: rule_diagnostics,
                    metadata: HashMap::new(),
                    execution_time: rule_time,
                });
            }
        }

        let execution_time = start_time.elapsed();

        AnalysisResult {
            diagnostics: diagnostics.clone(),
            rule_results: rule_results.clone(),
            execution_time,
            rules_executed: rule_results.len(),
            diagnostics_count: diagnostics.count(),
        }
    }

    /// Run AST analysis (AST-based rules)
    fn run_ast_analysis(&self, context: &AnalysisContext) -> AnalysisResult {
        debug_println!(
            "[DEBUG] [DIVZERO] run_ast_analysis called with {} AST rules",
            self.ast_rules.len()
        );
        let start_time = Instant::now();
        let mut diagnostics = DiagnosticCollection::new();
        let mut rule_results = Vec::new();

        // Run AST rules
        for rule in &self.ast_rules {
            debug_println!(
                "[DEBUG] [DIVZERO] Checking AST rule: {}, enabled={}",
                rule.name(),
                rule.is_enabled(&self.config)
            );
            if rule.is_enabled(&self.config) {
                debug_println!("[DEBUG] [DIVZERO] Running AST rule: {}", rule.name());
                let rule_start = Instant::now();
                let rule_diagnostics = rule.analyze(context);
                let rule_time = rule_start.elapsed();
                debug_println!(
                    "[DEBUG] [DIVZERO] AST rule {} found {} diagnostics",
                    rule.name(),
                    rule_diagnostics.count()
                );

                diagnostics.merge(rule_diagnostics.clone());

                rule_results.push(RuleResult {
                    rule_name: rule.name().to_string(),
                    diagnostics: rule_diagnostics,
                    metadata: HashMap::new(),
                    execution_time: rule_time,
                });
            }
        }

        let execution_time = start_time.elapsed();

        AnalysisResult {
            diagnostics: diagnostics.clone(),
            rule_results: rule_results.clone(),
            execution_time,
            rules_executed: rule_results.len(),
            diagnostics_count: diagnostics.count(),
        }
    }

    /// Run post-AST analysis (semantic analysis)
    fn run_post_ast_analysis(&self, context: &AnalysisContext) -> AnalysisResult {
        debug_println!(
            "[DEBUG] [POST_AST] run_post_ast_analysis called with {} Post-AST rules",
            self.post_ast_rules.len()
        );
        let start_time = Instant::now();
        let mut diagnostics = DiagnosticCollection::new();
        let mut rule_results = Vec::new();

        // Run post-AST rules
        for rule in &self.post_ast_rules {
            debug_println!(
                "[DEBUG] [POST_AST] Checking Post-AST rule: {}, enabled={}",
                rule.name(),
                rule.is_enabled(&self.config)
            );
            if rule.is_enabled(&self.config) {
                debug_println!("[DEBUG] [POST_AST] Running Post-AST rule: {}", rule.name());
                let rule_start = Instant::now();
                let rule_diagnostics = rule.analyze(context);
                let rule_time = rule_start.elapsed();
                debug_println!(
                    "[DEBUG] [POST_AST] Post-AST rule {} found {} diagnostics",
                    rule.name(),
                    rule_diagnostics.count()
                );

                diagnostics.merge(rule_diagnostics.clone());

                rule_results.push(RuleResult {
                    rule_name: rule.name().to_string(),
                    diagnostics: rule_diagnostics,
                    metadata: HashMap::new(),
                    execution_time: rule_time,
                });
            }
        }

        let execution_time = start_time.elapsed();

        AnalysisResult {
            diagnostics: diagnostics.clone(),
            rule_results: rule_results.clone(),
            execution_time,
            rules_executed: rule_results.len(),
            diagnostics_count: diagnostics.count(),
        }
    }

    /// Run analysis for a specific phase
    pub fn analyze_phase(
        &self,
        phase: AnalysisPhase,
        source: &str,
        file_id: codespan::FileId,
    ) -> AnalysisResult {
        match phase {
            AnalysisPhase::PreAST => {
                let context =
                    AnalysisContext::new(source.to_string(), file_id, self.config.clone());
                self.run_pre_ast_analysis(&context)
            }
            AnalysisPhase::AST => {
                if let Some(ast) = self.parse_ast(source, file_id) {
                    let context =
                        AnalysisContext::new(source.to_string(), file_id, self.config.clone())
                            .with_ast(ast);
                    self.run_ast_analysis(&context)
                } else {
                    AnalysisResult::empty()
                }
            }
            AnalysisPhase::PostAST => {
                if let Some(ast) = self.parse_ast(source, file_id) {
                    let context =
                        AnalysisContext::new(source.to_string(), file_id, self.config.clone())
                            .with_ast(ast);
                    self.run_post_ast_analysis(&context)
                } else {
                    AnalysisResult::empty()
                }
            }
        }
    }

    /// Run analysis for a specific rule category
    pub fn analyze_category(
        &self,
        category: RuleCategory,
        source: &str,
        file_id: codespan::FileId,
    ) -> AnalysisResult {
        let start_time = Instant::now();
        let mut all_diagnostics = DiagnosticCollection::new();
        let mut rule_results = Vec::new();

        // Create analysis context
        let mut context = AnalysisContext::new(source.to_string(), file_id, self.config.clone());

        // Get tokens and AST if needed
        let tokens = self.lex_tokens(&context.source);
        context.tokens = tokens;

        if let Some(ast) = self.parse_ast(source, file_id) {
            context = context.with_ast(ast);
        }

        // Run rules in the specified category
        for rule in &self.rules {
            if rule.category() == category && rule.is_enabled(&self.config) {
                let rule_start = Instant::now();
                let rule_diagnostics = self.run_single_rule(rule.as_ref(), &context);
                let rule_time = rule_start.elapsed();

                all_diagnostics.merge(rule_diagnostics.clone());

                rule_results.push(RuleResult {
                    rule_name: rule.name().to_string(),
                    diagnostics: rule_diagnostics,
                    metadata: HashMap::new(),
                    execution_time: rule_time,
                });
            }
        }

        let execution_time = start_time.elapsed();

        AnalysisResult {
            diagnostics: all_diagnostics.clone(),
            rule_results: rule_results.clone(),
            execution_time,
            rules_executed: rule_results.len(),
            diagnostics_count: all_diagnostics.count(),
        }
    }

    /// Run a single rule
    fn run_single_rule(
        &self,
        _rule: &dyn AnalysisRule,
        _context: &AnalysisContext,
    ) -> DiagnosticCollection {
        // This method is not used since we call rules directly from their specific collections
        // (pre_ast_rules, ast_rules, post_ast_rules)
        DiagnosticCollection::new()
    }

    /// Get all available rules
    pub fn get_rules(&self) -> &[Box<dyn AnalysisRule>] {
        &self.rules
    }

    /// Get rules by category
    pub fn get_rules_by_category(&self, category: RuleCategory) -> Vec<&dyn AnalysisRule> {
        self.rules
            .iter()
            .filter(|rule| rule.category() == category)
            .map(|rule| rule.as_ref())
            .collect()
    }

    /// Get rules by priority range
    pub fn get_rules_by_priority(
        &self,
        min_priority: u32,
        max_priority: u32,
    ) -> Vec<&dyn AnalysisRule> {
        self.rules
            .iter()
            .filter(|rule| rule.priority() >= min_priority && rule.priority() <= max_priority)
            .map(|rule| rule.as_ref())
            .collect()
    }

    /// Enable a rule
    pub fn enable_rule(&mut self, rule_name: &str) {
        self.config.enable_rule(rule_name);
    }

    /// Disable a rule
    pub fn disable_rule(&mut self, rule_name: &str) {
        self.config.disable_rule(rule_name);
    }

    /// Set rule severity
    pub fn set_rule_severity(&mut self, rule_name: &str, severity: RuleSeverity) {
        self.config.set_rule_severity(rule_name, severity);
    }

    /// Get configuration
    pub fn get_config(&self) -> &RuleConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: RuleConfig) {
        self.config = config;
    }

    /// Lex tokens from source code
    fn lex_tokens(&self, _source: &str) -> Vec<Token> {
        // TODO: Integrate with actual lexer
        Vec::new()
    }

    /// Parse AST from source code
    fn parse_ast(&self, source: &str, file_id: codespan::FileId) -> Option<Program> {
        // Use the TJLang parser to parse the source code
        match tjlang_parser::parse(source, file_id) {
            Ok((ast, _diagnostics)) => Some(ast),
            Err(_) => {
                // If parsing fails, we still want to run some analysis rules
                // that don't require an AST (like PreASTRule)
                None
            }
        }
    }
}

impl Default for AnalysisPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Analysis phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisPhase {
    /// Pre-AST analysis (token-based)
    PreAST,
    /// AST analysis (AST-based)
    AST,
    /// Post-AST analysis (semantic)
    PostAST,
}

impl AnalysisResult {
    /// Create an empty analysis result
    pub fn empty() -> Self {
        Self {
            diagnostics: DiagnosticCollection::new(),
            rule_results: Vec::new(),
            execution_time: std::time::Duration::ZERO,
            rules_executed: 0,
            diagnostics_count: 0,
        }
    }

    /// Check if analysis found any issues
    pub fn has_issues(&self) -> bool {
        self.diagnostics_count > 0
    }

    /// Get diagnostics by severity
    pub fn get_diagnostics_by_severity(&self, severity: Severity) -> Vec<&TJLangDiagnostic> {
        self.diagnostics.get_diagnostics_by_severity(severity)
    }

    /// Get rule results by name
    pub fn get_rule_result(&self, rule_name: &str) -> Option<&RuleResult> {
        self.rule_results.iter().find(|r| r.rule_name == rule_name)
    }

    /// Get execution time for a specific rule
    pub fn get_rule_execution_time(&self, rule_name: &str) -> Option<std::time::Duration> {
        self.get_rule_result(rule_name).map(|r| r.execution_time)
    }

    /// Get summary statistics
    pub fn get_summary(&self) -> AnalysisSummary {
        AnalysisSummary {
            total_rules: self.rules_executed,
            total_diagnostics: self.diagnostics_count,
            error_count: self
                .diagnostics
                .get_diagnostics_by_severity(Severity::Error)
                .len(),
            warning_count: self
                .diagnostics
                .get_diagnostics_by_severity(Severity::Warning)
                .len(),
            info_count: self
                .diagnostics
                .get_diagnostics_by_severity(Severity::Note)
                .len(),
            execution_time: self.execution_time,
        }
    }
}

/// Summary of analysis results
#[derive(Debug, Clone)]
pub struct AnalysisSummary {
    pub total_rules: usize,
    pub total_diagnostics: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub execution_time: std::time::Duration,
}
