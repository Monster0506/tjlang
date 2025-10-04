//! Interface System (Traits/Type Classes)
//! 
//! Implements the interface system from the formal specification:
//! I ::= { m_i : (τ_i^1, ..., τ_i^n) → ρ_i }
//! Each interface defines a set of function signatures over a self type variable

use crate::algebraic_types::AlgebraicType;
use tjlang_diagnostics::SourceSpan;

/// Interface definition (trait/type class)
/// 
/// I ::= { m_i : (τ_i^1, ..., τ_i^n) → ρ_i }
/// Each interface defines a set of function signatures over a self type variable
#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    pub name: String,
    pub methods: Vec<MethodSignature>,
    pub span: SourceSpan,
}

/// Method signature within an interface
#[derive(Debug, Clone, PartialEq)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<AlgebraicType>,
    pub return_type: AlgebraicType,
    pub span: SourceSpan,
}

/// Built-in interfaces for operator overloading
pub mod builtin_interfaces {
    use super::*;
    use crate::algebraic_types::AlgebraicType;
    
    /// Addable interface for + operator
    pub fn addable_interface() -> Interface {
        Interface {
            name: "Addable".to_string(),
            methods: vec![
                MethodSignature {
                    name: "add".to_string(),
                    params: vec![AlgebraicType::Variable("Self".to_string()), AlgebraicType::Variable("Self".to_string())],
                    return_type: AlgebraicType::Variable("Self".to_string()),
                    span: SourceSpan::default(),
                }
            ],
            span: SourceSpan::default(),
        }
    }
    
    /// Eq interface for == operator
    pub fn eq_interface() -> Interface {
        Interface {
            name: "Eq".to_string(),
            methods: vec![
                MethodSignature {
                    name: "eq".to_string(),
                    params: vec![AlgebraicType::Variable("Self".to_string()), AlgebraicType::Variable("Self".to_string())],
                    return_type: AlgebraicType::Bool,
                    span: SourceSpan::default(),
                }
            ],
            span: SourceSpan::default(),
        }
    }
    
    /// Order interface for <, > operators
    pub fn order_interface() -> Interface {
        Interface {
            name: "Order".to_string(),
            methods: vec![
                MethodSignature {
                    name: "lt".to_string(),
                    params: vec![AlgebraicType::Variable("Self".to_string()), AlgebraicType::Variable("Self".to_string())],
                    return_type: AlgebraicType::Bool,
                    span: SourceSpan::default(),
                },
                MethodSignature {
                    name: "gt".to_string(),
                    params: vec![AlgebraicType::Variable("Self".to_string()), AlgebraicType::Variable("Self".to_string())],
                    return_type: AlgebraicType::Bool,
                    span: SourceSpan::default(),
                }
            ],
            span: SourceSpan::default(),
        }
    }
    
    /// Indexable interface for [] operator
    pub fn indexable_interface() -> Interface {
        Interface {
            name: "Indexable".to_string(),
            methods: vec![
                MethodSignature {
                    name: "index".to_string(),
                    params: vec![AlgebraicType::Variable("Self".to_string()), AlgebraicType::Int],
                    return_type: AlgebraicType::Variable("T".to_string()),
                    span: SourceSpan::default(),
                }
            ],
            span: SourceSpan::default(),
        }
    }
}
