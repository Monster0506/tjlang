//! Type Environment for Typing Judgments
//! 
//! Implements the type environment Γ that maps:
//! - Variable identifiers → type
//! - Function identifiers → function type  
//! - Interfaces → type constraints (τ : I)

use std::collections::HashMap;
use crate::algebraic_types::AlgebraicType;
use tjlang_diagnostics::SourceSpan;

/// Type environment for typing judgments
/// 
/// Γ maps:
/// - Variable identifiers → type
/// - Function identifiers → function type  
/// - Interfaces → type constraints (τ : I)
#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    /// Variable bindings
    pub variables: HashMap<String, AlgebraicType>,
    
    /// Function bindings
    pub functions: HashMap<String, AlgebraicType>,
    
    /// Interface constraints
    pub constraints: Vec<InterfaceConstraint>,
    
    /// Interface implementations
    pub implementations: HashMap<String, Vec<String>>,
    
    /// Nested scopes
    pub scopes: Vec<HashMap<String, AlgebraicType>>,
}

/// Interface constraint
/// Represents the predicate I(τ) meaning "type τ implements interface I"
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceConstraint {
    pub type_var: String,
    pub interface: String,
    pub span: SourceSpan,
}

impl TypeEnvironment {
    /// Create a new type environment
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            constraints: Vec::new(),
            implementations: HashMap::new(),
            scopes: Vec::new(),
        }
    }
    
    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    /// Exit the current scope
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
    
    /// Look up a variable in the environment
    pub fn lookup_variable(&self, name: &str) -> Option<AlgebraicType> {
        // Check current scopes first (most recent first)
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        
        // Check global variables
        self.variables.get(name).cloned()
    }
    
    /// Look up a function in the environment
    pub fn lookup_function(&self, name: &str) -> Option<AlgebraicType> {
        self.functions.get(name).cloned()
    }
    
    /// Bind a variable in the current scope
    pub fn bind_variable(&mut self, name: String, ty: AlgebraicType) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        } else {
            self.variables.insert(name, ty);
        }
    }
    
    /// Bind a function
    pub fn bind_function(&mut self, name: String, ty: AlgebraicType) {
        self.functions.insert(name, ty);
    }
    
    /// Add an interface constraint
    pub fn add_constraint(&mut self, constraint: InterfaceConstraint) {
        self.constraints.push(constraint);
    }
    
    /// Check if a type implements an interface
    pub fn implements_interface(&self, type_name: &str, interface_name: &str) -> bool {
        self.implementations
            .get(type_name)
            .map(|interfaces| interfaces.contains(interface_name))
            .unwrap_or(false)
    }
    
    /// Get all constraints
    pub fn get_constraints(&self) -> &[InterfaceConstraint] {
        &self.constraints
    }
    
    /// Clear all constraints
    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }
}
