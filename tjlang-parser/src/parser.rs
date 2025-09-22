//! Recursive descent parser for TJLang

use tjlang_ast::*;
use tjlang_lexer::{Token, TokenKind};
use tjlang_diagnostics::{DiagnosticCollection, TJLangDiagnostic, ErrorCode, SourceSpan};
use codespan_reporting::diagnostic::Severity;
use codespan::FileId;

/// Parser for TJLang source code
pub struct Parser<'source> {
    pub tokens: Vec<Token>,
    pub current: usize,
    file_id: FileId,
    diagnostics: DiagnosticCollection,
}

impl<'source> Parser<'source> {
    /// Create a new parser from tokens
    pub fn new(tokens: Vec<Token>, file_id: FileId) -> Self {
        Self {
            tokens,
            current: 0,
            file_id,
            diagnostics: DiagnosticCollection::new(),
        }
    }

    /// Parse a complete program
    pub fn parse_program(mut self) -> Result<Program, DiagnosticCollection> {
        let start_span = self.tokens.first()
            .map(|t| t.span.clone())
            .unwrap_or_else(|| SourceSpan::new(self.file_id, codespan::Span::new(0, 0)));

        let mut units = Vec::new();
        
        while !self.is_at_end() {
            if let Some(unit) = self.parse_program_unit() {
                units.push(unit);
            } else {
                self.synchronize();
            }
        }

        let end_span = self.tokens.last()
            .map(|t| t.span.clone())
            .unwrap_or(start_span.clone());

        let program_span = SourceSpan::new(
            self.file_id,
            codespan::Span::new(start_span.span.start(), end_span.span.end())
        );

        Ok(Program {
            units,
            span: program_span,
        })
    }

    /// Parse a program unit
    fn parse_program_unit(&mut self) -> Option<ProgramUnit> {
        match self.peek()?.kind {
            TokenKind::Mod => Some(ProgramUnit::Module(self.parse_module_decl()?)),
            TokenKind::Import => Some(ProgramUnit::Import(self.parse_import_decl()?)),
            TokenKind::Export => Some(ProgramUnit::Export(self.parse_export_decl()?)),
            _ => Some(ProgramUnit::Declaration(self.parse_declaration()?)),
        }
    }

    /// Parse module declaration
    fn parse_module_decl(&mut self) -> Option<ModuleDecl> {
        let mod_token = self.advance()?;
        let name_token = self.consume(TokenKind::Identifier("".to_string()))?;
        
        Some(ModuleDecl {
            name: match &name_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return None,
            },
            span: mod_token.span,
        })
    }

    /// Parse import declaration
    fn parse_import_decl(&mut self) -> Option<ImportDecl> {
        let import_token = self.advance()?;
        let module = self.parse_qualified_name()?;
        
        if self.check(&TokenKind::As) {
            self.advance();
            let alias_token = self.consume(TokenKind::Identifier("".to_string()))?;
            let alias = match &alias_token.kind {
                TokenKind::Identifier(name) => Some(name.clone()),
                _ => return None,
            };
            
            Some(ImportDecl::Simple {
                module,
                alias,
                span: import_token.span,
            })
        } else if self.check(&TokenKind::Dot) {
            self.advance(); // consume '.'
            self.consume(TokenKind::LBrace)?;
            
            let mut items = Vec::new();
            loop {
                let item_token = self.consume(TokenKind::Identifier("".to_string()))?;
                let item = match &item_token.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => return None,
                };
                items.push(item);
                
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
            
            self.consume(TokenKind::RBrace)?;
            
            Some(ImportDecl::Selective {
                module,
                items,
                span: import_token.span,
            })
        } else {
            Some(ImportDecl::Simple {
                module,
                alias: None,
                span: import_token.span,
            })
        }
    }

    /// Parse qualified name
    fn parse_qualified_name(&mut self) -> Option<QualifiedName> {
        let start_token = self.advance()?;
        let mut parts = Vec::new();
        
        let first_part = match &start_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        parts.push(first_part);
        
        while self.check(&TokenKind::Dot) {
            self.advance(); // consume '.'
            let part_token = self.consume(TokenKind::Identifier("".to_string()))?;
            let part = match &part_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return None,
            };
            parts.push(part);
        }
        
        Some(QualifiedName {
            parts,
            span: start_token.span,
        })
    }

    /// Parse export declaration
    fn parse_export_decl(&mut self) -> Option<ExportDecl> {
        let export_token = self.advance()?;
        let item = self.parse_declaration()?;
        
        Some(ExportDecl {
            item,
            span: export_token.span,
        })
    }

    /// Parse any declaration
    fn parse_declaration(&mut self) -> Option<Declaration> {
        match self.peek()?.kind {
            TokenKind::Def => Some(Declaration::Function(self.parse_function_decl()?)),
            TokenKind::Interface => Some(Declaration::Interface(self.parse_interface_decl()?)),
            TokenKind::Type => {
                if self.check_ahead(1, &TokenKind::Identifier("".to_string())) {
                    Some(Declaration::Struct(self.parse_struct_decl()?))
                } else {
                    Some(Declaration::Type(self.parse_type_decl()?))
                }
            },
            TokenKind::Enum => Some(Declaration::Enum(self.parse_enum_decl()?)),
            _ => {
                // Try to parse as variable declaration
                if self.check_ahead(1, &TokenKind::Colon) {
                    Some(Declaration::Variable(self.parse_variable_decl()?))
                } else {
                    None
                }
            },
        }
    }

    /// Parse function declaration
    pub fn parse_function_decl(&mut self) -> Option<FunctionDecl> {
        declarations::Parser::parse_function_decl(self)
    }

    /// Parse interface declaration
    pub fn parse_interface_decl(&mut self) -> Option<InterfaceDecl> {
        declarations::Parser::parse_interface_decl(self)
    }

    /// Parse type declaration
    pub fn parse_type_decl(&mut self) -> Option<TypeDecl> {
        declarations::Parser::parse_type_decl(self)
    }

    /// Parse struct declaration
    pub fn parse_struct_decl(&mut self) -> Option<StructDecl> {
        declarations::Parser::parse_struct_decl(self)
    }

    /// Parse enum declaration
    pub fn parse_enum_decl(&mut self) -> Option<EnumDecl> {
        declarations::Parser::parse_enum_decl(self)
    }

    /// Parse variable declaration
    pub fn parse_variable_decl(&mut self) -> Option<VariableDecl> {
        declarations::Parser::parse_variable_decl(self)
    }

    /// Parse type
    pub fn parse_type(&mut self) -> Option<Type> {
        types::Parser::parse_type(self)
    }

    /// Parse expression
    pub fn parse_expression(&mut self) -> Option<Expression> {
        expressions::Parser::parse_expression(self)
    }

    /// Parse statement
    pub fn parse_statement(&mut self) -> Option<Statement> {
        statements::Parser::parse_statement(self)
    }

    /// Parse block
    pub fn parse_block(&mut self) -> Option<Block> {
        statements::Parser::parse_block(self)
    }

    /// Check if we're at the end of tokens
    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Peek at the current token
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Check if current token matches expected kind
    pub fn check(&self, expected: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().unwrap().kind) == std::mem::discriminant(expected)
        }
    }

    /// Check if token at offset matches expected kind
    pub fn check_ahead(&self, offset: usize, expected: &TokenKind) -> bool {
        if self.current + offset >= self.tokens.len() {
            false
        } else {
            let token = &self.tokens[self.current + offset];
            std::mem::discriminant(&token.kind) == std::mem::discriminant(expected)
        }
    }

    /// Advance to next token
    pub fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    /// Consume token of expected kind
    pub fn consume(&mut self, expected: TokenKind) -> Option<&Token> {
        if self.check(&expected) {
            self.advance()
        } else {
            self.error(&format!("Expected {:?}, got {:?}", expected, self.peek()?.kind));
            None
        }
    }

    /// Add error diagnostic
    fn error(&mut self, message: &str) {
        let token = self.peek().cloned().unwrap_or_else(|| {
            Token::new(TokenKind::Error, SourceSpan::new(self.file_id, codespan::Span::new(0, 0)), "".to_string())
        });
        
        let diagnostic = TJLangDiagnostic::new(
            ErrorCode::ParserUnexpectedToken,
            Severity::Error,
            message.to_string(),
            token.span,
        );
        self.diagnostics.add(diagnostic);
    }

    /// Synchronize parser after error
    pub fn synchronize(&mut self) {
        self.advance();
        
        while !self.is_at_end() {
            if self.previous()?.kind == TokenKind::Semicolon {
                return;
            }
            
            match self.peek()?.kind {
                TokenKind::Def | TokenKind::Type | TokenKind::Enum | 
                TokenKind::Interface | TokenKind::Mod | TokenKind::Import | 
                TokenKind::Export => return,
                _ => self.advance(),
            }
        }
    }

    /// Get previous token
    fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }
}
