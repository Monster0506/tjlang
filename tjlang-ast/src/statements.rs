//! Statement and expression definitions for TJLang AST

use super::*;

/// Interface declaration
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDecl {
    pub name: String,
    pub extends: Vec<String>,
    pub methods: Vec<MethodSignature>,
    pub span: SourceSpan,
}

/// Method signature
#[derive(Debug, Clone, PartialEq)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub span: SourceSpan,
}

/// Type declaration (type alias)
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl {
    pub name: String,
    pub type_def: Type,
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

/// Variable declaration
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub name: String,
    pub var_type: Type,
    pub value: Expression,
    pub span: SourceSpan,
}

/// Implementation block
#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    pub trait_name: Option<String>,
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

/// Block of statements
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: SourceSpan,
}

/// All possible statements
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Variable(VariableDecl),
    Expression(Expression),
    If(IfStatement),
    While(WhileStatement),
    DoWhile(DoWhileStatement),
    For(ForStatement),
    Match(MatchStatement),
    Return(ReturnStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Pass(PassStatement),
    Raise(RaiseStatement),
    Block(Block),
}

/// If statement
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub elif_branches: Vec<ElifBranch>,
    pub else_block: Option<Block>,
    pub span: SourceSpan,
}

/// Elif branch
#[derive(Debug, Clone, PartialEq)]
pub struct ElifBranch {
    pub condition: Expression,
    pub block: Block,
    pub span: SourceSpan,
}

/// While statement
#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
    pub span: SourceSpan,
}

/// Do-while statement
#[derive(Debug, Clone, PartialEq)]
pub struct DoWhileStatement {
    pub body: Block,
    pub condition: Expression,
    pub span: SourceSpan,
}

/// For statement
#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub var_name: String,
    pub var_type: Type,
    pub iterable: Expression,
    pub body: Block,
    pub span: SourceSpan,
}

/// Match statement
#[derive(Debug, Clone, PartialEq)]
pub struct MatchStatement {
    pub expression: Expression,
    pub arms: Vec<MatchArm>,
    pub span: SourceSpan,
}

/// Match arm
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Block,
    pub span: SourceSpan,
}

/// Pattern matching
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(Literal),
    Variable {
        name: String,
        pattern_type: Type,
        span: SourceSpan,
    },
    TraitCheck {
        name: String,
        trait_name: String,
        span: SourceSpan,
    },
    Constructor {
        name: String,
        fields: Vec<Pattern>,
        span: SourceSpan,
    },
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
        span: SourceSpan,
    },
    Tuple {
        patterns: Vec<Pattern>,
        span: SourceSpan,
    },
    Wildcard(SourceSpan),
}

/// Return statement
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: SourceSpan,
}

/// Break statement
#[derive(Debug, Clone, PartialEq)]
pub struct BreakStatement {
    pub span: SourceSpan,
}

/// Continue statement
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStatement {
    pub span: SourceSpan,
}

/// Pass statement
#[derive(Debug, Clone, PartialEq)]
pub struct PassStatement {
    pub span: SourceSpan,
}

/// Raise statement
#[derive(Debug, Clone, PartialEq)]
pub struct RaiseStatement {
    pub value: Expression,
    pub span: SourceSpan,
}
