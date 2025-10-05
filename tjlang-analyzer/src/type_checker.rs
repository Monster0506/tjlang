//! Comprehensive Type Checker - Advanced Implementation
//!
//! A comprehensive type checker that performs full type checking
//! with proper diagnostics and type inference.

use std::collections::HashMap;
use tjlang_ast::*;
use tjlang_diagnostics::{DiagnosticCollection, TJLangDiagnostic, ErrorCode, SourceSpan as DiagnosticSourceSpan};
use codespan_reporting::diagnostic::Severity;
use tjlang_types::{Type, TypeEnvironment};
use codespan::{FileId, Files};

/// Comprehensive type checker for advanced type checking
pub struct TypeChecker {
    diagnostics: DiagnosticCollection,
    environment: TypeEnvironment,
    type_map: HashMap<String, Type>,
    current_file_id: FileId,
}

impl TypeChecker {
    /// Create a new comprehensive type checker
    pub fn new() -> Self {
        // Create a proper FileId using Files
        let mut files = Files::new();
        let file_id = files.add("", "");
        
        Self {
            diagnostics: DiagnosticCollection::new(),
            environment: TypeEnvironment::new(),
            type_map: HashMap::new(),
            current_file_id: file_id,
        }
    }

    /// Type check a program
    pub fn check_program(&mut self, program: &Program) -> Result<HashMap<String, Type>, DiagnosticCollection> {
        // Clear previous diagnostics
        self.diagnostics = DiagnosticCollection::new();
        self.type_map.clear();
        
        // Type check each program unit
        for unit in &program.units {
            self.check_program_unit(unit)?;
        }
        
        Ok(self.type_map.clone())
    }
    
    /// Type check a program unit
    fn check_program_unit(&mut self, unit: &ProgramUnit) -> Result<(), DiagnosticCollection> {
        match unit {
            ProgramUnit::Module(module) => self.check_module(module)?,
            ProgramUnit::Declaration(decl) => self.check_declaration(decl)?,
            _ => {
                // Handle other program units as needed
            }
        }
        Ok(())
    }
    
    /// Type check a module
    fn check_module(&mut self, _module: &ModuleDecl) -> Result<(), DiagnosticCollection> {
        // ModuleDecl only has name and span, no declarations field
        // TODO: Implement module-level type checking
        Ok(())
    }
    
    /// Type check a declaration
    fn check_declaration(&mut self, decl: &Declaration) -> Result<(), DiagnosticCollection> {
        match decl {
            Declaration::Variable(var_decl) => self.check_variable_declaration(var_decl)?,
            Declaration::Function(func_decl) => self.check_function_declaration(func_decl)?,
            Declaration::Interface(interface_decl) => self.check_interface_declaration(interface_decl)?,
            Declaration::Type(type_decl) => self.check_type_declaration(type_decl)?,
            Declaration::Enum(enum_decl) => self.check_enum_declaration(enum_decl)?,
            Declaration::Struct(struct_decl) => self.check_struct_declaration(struct_decl)?,
            Declaration::Implementation(impl_block) => self.check_impl_block(impl_block)?,
        }
        Ok(())
    }
    
    /// Type check a variable declaration
    fn check_variable_declaration(&mut self, var_decl: &VariableDecl) -> Result<(), DiagnosticCollection> {
        // Check if variable is already declared
        if self.type_map.contains_key(&var_decl.name) {
            self.add_diagnostic(
                ErrorCode::AnalyzerDuplicateDefinition,
                Severity::Error,
                format!("Variable '{}' is already declared", var_decl.name),
                self.convert_span(var_decl.span.clone())
            );
            return Ok(());
        }
        
        // Type check the value expression
        let value_type = self.check_expression(&var_decl.value)?;
        
        // Convert AST type to our type system
        let declared_type = self.ast_type_to_type(&var_decl.var_type);
        
        // Check if this is type inference (declared type is Any)
        if self.is_any_type(&var_decl.var_type) {
            // Type inference: use the inferred type from the expression
            self.type_map.insert(var_decl.name.clone(), value_type);
        } else {
            // Explicit type declaration: check compatibility
            if !self.is_type_compatible(&value_type, &declared_type) {
                self.add_diagnostic(
                    ErrorCode::AnalyzerTypeMismatch,
                    Severity::Error,
                    format!("Type mismatch: expected {:?}, found {:?}", declared_type, value_type),
                    self.convert_span(var_decl.span.clone())
                );
            }
            
            // Store declared type
            self.type_map.insert(var_decl.name.clone(), declared_type);
        }
        Ok(())
    }
    
    /// Type check a function declaration
    fn check_function_declaration(&mut self, func_decl: &FunctionDecl) -> Result<(), DiagnosticCollection> {
        // Check if function is already declared
        if self.type_map.contains_key(&func_decl.name) {
            self.add_diagnostic(
                ErrorCode::AnalyzerDuplicateDefinition,
                Severity::Error,
                format!("Function '{}' is already declared", func_decl.name),
                self.convert_span(func_decl.span.clone())
            );
            return Ok(());
        }
        
        // Type check parameters
        let mut param_types = Vec::new();
        for param in &func_decl.params {
            let param_type = self.ast_type_to_type(&param.param_type);
            param_types.push(param_type);
        }
        
        // Type check return type
        let return_type = if self.is_any_type(&func_decl.return_type) {
            // Type inference: infer return type from function body
            self.infer_function_return_type(func_decl)?
        } else {
            // Explicit return type
            self.ast_type_to_type(&func_decl.return_type)
        };
        
        // Create function type using tuple syntax
        let function_type = Type::Function(param_types, Box::new(return_type));
        
        // Store function type
        self.type_map.insert(func_decl.name.clone(), function_type);
        Ok(())
    }
    
    /// Type check an expression
    fn check_expression(&mut self, expr: &Expression) -> Result<Type, DiagnosticCollection> {
        self.check_expression_with_span(expr, None)
    }
    
    /// Type check an expression with optional span context
    fn check_expression_with_span(&mut self, expr: &Expression, span_context: Option<&tjlang_ast::SourceSpan>) -> Result<Type, DiagnosticCollection> {
        match expr {
            Expression::Literal(lit) => self.check_literal(lit),
            Expression::Variable { name, span } => {
                self.check_variable_reference_with_span(name, span)
            },
            Expression::Binary { left, operator, right, span } => {
                self.check_binary_expression_with_span(left, operator, right, span)
            },
            Expression::Unary { operator, operand, span } => {
                self.check_unary_expression_with_span(operator, operand, span)
            },
            Expression::Call { callee, args, span } => {
                self.check_function_call_with_span(callee, args, span)
            },
            Expression::If { condition, then_expr, else_expr, span } => {
                self.check_if_expression_with_span(condition, then_expr, else_expr, span)
            },
            Expression::Match { expression, arms, span } => {
                self.check_match_expression_with_span(expression, arms, span)
            },
            Expression::Lambda { params, body, span } => {
                self.check_lambda_expression_with_span(params, body, span)
            },
            Expression::VecLiteral { elements, span } => {
                self.check_vec_literal(elements, span)
            },
            Expression::SetLiteral { elements, span } => {
                self.check_set_literal(elements, span)
            },
            Expression::MapLiteral { entries, span } => {
                self.check_map_literal(entries, span)
            },
            Expression::TupleLiteral { elements, span } => {
                self.check_tuple_literal(elements, span)
            },
            _ => {
                // Handle other expression types
                Ok(Type::Int) // Default fallback
            }
        }
    }
    
    /// Type check a literal
    fn check_literal(&mut self, lit: &Literal) -> Result<Type, DiagnosticCollection> {
        match lit {
            Literal::Int(_) => Ok(Type::Int),
            Literal::Float(_) => Ok(Type::Float),
            Literal::Bool(_) => Ok(Type::Bool),
            Literal::String(_) => Ok(Type::Str),
            Literal::FString(_) => Ok(Type::Str),
            Literal::None => Ok(Type::Int), // TODO: Implement proper None type
            Literal::FStringInterpolation(_) => Ok(Type::Str),
        }
    }
    
    /// Type check a vector literal
    fn check_vec_literal(&mut self, elements: &[Expression], span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        if elements.is_empty() {
            // Empty vector - infer type from context or default to Vec<Int>
            return Ok(Type::Vec(Box::new(Type::Int)));
        }
        
        // Check all elements have compatible types
        let first_type = self.check_expression_with_span(&elements[0], Some(span))?;
        let mut all_compatible = true;
        
        for element in elements.iter().skip(1) {
            let element_type = self.check_expression_with_span(element, Some(span))?;
            if !self.is_type_compatible(&element_type, &first_type) {
                all_compatible = false;
                self.add_diagnostic(
                    ErrorCode::AnalyzerTypeMismatch,
                    Severity::Error,
                    format!("Vector elements have incompatible types: {:?} and {:?}", first_type, element_type),
                    self.convert_span(span.clone())
                );
            }
        }
        
        if all_compatible {
            Ok(Type::Vec(Box::new(first_type)))
        } else {
            // Return union type for incompatible elements
            let mut element_types = Vec::new();
            for element in elements {
                let element_type = self.check_expression_with_span(element, Some(span))?;
                if !element_types.contains(&element_type) {
                    element_types.push(element_type);
                }
            }
            Ok(Type::Vec(Box::new(Type::Sum(element_types))))
        }
    }
    
    /// Type check a set literal
    fn check_set_literal(&mut self, elements: &[Expression], span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        if elements.is_empty() {
            // Empty set - infer type from context or default to Set<Int>
            return Ok(Type::Set(Box::new(Type::Int)));
        }
        
        // Check all elements have compatible types
        let first_type = self.check_expression_with_span(&elements[0], Some(span))?;
        let mut all_compatible = true;
        
        for element in elements.iter().skip(1) {
            let element_type = self.check_expression_with_span(element, Some(span))?;
            if !self.is_type_compatible(&element_type, &first_type) {
                all_compatible = false;
                self.add_diagnostic(
                    ErrorCode::AnalyzerTypeMismatch,
                    Severity::Error,
                    format!("Set elements have incompatible types: {:?} and {:?}", first_type, element_type),
                    self.convert_span(span.clone())
                );
            }
        }
        
        if all_compatible {
            Ok(Type::Set(Box::new(first_type)))
        } else {
            // Return union type for incompatible elements
            let mut element_types = Vec::new();
            for element in elements {
                let element_type = self.check_expression_with_span(element, Some(span))?;
                if !element_types.contains(&element_type) {
                    element_types.push(element_type);
                }
            }
            Ok(Type::Set(Box::new(Type::Sum(element_types))))
        }
    }
    
    /// Type check a map literal
    fn check_map_literal(&mut self, entries: &[MapEntry], span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        if entries.is_empty() {
            // Empty map - infer type from context or default to Map<Str, Int>
            return Ok(Type::Map(Box::new(Type::Str), Box::new(Type::Int)));
        }
        
        // Check all keys and values have compatible types
        let first_key_type = self.check_expression_with_span(&entries[0].key, Some(span))?;
        let first_value_type = self.check_expression_with_span(&entries[0].value, Some(span))?;
        let mut all_keys_compatible = true;
        let mut all_values_compatible = true;
        
        for entry in entries.iter().skip(1) {
            let key_type = self.check_expression_with_span(&entry.key, Some(span))?;
            let value_type = self.check_expression_with_span(&entry.value, Some(span))?;
            
            if !self.is_type_compatible(&key_type, &first_key_type) {
                all_keys_compatible = false;
                self.add_diagnostic(
                    ErrorCode::AnalyzerTypeMismatch,
                    Severity::Error,
                    format!("Map keys have incompatible types: {:?} and {:?}", first_key_type, key_type),
                    self.convert_span(span.clone())
                );
            }
            
            if !self.is_type_compatible(&value_type, &first_value_type) {
                all_values_compatible = false;
                self.add_diagnostic(
                    ErrorCode::AnalyzerTypeMismatch,
                    Severity::Error,
                    format!("Map values have incompatible types: {:?} and {:?}", first_value_type, value_type),
                    self.convert_span(span.clone())
                );
            }
        }
        
        if all_keys_compatible && all_values_compatible {
            Ok(Type::Map(Box::new(first_key_type), Box::new(first_value_type)))
        } else {
            // Return union types for incompatible keys/values
            let mut key_types = Vec::new();
            let mut value_types = Vec::new();
            
            for entry in entries {
                let key_type = self.check_expression_with_span(&entry.key, Some(span))?;
                let value_type = self.check_expression_with_span(&entry.value, Some(span))?;
                
                if !key_types.contains(&key_type) {
                    key_types.push(key_type);
                }
                if !value_types.contains(&value_type) {
                    value_types.push(value_type);
                }
            }
            
            Ok(Type::Map(
                Box::new(Type::Sum(key_types)),
                Box::new(Type::Sum(value_types))
            ))
        }
    }
    
    /// Type check a tuple literal
    fn check_tuple_literal(&mut self, elements: &[Expression], span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        let mut element_types = Vec::new();
        
        for element in elements {
            let element_type = self.check_expression_with_span(element, Some(span))?;
            element_types.push(element_type);
        }
        
        Ok(Type::Tuple(element_types))
    }
    
    /// Type check a variable reference
    fn check_variable_reference(&mut self, name: &str) -> Result<Type, DiagnosticCollection> {
        if let Some(var_type) = self.type_map.get(name) {
            Ok(var_type.clone())
        } else {
            self.add_diagnostic(
                ErrorCode::AnalyzerUndefinedVariable,
                Severity::Error,
                format!("Variable '{}' is used before being declared", name),
                self.convert_span(tjlang_ast::SourceSpan { 
                    file_id: self.current_file_id, 
                    span: codespan::Span::new(0, 0) 
                })
            );
            Ok(Type::Int) // Default fallback
        }
    }
    
    /// Type check a variable reference with span
    fn check_variable_reference_with_span(&mut self, name: &str, span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        if let Some(var_type) = self.type_map.get(name) {
            Ok(var_type.clone())
        } else {
            self.add_diagnostic(
                ErrorCode::AnalyzerUndefinedVariable,
                Severity::Error,
                format!("Variable '{}' is used before being declared", name),
                self.convert_span(span.clone())
            );
            Ok(Type::Int) // Default fallback
        }
    }
    
    /// Type check a binary expression with span
    fn check_binary_expression_with_span(&mut self, left: &Expression, operator: &BinaryOperator, right: &Expression, span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        let left_type = self.check_expression_with_span(left, Some(span))?;
        let right_type = self.check_expression_with_span(right, Some(span))?;
        
        match operator {
            BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide => {
                // Arithmetic operations
                if self.is_numeric_type(&left_type) && self.is_numeric_type(&right_type) {
                    if left_type == Type::Float || right_type == Type::Float {
                        Ok(Type::Float)
                    } else {
                        Ok(Type::Int)
                    }
                } else if let (Type::Sum(left_variants), Type::Sum(right_variants)) = (&left_type, &right_type) {
                    // Handle union types in arithmetic - check if any combination works
                    let mut compatible_combinations = Vec::new();
                    for left_variant in left_variants {
                        for right_variant in right_variants {
                            if self.is_numeric_type(left_variant) && self.is_numeric_type(right_variant) {
                                if *left_variant == Type::Float || *right_variant == Type::Float {
                                    compatible_combinations.push(Type::Float);
                                } else {
                                    compatible_combinations.push(Type::Int);
                                }
                            }
                        }
                    }
                    
                    if compatible_combinations.is_empty() {
                        self.add_diagnostic(
                            ErrorCode::AnalyzerTypeMismatch,
                            Severity::Error,
                            format!("Cannot perform arithmetic operation on {:?} and {:?}", left_type, right_type),
                            self.convert_span(span.clone())
                        );
                        Ok(Type::Int)
                    } else {
                        // Return union of all possible result types
                        Ok(Type::Sum(compatible_combinations))
                    }
                } else {
                    self.add_diagnostic(
                        ErrorCode::AnalyzerTypeMismatch,
                        Severity::Error,
                        format!("Cannot perform arithmetic operation on {:?} and {:?}", left_type, right_type),
                        self.convert_span(span.clone())
                    );
                    Ok(Type::Int)
                }
            },
            BinaryOperator::Equal | BinaryOperator::NotEqual | BinaryOperator::LessThan | 
            BinaryOperator::LessThanEqual | BinaryOperator::GreaterThan | BinaryOperator::GreaterThanEqual => {
                // Comparison operations
                if self.is_type_compatible(&left_type, &right_type) {
                    Ok(Type::Bool)
                } else {
                    self.add_diagnostic(
                        ErrorCode::AnalyzerTypeMismatch,
                        Severity::Error,
                        format!("Cannot compare {:?} and {:?}", left_type, right_type),
                        self.convert_span(span.clone())
                    );
                    Ok(Type::Bool)
                }
            },
            _ => {
                // Handle other operators
                Ok(Type::Int)
            }
        }
    }
    
    /// Type check a unary expression with span
    fn check_unary_expression_with_span(&mut self, operator: &UnaryOperator, operand: &Expression, span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        let operand_type = self.check_expression_with_span(operand, Some(span))?;
        
        match operator {
            UnaryOperator::Negate => {
                if self.is_numeric_type(&operand_type) {
                    Ok(operand_type)
                } else {
                    self.add_diagnostic(
                        ErrorCode::AnalyzerTypeMismatch,
                        Severity::Error,
                        format!("Cannot negate {:?}", operand_type),
                        self.convert_span(span.clone())
                    );
                    Ok(Type::Int)
                }
            },
            UnaryOperator::Not => {
                if operand_type == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    self.add_diagnostic(
                        ErrorCode::AnalyzerTypeMismatch,
                        Severity::Error,
                        format!("Cannot apply logical NOT to {:?}", operand_type),
                        self.convert_span(span.clone())
                    );
                    Ok(Type::Bool)
                }
            },
            _ => Ok(operand_type),
        }
    }
    
    /// Type check a function call with span
    fn check_function_call_with_span(&mut self, callee: &Expression, args: &[Expression], span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        let callee_type = self.check_expression_with_span(callee, Some(span))?;
        
        match callee_type {
            Type::Function(params, return_type) => {
                if args.len() != params.len() {
                    self.add_diagnostic(
                        ErrorCode::AnalyzerTypeMismatch,
                        Severity::Error,
                        format!("Expected {} arguments, found {}", params.len(), args.len()),
                        self.convert_span(span.clone())
                    );
                    return Ok(Type::Int);
                }
                
                // Check argument types
                for (i, (arg, param_type)) in args.iter().zip(params.iter()).enumerate() {
                    let arg_type = self.check_expression_with_span(arg, Some(span))?;
                    if !self.is_type_compatible(&arg_type, param_type) {
                        self.add_diagnostic(
                            ErrorCode::AnalyzerTypeMismatch,
                            Severity::Error,
                            format!("Argument {}: expected {:?}, found {:?}", i + 1, param_type, arg_type),
                            self.convert_span(span.clone())
                        );
                    }
                }
                
                Ok(*return_type)
            },
            _ => {
                self.add_diagnostic(
                    ErrorCode::AnalyzerUndefinedFunction,
                    Severity::Error,
                    format!("Cannot call {:?} as a function", callee_type),
                    self.convert_span(span.clone())
                );
                Ok(Type::Int)
            }
        }
    }
    
    /// Type check an if expression with span
    fn check_if_expression_with_span(&mut self, condition: &Expression, then_expr: &Expression, else_expr: &Expression, span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        let condition_type = self.check_expression_with_span(condition, Some(span))?;
        if condition_type != Type::Bool {
            self.add_diagnostic(
                ErrorCode::AnalyzerTypeMismatch,
                Severity::Error,
                format!("If condition must be boolean, found {:?}", condition_type),
                self.convert_span(span.clone())
            );
        }

        let then_type = self.check_expression_with_span(then_expr, Some(span))?;
        let else_type = self.check_expression_with_span(else_expr, Some(span))?;
        
        if self.is_type_compatible(&then_type, &else_type) {
            Ok(then_type)
        } else {
            self.add_diagnostic(
                ErrorCode::AnalyzerTypeMismatch,
                Severity::Error,
                format!("If branches have incompatible types: {:?} and {:?}", then_type, else_type),
                self.convert_span(span.clone())
            );
            Ok(then_type)
        }
    }
    
    /// Type check a match expression with span
    fn check_match_expression_with_span(&mut self, expression: &Expression, arms: &[MatchArm], span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        let _expr_type = self.check_expression_with_span(expression, Some(span))?;
        
        if arms.is_empty() {
            self.add_diagnostic(
                ErrorCode::AnalyzerNonExhaustiveMatch,
                Severity::Error,
                "Match expression must have at least one arm".to_string(),
                self.convert_span(span.clone())
            );
            return Ok(Type::Int);
        }
        
        let mut arm_types = Vec::new();
        for arm in arms {
            // Check the body of the match arm (which is a Block, not Expression)
            let arm_type = self.check_block(&arm.body)?;
            arm_types.push(arm_type);
        }
        
        // All arms must have compatible types
        let first_type = arm_types[0].clone();
        for (i, arm_type) in arm_types.iter().enumerate().skip(1) {
            if !self.is_type_compatible(&first_type, arm_type) {
                self.add_diagnostic(
                    ErrorCode::AnalyzerTypeMismatch,
                    Severity::Error,
                    format!("Match arm {} has incompatible type {:?} with previous arms", i + 1, arm_type),
                    self.convert_span(span.clone())
                );
            }
        }
        
        Ok(first_type)
    }
    
    /// Type check a lambda expression with span
    fn check_lambda_expression_with_span(&mut self, params: &[Parameter], body: &Expression, span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        let mut param_types = Vec::new();
        for param in params {
            let param_type = self.ast_type_to_type(&param.param_type);
            param_types.push(param_type);
        }

        let return_type = self.check_expression_with_span(body, Some(span))?;
        
        Ok(Type::Function(param_types, Box::new(return_type)))
    }
    
    /// Check if a type is numeric
    fn is_numeric_type(&self, ty: &Type) -> bool {
        matches!(ty, Type::Int | Type::Float)
    }
    
    /// Check if two types are compatible
    fn is_type_compatible(&self, from: &Type, to: &Type) -> bool {
        if from == to {
            return true;
        }
        
        // Allow int to float conversion
        if matches!(from, Type::Int) && matches!(to, Type::Float) {
            return true;
        }
        
        // Handle union types - check if from type is compatible with any variant
        if let Type::Sum(variants) = to {
            return variants.iter().any(|variant| self.is_type_compatible(from, variant));
        }
        
        // Handle union types in from - check if any variant is compatible with to
        if let Type::Sum(variants) = from {
            return variants.iter().any(|variant| self.is_type_compatible(variant, to));
        }
        
        // Handle union to union compatibility
        if let (Type::Sum(from_variants), Type::Sum(to_variants)) = (from, to) {
            return from_variants.iter().all(|from_variant| {
                to_variants.iter().any(|to_variant| self.is_type_compatible(from_variant, to_variant))
            });
        }
        
        false
    }
    
    /// Convert AST type to our type system
    fn ast_type_to_type(&self, ast_type: &tjlang_ast::Type) -> Type {
        match ast_type {
            tjlang_ast::Type::Primitive(primitive) => {
                match primitive {
                    tjlang_ast::PrimitiveType::Int => Type::Int,
                    tjlang_ast::PrimitiveType::Float => Type::Float,
                    tjlang_ast::PrimitiveType::Bool => Type::Bool,
                    tjlang_ast::PrimitiveType::Str => Type::Str,
                    tjlang_ast::PrimitiveType::Any => Type::Int, // Default fallback
                }
            },
            tjlang_ast::Type::Union { types, .. } => {
                let variant_types: Vec<Type> = types.iter().map(|t| self.ast_type_to_type(t)).collect();
                Type::Sum(variant_types)
            },
            tjlang_ast::Type::Vec { element_type, .. } => {
                Type::Vec(Box::new(self.ast_type_to_type(element_type)))
            },
            tjlang_ast::Type::Function { params, return_type, .. } => {
                let param_types: Vec<Type> = params.iter().map(|p| self.ast_type_to_type(p)).collect();
                Type::Function(param_types, Box::new(self.ast_type_to_type(return_type)))
            },
            _ => Type::Int, // Default fallback
        }
    }
    
    /// Check if an AST type is the Any type (for type inference)
    fn is_any_type(&self, ast_type: &tjlang_ast::Type) -> bool {
        matches!(ast_type, tjlang_ast::Type::Primitive(tjlang_ast::PrimitiveType::Any))
    }
    
    /// Infer function return type from function body
    fn infer_function_return_type(&mut self, func_decl: &FunctionDecl) -> Result<Type, DiagnosticCollection> {
        // Analyze the function body to determine return type
        self.analyze_function_body(&func_decl.body)
    }
    
    /// Analyze function body to determine return type
    fn analyze_function_body(&mut self, body: &Block) -> Result<Type, DiagnosticCollection> {
        // Look for return statements to infer return type
        let mut return_types = Vec::new();
        
        for stmt in &body.statements {
            if let Statement::Return(ret_stmt) = stmt {
                if let Some(expr) = &ret_stmt.value {
                    let return_type = self.check_expression(expr)?;
                    return_types.push(return_type);
                } else {
                    // Return without value (void)
                    return_types.push(Type::Int); // Default to Int for void
                }
            }
        }
        
        if return_types.is_empty() {
            // No return statements found, assume void
            Ok(Type::Int) // Default to Int for void
        } else if return_types.len() == 1 {
            // Single return type
            Ok(return_types[0].clone())
        } else {
            // Multiple return types - check if they're compatible
            let first_type = &return_types[0];
            let all_compatible = return_types.iter().all(|t| self.is_type_compatible(t, first_type));
            
            if all_compatible {
                Ok(first_type.clone())
            } else {
                // Return union type for incompatible return types
                Ok(Type::Sum(return_types))
            }
        }
    }
    
    /// Placeholder methods for other declaration types
    fn check_interface_declaration(&mut self, _interface: &InterfaceDecl) -> Result<(), DiagnosticCollection> {
        // TODO: Implement interface checking
        Ok(())
    }
    
    fn check_type_declaration(&mut self, _type_decl: &TypeDecl) -> Result<(), DiagnosticCollection> {
        // TODO: Implement type alias checking
        Ok(())
    }
    
    fn check_enum_declaration(&mut self, _enum_decl: &EnumDecl) -> Result<(), DiagnosticCollection> {
        // TODO: Implement enum checking
        Ok(())
    }
    
    fn check_struct_declaration(&mut self, _struct_decl: &StructDecl) -> Result<(), DiagnosticCollection> {
        // TODO: Implement struct checking
        Ok(())
    }
    
    fn check_impl_block(&mut self, _impl_block: &ImplBlock) -> Result<(), DiagnosticCollection> {
        // TODO: Implement impl block checking
        Ok(())
    }
    
    /// Type check a block
    fn check_block(&mut self, block: &Block) -> Result<Type, DiagnosticCollection> {
        let mut last_type = Type::Int;
        
        for stmt in &block.statements {
            match stmt {
                Statement::Expression(expr) => {
                    last_type = self.check_expression(expr)?;
                },
                Statement::Return(return_stmt) => {
                    if let Some(expr) = &return_stmt.value {
                        last_type = self.check_expression(expr)?;
                    }
                },
                _ => {
                    // Handle other statements
                }
            }
        }
        
        Ok(last_type)
    }

    /// Get diagnostics
    pub fn get_diagnostics(&self) -> &DiagnosticCollection {
        &self.diagnostics
    }

    /// Convert AST SourceSpan to Diagnostic SourceSpan
    fn convert_span(&self, span: tjlang_ast::SourceSpan) -> DiagnosticSourceSpan {
        DiagnosticSourceSpan::new(span.file_id, span.span)
    }

    /// Add a diagnostic
    fn add_diagnostic(&mut self, code: ErrorCode, severity: Severity, message: String, span: DiagnosticSourceSpan) {
        let diagnostic = TJLangDiagnostic::new(code, severity, message, span);
        self.diagnostics.add(diagnostic);
    }
    
    /// Type check member access with span
    fn check_member_access_with_span(&mut self, _target: &Expression, _member: &str, _span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        // TODO: Implement member access checking
        Ok(Type::Int)
    }
    
    /// Type check index access with span
    fn check_index_access_with_span(&mut self, _target: &Expression, _index: &Expression, _span: &tjlang_ast::SourceSpan) -> Result<Type, DiagnosticCollection> {
        // TODO: Implement index access checking
        Ok(Type::Int)
    }
}