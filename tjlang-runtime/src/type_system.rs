//! TJLang Type System
//! 
//! Advanced runtime type checking and inference.

use std::collections::HashMap;
use crate::values::Value;
use tjlang_ast::{Type, PrimitiveType, SourceSpan};

/// Runtime type checker
pub struct TypeChecker {
    /// Type environment
    environment: HashMap<String, Type>,
    
    /// Type constraints
    constraints: Vec<TypeConstraint>,
    
    /// Type inference cache
    inference_cache: HashMap<String, Type>,
}

/// Type constraint for inference
#[derive(Debug, Clone)]
struct TypeConstraint {
    pub variable: String,
    pub constraint: Type,
    pub source: SourceSpan,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
            constraints: Vec::new(),
            inference_cache: HashMap::new(),
        }
    }
    
    /// Check if a value matches a type
    pub fn check_type(&self, value: &Value, expected_type: &Type) -> bool {
        match (value, expected_type) {
            (Value::Int(_), Type::Primitive(PrimitiveType::Int)) => true,
            (Value::Float(_), Type::Primitive(PrimitiveType::Float)) => true,
            (Value::Bool(_), Type::Primitive(PrimitiveType::Bool)) => true,
            (Value::String(_), Type::Primitive(PrimitiveType::Str)) => true,
            (Value::None, Type::Primitive(PrimitiveType::Any)) => true,
            
            (Value::Struct { name, .. }, Type::Identifier(type_name)) => {
                name == type_name
            },
            (Value::Enum { name, .. }, Type::Identifier(type_name)) => {
                name == type_name
            },
            
            (Value::Tuple(elements), Type::Tuple { types, .. }) => {
                if elements.len() == types.len() {
                    elements.iter().zip(types.iter()).all(|(elem, typ)| {
                        self.check_type(elem, typ)
                    })
                } else {
                    false
                }
            },
            
            (Value::Vec(elements), Type::Vec { element_type, .. }) => {
                elements.iter().all(|elem| self.check_type(elem, element_type))
            },
            
            (Value::Set(elements), Type::Set { element_type, .. }) => {
                elements.iter().all(|elem| self.check_type(elem, element_type))
            },
            
            (Value::Map(entries), Type::Map { key_type, value_type, .. }) => {
                entries.iter().all(|(k, v)| {
                    self.check_type(k, key_type) && self.check_type(v, value_type)
                })
            },
            
            (Value::Function { .. }, Type::Function { .. }) => {
                // TODO: Implement function type checking
                true
            },
            
            (Value::Closure { .. }, Type::Function { .. }) => {
                // TODO: Implement closure type checking
                true
            },
            
            (Value::Channel { .. }, Type::Identifier(type_name)) => {
                type_name == "Channel"
            },
            
            (Value::Task { .. }, Type::Identifier(type_name)) => {
                type_name == "Task"
            },
            
            // Union types
            (value, Type::Union { types, .. }) => {
                types.iter().any(|typ| self.check_type(value, typ))
            },
            
            // Option types
            (Value::None, Type::Option { .. }) => true,
            (value, Type::Option { inner, .. }) => {
                self.check_type(value, inner)
            },
            
            // Result types
            (value, Type::Result { ok_type, error_type, .. }) => {
                // TODO: Implement Result type checking
                self.check_type(value, ok_type) || self.check_type(value, error_type)
            },
            
            // Generic types
            (value, Type::Generic { name, type_args, .. }) => {
                // TODO: Implement generic type checking
                true
            },
            
            _ => false,
        }
    }
    
    /// Infer the type of a value
    pub fn infer_type(&self, value: &Value) -> Type {
        value.get_type()
    }
    
    /// Add a type constraint
    pub fn add_constraint(&mut self, variable: String, constraint: Type, source: SourceSpan) {
        self.constraints.push(TypeConstraint {
            variable,
            constraint,
            source,
        });
    }
    
    /// Solve type constraints
    pub fn solve_constraints(&mut self) -> Result<HashMap<String, Type>, String> {
        let mut solution = HashMap::new();
        
        for constraint in &self.constraints {
            match &constraint.constraint {
                Type::Primitive(primitive) => {
                    solution.insert(constraint.variable.clone(), Type::Primitive(primitive.clone()));
                },
                Type::Identifier(name) => {
                    solution.insert(constraint.variable.clone(), Type::Identifier(name.clone()));
                },
                Type::Tuple { types, span } => {
                    solution.insert(constraint.variable.clone(), Type::Tuple {
                        types: types.clone(),
                        span: span.clone(),
                    });
                },
                Type::Vec { element_type, span } => {
                    solution.insert(constraint.variable.clone(), Type::Vec {
                        element_type: element_type.clone(),
                        span: span.clone(),
                    });
                },
                Type::Set { element_type, span } => {
                    solution.insert(constraint.variable.clone(), Type::Set {
                        element_type: element_type.clone(),
                        span: span.clone(),
                    });
                },
                Type::Map { key_type, value_type, span } => {
                    solution.insert(constraint.variable.clone(), Type::Map {
                        key_type: key_type.clone(),
                        value_type: value_type.clone(),
                        span: span.clone(),
                    });
                },
                Type::Function { params, return_type, span } => {
                    solution.insert(constraint.variable.clone(), Type::Function {
                        params: params.clone(),
                        return_type: return_type.clone(),
                        span: span.clone(),
                    });
                },
                Type::Union { types, span } => {
                    solution.insert(constraint.variable.clone(), Type::Union {
                        types: types.clone(),
                        span: span.clone(),
                    });
                },
                Type::Option { inner, span } => {
                    solution.insert(constraint.variable.clone(), Type::Option {
                        inner: inner.clone(),
                        span: span.clone(),
                    });
                },
                Type::Result { ok_type, error_type, span } => {
                    solution.insert(constraint.variable.clone(), Type::Result {
                        ok_type: ok_type.clone(),
                        error_type: error_type.clone(),
                        span: span.clone(),
                    });
                },
                Type::Generic { name, type_args, span } => {
                    solution.insert(constraint.variable.clone(), Type::Generic {
                        name: name.clone(),
                        type_args: type_args.clone(),
                        span: span.clone(),
                    });
                },
            }
        }
        
        Ok(solution)
    }
    
    /// Check type compatibility
    pub fn are_compatible(&self, type1: &Type, type2: &Type) -> bool {
        match (type1, type2) {
            (Type::Primitive(p1), Type::Primitive(p2)) => p1 == p2,
            (Type::Identifier(n1), Type::Identifier(n2)) => n1 == n2,
            (Type::Tuple { types: t1, .. }, Type::Tuple { types: t2, .. }) => {
                t1.len() == t2.len() && t1.iter().zip(t2.iter()).all(|(a, b)| self.are_compatible(a, b))
            },
            (Type::Vec { element_type: e1, .. }, Type::Vec { element_type: e2, .. }) => {
                self.are_compatible(e1, e2)
            },
            (Type::Set { element_type: e1, .. }, Type::Set { element_type: e2, .. }) => {
                self.are_compatible(e1, e2)
            },
            (Type::Map { key_type: k1, value_type: v1, .. }, Type::Map { key_type: k2, value_type: v2, .. }) => {
                self.are_compatible(k1, k2) && self.are_compatible(v1, v2)
            },
            (Type::Function { params: p1, return_type: r1, .. }, Type::Function { params: p2, return_type: r2, .. }) => {
                p1.len() == p2.len() && 
                p1.iter().zip(p2.iter()).all(|(a, b)| self.are_compatible(a, b)) &&
                self.are_compatible(r1, r2)
            },
            (Type::Union { types: t1, .. }, Type::Union { types: t2, .. }) => {
                t1.iter().any(|t1| t2.iter().any(|t2| self.are_compatible(t1, t2)))
            },
            (Type::Option { inner: i1, .. }, Type::Option { inner: i2, .. }) => {
                self.are_compatible(i1, i2)
            },
            (Type::Result { ok_type: o1, error_type: e1, .. }, Type::Result { ok_type: o2, error_type: e2, .. }) => {
                self.are_compatible(o1, o2) && self.are_compatible(e1, e2)
            },
            (Type::Generic { name: n1, type_args: a1, .. }, Type::Generic { name: n2, type_args: a2, .. }) => {
                n1 == n2 && a1.len() == a2.len() && a1.iter().zip(a2.iter()).all(|(a, b)| self.are_compatible(a, b))
            },
            _ => false,
        }
    }
    
    /// Get the most specific common type
    pub fn common_type(&self, type1: &Type, type2: &Type) -> Option<Type> {
        if self.are_compatible(type1, type2) {
            Some(type1.clone())
        } else {
            // Try to find a common supertype
            match (type1, type2) {
                (Type::Primitive(PrimitiveType::Int), Type::Primitive(PrimitiveType::Float)) => {
                    Some(Type::Primitive(PrimitiveType::Float))
                },
                (Type::Primitive(PrimitiveType::Float), Type::Primitive(PrimitiveType::Int)) => {
                    Some(Type::Primitive(PrimitiveType::Float))
                },
                (Type::Union { types, .. }, other) => {
                    if types.iter().any(|t| self.are_compatible(t, other)) {
                        Some(other.clone())
                    } else {
                        None
                    }
                },
                (other, Type::Union { types, .. }) => {
                    if types.iter().any(|t| self.are_compatible(t, other)) {
                        Some(other.clone())
                    } else {
                        None
                    }
                },
                _ => None,
            }
        }
    }
    
    /// Check if a type is a subtype of another
    pub fn is_subtype(&self, subtype: &Type, supertype: &Type) -> bool {
        match (subtype, supertype) {
            (Type::Primitive(p1), Type::Primitive(p2)) => p1 == p2,
            (Type::Identifier(n1), Type::Identifier(n2)) => n1 == n2,
            (Type::Tuple { types: t1, .. }, Type::Tuple { types: t2, .. }) => {
                t1.len() == t2.len() && t1.iter().zip(t2.iter()).all(|(a, b)| self.is_subtype(a, b))
            },
            (Type::Vec { element_type: e1, .. }, Type::Vec { element_type: e2, .. }) => {
                self.is_subtype(e1, e2)
            },
            (Type::Set { element_type: e1, .. }, Type::Set { element_type: e2, .. }) => {
                self.is_subtype(e1, e2)
            },
            (Type::Map { key_type: k1, value_type: v1, .. }, Type::Map { key_type: k2, value_type: v2, .. }) => {
                self.is_subtype(k1, k2) && self.is_subtype(v1, v2)
            },
            (Type::Function { params: p1, return_type: r1, .. }, Type::Function { params: p2, return_type: r2, .. }) => {
                p1.len() == p2.len() && 
                p1.iter().zip(p2.iter()).all(|(a, b)| self.is_subtype(b, a)) && // Contravariant parameters
                self.is_subtype(r1, r2) // Covariant return type
            },
            (Type::Union { types, .. }, other) => {
                types.iter().all(|t| self.is_subtype(t, other))
            },
            (other, Type::Union { types, .. }) => {
                types.iter().any(|t| self.is_subtype(other, t))
            },
            (Type::Option { inner: i1, .. }, Type::Option { inner: i2, .. }) => {
                self.is_subtype(i1, i2)
            },
            (Type::Result { ok_type: o1, error_type: e1, .. }, Type::Result { ok_type: o2, error_type: e2, .. }) => {
                self.is_subtype(o1, o2) && self.is_subtype(e1, e2)
            },
            (Type::Generic { name: n1, type_args: a1, .. }, Type::Generic { name: n2, type_args: a2, .. }) => {
                n1 == n2 && a1.len() == a2.len() && a1.iter().zip(a2.iter()).all(|(a, b)| self.is_subtype(a, b))
            },
            _ => false,
        }
    }
    
    /// Get type environment
    pub fn get_environment(&self) -> &HashMap<String, Type> {
        &self.environment
    }
    
    /// Set type environment
    pub fn set_environment(&mut self, environment: HashMap<String, Type>) {
        self.environment = environment;
    }
    
    /// Clear constraints
    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
