//! Constraint Solver for Interface Satisfaction
//! 
//! Implements constraint solving for interface satisfaction:
//! - Collects interface constraints during type checking
//! - Solves constraints by finding appropriate implementations
//! - Provides diagnostics for unsatisfied constraints

use std::collections::HashMap;
use crate::algebraic_types::AlgebraicType;
use crate::interfaces::{Interface, MethodSignature};
use crate::type_environment::InterfaceConstraint;
use tjlang_diagnostics::{DiagnosticCollection, TJLangDiagnostic, ErrorCode, SourceSpan, Severity};

/// Constraint solver for interface satisfaction
#[derive(Debug, Clone)]
pub struct ConstraintSolver {
    /// Available interface implementations
    pub implementations: HashMap<String, HashMap<String, Vec<MethodSignature>>>,
}

impl ConstraintSolver {
    /// Create a new constraint solver
    pub fn new() -> Self {
        let mut solver = Self {
            implementations: HashMap::new(),
        };
        
        // Register built-in interface implementations
        solver.register_builtin_implementations();
        solver
    }
    
    /// Register built-in interface implementations
    fn register_builtin_implementations(&mut self) {
        // Int implements Addable, Eq, Order
        self.register_implementation(
            "int".to_string(),
            "Addable".to_string(),
            vec![MethodSignature {
                name: "add".to_string(),
                params: vec![AlgebraicType::Int, AlgebraicType::Int],
                return_type: AlgebraicType::Int,
                span: SourceSpan::default(),
            }]
        );
        
        // Float implements Addable, Eq, Order
        self.register_implementation(
            "float".to_string(),
            "Addable".to_string(),
            vec![MethodSignature {
                name: "add".to_string(),
                params: vec![AlgebraicType::Float, AlgebraicType::Float],
                return_type: AlgebraicType::Float,
                span: SourceSpan::default(),
            }]
        );
        
        // String implements Addable, Eq
        self.register_implementation(
            "str".to_string(),
            "Addable".to_string(),
            vec![MethodSignature {
                name: "add".to_string(),
                params: vec![AlgebraicType::Str, AlgebraicType::Str],
                return_type: AlgebraicType::Str,
                span: SourceSpan::default(),
            }]
        );
    }
    
    /// Register an interface implementation
    pub fn register_implementation(&mut self, type_name: String, interface_name: String, methods: Vec<MethodSignature>) {
        self.implementations
            .entry(type_name)
            .or_insert_with(HashMap::new)
            .insert(interface_name, methods);
    }
    
    /// Solve interface constraints
    /// Returns true if all constraints can be satisfied
    pub fn solve_constraints(&self, constraints: &[InterfaceConstraint]) -> Result<(), DiagnosticCollection> {
        let mut diagnostics = DiagnosticCollection::new();
        
        for constraint in constraints {
            if !self.can_satisfy_constraint(constraint) {
                let diagnostic = TJLangDiagnostic::new(
                    ErrorCode::AnalyzerTraitNotImplemented,
                    Severity::Error,
                    format!("Type {} does not implement interface {}", constraint.type_var, constraint.interface),
                    constraint.span,
                );
                diagnostics.add(diagnostic);
            }
        }
        
        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
    
    /// Check if a constraint can be satisfied
    fn can_satisfy_constraint(&self, constraint: &InterfaceConstraint) -> bool {
        self.implementations
            .get(&constraint.type_var)
            .and_then(|interfaces| interfaces.get(&constraint.interface))
            .is_some()
    }
    
    /// Get all available implementations for a type
    pub fn get_implementations(&self, type_name: &str) -> Option<&HashMap<String, Vec<MethodSignature>>> {
        self.implementations.get(type_name)
    }
    
    /// Check if a type implements a specific interface
    pub fn type_implements_interface(&self, type_name: &str, interface_name: &str) -> bool {
        self.implementations
            .get(type_name)
            .and_then(|interfaces| interfaces.get(interface_name))
            .is_some()
    }
}
