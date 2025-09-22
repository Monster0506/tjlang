//! Declaration parsing for TJLang

use super::parser::Parser;
use tjlang_ast::*;
use tjlang_lexer::TokenKind;

impl<'source> Parser<'source> {
    /// Parse function declaration
    fn parse_function_decl(&mut self) -> Option<FunctionDecl> {
        let def_token = self.advance()?;
        let name_token = self.consume(TokenKind::Identifier("".to_string()))?;
        
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        let generic_params = if self.check(&TokenKind::Lt) {
            self.parse_generic_params()?
        } else {
            Vec::new()
        };
        
        self.consume(TokenKind::LParen)?;
        let params = self.parse_param_list()?;
        self.consume(TokenKind::RParen)?;
        
        self.consume(TokenKind::Arrow)?;
        let return_type = self.parse_type()?;
        
        let body = self.parse_block()?;
        
        Some(FunctionDecl {
            name,
            generic_params,
            params,
            return_type,
            body,
            span: def_token.span,
        })
    }

    /// Parse generic parameters
    fn parse_generic_params(&mut self) -> Option<Vec<GenericParam>> {
        self.consume(TokenKind::Lt)?;
        let mut params = Vec::new();
        
        loop {
            let name_token = self.consume(TokenKind::Identifier("".to_string()))?;
            let name = match &name_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return None,
            };
            
            let bounds = if self.check(&TokenKind::Colon) {
                self.advance(); // consume ':'
                self.consume(TokenKind::Implements)?;
                self.consume(TokenKind::LBrack)?;
                
                let mut bounds = Vec::new();
                loop {
                    let bound_token = self.consume(TokenKind::Identifier("".to_string()))?;
                    let bound = match &bound_token.kind {
                        TokenKind::Identifier(name) => name.clone(),
                        _ => return None,
                    };
                    bounds.push(bound);
                    
                    if !self.check(&TokenKind::Comma) {
                        break;
                    }
                    self.advance(); // consume ','
                }
                
                self.consume(TokenKind::RBrack)?;
                bounds
            } else {
                Vec::new()
            };
            
            params.push(GenericParam {
                name,
                bounds,
                span: name_token.span,
            });
            
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance(); // consume ','
        }
        
        self.consume(TokenKind::Gt)?;
        Some(params)
    }

    /// Parse parameter list
    fn parse_param_list(&mut self) -> Option<Vec<Parameter>> {
        let mut params = Vec::new();
        
        if self.check(&TokenKind::RParen) {
            return Some(params);
        }
        
        loop {
            let name_token = self.consume(TokenKind::Identifier("".to_string()))?;
            let name = match &name_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return None,
            };
            
            self.consume(TokenKind::Colon)?;
            let param_type = self.parse_type()?;
            
            params.push(Parameter {
                name,
                param_type,
                span: name_token.span,
            });
            
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance(); // consume ','
        }
        
        Some(params)
    }

    /// Parse interface declaration
    fn parse_interface_decl(&mut self) -> Option<InterfaceDecl> {
        let interface_token = self.advance()?;
        let name_token = self.consume(TokenKind::Identifier("".to_string()))?;
        
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        let extends = if self.check(&TokenKind::Extends) {
            self.advance(); // consume 'extends'
            self.parse_identifier_list()?
        } else {
            Vec::new()
        };
        
        self.consume(TokenKind::LBrace)?;
        
        let mut methods = Vec::new();
        while !self.check(&TokenKind::RBrace) {
            methods.push(self.parse_method_signature()?);
        }
        
        self.consume(TokenKind::RBrace)?;
        
        Some(InterfaceDecl {
            name,
            extends,
            methods,
            span: interface_token.span,
        })
    }

    /// Parse method signature
    fn parse_method_signature(&mut self) -> Option<MethodSignature> {
        let name_token = self.advance()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        self.consume(TokenKind::LParen)?;
        let params = self.parse_param_list()?;
        self.consume(TokenKind::RParen)?;
        
        self.consume(TokenKind::Arrow)?;
        let return_type = self.parse_type()?;
        
        Some(MethodSignature {
            name,
            params,
            return_type,
            span: name_token.span,
        })
    }

    /// Parse type declaration
    fn parse_type_decl(&mut self) -> Option<TypeDecl> {
        let type_token = self.advance()?;
        let name_token = self.consume(TokenKind::Identifier("".to_string()))?;
        
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        self.consume(TokenKind::Assign)?;
        let type_def = self.parse_type()?;
        
        Some(TypeDecl {
            name,
            type_def,
            span: type_token.span,
        })
    }

    /// Parse struct declaration
    fn parse_struct_decl(&mut self) -> Option<StructDecl> {
        let type_token = self.advance()?;
        let name_token = self.consume(TokenKind::Identifier("".to_string()))?;
        
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        self.consume(TokenKind::LBrace)?;
        
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RBrace) {
            let field_name_token = self.consume(TokenKind::Identifier("".to_string()))?;
            let field_name = match &field_name_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return None,
            };
            
            self.consume(TokenKind::Colon)?;
            let field_type = self.parse_type()?;
            
            fields.push(FieldDecl {
                name: field_name,
                field_type,
                span: field_name_token.span,
            });
            
            if self.check(&TokenKind::Comma) {
                self.advance(); // consume ','
            }
        }
        
        self.consume(TokenKind::RBrace)?;
        
        Some(StructDecl {
            name,
            fields,
            span: type_token.span,
        })
    }

    /// Parse enum declaration
    fn parse_enum_decl(&mut self) -> Option<EnumDecl> {
        let enum_token = self.advance()?;
        let name_token = self.consume(TokenKind::Identifier("".to_string()))?;
        
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        let type_params = if self.check(&TokenKind::Lt) {
            self.advance(); // consume '<'
            let mut params = Vec::new();
            
            loop {
                let param_token = self.consume(TokenKind::Identifier("".to_string()))?;
                let param = match &param_token.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => return None,
                };
                params.push(param);
                
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
            
            self.consume(TokenKind::Gt)?;
            params
        } else {
            Vec::new()
        };
        
        self.consume(TokenKind::LBrace)?;
        
        let mut variants = Vec::new();
        while !self.check(&TokenKind::RBrace) {
            let variant_name_token = self.consume(TokenKind::Identifier("".to_string()))?;
            let variant_name = match &variant_name_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return None,
            };
            
            let fields = if self.check(&TokenKind::LParen) {
                self.advance(); // consume '('
                let mut fields = Vec::new();
                
                if !self.check(&TokenKind::RParen) {
                    loop {
                        fields.push(self.parse_type()?);
                        
                        if !self.check(&TokenKind::Comma) {
                            break;
                        }
                        self.advance(); // consume ','
                    }
                }
                
                self.consume(TokenKind::RParen)?;
                fields
            } else {
                Vec::new()
            };
            
            variants.push(EnumVariant {
                name: variant_name,
                fields,
                span: variant_name_token.span,
            });
            
            if self.check(&TokenKind::Comma) {
                self.advance(); // consume ','
            }
        }
        
        self.consume(TokenKind::RBrace)?;
        
        Some(EnumDecl {
            name,
            type_params,
            variants,
            span: enum_token.span,
        })
    }

    /// Parse variable declaration
    fn parse_variable_decl(&mut self) -> Option<VariableDecl> {
        let name_token = self.advance()?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        
        self.consume(TokenKind::Colon)?;
        let var_type = self.parse_type()?;
        self.consume(TokenKind::Assign)?;
        let value = self.parse_expression()?;
        
        Some(VariableDecl {
            name,
            var_type,
            value,
            span: name_token.span,
        })
    }

    /// Parse identifier list
    fn parse_identifier_list(&mut self) -> Option<Vec<String>> {
        let mut identifiers = Vec::new();
        
        loop {
            let id_token = self.consume(TokenKind::Identifier("".to_string()))?;
            let id = match &id_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return None,
            };
            identifiers.push(id);
            
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance(); // consume ','
        }
        
        Some(identifiers)
    }
}
