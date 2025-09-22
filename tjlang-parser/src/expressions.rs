//! Expression parsing for TJLang

use super::parser::Parser;
use tjlang_ast::*;
use tjlang_lexer::TokenKind;

impl<'source> Parser<'source> {
    /// Parse assignment expression
    fn parse_assignment(&mut self) -> Option<Expression> {
        let expr = self.parse_or_expression()?;
        
        if self.check(&TokenKind::Assign) {
            let assign_token = self.advance()?;
            let right = self.parse_expression()?;
            
            Some(Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::Assign,
                right: Box::new(right),
                span: assign_token.span,
            })
        } else {
            Some(expr)
        }
    }

    /// Parse OR expression
    fn parse_or_expression(&mut self) -> Option<Expression> {
        let mut expr = self.parse_and_expression()?;
        
        while self.check(&TokenKind::Or) {
            let op_token = self.advance()?;
            let right = self.parse_and_expression()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
                span: op_token.span,
            };
        }
        
        Some(expr)
    }

    /// Parse AND expression
    fn parse_and_expression(&mut self) -> Option<Expression> {
        let mut expr = self.parse_equality()?;
        
        while self.check(&TokenKind::And) {
            let op_token = self.advance()?;
            let right = self.parse_equality()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
                span: op_token.span,
            };
        }
        
        Some(expr)
    }

    /// Parse equality expression
    fn parse_equality(&mut self) -> Option<Expression> {
        let mut expr = self.parse_relational()?;
        
        while self.check(&TokenKind::Eq) || self.check(&TokenKind::Neq) {
            let op_token = self.advance()?;
            let operator = match op_token.kind {
                TokenKind::Eq => BinaryOperator::Equal,
                TokenKind::Neq => BinaryOperator::NotEqual,
                _ => return None,
            };
            
            let right = self.parse_relational()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span: op_token.span,
            };
        }
        
        Some(expr)
    }

    /// Parse relational expression
    fn parse_relational(&mut self) -> Option<Expression> {
        let mut expr = self.parse_additive()?;
        
        while self.check(&TokenKind::Lt) || self.check(&TokenKind::Gt) || 
              self.check(&TokenKind::Lte) || self.check(&TokenKind::Gte) {
            let op_token = self.advance()?;
            let operator = match op_token.kind {
                TokenKind::Lt => BinaryOperator::LessThan,
                TokenKind::Gt => BinaryOperator::GreaterThan,
                TokenKind::Lte => BinaryOperator::LessThanEqual,
                TokenKind::Gte => BinaryOperator::GreaterThanEqual,
                _ => return None,
            };
            
            let right = self.parse_additive()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span: op_token.span,
            };
        }
        
        Some(expr)
    }

    /// Parse additive expression
    fn parse_additive(&mut self) -> Option<Expression> {
        let mut expr = self.parse_multiplicative()?;
        
        while self.check(&TokenKind::Plus) || self.check(&TokenKind::Minus) {
            let op_token = self.advance()?;
            let operator = match op_token.kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Subtract,
                _ => return None,
            };
            
            let right = self.parse_multiplicative()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span: op_token.span,
            };
        }
        
        Some(expr)
    }

    /// Parse multiplicative expression
    fn parse_multiplicative(&mut self) -> Option<Expression> {
        let mut expr = self.parse_unary()?;
        
        while self.check(&TokenKind::Star) || self.check(&TokenKind::Slash) || 
              self.check(&TokenKind::Percent) {
            let op_token = self.advance()?;
            let operator = match op_token.kind {
                TokenKind::Star => BinaryOperator::Multiply,
                TokenKind::Slash => BinaryOperator::Divide,
                TokenKind::Percent => BinaryOperator::Modulo,
                _ => return None,
            };
            
            let right = self.parse_unary()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span: op_token.span,
            };
        }
        
        Some(expr)
    }

    /// Parse unary expression
    fn parse_unary(&mut self) -> Option<Expression> {
        if self.check(&TokenKind::Minus) || self.check(&TokenKind::Bang) {
            let op_token = self.advance()?;
            let operator = match op_token.kind {
                TokenKind::Minus => UnaryOperator::Negate,
                TokenKind::Bang => UnaryOperator::Not,
                _ => return None,
            };
            
            let operand = self.parse_unary()?;
            
            Some(Expression::Unary {
                operator,
                operand: Box::new(operand),
                span: op_token.span,
            })
        } else {
            self.parse_postfix()
        }
    }

    /// Parse postfix expression
    fn parse_postfix(&mut self) -> Option<Expression> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.check(&TokenKind::LParen) {
                expr = self.parse_call(expr)?;
            } else if self.check(&TokenKind::LBrack) {
                expr = self.parse_index(expr)?;
            } else if self.check(&TokenKind::Dot) {
                expr = self.parse_member(expr)?;
            } else {
                break;
            }
        }
        
        Some(expr)
    }

    /// Parse primary expression
    fn parse_primary(&mut self) -> Option<Expression> {
        match self.peek()?.kind {
            TokenKind::IntLiteral(_) | TokenKind::FloatLiteral(_) | 
            TokenKind::StringLiteral(_) | TokenKind::FStringLiteral(_) |
            TokenKind::True | TokenKind::False | TokenKind::None => {
                Some(Expression::Literal(self.parse_literal()?))
            },
            TokenKind::Identifier(_) => {
                let name_token = self.advance()?;
                let name = match &name_token.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => return None,
                };
                Some(Expression::Variable(name))
            },
            TokenKind::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RParen)?;
                Some(expr)
            },
            TokenKind::LBrack => {
                self.parse_vec_literal()
            },
            TokenKind::LBrace => {
                self.parse_collection_literal()
            },
            TokenKind::Spawn => {
                let spawn_token = self.advance()?;
                let expr = self.parse_expression()?;
                Some(Expression::Spawn {
                    expression: Box::new(expr),
                    span: spawn_token.span,
                })
            },
            _ => None,
        }
    }

    /// Parse function call
    fn parse_call(&mut self, callee: Expression) -> Option<Expression> {
        self.consume(TokenKind::LParen)?;
        
        let mut args = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                args.push(self.parse_expression()?);
                
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
        }
        
        let rparen_token = self.consume(TokenKind::RParen)?;
        
        Some(Expression::Call {
            callee: Box::new(callee),
            args,
            span: rparen_token.span,
        })
    }

    /// Parse index expression
    fn parse_index(&mut self, target: Expression) -> Option<Expression> {
        self.consume(TokenKind::LBrack)?;
        let index = self.parse_expression()?;
        let rbrack_token = self.consume(TokenKind::RBrack)?;
        
        Some(Expression::Index {
            target: Box::new(target),
            index: Box::new(index),
            span: rbrack_token.span,
        })
    }

    /// Parse member access
    fn parse_member(&mut self, target: Expression) -> Option<Expression> {
        self.advance(); // consume '.'
        let member_token = self.consume(TokenKind::Identifier("".to_string()))?;
        let member = match &member_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        Some(Expression::Member {
            target: Box::new(target),
            member,
            span: member_token.span,
        })
    }

    /// Parse literal
    pub fn parse_literal(&mut self) -> Option<Literal> {
        match &self.peek()?.kind {
            TokenKind::IntLiteral(val) => {
                self.advance();
                Some(Literal::Int(*val))
            },
            TokenKind::FloatLiteral(val) => {
                self.advance();
                Some(Literal::Float(*val as f64))
            },
            TokenKind::StringLiteral(val) => {
                self.advance();
                Some(Literal::String(val.clone()))
            },
            TokenKind::FStringLiteral(val) => {
                self.advance();
                Some(Literal::FString(val.clone()))
            },
            TokenKind::True => {
                self.advance();
                Some(Literal::Bool(true))
            },
            TokenKind::False => {
                self.advance();
                Some(Literal::Bool(false))
            },
            TokenKind::None => {
                self.advance();
                Some(Literal::None)
            },
            _ => None,
        }
    }

    /// Parse vector literal
    fn parse_vec_literal(&mut self) -> Option<Expression> {
        let lbrack_token = self.advance()?;
        
        let mut elements = Vec::new();
        if !self.check(&TokenKind::RBrack) {
            loop {
                elements.push(self.parse_expression()?);
                
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
        }
        
        self.consume(TokenKind::RBrack)?;
        
        Some(Expression::VecLiteral {
            elements,
            span: lbrack_token.span,
        })
    }

    /// Parse collection literal (set or map)
    fn parse_collection_literal(&mut self) -> Option<Expression> {
        let lbrace_token = self.advance()?;
        
        if self.check(&TokenKind::RBrace) {
            // Empty collection - treat as set
            self.advance();
            return Some(Expression::SetLiteral {
                elements: Vec::new(),
                span: lbrace_token.span,
            });
        }
        
        // Check if it's a map (has colons) or set
        let mut is_map = false;
        let mut i = self.current;
        while i < self.tokens.len() && self.tokens[i].kind != TokenKind::RBrace {
            if self.tokens[i].kind == TokenKind::Colon {
                is_map = true;
                break;
            }
            i += 1;
        }
        
        if is_map {
            // Parse as map
            let mut entries = Vec::new();
            loop {
                let key = self.parse_expression()?;
                self.consume(TokenKind::Colon)?;
                let value = self.parse_expression()?;
                
                entries.push(MapEntry {
                    key,
                    value,
                    span: lbrace_token.span,
                });
                
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
            
            self.consume(TokenKind::RBrace)?;
            
            Some(Expression::MapLiteral {
                entries,
                span: lbrace_token.span,
            })
        } else {
            // Parse as set
            let mut elements = Vec::new();
            loop {
                elements.push(self.parse_expression()?);
                
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
            
            self.consume(TokenKind::RBrace)?;
            
            Some(Expression::SetLiteral {
                elements,
                span: lbrace_token.span,
            })
        }
    }
}
