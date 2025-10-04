//! Core Type System Types
//! 
//! Implements the basic type system types that can be shared
//! between runtime and static analysis.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Core type representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    // Primitive types
    Int,
    Float,
    Bool,
    Str,
    Any,
    
    // Product types (struct-like composition)
    Product(Vec<Type>),
    
    // Sum types (union types)
    Sum(Vec<Type>),
    
    // Container types
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Vec(Box<Type>),
    Set(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    
    // Function types
    Function(Vec<Type>, Box<Type>),
    
    // Task types
    Task(Box<Type>),
    
    // Generic type application
    Generic(String, Vec<Type>),
    
    // Type variables for inference
    Variable(String),
}

impl Type {
    /// Check if this type is a subtype of another type
    pub fn is_subtype_of(&self, other: &Type) -> bool {
        match (self, other) {
            // τ ⊆ any (any is top type)
            (_, Type::Any) => true,
            
            // τ ⊆ τ|σ (union introduction)
            (left, Type::Sum(types)) => {
                types.iter().any(|t| left.is_subtype_of(t))
            },
            
            // Reflexivity
            (left, right) if left == right => true,
            
            // Product type subtyping (structural)
            (Type::Product(left_types), Type::Product(right_types)) => {
                left_types.len() == right_types.len() &&
                left_types.iter().zip(right_types.iter()).all(|(l, r)| l.is_subtype_of(r))
            },
            
            // Function type subtyping (contravariant in params, covariant in return)
            (Type::Function(left_params, left_return), Type::Function(right_params, right_return)) => {
                left_params.len() == right_params.len() &&
                right_params.iter().zip(left_params.iter()).all(|(r, l)| r.is_subtype_of(l)) &&
                left_return.is_subtype_of(right_return)
            },
            
            _ => false,
        }
    }
    
    /// Convert type to string for error messages
    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Str => "str".to_string(),
            Type::Any => "any".to_string(),
            Type::Product(types) => {
                format!("({})", types.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "))
            },
            Type::Sum(types) => {
                format!("{}", types.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(" | "))
            },
            Type::Function(params, return_type) => {
                format!("({}) -> {}", 
                       params.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "),
                       return_type.to_string())
            },
            Type::Vec(element_type) => {
                format!("Vec<{}>", element_type.to_string())
            },
            Type::Option(inner) => {
                format!("Option<{}>", inner.to_string())
            },
            Type::Result(ok_type, error_type) => {
                format!("Result<{}, {}>", ok_type.to_string(), error_type.to_string())
            },
            Type::Generic(name, type_args) => {
                if type_args.is_empty() {
                    name.clone()
                } else {
                    format!("{}<{}>", name, type_args.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "))
                }
            },
            Type::Variable(name) => name.clone(),
            _ => "unknown".to_string(),
        }
    }
}

/// Type environment for tracking variable and function types
#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    /// Variable bindings
    pub variables: HashMap<String, Type>,
    
    /// Function bindings
    pub functions: HashMap<String, Type>,
    
    /// Nested scopes
    pub scopes: Vec<HashMap<String, Type>>,
}

impl TypeEnvironment {
    /// Create a new type environment
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
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
    pub fn lookup_variable(&self, name: &str) -> Option<Type> {
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
    pub fn lookup_function(&self, name: &str) -> Option<Type> {
        self.functions.get(name).cloned()
    }
    
    /// Bind a variable in the current scope
    pub fn bind_variable(&mut self, name: String, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        } else {
            self.variables.insert(name, ty);
        }
    }
    
    /// Bind a function
    pub fn bind_function(&mut self, name: String, ty: Type) {
        self.functions.insert(name, ty);
    }
}
