//! Type parsing for TJLang

use super::parser::Parser;
use tjlang_ast::*;
use tjlang_lexer::TokenKind;

impl<'source> Parser<'source> {
    /// Parse a type
    fn parse_type(&mut self) -> Option<Type> {
        self.parse_union_type()
    }

    /// Parse union type (A | B | C)
    fn parse_union_type(&mut self) -> Option<Type> {
        let mut types = Vec::new();
        types.push(self.parse_option_type()?);
        
        while self.check(&TokenKind::Pipe) {
            self.advance(); // consume '|'
            types.push(self.parse_option_type()?);
        }
        
        if types.len() == 1 {
            Some(types.into_iter().next().unwrap())
        } else {
            Some(Type::Union {
                types,
                span: self.peek()?.span.clone(),
            })
        }
    }

    /// Parse option type (?T)
    fn parse_option_type(&mut self) -> Option<Type> {
        if self.check(&TokenKind::Question) {
            self.advance(); // consume '?'
            let inner = self.parse_function_type()?;
            Some(Type::Option {
                inner: Box::new(inner),
                span: self.peek()?.span.clone(),
            })
        } else {
            self.parse_function_type()
        }
    }

    /// Parse function type
    fn parse_function_type(&mut self) -> Option<Type> {
        if self.check(&TokenKind::LParen) {
            self.advance(); // consume '('
            
            let mut params = Vec::new();
            if !self.check(&TokenKind::RParen) {
                loop {
                    params.push(self.parse_type()?);
                    
                    if !self.check(&TokenKind::Comma) {
                        break;
                    }
                    self.advance(); // consume ','
                }
            }
            
            self.consume(TokenKind::RParen)?;
            self.consume(TokenKind::Arrow)?;
            
            let return_type = self.parse_function_type()?;
            
            Some(Type::Function {
                params,
                return_type: Box::new(return_type),
                span: self.peek()?.span.clone(),
            })
        } else {
            self.parse_collection_type()
        }
    }

    /// Parse collection type
    fn parse_collection_type(&mut self) -> Option<Type> {
        if self.check(&TokenKind::LBrack) {
            self.parse_vec_type()
        } else if self.check(&TokenKind::Lt) {
            self.parse_map_type()
        } else if self.check(&TokenKind::LParen) {
            self.parse_tuple_type()
        } else {
            self.parse_primary_type()
        }
    }

    /// Parse vector type [T]
    fn parse_vec_type(&mut self) -> Option<Type> {
        self.consume(TokenKind::LBrack)?;
        let element_type = self.parse_type()?;
        self.consume(TokenKind::RBrack)?;
        
        Some(Type::Vec {
            element_type: Box::new(element_type),
            span: self.peek()?.span.clone(),
        })
    }

    /// Parse map type Map<K, V>
    fn parse_map_type(&mut self) -> Option<Type> {
        let name_token = self.advance()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        if name == "Map" {
            self.consume(TokenKind::Lt)?;
            let key_type = self.parse_type()?;
            self.consume(TokenKind::Comma)?;
            let value_type = self.parse_type()?;
            self.consume(TokenKind::Gt)?;
            
            Some(Type::Map {
                key_type: Box::new(key_type),
                value_type: Box::new(value_type),
                span: name_token.span,
            })
        } else {
            // Not a map, treat as generic type
            self.parse_generic_type(name, name_token.span)
        }
    }

    /// Parse tuple type (T1, T2, T3)
    fn parse_tuple_type(&mut self) -> Option<Type> {
        self.consume(TokenKind::LParen)?;
        
        let mut types = Vec::new();
        types.push(self.parse_type()?);
        
        while self.check(&TokenKind::Comma) {
            self.advance(); // consume ','
            types.push(self.parse_type()?);
        }
        
        self.consume(TokenKind::RParen)?;
        
        if types.len() == 1 {
            Some(types.into_iter().next().unwrap())
        } else {
            Some(Type::Tuple {
                types,
                span: self.peek()?.span.clone(),
            })
        }
    }

    /// Parse primary type
    fn parse_primary_type(&mut self) -> Option<Type> {
        match self.peek()?.kind {
            TokenKind::Int => {
                self.advance();
                Some(Type::Primitive(PrimitiveType::Int))
            },
            TokenKind::Float => {
                self.advance();
                Some(Type::Primitive(PrimitiveType::Float))
            },
            TokenKind::Bool => {
                self.advance();
                Some(Type::Primitive(PrimitiveType::Bool))
            },
            TokenKind::Str => {
                self.advance();
                Some(Type::Primitive(PrimitiveType::Str))
            },
            TokenKind::Any => {
                self.advance();
                Some(Type::Primitive(PrimitiveType::Any))
            },
            TokenKind::Result => {
                self.parse_result_type()
            },
            TokenKind::Option => {
                self.parse_option_type()
            },
            TokenKind::Identifier(_) => {
                let name_token = self.advance()?;
                let name = match &name_token.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => return None,
                };
                
                if self.check(&TokenKind::Lt) {
                    self.parse_generic_type(name, name_token.span)
                } else {
                    Some(Type::Identifier(name))
                }
            },
            _ => None,
        }
    }

    /// Parse Result<T, E> type
    fn parse_result_type(&mut self) -> Option<Type> {
        let result_token = self.advance()?;
        self.consume(TokenKind::Lt)?;
        let ok_type = self.parse_type()?;
        self.consume(TokenKind::Comma)?;
        let error_type = self.parse_type()?;
        self.consume(TokenKind::Gt)?;
        
        Some(Type::Result {
            ok_type: Box::new(ok_type),
            error_type: Box::new(error_type),
            span: result_token.span,
        })
    }

    /// Parse generic type with type arguments
    fn parse_generic_type(&mut self, name: String, span: SourceSpan) -> Option<Type> {
        self.consume(TokenKind::Lt)?;
        
        let mut type_args = Vec::new();
        loop {
            type_args.push(self.parse_type()?);
            
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance(); // consume ','
        }
        
        self.consume(TokenKind::Gt)?;
        
        Some(Type::Generic {
            name,
            type_args,
            span,
        })
    }
}
