//! Algebraic Types - Core Type System
//! 
//! Implements the formal type system specification:
//! τ ::= int | float | bool | str | any
//!     | τ₁ × τ₂ × ... × τₙ  (product types)
//!     | τ₁ + τ₂             (sum/union types)
//!     | Option(τ)           (optional types)
//!     | Result(τ, σ)        (result types)
//!     | Vec(τ)              (vector types)
//!     | Set(τ)              (set types)
//!     | Map(τ, σ)           (map types)
//!     | Tuple(τ₁, ..., τₙ) (tuple types)
//!     | fn(τ₁, ..., τₙ) → ρ (function types)
//!     | Task(τ)             (task types)
//!     | T[τ₁, ...]         (generic type application)

use std::collections::HashMap;
use tjlang_ast::{Type, PrimitiveType, SourceSpan};
use tjlang_diagnostics::SourceSpan as DiagnosticSourceSpan;

/// Algebraic type representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AlgebraicType {
    // Primitive types
    Int,
    Float,
    Bool,
    Str,
    Any,
    
    // Product types (struct-like composition)
    Product(Vec<AlgebraicType>),
    
    // Sum types (union types)
    Sum(Vec<AlgebraicType>),
    
    // Container types
    Option(Box<AlgebraicType>),
    Result(Box<AlgebraicType>, Box<AlgebraicType>),
    Vec(Box<AlgebraicType>),
    Set(Box<AlgebraicType>),
    Map(Box<AlgebraicType>, Box<AlgebraicType>),
    Tuple(Vec<AlgebraicType>),
    
    // Function types
    Function(Vec<AlgebraicType>, Box<AlgebraicType>),
    
    // Task types
    Task(Box<AlgebraicType>),
    
    // Generic type application
    Generic(String, Vec<AlgebraicType>),
    
    // Type variables for inference
    Variable(String),
}

impl AlgebraicType {
    /// Convert from TJLang AST Type to AlgebraicType
    pub fn from_ast_type(ast_type: &Type) -> Self {
        match ast_type {
            Type::Primitive(primitive) => match primitive {
                PrimitiveType::Int => AlgebraicType::Int,
                PrimitiveType::Float => AlgebraicType::Float,
                PrimitiveType::Bool => AlgebraicType::Bool,
                PrimitiveType::Str => AlgebraicType::Str,
                PrimitiveType::Any => AlgebraicType::Any,
            },
            Type::Identifier(name) => AlgebraicType::Generic(name.clone(), vec![]),
            Type::Generic { name, type_args, .. } => {
                AlgebraicType::Generic(name.clone(), type_args.iter().map(Self::from_ast_type).collect())
            },
            Type::Union { types, .. } => {
                AlgebraicType::Sum(types.iter().map(Self::from_ast_type).collect())
            },
            Type::Option { inner, .. } => {
                AlgebraicType::Option(Box::new(Self::from_ast_type(inner)))
            },
            Type::Result { ok_type, error_type, .. } => {
                AlgebraicType::Result(
                    Box::new(Self::from_ast_type(ok_type)),
                    Box::new(Self::from_ast_type(error_type))
                )
            },
            Type::Function { params, return_type, .. } => {
                AlgebraicType::Function(
                    params.iter().map(Self::from_ast_type).collect(),
                    Box::new(Self::from_ast_type(return_type))
                )
            },
            Type::Vec { element_type, .. } => {
                AlgebraicType::Vec(Box::new(Self::from_ast_type(element_type)))
            },
            Type::Set { element_type, .. } => {
                AlgebraicType::Set(Box::new(Self::from_ast_type(element_type)))
            },
            Type::Map { key_type, value_type, .. } => {
                AlgebraicType::Map(
                    Box::new(Self::from_ast_type(key_type)),
                    Box::new(Self::from_ast_type(value_type))
                )
            },
            Type::Tuple { types, .. } => {
                AlgebraicType::Tuple(types.iter().map(Self::from_ast_type).collect())
            },
        }
    }
    
    /// Convert to TJLang AST Type
    pub fn to_ast_type(&self) -> Type {
        match self {
            AlgebraicType::Int => Type::Primitive(PrimitiveType::Int),
            AlgebraicType::Float => Type::Primitive(PrimitiveType::Float),
            AlgebraicType::Bool => Type::Primitive(PrimitiveType::Bool),
            AlgebraicType::Str => Type::Primitive(PrimitiveType::Str),
            AlgebraicType::Any => Type::Primitive(PrimitiveType::Any),
            AlgebraicType::Product(types) => {
                Type::Tuple {
                    types: types.iter().map(|t| t.to_ast_type()).collect(),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Sum(types) => {
                Type::Union {
                    types: types.iter().map(|t| t.to_ast_type()).collect(),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Option(inner) => {
                Type::Option {
                    inner: Box::new(inner.to_ast_type()),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Result(ok_type, error_type) => {
                Type::Result {
                    ok_type: Box::new(ok_type.to_ast_type()),
                    error_type: Box::new(error_type.to_ast_type()),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Function(params, return_type) => {
                Type::Function {
                    params: params.iter().map(|t| t.to_ast_type()).collect(),
                    return_type: Box::new(return_type.to_ast_type()),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Vec(element_type) => {
                Type::Vec {
                    element_type: Box::new(element_type.to_ast_type()),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Set(element_type) => {
                Type::Set {
                    element_type: Box::new(element_type.to_ast_type()),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Map(key_type, value_type) => {
                Type::Map {
                    key_type: Box::new(key_type.to_ast_type()),
                    value_type: Box::new(value_type.to_ast_type()),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Tuple(types) => {
                Type::Tuple {
                    types: types.iter().map(|t| t.to_ast_type()).collect(),
                    span: SourceSpan::default(),
                }
            },
            AlgebraicType::Task(inner) => {
                Type::Identifier("Task".to_string())
            },
            AlgebraicType::Generic(name, type_args) => {
                if type_args.is_empty() {
                    Type::Identifier(name.clone())
                } else {
                    Type::Generic {
                        name: name.clone(),
                        type_args: type_args.iter().map(|t| t.to_ast_type()).collect(),
                        span: SourceSpan::default(),
                    }
                }
            },
            AlgebraicType::Variable(name) => {
                Type::Identifier(name.clone())
            },
        }
    }
    
    /// Check if this type is a subtype of another type
    /// Implements the subtyping rules from the formal specification
    pub fn is_subtype_of(&self, other: &AlgebraicType) -> bool {
        match (self, other) {
            // τ ⊆ any (any is top type)
            (_, AlgebraicType::Any) => true,
            
            // τ ⊆ τ|σ (union introduction)
            (left, AlgebraicType::Sum(types)) => {
                types.iter().any(|t| left.is_subtype_of(t))
            },
            
            // Reflexivity
            (left, right) if left == right => true,
            
            // Product type subtyping (structural)
            (AlgebraicType::Product(left_types), AlgebraicType::Product(right_types)) => {
                left_types.len() == right_types.len() &&
                left_types.iter().zip(right_types.iter()).all(|(l, r)| l.is_subtype_of(r))
            },
            
            // Function type subtyping (contravariant in params, covariant in return)
            (AlgebraicType::Function(left_params, left_return), AlgebraicType::Function(right_params, right_return)) => {
                left_params.len() == right_params.len() &&
                right_params.iter().zip(left_params.iter()).all(|(r, l)| r.is_subtype_of(l)) &&
                left_return.is_subtype_of(right_return)
            },
            
            _ => false,
        }
    }
    
    /// Get the kind of this type
    pub fn kind(&self) -> TypeKind {
        match self {
            AlgebraicType::Int | AlgebraicType::Float | AlgebraicType::Bool | 
            AlgebraicType::Str | AlgebraicType::Any | AlgebraicType::Variable(_) => TypeKind::Proper,
            
            AlgebraicType::Vec(_) | AlgebraicType::Set(_) | AlgebraicType::Option(_) => {
                TypeKind::Constructor(Box::new(TypeKind::Proper))
            },
            
            AlgebraicType::Map(_, _) | AlgebraicType::Result(_, _) => {
                TypeKind::MultiConstructor(vec![TypeKind::Proper, TypeKind::Proper])
            },
            
            AlgebraicType::Function(_params, _) => {
                TypeKind::Constructor(Box::new(TypeKind::Proper))
            },
            
            AlgebraicType::Product(_types) | AlgebraicType::Sum(_types) | AlgebraicType::Tuple(_types) => {
                TypeKind::Proper
            },
            
            AlgebraicType::Task(_) => {
                TypeKind::Constructor(Box::new(TypeKind::Proper))
            },
            
            AlgebraicType::Generic(_, type_args) => {
                if type_args.is_empty() {
                    TypeKind::Proper
                } else {
                    TypeKind::Constructor(Box::new(TypeKind::Proper))
                }
            },
        }
    }
    
    /// Convert type to string for error messages
    pub fn to_string(&self) -> String {
        match self {
            AlgebraicType::Int => "int".to_string(),
            AlgebraicType::Float => "float".to_string(),
            AlgebraicType::Bool => "bool".to_string(),
            AlgebraicType::Str => "str".to_string(),
            AlgebraicType::Any => "any".to_string(),
            AlgebraicType::Product(types) => {
                format!("({})", types.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "))
            },
            AlgebraicType::Sum(types) => {
                format!("{}", types.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(" | "))
            },
            AlgebraicType::Function(params, return_type) => {
                format!("({}) -> {}", 
                       params.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "),
                       return_type.to_string())
            },
            AlgebraicType::Vec(element_type) => {
                format!("Vec<{}>", element_type.to_string())
            },
            AlgebraicType::Option(inner) => {
                format!("Option<{}>", inner.to_string())
            },
            AlgebraicType::Result(ok_type, error_type) => {
                format!("Result<{}, {}>", ok_type.to_string(), error_type.to_string())
            },
            AlgebraicType::Generic(name, type_args) => {
                if type_args.is_empty() {
                    name.clone()
                } else {
                    format!("{}<{}>", name, type_args.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "))
                }
            },
            AlgebraicType::Variable(name) => name.clone(),
            _ => "unknown".to_string(),
        }
    }
}

/// Type kinds as defined in the formal specification
/// 
/// κ : 𝒦 = * | * → * | * → * → * | ...
/// where * denotes a proper type
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TypeKind {
    /// Proper type (*)
    Proper,
    /// Type constructor (* → *)
    Constructor(Box<TypeKind>),
    /// Multi-argument constructor (* → * → *)
    MultiConstructor(Vec<TypeKind>),
}
