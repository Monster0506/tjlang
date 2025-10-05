//! Expression definitions for TJLang AST

use super::*;

/// Expression system
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Variable {
        name: String,
        span: SourceSpan,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
        span: SourceSpan,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
        span: SourceSpan,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
        span: SourceSpan,
    },
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
        span: SourceSpan,
    },
    Member {
        target: Box<Expression>,
        member: String,
        span: SourceSpan,
    },
    Lambda {
        params: Vec<Parameter>,
        body: Box<Expression>,
        span: SourceSpan,
    },
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        inclusive: bool,
        span: SourceSpan,
    },
    Spawn {
        expression: Box<Expression>,
        span: SourceSpan,
    },
    If {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
        span: SourceSpan,
    },
    Match {
        expression: Box<Expression>,
        arms: Vec<MatchArm>,
        span: SourceSpan,
    },
    StructLiteral {
        name: String,
        fields: Vec<FieldInit>,
        span: SourceSpan,
    },
    TupleLiteral {
        elements: Vec<Expression>,
        span: SourceSpan,
    },
    VecLiteral {
        elements: Vec<Expression>,
        span: SourceSpan,
    },
    SetLiteral {
        elements: Vec<Expression>,
        span: SourceSpan,
    },
    MapLiteral {
        entries: Vec<MapEntry>,
        span: SourceSpan,
    },
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    ShiftLeft,
    ShiftRight,
    BitAnd,
    BitXor,
    BitOr,
    And,
    Or,
    Assign,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not,
    BitNot,
}

/// Literal values
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    FString(String),
    FStringInterpolation(Vec<FStringPart>),
    Bool(bool),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FStringPart {
    Text(String),
    Expression(Box<Expression>),
}

/// Field initialization
#[derive(Debug, Clone, PartialEq)]
pub struct FieldInit {
    pub name: String,
    pub value: Expression,
    pub span: SourceSpan,
}

/// Map entry
#[derive(Debug, Clone, PartialEq)]
pub struct MapEntry {
    pub key: Expression,
    pub value: Expression,
    pub span: SourceSpan,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveType::Int => write!(f, "int"),
            PrimitiveType::Float => write!(f, "float"),
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::Str => write!(f, "str"),
            PrimitiveType::Any => write!(f, "any"),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Power => write!(f, "**"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::LessThanEqual => write!(f, "<="),
            BinaryOperator::GreaterThanEqual => write!(f, ">="),
            BinaryOperator::ShiftLeft => write!(f, "<<"),
            BinaryOperator::ShiftRight => write!(f, ">>"),
            BinaryOperator::BitAnd => write!(f, "&"),
            BinaryOperator::BitXor => write!(f, "^"),
            BinaryOperator::BitOr => write!(f, "|"),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Or => write!(f, "or"),
            BinaryOperator::Assign => write!(f, "="),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::BitNot => write!(f, "~"),
        }
    }
}
