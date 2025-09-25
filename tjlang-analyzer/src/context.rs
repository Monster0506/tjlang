//! Analysis context for sharing data between rules

use std::collections::HashMap;
use tjlang_ast::*;
use tjlang_lexer::Token;
use tjlang_diagnostics::{DiagnosticCollection, SourceSpan};
use crate::config::RuleConfig;

/// Analysis context that provides shared data and utilities for rules
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    /// Source code being analyzed
    pub source: String,
    
    /// Tokens from lexer (for pre-AST rules)
    pub tokens: Vec<Token>,
    
    /// AST from parser (for AST and post-AST rules)
    pub ast: Option<Program>,
    
    /// Symbol table for semantic analysis
    pub symbol_table: SymbolTable,
    
    /// Type information
    pub type_info: TypeInfo,
    
    /// Control flow graph
    pub control_flow: ControlFlowGraph,
    
    /// File information
    pub file_id: codespan::FileId,
    
    /// Cached results from previous rules
    pub cached_results: HashMap<String, RuleResult>,
    
    /// Configuration for rules
    pub config: RuleConfig,
}

/// Symbol table for tracking variables, functions, types, etc.
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    /// Global scope symbols
    pub globals: HashMap<String, Symbol>,
    
    /// Function scopes
    pub functions: HashMap<String, FunctionScope>,
    
    /// Type definitions
    pub types: HashMap<String, TypeDefinition>,
}

/// Function scope containing local variables and parameters
#[derive(Debug, Clone)]
pub struct FunctionScope {
    pub name: String,
    pub parameters: HashMap<String, Symbol>,
    pub locals: HashMap<String, Symbol>,
    pub return_type: Type,
}

/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub data_type: Type,
    pub span: SourceSpan,
    pub is_used: bool,
    pub is_defined: bool,
}

/// Type of symbol (variable, function, type, etc.)
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Variable,
    Function,
    Type,
    Parameter,
    Constant,
}

/// Type definition information
#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub definition: Type,
    pub span: SourceSpan,
    pub is_used: bool,
}

/// Type information for analysis
#[derive(Debug, Clone, Default)]
pub struct TypeInfo {
    /// Type inference results
    pub inferred_types: HashMap<String, Type>,
    
    /// Type constraints
    pub constraints: Vec<TypeConstraint>,
    
    /// Generic type parameters
    pub generics: HashMap<String, GenericInfo>,
}

/// Type constraint for generic analysis
#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub left: Type,
    pub right: Type,
    pub constraint_type: ConstraintType,
}

/// Type of constraint
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    Equality,
    Subtype,
    Supertype,
    Implements,
}

/// Generic type information
#[derive(Debug, Clone)]
pub struct GenericInfo {
    pub name: String,
    pub bounds: Vec<Type>,
    pub variance: Variance,
}

/// Variance for generic types
#[derive(Debug, Clone, PartialEq)]
pub enum Variance {
    Covariant,
    Contravariant,
    Invariant,
}

/// Control flow graph for unreachable code analysis
#[derive(Debug, Clone, Default)]
pub struct ControlFlowGraph {
    /// Basic blocks
    pub blocks: Vec<BasicBlock>,
    
    /// Edges between blocks
    pub edges: Vec<Edge>,
    
    /// Entry and exit points
    pub entry: Option<usize>,
    pub exit: Option<usize>,
}

/// Basic block in control flow
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub statements: Vec<Statement>,
    pub predecessors: Vec<usize>,
    pub successors: Vec<usize>,
    pub is_reachable: bool,
}

/// Edge in control flow graph
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub edge_type: EdgeType,
}

/// Type of control flow edge
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeType {
    Normal,
    Conditional,
    Return,
    Break,
    Continue,
    Exception,
}

/// Result from a rule analysis
#[derive(Debug, Clone)]
pub struct RuleResult {
    pub rule_name: String,
    pub diagnostics: DiagnosticCollection,
    pub metadata: HashMap<String, String>,
    pub execution_time: std::time::Duration,
}

impl AnalysisContext {
    /// Create a new analysis context
    pub fn new(source: String, file_id: codespan::FileId, config: RuleConfig) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            ast: None,
            symbol_table: SymbolTable::default(),
            type_info: TypeInfo::default(),
            control_flow: ControlFlowGraph::default(),
            file_id,
            cached_results: HashMap::new(),
            config,
        }
    }
    
    /// Add tokens from lexer
    pub fn with_tokens(mut self, tokens: Vec<Token>) -> Self {
        self.tokens = tokens;
        self
    }
    
    /// Add AST from parser
    pub fn with_ast(mut self, ast: Program) -> Self {
        self.ast = Some(ast);
        self
    }
    
    /// Cache a rule result
    pub fn cache_result(&mut self, result: RuleResult) {
        self.cached_results.insert(result.rule_name.clone(), result);
    }
    
    /// Get cached result for a rule
    pub fn get_cached_result(&self, rule_name: &str) -> Option<&RuleResult> {
        self.cached_results.get(rule_name)
    }
    
    /// Check if a rule result is cached
    pub fn is_cached(&self, rule_name: &str) -> bool {
        self.cached_results.contains_key(rule_name)
    }
}

impl SymbolTable {
    /// Add a symbol to the global scope
    pub fn add_global(&mut self, symbol: Symbol) {
        self.globals.insert(symbol.name.clone(), symbol);
    }
    
    /// Add a function scope
    pub fn add_function(&mut self, scope: FunctionScope) {
        self.functions.insert(scope.name.clone(), scope);
    }
    
    /// Look up a symbol in all scopes
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        // Check globals first
        if let Some(symbol) = self.globals.get(name) {
            return Some(symbol);
        }
        
        // Check function scopes
        for scope in self.functions.values() {
            if let Some(symbol) = scope.locals.get(name) {
                return Some(symbol);
            }
            if let Some(symbol) = scope.parameters.get(name) {
                return Some(symbol);
            }
        }
        
        None
    }
    
    /// Mark a symbol as used
    pub fn mark_used(&mut self, name: &str) {
        if let Some(symbol) = self.globals.get_mut(name) {
            symbol.is_used = true;
        }
        
        for scope in self.functions.values_mut() {
            if let Some(symbol) = scope.locals.get_mut(name) {
                symbol.is_used = true;
            }
            if let Some(symbol) = scope.parameters.get_mut(name) {
                symbol.is_used = true;
            }
        }
    }
}
