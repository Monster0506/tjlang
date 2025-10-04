//! TJLang Lexer
//!
//! Tokenizes TJLang source code into a stream of tokens with source spans.

use codespan::{FileId, Span};
use codespan_reporting::diagnostic::Severity;
use logos::Logos;
use std::fmt;
use tjlang_diagnostics::{DiagnosticCollection, ErrorCode, SourceSpan, TJLangDiagnostic};

/// A TJLang token with source span information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: SourceSpan, text: String) -> Self {
        Self { kind, span, text }
    }
}

/// Token kinds for TJLang
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    #[token("def")]
    Def,
    #[token("return")]
    Return,
    #[token("type")]
    Type,
    #[token("enum")]
    Enum,
    #[token("interface")]
    Interface,
    #[token("mod")]
    Mod,
    #[token("import")]
    Import,
    #[token("export")]
    Export,
    #[token("if")]
    If,
    #[token("elif")]
    Elif,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("for")]
    For,
    #[token("match")]
    Match,
    #[token("Implements")]
    Implements,
    #[token("spawn")]
    Spawn,
    #[token("raise")]
    Raise,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("pass")]
    Pass,
    #[token("as")]
    As,
    #[token("extends")]
    Extends,

    // Primitive Types
    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("bool")]
    Bool,
    #[token("str")]
    Str,
    #[token("any")]
    Any,

    // Built-in Types
    #[token("Result")]
    Result,
    #[token("Option")]
    Option,

    // Symbols
    #[token("->")]
    Arrow,
    #[token("=")]
    Assign,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Lte,
    #[token(">=")]
    Gte,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("|")]
    Pipe,
    #[token("?")]
    Question,
    #[token("or")]
    Or,
    #[token("and")]
    And,
    #[token("!")]
    Bang,
    #[token(";")]
    Semicolon,
    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntLiteral(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f32>().ok())]
    FloatLiteral(f32),
    #[regex(r#""([^"\\]|\\.)*""#, |lex| Some(lex.slice()[1..lex.slice().len()-1].to_string()))]
    StringLiteral(String),
    #[regex(r#"f"[^"]*""#, |lex| Some(lex.slice()[2..lex.slice().len()-1].to_string()))]
    FStringLiteral(String),
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("None")]
    None,

    // Identifier (must not match standalone underscore)
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),

    #[token("_")]
    Underscore,

    // Whitespace (skipped)
    #[regex(r"[ \t\r\n]+", logos::skip)]
    #[regex(r"#.*", logos::skip)]
    Whitespace,

    // Error token for invalid characters
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Keywords
            TokenKind::Def => write!(f, "def"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Type => write!(f, "type"),
            TokenKind::Enum => write!(f, "enum"),
            TokenKind::Interface => write!(f, "interface"),
            TokenKind::Mod => write!(f, "mod"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::Export => write!(f, "export"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Elif => write!(f, "elif"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Do => write!(f, "do"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Match => write!(f, "match"),
            TokenKind::Implements => write!(f, "Implements"),
            TokenKind::Spawn => write!(f, "spawn"),
            TokenKind::Raise => write!(f, "raise"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Pass => write!(f, "pass"),
            TokenKind::As => write!(f, "as"),
            TokenKind::Extends => write!(f, "extends"),

            // Primitive Types
            TokenKind::Int => write!(f, "int"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::Bool => write!(f, "bool"),
            TokenKind::Str => write!(f, "str"),
            TokenKind::Any => write!(f, "any"),

            // Built-in Types
            TokenKind::Result => write!(f, "Result"),
            TokenKind::Option => write!(f, "Option"),

            // Symbols
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::LBrack => write!(f, "["),
            TokenKind::RBrack => write!(f, "]"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::Lte => write!(f, "<="),
            TokenKind::Gte => write!(f, ">="),
            TokenKind::Eq => write!(f, "=="),
            TokenKind::Neq => write!(f, "!="),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Underscore => write!(f, "_"),

            // Literals
            TokenKind::IntLiteral(val) => write!(f, "{}", val),
            TokenKind::FloatLiteral(val) => write!(f, "{}", val),
            TokenKind::StringLiteral(val) => write!(f, "\"{}\"", val),
            TokenKind::FStringLiteral(val) => write!(f, "f\"{}\"", val),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::None => write!(f, "None"),

            // Identifier
            TokenKind::Identifier(name) => write!(f, "{}", name),

            // Special
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::Error => write!(f, "error"),
        }
    }
}

/// A lexer for TJLang source code
pub struct Lexer<'source> {
    inner: logos::Lexer<'source, TokenKind>,
    file_id: FileId,
    source: &'source str,
    diagnostics: DiagnosticCollection,
}

impl<'source> Lexer<'source> {
    /// Create a new lexer for the given source code
    pub fn new(source: &'source str, file_id: FileId) -> Self {
        Self {
            inner: TokenKind::lexer(source),
            file_id,
            source,
            diagnostics: DiagnosticCollection::new(),
        }
    }

    /// Get the diagnostics collected during lexing
    pub fn diagnostics(&self) -> &DiagnosticCollection {
        &self.diagnostics
    }

    /// Take ownership of the diagnostics
    pub fn take_diagnostics(&mut self) -> DiagnosticCollection {
        std::mem::replace(&mut self.diagnostics, DiagnosticCollection::new())
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner.next()?;
        let span = self.inner.span();
        let text = &self.source[span.clone()];

        let source_span =
            SourceSpan::new(self.file_id, Span::new(span.start as u32, span.end as u32));

        match token {
            Ok(kind) => Some(Token::new(kind, source_span, text.to_string())),
            Err(_) => {
                // Create a diagnostic for lexer errors
                let diagnostic = TJLangDiagnostic::new(
                    ErrorCode::LexerInvalidCharacter,
                    Severity::Error,
                    format!(
                        "invalid character: '{}'",
                        text.chars().next().unwrap_or('?')
                    ),
                    source_span,
                );
                self.diagnostics.add(diagnostic);
                Some(Token::new(TokenKind::Error, source_span, text.to_string()))
            }
        }
    }
}

/// Lex a source string and return tokens with diagnostics
pub fn lex(source: &str, file_id: FileId) -> (Vec<Token>, DiagnosticCollection) {
    let mut lexer = Lexer::new(source, file_id);
    let mut tokens = Vec::new();

    for token in lexer.by_ref() {
        tokens.push(token);
    }

    let diagnostics = lexer.take_diagnostics();
    (tokens, diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codespan::Files;

    fn create_test_file_id() -> FileId {
        let mut files = Files::new();
        files.add("test.tj", "test content")
    }

    #[test]
    fn test_keywords() {
        let source = "def return type enum interface mod import export if elif else while do for match Implements spawn raise break continue pass as extends";
        let file_id = create_test_file_id();
        let (tokens, diagnostics) = lex(source, file_id);

        assert!(diagnostics.is_empty());
        assert_eq!(tokens.len(), 23);

        let expected = [
            TokenKind::Def,
            TokenKind::Return,
            TokenKind::Type,
            TokenKind::Enum,
            TokenKind::Interface,
            TokenKind::Mod,
            TokenKind::Import,
            TokenKind::Export,
            TokenKind::If,
            TokenKind::Elif,
            TokenKind::Else,
            TokenKind::While,
            TokenKind::Do,
            TokenKind::For,
            TokenKind::Match,
            TokenKind::Implements,
            TokenKind::Spawn,
            TokenKind::Raise,
            TokenKind::Break,
            TokenKind::Continue,
            TokenKind::Pass,
            TokenKind::As,
            TokenKind::Extends,
        ];

        for (i, token) in tokens.iter().enumerate() {
            if i < expected.len() {
                assert_eq!(token.kind, expected[i]);
            }
        }
    }

    #[test]
    fn test_primitive_types() {
        let source = "int float bool str any";
        let file_id = create_test_file_id();
        let (tokens, diagnostics) = lex(source, file_id);

        assert!(diagnostics.is_empty());
        assert_eq!(tokens.len(), 5);

        let expected = [
            TokenKind::Int,
            TokenKind::Float,
            TokenKind::Bool,
            TokenKind::Str,
            TokenKind::Any,
        ];
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected[i]);
        }
    }

    #[test]
    fn test_symbols() {
        let source = "-> = : , . ( ) { } [ ] + - * / % < > <= >= == != | ? or and ! _";
        let file_id = create_test_file_id();
        let (tokens, diagnostics) = lex(source, file_id);

        assert!(diagnostics.is_empty());
        assert_eq!(tokens.len(), 28);

        let expected = [
            TokenKind::Arrow,
            TokenKind::Assign,
            TokenKind::Colon,
            TokenKind::Comma,
            TokenKind::Dot,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::RBrace,
            TokenKind::LBrack,
            TokenKind::RBrack,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Percent,
            TokenKind::Lt,
            TokenKind::Gt,
            TokenKind::Lte,
            TokenKind::Gte,
            TokenKind::Eq,
            TokenKind::Neq,
            TokenKind::Pipe,
            TokenKind::Question,
            TokenKind::Or,
            TokenKind::And,
            TokenKind::Bang,
            TokenKind::Underscore,
        ];

        for (i, token) in tokens.iter().enumerate() {
            if i < expected.len() {
                assert_eq!(token.kind, expected[i]);
            }
        }
    }

    #[test]
    fn test_literals() {
        let source = "42 3.14 \"hello\" f\"world {name}\" true false None";
        let file_id = create_test_file_id();
        let (tokens, diagnostics) = lex(source, file_id);

        assert!(diagnostics.is_empty());
        assert_eq!(tokens.len(), 7);

        match &tokens[0].kind {
            TokenKind::IntLiteral(42) => {}
            _ => panic!("Expected IntLiteral(42)"),
        }

        match &tokens[1].kind {
            TokenKind::FloatLiteral(3.14) => {}
            _ => panic!("Expected FloatLiteral(3.14)"),
        }

        match &tokens[2].kind {
            TokenKind::StringLiteral(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected StringLiteral(\"hello\")"),
        }

        match &tokens[3].kind {
            TokenKind::FStringLiteral(s) => assert_eq!(s, "world {name}"),
            _ => panic!("Expected FStringLiteral(\"world {{name}}\")"),
        }

        assert_eq!(tokens[4].kind, TokenKind::True);
        assert_eq!(tokens[5].kind, TokenKind::False);
        assert_eq!(tokens[6].kind, TokenKind::None);
    }

    #[test]
    fn test_identifiers() {
        let source = "hello world _private var123";
        let file_id = create_test_file_id();
        let (tokens, diagnostics) = lex(source, file_id);

        assert!(diagnostics.is_empty());
        assert_eq!(tokens.len(), 5); // hello, world, _, private, var123

        // Check that we have the expected tokens
        assert_eq!(tokens[0].kind, TokenKind::Identifier("hello".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Identifier("world".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Underscore);
        assert_eq!(tokens[3].kind, TokenKind::Identifier("private".to_string()));
        assert_eq!(tokens[4].kind, TokenKind::Identifier("var123".to_string()));
    }

    #[test]
    fn test_comments_and_whitespace() {
        let source = "hello # this is a comment\nworld   \t  # another comment";
        let file_id = create_test_file_id();
        let (tokens, diagnostics) = lex(source, file_id);

        assert!(diagnostics.is_empty());
        // Should only have the identifiers, comments and whitespace are skipped
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Identifier("hello".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Identifier("world".to_string()));
    }

    #[test]
    fn test_error_handling() {
        let source = "hello @ invalid";
        let file_id = create_test_file_id();
        let (tokens, diagnostics) = lex(source, file_id);

        // Should have a diagnostic for the invalid character
        assert!(!diagnostics.is_empty());
        assert_eq!(tokens.len(), 3); // hello, error token, invalid

        assert_eq!(tokens[0].kind, TokenKind::Identifier("hello".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Error); // Error token for invalid character
        assert_eq!(tokens[2].kind, TokenKind::Identifier("invalid".to_string()));
    }

    #[test]
    fn test_demo_lexing() {
        let source = r#"
# TJLang Demo
def add(x: int, y: int) -> int {
    return x + y
}

x: int = 42
name: str = "hello"
result: int = add(x, 10)
"#;
        let file_id = create_test_file_id();
        let (tokens, diagnostics) = lex(source, file_id);

        // Should have no diagnostics for valid code
        assert!(diagnostics.is_empty());

        // Should have tokens for the function definition
        let token_kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();

        // Check for key tokens
        assert!(token_kinds.contains(&&TokenKind::Def));
        assert!(token_kinds.contains(&&TokenKind::Identifier("add".to_string())));
        assert!(token_kinds.contains(&&TokenKind::Int));
        assert!(token_kinds.contains(&&TokenKind::Arrow));
        assert!(token_kinds.contains(&&TokenKind::Return));
        assert!(token_kinds.contains(&&TokenKind::Plus));

        println!("Demo lexing successful! Found {} tokens", tokens.len());
    }
}
