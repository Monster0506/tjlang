//! TJLang Runtime Values
//! 
//! Advanced value system supporting all TJLang types with garbage collection.

use std::collections::HashMap;
use std::sync::Arc;
use std::rc::Rc;
use tjlang_ast::{Expression, Type, PrimitiveType, SourceSpan};
use codespan::{Files, Span};

/// Create a dummy SourceSpan for runtime values
fn dummy_span() -> SourceSpan {
    let mut files = Files::new();
    let file_id = files.add("runtime", "");
    SourceSpan { 
        file_id, 
        span: Span::new(0, 0) 
    }
}

/// Runtime value that can represent any TJLang value
#[derive(Debug)]
pub enum Value {
    // Primitive types
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    None,
    
    // Complex types
    Struct {
        name: String,
        fields: HashMap<String, Value>,
    },
    Enum {
        name: String,
        variant: String,
        fields: Vec<Value>,
    },
    Tuple(Vec<Value>),
    Vec(Vec<Value>),
    Set(std::collections::HashSet<Value>),
    Map(HashMap<Value, Value>),
    
    // Function types
    Function {
        name: String,
        params: Vec<String>,
        body: Expression,
        closure: HashMap<String, Value>,
    },
    Closure {
        params: Vec<String>,
        body: Expression,
        closure: HashMap<String, Value>,
    },
    
    // Concurrency types
    Channel {
        sender: std::sync::mpsc::Sender<Value>,
        receiver: std::sync::mpsc::Receiver<Value>,
    },
    Task {
        id: u64,
        handle: std::thread::JoinHandle<Value>,
    },
    
    // Reference types (for GC)
    Reference(usize),
    
    // Type information
    Type(Type),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Int(i) => Value::Int(*i),
            Value::Float(f) => Value::Float(*f),
            Value::Bool(b) => Value::Bool(*b),
            Value::String(s) => Value::String(s.clone()),
            Value::None => Value::None,
            Value::Struct { name, fields } => Value::Struct {
                name: name.clone(),
                fields: fields.clone(),
            },
            Value::Enum { name, variant, fields } => Value::Enum {
                name: name.clone(),
                variant: variant.clone(),
                fields: fields.clone(),
            },
            Value::Tuple(elements) => Value::Tuple(elements.clone()),
            Value::Vec(elements) => Value::Vec(elements.clone()),
            Value::Set(elements) => Value::Set(elements.clone()),
            Value::Map(entries) => Value::Map(entries.clone()),
            Value::Function { name, params, body, closure } => Value::Function {
                name: name.clone(),
                params: params.clone(),
                body: body.clone(),
                closure: closure.clone(),
            },
            Value::Closure { params, body, closure } => Value::Closure {
                params: params.clone(),
                body: body.clone(),
                closure: closure.clone(),
            },
            Value::Channel { .. } => Value::Channel {
                sender: std::sync::mpsc::channel().0,
                receiver: std::sync::mpsc::channel().1,
            },
            Value::Task { id, .. } => Value::Task {
                id: *id,
                handle: std::thread::spawn(|| Value::None),
            },
            Value::Reference(addr) => Value::Reference(*addr),
            Value::Type(t) => Value::Type(t.clone()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::None, Value::None) => true,
            (Value::Struct { name: a, fields: fa }, Value::Struct { name: b, fields: fb }) => {
                a == b && fa == fb
            },
            (Value::Enum { name: a, variant: va, fields: fa }, Value::Enum { name: b, variant: vb, fields: fb }) => {
                a == b && va == vb && fa == fb
            },
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            (Value::Vec(a), Value::Vec(b)) => a == b,
            (Value::Set(a), Value::Set(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::Reference(a), Value::Reference(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Int(i) => i.hash(state),
            Value::Float(f) => f.to_bits().hash(state),
            Value::Bool(b) => b.hash(state),
            Value::String(s) => s.hash(state),
            Value::None => 0.hash(state),
            Value::Struct { name, fields } => {
                name.hash(state);
                for (k, v) in fields {
                    k.hash(state);
                    v.hash(state);
                }
            },
            Value::Enum { name, variant, fields } => {
                name.hash(state);
                variant.hash(state);
                for field in fields {
                    field.hash(state);
                }
            },
            Value::Tuple(values) => {
                for value in values {
                    value.hash(state);
                }
            },
            Value::Vec(values) => {
                for value in values {
                    value.hash(state);
                }
            },
            Value::Set(set) => {
                for value in set {
                    value.hash(state);
                }
            },
            Value::Map(map) => {
                for (k, v) in map {
                    k.hash(state);
                    v.hash(state);
                }
            },
            Value::Reference(addr) => addr.hash(state),
            _ => 0.hash(state), // Functions, channels, tasks are not hashable
        }
    }
}

impl Value {
    /// Get the type of this value
    pub fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Primitive(PrimitiveType::Int),
            Value::Float(_) => Type::Primitive(PrimitiveType::Float),
            Value::Bool(_) => Type::Primitive(PrimitiveType::Bool),
            Value::String(_) => Type::Primitive(PrimitiveType::Str),
            Value::None => Type::Primitive(PrimitiveType::Any),
            Value::Struct { name, .. } => Type::Identifier(name.clone()),
            Value::Enum { name, .. } => Type::Identifier(name.clone()),
            Value::Tuple(types) => Type::Tuple {
                types: types.iter().map(|v| v.get_type()).collect(),
                span: dummy_span(),
            },
            Value::Vec(elements) => {
                if elements.is_empty() {
                    Type::Vec {
                        element_type: Box::new(Type::Primitive(PrimitiveType::Any)),
                        span: dummy_span(),
                    }
                } else {
                    Type::Vec {
                        element_type: Box::new(elements[0].get_type()),
                        span: dummy_span(),
                    }
                }
            },
            Value::Set(elements) => {
                if elements.is_empty() {
                    Type::Set {
                        element_type: Box::new(Type::Primitive(PrimitiveType::Any)),
                        span: dummy_span(),
                    }
                } else {
                    Type::Set {
                        element_type: Box::new(elements.iter().next().unwrap().get_type()),
                        span: dummy_span(),
                    }
                }
            },
            Value::Map(entries) => {
                if entries.is_empty() {
                    Type::Map {
                        key_type: Box::new(Type::Primitive(PrimitiveType::Any)),
                        value_type: Box::new(Type::Primitive(PrimitiveType::Any)),
                        span: dummy_span(),
                    }
                } else {
                    let (k, v) = entries.iter().next().unwrap();
                    Type::Map {
                        key_type: Box::new(k.get_type()),
                        value_type: Box::new(v.get_type()),
                        span: dummy_span(),
                    }
                }
            },
            Value::Function { .. } => Type::Function {
                params: vec![], // TODO: Extract from function
                return_type: Box::new(Type::Primitive(PrimitiveType::Any)),
                span: dummy_span(),
            },
            Value::Closure { .. } => Type::Function {
                params: vec![], // TODO: Extract from closure
                return_type: Box::new(Type::Primitive(PrimitiveType::Any)),
                span: dummy_span(),
            },
            Value::Channel { .. } => Type::Identifier("Channel".to_string()),
            Value::Task { .. } => Type::Identifier("Task".to_string()),
            Value::Reference(_) => Type::Primitive(PrimitiveType::Any),
            Value::Type(t) => t.clone(),
        }
    }
    
    /// Check if this value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::None => false,
            Value::Int(0) => false,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Vec(v) => !v.is_empty(),
            Value::Set(s) => !s.is_empty(),
            Value::Map(m) => !m.is_empty(),
            _ => true,
        }
    }
    
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::None => "None".to_string(),
            Value::Struct { name, fields } => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{} {{ {} }}", name, field_strs.join(", "))
            },
            Value::Enum { name, variant, fields } => {
                if fields.is_empty() {
                    format!("{}::{}", name, variant)
                } else {
                    let field_strs: Vec<String> = fields
                        .iter()
                        .map(|f| f.to_string())
                        .collect();
                    format!("{}::{} ({})", name, variant, field_strs.join(", "))
                }
            },
            Value::Tuple(values) => {
                let value_strs: Vec<String> = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect();
                format!("({})", value_strs.join(", "))
            },
            Value::Vec(values) => {
                let value_strs: Vec<String> = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect();
                format!("[{}]", value_strs.join(", "))
            },
            Value::Set(values) => {
                let value_strs: Vec<String> = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect();
                format!("{{{}}}", value_strs.join(", "))
            },
            Value::Map(entries) => {
                let entry_strs: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k.to_string(), v.to_string()))
                    .collect();
                format!("{{{}}}", entry_strs.join(", "))
            },
            Value::Function { name, .. } => format!("<function {}>", name),
            Value::Closure { .. } => "<closure>".to_string(),
            Value::Channel { .. } => "<channel>".to_string(),
            Value::Task { id, .. } => format!("<task {}>", id),
            Value::Reference(addr) => format!("<ref {}>", addr),
            Value::Type(t) => format!("<type {:?}>", t),
        }
    }
}
