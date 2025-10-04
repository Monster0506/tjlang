//! Unified Type Checker
//! 
//! Implements the formal typing judgments:
//! - Γ ⊢ e : τ (expression typing)
//! - Γ ⊨ τ₁ ⊆ τ₂ (subtyping)
//! - Γ ⊢ e : τ | C (constrained typing)
//! 
//! Works for both runtime and static analysis contexts.

use std::collections::HashMap;
use tjlang_ast::*;
use tjlang_diagnostics::{DiagnosticCollection, TJLangDiagnostic, ErrorCode, SourceSpan, Severity};
use crate::algebraic_types::AlgebraicType;
use crate::type_environment::{TypeEnvironment, InterfaceConstraint};
use crate::constraint_solver::ConstraintSolver;

/// Unified type checker for both runtime and static analysis
pub struct TypeChecker {
    environment: TypeEnvironment,
    constraint_solver: ConstraintSolver,
    diagnostics: DiagnosticCollection,
}

/// Typing judgment result
#[derive(Debug, Clone)]
pub struct TypingJudgment {
    pub expression_type: AlgebraicType,
    pub constraints: Vec<InterfaceConstraint>,
    pub diagnostics: DiagnosticCollection,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            environment: TypeEnvironment::new(),
            constraint_solver: ConstraintSolver::new(),
            diagnostics: DiagnosticCollection::new(),
        }
    }
    
    /// Type check a program
    pub fn check_program(&mut self, program: &Program) -> Result<HashMap<String, AlgebraicType>, DiagnosticCollection> {
        let mut type_map = HashMap::new();
        
        for unit in &program.units {
            for decl in &unit.declarations {
                match decl {
                    Declaration::Function(func) => {
                        let func_type = self.check_function(func)?;
                        type_map.insert(func.name.clone(), func_type);
                    },
                    Declaration::Struct(struct_decl) => {
                        self.check_struct(struct_decl)?;
                    },
                    Declaration::Enum(enum_decl) => {
                        self.check_enum(enum_decl)?;
                    },
                    Declaration::Implementation(impl_block) => {
                        self.check_implementation(impl_block)?;
                    },
                    _ => {
                        // Handle other declaration types
                    }
                }
            }
        }
        
        if self.diagnostics.is_empty() {
            Ok(type_map)
        } else {
            Err(self.diagnostics.clone())
        }
    }
    
    /// Check a function declaration
    /// Implements: Γ ⊢ (λ(x₁:τ₁,...,xₙ:τₙ).e) : (τ₁,...,τₙ)→ρ
    fn check_function(&mut self, func: &FunctionDecl) -> Result<AlgebraicType, DiagnosticCollection> {
        self.environment.enter_scope();
        
        // Bind parameters
        let mut param_types = Vec::new();
        for param in &func.params {
            let param_type = AlgebraicType::from_ast_type(&param.param_type);
            self.environment.bind_variable(param.name.clone(), param_type.clone());
            param_types.push(param_type);
        }
        
        // Check return type
        let return_type = AlgebraicType::from_ast_type(&func.return_type);
        
        // Check function body
        let body_type = self.check_block(&func.body)?;
        
        // Verify return type compatibility
        if !body_type.is_subtype_of(&return_type) {
            self.diagnostics.add(TJLangDiagnostic::new(
                ErrorCode::AnalyzerTypeMismatch,
                Severity::Error,
                format!("Function body type {} is not compatible with return type {}", 
                       body_type.to_string(), return_type.to_string()),
                func.span,
            ));
        }
        
        self.environment.exit_scope();
        
        Ok(AlgebraicType::Function(param_types, Box::new(return_type)))
    }
    
    /// Check a block of statements
    fn check_block(&mut self, block: &Block) -> Result<AlgebraicType, DiagnosticCollection> {
        self.environment.enter_scope();
        
        let mut last_type = AlgebraicType::Int; // Default return type
        
        for stmt in &block.statements {
            match stmt {
                Statement::Variable(var_decl) => {
                    let expr_type = self.check_expression(&var_decl.value)?;
                    let declared_type = AlgebraicType::from_ast_type(&var_decl.var_type);
                    
                    if !expr_type.is_subtype_of(&declared_type) {
                        self.diagnostics.add(TJLangDiagnostic::new(
                            ErrorCode::AnalyzerTypeMismatch,
                            Severity::Error,
                            format!("Variable {} type {} is not compatible with declared type {}", 
                                   var_decl.name, expr_type.to_string(), declared_type.to_string()),
                            var_decl.span,
                        ));
                    }
                    
                    self.environment.bind_variable(var_decl.name.clone(), declared_type);
                },
                Statement::Expression(expr) => {
                    last_type = self.check_expression(expr)?;
                },
                Statement::Return(return_stmt) => {
                    if let Some(expr) = &return_stmt.value {
                        last_type = self.check_expression(expr)?;
                    }
                },
                _ => {
                    // Handle other statement types
                }
            }
        }
        
        self.environment.exit_scope();
        Ok(last_type)
    }
    
    /// Check an expression
    /// Implements the core typing judgment: Γ ⊢ e : τ
    fn check_expression(&mut self, expr: &Expression) -> Result<AlgebraicType, DiagnosticCollection> {
        match expr {
            Expression::Literal(lit) => {
                Ok(match lit {
                    Literal::Int(_) => AlgebraicType::Int,
                    Literal::Float(_) => AlgebraicType::Float,
                    Literal::Bool(_) => AlgebraicType::Bool,
                    Literal::String(_) => AlgebraicType::Str,
                })
            },
            
            Expression::Variable(var) => {
                // Variable rule: x:τ ∈ Γ ⟹ Γ ⊢ x:τ
                if let Some(var_type) = self.environment.lookup_variable(&var.name) {
                    Ok(var_type)
                } else {
                    self.diagnostics.add(TJLangDiagnostic::new(
                        ErrorCode::AnalyzerUndefinedVariable,
                        Severity::Error,
                        format!("Variable '{}' is not defined", var.name),
                        var.span,
                    ));
                    Ok(AlgebraicType::Any) // Return any type to continue checking
                }
            },
            
            Expression::Binary(bin_op) => {
                let left_type = self.check_expression(&bin_op.left)?;
                let right_type = self.check_expression(&bin_op.right)?;
                
                // Check operator overloading
                self.check_binary_operation(&bin_op.operator, &left_type, &right_type, bin_op.span)
            },
            
            Expression::Call { callee, args, span } => {
                // Function application rule:
                // Γ ⊢ f:(τ₁,...,τₙ)→ρ ∧ ∀i, Γ⊢eᵢ:τᵢ ⟹ Γ ⊢ f(e₁,...,eₙ):ρ
                if let Some(func_type) = self.environment.lookup_function(callee) {
                    if let AlgebraicType::Function(params, return_type) = func_type {
                        if args.len() != params.len() {
                            self.diagnostics.add(TJLangDiagnostic::new(
                                ErrorCode::AnalyzerWrongArgumentCount,
                                Severity::Error,
                                format!("Function '{}' expects {} arguments, got {}", 
                                       callee, params.len(), args.len()),
                                *span,
                            ));
                            return Ok(AlgebraicType::Any);
                        }
                        
                        // Check argument types
                        for (arg, param_type) in args.iter().zip(params.iter()) {
                            let arg_type = self.check_expression(arg)?;
                            if !arg_type.is_subtype_of(param_type) {
                                self.diagnostics.add(TJLangDiagnostic::new(
                                    ErrorCode::AnalyzerTypeMismatch,
                                    Severity::Error,
                                    format!("Argument type {} is not compatible with parameter type {}", 
                                           arg_type.to_string(), param_type.to_string()),
                                    *span,
                                ));
                            }
                        }
                        
                        Ok(*return_type)
                    } else {
                        self.diagnostics.add(TJLangDiagnostic::new(
                            ErrorCode::AnalyzerTypeMismatch,
                            Severity::Error,
                            format!("'{}' is not a function", callee),
                            *span,
                        ));
                        Ok(AlgebraicType::Any)
                    }
                } else {
                    self.diagnostics.add(TJLangDiagnostic::new(
                        ErrorCode::AnalyzerUndefinedFunction,
                        Severity::Error,
                        format!("Function '{}' is not defined", callee),
                        *span,
                    ));
                    Ok(AlgebraicType::Any)
                }
            },
            
            _ => {
                // Handle other expression types
                Ok(AlgebraicType::Any)
            }
        }
    }
    
    /// Check binary operation with operator overloading
    fn check_binary_operation(&mut self, op: &BinaryOperator, left_type: &AlgebraicType, right_type: &AlgebraicType, span: SourceSpan) -> Result<AlgebraicType, DiagnosticCollection> {
        match op {
            BinaryOperator::Add => {
                // Check if types implement Addable interface
                if self.check_interface_constraint(left_type, "Addable") && 
                   self.check_interface_constraint(right_type, "Addable") {
                    // Both types should be the same for addition
                    if left_type == right_type {
                        Ok(left_type.clone())
                    } else {
                        self.diagnostics.add(TJLangDiagnostic::new(
                            ErrorCode::AnalyzerTypeMismatch,
                            Severity::Error,
                            format!("Cannot add {} and {} - types must match", 
                                   left_type.to_string(), right_type.to_string()),
                            span,
                        ));
                        Ok(AlgebraicType::Any)
                    }
                } else {
                    self.diagnostics.add(TJLangDiagnostic::new(
                        ErrorCode::AnalyzerTypeMismatch,
                        Severity::Error,
                        format!("Operator + not defined for types {} and {}", 
                               left_type.to_string(), right_type.to_string()),
                        span,
                    ));
                    Ok(AlgebraicType::Any)
                }
            },
            BinaryOperator::Equal => {
                // Check if types implement Eq interface
                if self.check_interface_constraint(left_type, "Eq") && 
                   self.check_interface_constraint(right_type, "Eq") {
                    Ok(AlgebraicType::Bool)
                } else {
                    self.diagnostics.add(TJLangDiagnostic::new(
                        ErrorCode::AnalyzerTypeMismatch,
                        Severity::Error,
                        format!("Operator == not defined for types {} and {}", 
                               left_type.to_string(), right_type.to_string()),
                        span,
                    ));
                    Ok(AlgebraicType::Any)
                }
            },
            _ => {
                // Handle other operators
                Ok(AlgebraicType::Any)
            }
        }
    }
    
    /// Check if a type implements an interface
    fn check_interface_constraint(&self, type_: &AlgebraicType, interface: &str) -> bool {
        match type_ {
            AlgebraicType::Int => interface == "Addable" || interface == "Eq" || interface == "Order",
            AlgebraicType::Float => interface == "Addable" || interface == "Eq" || interface == "Order",
            AlgebraicType::Str => interface == "Addable" || interface == "Eq",
            AlgebraicType::Bool => interface == "Eq",
            _ => false,
        }
    }
    
    /// Check struct declaration
    fn check_struct(&mut self, struct_decl: &StructDecl) -> Result<(), DiagnosticCollection> {
        // Register struct type
        let struct_type = AlgebraicType::Product(
            struct_decl.fields.iter()
                .map(|field| AlgebraicType::from_ast_type(&field.field_type))
                .collect()
        );
        
        self.environment.bind_variable(struct_decl.name.clone(), struct_type);
        Ok(())
    }
    
    /// Check enum declaration
    fn check_enum(&mut self, enum_decl: &EnumDecl) -> Result<(), DiagnosticCollection> {
        // Register enum type
        let enum_type = AlgebraicType::Sum(
            enum_decl.variants.iter()
                .map(|variant| AlgebraicType::from_ast_type(&variant.variant_type))
                .collect()
        );
        
        self.environment.bind_variable(enum_decl.name.clone(), enum_type);
        Ok(())
    }
    
    /// Check implementation block
    fn check_implementation(&mut self, impl_block: &ImplBlock) -> Result<(), DiagnosticCollection> {
        // Register interface implementation
        for method in &impl_block.methods {
            // TODO: Implement method checking
        }
        Ok(())
    }
    
    /// Get diagnostics
    pub fn get_diagnostics(&self) -> &DiagnosticCollection {
        &self.diagnostics
    }
}
