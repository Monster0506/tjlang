//! Statement parsing for TJLang

use super::parser::Parser;
use tjlang_ast::*;
use tjlang_lexer::TokenKind;

impl<'source> Parser<'source> {
    /// Parse a block of statements
    fn parse_block(&mut self) -> Option<Block> {
        let lbrace_token = self.consume(TokenKind::LBrace)?;
        
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RBrace) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                self.synchronize();
            }
        }
        
        let rbrace_token = self.consume(TokenKind::RBrace)?;
        
        Some(Block {
            statements,
            span: lbrace_token.span,
        })
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek()?.kind {
            TokenKind::If => Some(Statement::If(self.parse_if_statement()?)),
            TokenKind::While => Some(Statement::While(self.parse_while_statement()?)),
            TokenKind::Do => Some(Statement::DoWhile(self.parse_do_while_statement()?)),
            TokenKind::For => Some(Statement::For(self.parse_for_statement()?)),
            TokenKind::Match => Some(Statement::Match(self.parse_match_statement()?)),
            TokenKind::Return => Some(Statement::Return(self.parse_return_statement()?)),
            TokenKind::Break => Some(Statement::Break(self.parse_break_statement()?)),
            TokenKind::Continue => Some(Statement::Continue(self.parse_continue_statement()?)),
            TokenKind::Pass => Some(Statement::Pass(self.parse_pass_statement()?)),
            TokenKind::Raise => Some(Statement::Raise(self.parse_raise_statement()?)),
            TokenKind::LBrace => Some(Statement::Block(self.parse_block()?)),
            _ => {
                // Try variable declaration first
                if self.check_ahead(1, &TokenKind::Colon) {
                    Some(Statement::Variable(self.parse_variable_decl()?))
                } else {
                    Some(Statement::Expression(self.parse_expression()?))
                }
            },
        }
    }

    /// Parse if statement
    fn parse_if_statement(&mut self) -> Option<IfStatement> {
        let if_token = self.advance()?;
        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;
        
        let mut elif_branches = Vec::new();
        while self.check(&TokenKind::Elif) {
            let elif_token = self.advance()?;
            let elif_condition = self.parse_expression()?;
            let elif_block = self.parse_block()?;
            
            elif_branches.push(ElifBranch {
                condition: elif_condition,
                block: elif_block,
                span: elif_token.span,
            });
        }
        
        let else_block = if self.check(&TokenKind::Else) {
            self.advance(); // consume 'else'
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Some(IfStatement {
            condition,
            then_block,
            elif_branches,
            else_block,
            span: if_token.span,
        })
    }

    /// Parse while statement
    fn parse_while_statement(&mut self) -> Option<WhileStatement> {
        let while_token = self.advance()?;
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        
        Some(WhileStatement {
            condition,
            body,
            span: while_token.span,
        })
    }

    /// Parse do-while statement
    fn parse_do_while_statement(&mut self) -> Option<DoWhileStatement> {
        let do_token = self.advance()?;
        let body = self.parse_block()?;
        self.consume(TokenKind::While)?;
        let condition = self.parse_expression()?;
        
        Some(DoWhileStatement {
            body,
            condition,
            span: do_token.span,
        })
    }

    /// Parse for statement
    fn parse_for_statement(&mut self) -> Option<ForStatement> {
        let for_token = self.advance()?;
        self.consume(TokenKind::LParen)?;
        
        let var_name_token = self.consume(TokenKind::Identifier("".to_string()))?;
        let var_name = match &var_name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        self.consume(TokenKind::Colon)?;
        let var_type = self.parse_type()?;
        self.consume(TokenKind::Pipe)?;
        let iterable = self.parse_expression()?;
        self.consume(TokenKind::RParen)?;
        
        let body = self.parse_block()?;
        
        Some(ForStatement {
            var_name,
            var_type,
            iterable,
            body,
            span: for_token.span,
        })
    }

    /// Parse match statement
    fn parse_match_statement(&mut self) -> Option<MatchStatement> {
        let match_token = self.advance()?;
        let expression = self.parse_expression()?;
        self.consume(TokenKind::LBrace)?;
        
        let mut arms = Vec::new();
        while !self.check(&TokenKind::RBrace) {
            arms.push(self.parse_match_arm()?);
        }
        
        self.consume(TokenKind::RBrace)?;
        
        Some(MatchStatement {
            expression,
            arms,
            span: match_token.span,
        })
    }

    /// Parse match arm
    fn parse_match_arm(&mut self) -> Option<MatchArm> {
        let pattern = self.parse_pattern()?;
        
        let guard = if self.check(&TokenKind::If) {
            self.advance(); // consume 'if'
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Colon)?;
        let body = self.parse_block()?;
        
        Some(MatchArm {
            pattern,
            guard,
            body,
            span: self.peek()?.span.clone(),
        })
    }

    /// Parse pattern
    fn parse_pattern(&mut self) -> Option<Pattern> {
        match self.peek()?.kind {
            TokenKind::IntLiteral(_) | TokenKind::FloatLiteral(_) | 
            TokenKind::StringLiteral(_) | TokenKind::FStringLiteral(_) |
            TokenKind::True | TokenKind::False | TokenKind::None => {
                Some(Pattern::Literal(self.parse_literal()?))
            },
            TokenKind::Identifier(_) => {
                let name_token = self.advance()?;
                let name = match &name_token.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => return None,
                };
                
                if self.check(&TokenKind::Colon) {
                    self.advance(); // consume ':'
                    
                    if self.check(&TokenKind::Implements) {
                        self.advance(); // consume 'Implements'
                        self.consume(TokenKind::LBrack)?;
                        let trait_token = self.consume(TokenKind::Identifier("".to_string()))?;
                        let trait_name = match &trait_token.kind {
                            TokenKind::Identifier(name) => name.clone(),
                            _ => return None,
                        };
                        self.consume(TokenKind::RBrack)?;
                        
                        Some(Pattern::TraitCheck {
                            name,
                            trait_name,
                            span: name_token.span,
                        })
                    } else {
                        let pattern_type = self.parse_type()?;
                        Some(Pattern::Variable {
                            name,
                            pattern_type,
                            span: name_token.span,
                        })
                    }
                } else if self.check(&TokenKind::LParen) {
                    // Constructor pattern
                    self.advance(); // consume '('
                    let mut fields = Vec::new();
                    
                    if !self.check(&TokenKind::RParen) {
                        loop {
                            fields.push(self.parse_pattern()?);
                            
                            if !self.check(&TokenKind::Comma) {
                                break;
                            }
                            self.advance(); // consume ','
                        }
                    }
                    
                    self.consume(TokenKind::RParen)?;
                    
                    Some(Pattern::Constructor {
                        name,
                        fields,
                        span: name_token.span,
                    })
                } else {
                    // Simple variable pattern
                    Some(Pattern::Variable {
                        name,
                        pattern_type: Type::Identifier("_".to_string()),
                        span: name_token.span,
                    })
                }
            },
            TokenKind::Underscore => {
                self.advance();
                Some(Pattern::Wildcard(self.peek()?.span.clone()))
            },
            TokenKind::LParen => {
                self.advance(); // consume '('
                let mut patterns = Vec::new();
                
                loop {
                    patterns.push(self.parse_pattern()?);
                    
                    if !self.check(&TokenKind::Comma) {
                        break;
                    }
                    self.advance(); // consume ','
                }
                
                self.consume(TokenKind::RParen)?;
                
                Some(Pattern::Tuple {
                    patterns,
                    span: self.peek()?.span.clone(),
                })
            },
            _ => None,
        }
    }

    /// Parse return statement
    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let return_token = self.advance()?;
        let value = if !self.check(&TokenKind::Semicolon) && !self.check(&TokenKind::RBrace) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Some(ReturnStatement {
            value,
            span: return_token.span,
        })
    }

    /// Parse break statement
    fn parse_break_statement(&mut self) -> Option<BreakStatement> {
        let break_token = self.advance()?;
        Some(BreakStatement {
            span: break_token.span,
        })
    }

    /// Parse continue statement
    fn parse_continue_statement(&mut self) -> Option<ContinueStatement> {
        let continue_token = self.advance()?;
        Some(ContinueStatement {
            span: continue_token.span,
        })
    }

    /// Parse pass statement
    fn parse_pass_statement(&mut self) -> Option<PassStatement> {
        let pass_token = self.advance()?;
        Some(PassStatement {
            span: pass_token.span,
        })
    }

    /// Parse raise statement
    fn parse_raise_statement(&mut self) -> Option<RaiseStatement> {
        let raise_token = self.advance()?;
        let value = self.parse_expression()?;
        
        Some(RaiseStatement {
            value,
            span: raise_token.span,
        })
    }
}
