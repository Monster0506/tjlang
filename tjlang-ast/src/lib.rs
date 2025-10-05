//! TJLang AST
//!
//! Abstract Syntax Tree nodes for TJLang language constructs.

use std::fmt;

// Re-export SourceSpan from codespan to avoid duplication
pub use codespan::FileId;
use codespan::Span;

/// A source span representing a location in source code
#[derive(Debug, Clone, PartialEq)]
pub struct SourceSpan {
    pub file_id: FileId,
    pub span: Span,
}

/// A complete TJLang program
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub units: Vec<ProgramUnit>,
    pub span: SourceSpan,
}

/// Top-level program units
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramUnit {
    Module(ModuleDecl),
    Import(ImportDecl),
    Export(ExportDecl),
    Declaration(Declaration),
    Expression(Expression),
    Statement(Statement),
}

/// Module declaration
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDecl {
    pub name: String,
    pub span: SourceSpan,
}

/// Import declaration
#[derive(Debug, Clone, PartialEq)]
pub enum ImportDecl {
    Simple {
        module: QualifiedName,
        alias: Option<String>,
        span: SourceSpan,
    },
    Selective {
        module: QualifiedName,
        items: Vec<String>,
        span: SourceSpan,
    },
}

/// Qualified name (e.g., "std.collections.Map")
#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedName {
    pub parts: Vec<String>,
    pub span: SourceSpan,
}

/// Export declaration
#[derive(Debug, Clone, PartialEq)]
pub enum ExportDecl {
    Declaration(Declaration),
    Identifier(String),
    IdentifierList(Vec<String>),
}

/// All possible declarations
#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Function(FunctionDecl),
    Interface(InterfaceDecl),
    Type(TypeDecl),
    Enum(EnumDecl),
    Struct(StructDecl),
    Variable(VariableDecl),
    Implementation(ImplBlock),
    Module(ModuleDecl),
}

/// Function declaration
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub generic_params: Vec<GenericParam>,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
    pub span: SourceSpan,
}

/// Generic parameter with trait bounds
#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name: String,
    pub bounds: Vec<String>,
    pub span: SourceSpan,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub span: SourceSpan,
}

/// Type system
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Identifier(String),
    Generic {
        name: String,
        type_args: Vec<Type>,
        span: SourceSpan,
    },
    Union {
        types: Vec<Type>,
        span: SourceSpan,
    },
    Option {
        inner: Box<Type>,
        span: SourceSpan,
    },
    Result {
        ok_type: Box<Type>,
        error_type: Box<Type>,
        span: SourceSpan,
    },
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
        span: SourceSpan,
    },
    Vec {
        element_type: Box<Type>,
        span: SourceSpan,
    },
    Set {
        element_type: Box<Type>,
        span: SourceSpan,
    },
    Map {
        key_type: Box<Type>,
        value_type: Box<Type>,
        span: SourceSpan,
    },
    Tuple {
        types: Vec<Type>,
        span: SourceSpan,
    },
}

/// Primitive types
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Int,
    Float,
    Bool,
    Str,
    Any,
}

// Include submodules
mod expressions;
mod statements;

// Custom type declarations
/// Type alias declaration
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl {
    pub name: String,
    pub type_alias: Type,
    pub span: SourceSpan,
}

/// Struct declaration
#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<FieldDecl>,
    pub span: SourceSpan,
}

/// Field declaration
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDecl {
    pub name: String,
    pub field_type: Type,
    pub span: SourceSpan,
}

/// Enum declaration
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: String,
    pub type_params: Vec<String>,
    pub variants: Vec<EnumVariant>,
    pub span: SourceSpan,
}

/// Enum variant
#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Type>,
    pub span: SourceSpan,
}

/// Interface declaration
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDecl {
    pub name: String,
    pub extends: Vec<String>,
    pub methods: Vec<MethodSig>,
    pub span: SourceSpan,
}

/// Method signature
#[derive(Debug, Clone, PartialEq)]
pub struct MethodSig {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub span: SourceSpan,
}

/// Implementation block
#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    pub trait_name: String,
    pub type_name: String,
    pub methods: Vec<MethodDecl>,
    pub span: SourceSpan,
}

/// Method declaration
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDecl {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
    pub span: SourceSpan,
}

// Re-export all types from submodules
pub use expressions::*;
pub use statements::*;

pub use statements::*;

pub use statements::*;

pub use statements::*;
