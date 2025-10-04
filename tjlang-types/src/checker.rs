//! Type Checker
//! 
//! Implements basic type checking functionality that can be used
//! by both runtime and static analysis.

use crate::types::{Type, TypeEnvironment};

/// Type checker for basic type checking
pub struct TypeChecker {
    environment: TypeEnvironment,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            environment: TypeEnvironment::new(),
        }
    }
    
    /// Check if a type is valid
    pub fn check_type(&self, type_: &Type) -> bool {
        match type_ {
            Type::Int | Type::Float | Type::Bool | Type::Str | Type::Any => true,
            Type::Function(params, return_type) => {
                params.iter().all(|p| self.check_type(p)) && self.check_type(return_type)
            },
            Type::Vec(element_type) => self.check_type(element_type),
            Type::Option(inner) => self.check_type(inner),
            Type::Result(ok_type, error_type) => {
                self.check_type(ok_type) && self.check_type(error_type)
            },
            Type::Product(types) => types.iter().all(|t| self.check_type(t)),
            Type::Sum(types) => types.iter().all(|t| self.check_type(t)),
            Type::Tuple(types) => types.iter().all(|t| self.check_type(t)),
            Type::Set(element_type) => self.check_type(element_type),
            Type::Map(key_type, value_type) => {
                self.check_type(key_type) && self.check_type(value_type)
            },
            Type::Task(inner) => self.check_type(inner),
            Type::Generic(_, type_args) => type_args.iter().all(|t| self.check_type(t)),
            Type::Variable(_) => true, // Type variables are always valid
        }
    }
    
    /// Check if two types are compatible
    pub fn check_compatibility(&self, left: &Type, right: &Type) -> bool {
        left.is_subtype_of(right) || right.is_subtype_of(left)
    }
    
    /// Get the type environment
    pub fn environment(&self) -> &TypeEnvironment {
        &self.environment
    }
    
    /// Get a mutable reference to the type environment
    pub fn environment_mut(&mut self) -> &mut TypeEnvironment {
        &mut self.environment
    }
}
