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

    /// Initialize all available rules
    fn initialize_rules(&mut self) {
        // Critical rules (Phase 1)
        self.add_post_ast_rule(Box::new(TypeSafetyRule));
        self.add_post_ast_rule(Box::new(NullPointerRule));
        self.add_post_ast_rule(Box::new(BufferOverflowRule));
        self.add_post_ast_rule(Box::new(UnsafeOperationRule));
        self.add_post_ast_rule(Box::new(UnusedVariableRule));
        self.add_post_ast_rule(Box::new(DeadCodeRule));
        self.add_post_ast_rule(Box::new(UnusedParameterRule));
        self.add_post_ast_rule(Box::new(DuplicateNameRule));
        // UndefinedVariableRule is registered as an AST rule (line 89)
        self.add_post_ast_rule(Box::new(CircularDependencyRule));

        // Static semantic analysis rules (prevents runtime crashes)
        self.add_ast_rule(Box::new(LiteralIndexBoundsRule));
        self.add_ast_rule(Box::new(LiteralDivisionByZeroRule));
        self.add_ast_rule(Box::new(UndefinedVariableRule));
        self.add_ast_rule(Box::new(UndefinedFunctionRule));

        // High priority rules (Phase 2)
        self.add_post_ast_rule(Box::new(NamingConventionRule));
        self.add_post_ast_rule(Box::new(FunctionComplexityRule));
        self.add_post_ast_rule(Box::new(MagicNumberRule));
        self.add_post_ast_rule(Box::new(ParameterCountRule));
        self.add_post_ast_rule(Box::new(InefficientLoopRule));
        self.add_post_ast_rule(Box::new(MemoryAllocationRule));
        self.add_post_ast_rule(Box::new(StringConcatenationRule));
        self.add_pre_ast_rule(Box::new(LargeFileRule));
        self.add_rule(Box::new(TooManyImportsRule));
        self.add_post_ast_rule(Box::new(GlobalVariableRule));

        // Medium priority rules (Phase 3)
        self.add_pre_ast_rule(Box::new(FormattingConventionRule));
        self.add_pre_ast_rule(Box::new(IndentationRule));
        self.add_pre_ast_rule(Box::new(TrailingWhitespaceRule));
        self.add_pre_ast_rule(Box::new(LineLengthRule));
        self.add_rule(Box::new(CommentCoverageRule));
        self.add_post_ast_rule(Box::new(FunctionLengthRule));
        self.add_post_ast_rule(Box::new(NestingDepthRule));
        self.add_post_ast_rule(Box::new(EmptyFunctionRule));

        // Low priority rules (Phase 4)
        self.add_post_ast_rule(Box::new(UnreachableCodeRule));
        self.add_post_ast_rule(Box::new(RecursionDepthRule));
        self.add_post_ast_rule(Box::new(ResourceLeakRule));
        self.add_post_ast_rule(Box::new(AsyncAwaitRule));
        self.add_post_ast_rule(Box::new(ErrorHandlingRule));
        self.add_post_ast_rule(Box::new(PatternMatchingRule));
        self.add_post_ast_rule(Box::new(GenericConstraintRule));
        self.add_rule(Box::new(CommentStyleRule));
        self.add_rule(Box::new(SemicolonRule));
        self.add_rule(Box::new(BracketMatchingRule));
        self.add_rule(Box::new(ImportOrderRule));

        // Advanced rules (Phase 5)
        self.add_post_ast_rule(Box::new(CacheEfficiencyRule));
        self.add_post_ast_rule(Box::new(BranchPredictionRule));
        self.add_post_ast_rule(Box::new(VectorizationRule));
        self.add_post_ast_rule(Box::new(ConcurrencyRule));
        self.add_post_ast_rule(Box::new(MemoryLeakRule));
        self.add_post_ast_rule(Box::new(RaceConditionRule));
        self.add_post_ast_rule(Box::new(InputValidationRule));
        self.add_post_ast_rule(Box::new(HardcodedCredentialsRule));
        self.add_post_ast_rule(Box::new(SQLInjectionRule));
        self.add_post_ast_rule(Box::new(CouplingRule));
        self.add_post_ast_rule(Box::new(CohesionRule));
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
        debug_println!("[DEBUG] [DIVZERO] Attempting to parse AST...");
        if let Some(ast) = self.parse_ast(source, file_id) {
            debug_println!("[DEBUG] [DIVZERO] AST parsed successfully, running AST analysis");
            context = context.with_ast(ast);
            let ast_result = self.run_ast_analysis(&context);
            debug_println!(
                "[DEBUG] [DIVZERO] AST analysis complete: {} diagnostics",
                ast_result.diagnostics_count
            );
            all_diagnostics.merge(ast_result.diagnostics);
            rule_results.extend(ast_result.rule_results);
        } else {
            debug_println!("[DEBUG] [DIVZERO] AST parsing failed");
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
        let start_time = Instant::now();
        let mut diagnostics = DiagnosticCollection::new();
        let mut rule_results = Vec::new();

        // Run post-AST rules
        for rule in &self.post_ast_rules {
            if rule.is_enabled(&self.config) {
                let rule_start = Instant::now();
                let rule_diagnostics = rule.analyze(context);
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
