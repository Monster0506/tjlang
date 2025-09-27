//! TJLang Interpreter
//! 
//! A real interpreter that works with the TJLang AST.
use std::collections::HashMap;
use tjlang_ast::*;
use tjlang_diagnostics::debug_println;
use crate::values::Value;
use crate::stdlib_integration::StdlibRegistry;


/// Runtime environment for variable storage
#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
    
    pub fn set(&mut self, name: &str, value: Value) -> bool {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            true
        } else if let Some(parent) = &mut self.parent {
            parent.set(name, value)
        } else {
            false
        }
    }
}

/// TJLang Interpreter
pub struct Interpreter {
    environment: Environment,
    functions: HashMap<String, FunctionDecl>,
    stdlib: StdlibRegistry,
}

impl Interpreter {
    pub fn new() -> Self {
        debug_println!("📦 Registering stdlib functions...");
        let environment = Environment::new();
        let functions = HashMap::new();
        let stdlib = StdlibRegistry::new();
        
        let mut interpreter = Self {
            environment,
            functions,
            stdlib,
        };
        interpreter.register_stdlib_functions();
        debug_println!("✅ Interpreter created successfully (stdlib enabled)");
        
        interpreter
    }
    
    /// Register all stdlib functions in the environment
    fn register_stdlib_functions(&mut self) {
        let function_names = self.stdlib.get_function_names();
        debug_println!("📋 Found {} stdlib functions to register", function_names.len());
        
        // First, collect all module names
        let mut modules: HashMap<String, HashMap<String, Value>> = HashMap::new();
        
        for function_name in &function_names {
            if let Some((module, func)) = function_name.split_once("::") {
                let func_value = Value::Function {
                    name: function_name.clone(),
                    params: vec![], // Native functions handle their own parameter validation
                    body: Expression::Literal(Literal::None), // Native functions don't have TJLang bodies
                    closure: HashMap::new(),
                };
                modules.entry(module.to_string()).or_insert_with(HashMap::new)
                    .insert(func.to_string(), func_value);
            }
        }
        
        // Register modules as structs
        for (module_name, functions) in modules {
            debug_println!("  📝 Registering module: {}", module_name);
            let module_value = Value::Struct {
                name: module_name.clone(),
                fields: functions,
            };
            self.environment.define(module_name.clone(), module_value);
            debug_println!("  ✅ Module {} registered successfully", module_name);
        }
        
        debug_println!("🎉 All stdlib functions registered successfully");
    }
    
    /// Interpret a complete program
    pub fn interpret_program(&mut self, program: &Program) -> Result<Value, String> {
        debug_println!("🔍 Starting program interpretation...");
        debug_println!("📊 Program has {} units", program.units.len());
        
        // First pass: collect all function declarations
        debug_println!("📝 First pass: collecting function declarations...");
        for (i, unit) in program.units.iter().enumerate() {
            debug_println!("  Unit {}: {:?}", i, std::mem::discriminant(unit));
            if let ProgramUnit::Declaration(Declaration::Function(func)) = unit {
                debug_println!("    📋 Registering function: {}", func.name);
                self.functions.insert(func.name.clone(), func.clone());
            }
        }
        debug_println!("✅ Registered {} functions", self.functions.len());
        
        // Second pass: execute the program
        debug_println!("🏃 Second pass: executing program...");
        let mut result = Value::None;
        for (i, unit) in program.units.iter().enumerate() {
            debug_println!("  Executing unit {}: {:?}", i, std::mem::discriminant(unit));
            match unit {
                ProgramUnit::Declaration(decl) => {
                    debug_println!("    🔧 Interpreting declaration: {:?}", std::mem::discriminant(decl));
                    result = self.interpret_declaration(decl)?;
                    debug_println!("    ✅ Declaration result: {:?}", result);
                },
                ProgramUnit::Expression(expr) => {
                    debug_println!("    🎯 Interpreting expression: {:?}", std::mem::discriminant(expr));
                    result = self.interpret_expression(expr)?;
                    debug_println!("    ✅ Expression result: {:?}", result);
                },
                _ => {
                    debug_println!("    ⏭️ Skipping unknown unit type");
                }
            }
        }
        
        debug_println!("🎉 Program interpretation completed successfully");
        Ok(result)
    }
    
    /// Interpret a declaration
    fn interpret_declaration(&mut self, decl: &Declaration) -> Result<Value, String> {
        debug_println!("      🔍 Interpreting declaration: {:?}", std::mem::discriminant(decl));
        match decl {
            Declaration::Function(func) => {
                debug_println!("        📋 Processing function: {}", func.name);
                // Store function in environment so it can be called
                // We'll store the function with a reference to the actual function
                // The function body will be interpreted when the function is called
                let func_value = Value::Function {
                    name: func.name.clone(),
                    params: func.params.iter().map(|p| p.name.clone()).collect(),
                    body: Expression::Literal(Literal::None), // This will be replaced with actual body handling
                    closure: HashMap::new(),
                };
                self.environment.define(func.name.clone(), func_value);
                debug_println!("        ✅ Function {} stored in environment", func.name);
                Ok(Value::None)
            },
            Declaration::Variable(var) => {
                debug_println!("        📝 Processing variable: {}", var.name);
                let value = self.interpret_expression(&var.value)?;
                self.environment.define(var.name.clone(), value.clone());
                debug_println!("        ✅ Variable {} stored with value: {:?}", var.name, value);
                Ok(value)
            },
            _ => {
                debug_println!("        ⏭️ Skipping unknown declaration type");
                Ok(Value::None)
            },
        }
    }
    
    /// Interpret an expression
    pub fn interpret_expression(&mut self, expr: &Expression) -> Result<Value, String> {
        debug_println!("        🔍 Interpreting expression: {:?}", std::mem::discriminant(expr));
        match expr {
            Expression::Literal(literal) => {
                debug_println!("          📝 Literal: {:?}", literal);
                self.interpret_literal(literal)
            },
            Expression::Variable(name) => {
                debug_println!("          🔍 Variable: {}", name);
                self.environment.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            },
            Expression::Binary { left, operator, right, .. } => {
                debug_println!("          ➕ Binary operation: {:?}", operator);
                debug_println!("          📝 Left operand: {:?}", left);
                debug_println!("          📝 Right operand: {:?}", right);
                let left_val = self.interpret_expression(left)?;
                debug_println!("          📝 Left value: {:?}", left_val);
                let right_val = self.interpret_expression(right)?;
                debug_println!("          📝 Right value: {:?}", right_val);
                let result = self.interpret_binary_operation(&left_val, operator, &right_val);
                debug_println!("          📝 Binary result: {:?}", result);
                result
            },
            Expression::Unary { operator, operand, .. } => {
                debug_println!("          🔢 Unary operation: {:?}", operator);
                let operand_val = self.interpret_expression(operand)?;
                self.interpret_unary_operation(operator, &operand_val)
            },
            Expression::Call { callee, args, .. } => {
                debug_println!("          📞 Function call with {} args", args.len());
                debug_println!("          🔍 Callee expression: {:?}", callee);
                
                // Special handling for method calls on primitives
                if let Expression::Member { target, member, .. } = callee.as_ref() {
                    debug_println!("          🎯 Method call: {}.{}()", "target", member);
                    let target_val = self.interpret_expression(target)?;
                    debug_println!("          🎯 Target value: {:?}", std::mem::discriminant(&target_val));
                    
                    // For primitive methods that don't take arguments, execute directly
                    if args.is_empty() {
                        match member.as_str() {
                            "to_string" => return Ok(Value::String(target_val.to_string())),
                            "clone" => return Ok(target_val.clone()),
                            "type_name" => return Ok(Value::String(crate::primitive_methods::get_type_name(&target_val))),
                            "is_null" => return Ok(Value::Bool(matches!(target_val, Value::None))),
                            "is_not_null" => return Ok(Value::Bool(!matches!(target_val, Value::None))),
                            "hash" => return Ok(Value::Int(crate::primitive_methods::get_hash_code(&target_val))),
                            "is_int" => return Ok(Value::Bool(matches!(target_val, Value::Int(_)))),
                            "is_float" => return Ok(Value::Bool(matches!(target_val, Value::Float(_)))),
                            "is_bool" => return Ok(Value::Bool(matches!(target_val, Value::Bool(_)))),
                            "is_str" => return Ok(Value::Bool(matches!(target_val, Value::String(_)))),
                            "is_none" => return Ok(Value::Bool(matches!(target_val, Value::None))),
                            "debug_string" => return Ok(Value::String(format!("{:?}", target_val))),
                            "pretty_string" => return Ok(Value::String(crate::primitive_methods::get_pretty_string(&target_val))),
                            _ => {
                                // Try type-specific methods
                                return crate::primitive_methods::get_primitive_method(&target_val, member);
                            }
                        }
                    }
                }
                
                let callee_val = self.interpret_expression(callee)?;
                debug_println!("          🎯 Callee resolved to: {:?}", callee_val);
                let mut arg_values = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    debug_println!("            📥 Evaluating arg {}: {:?}", i, std::mem::discriminant(arg));
                    let arg_val = self.interpret_expression(arg)?;
                    debug_println!("            📥 Arg {} resolved to: {:?}", i, arg_val);
                    arg_values.push(arg_val);
                }
                debug_println!("          🚀 Calling function: {:?} with args: {:?}", callee_val, arg_values);
                let result = self.interpret_call(&callee_val, &arg_values);
                debug_println!("          ✅ Function call result: {:?}", result);
                result
            },
            Expression::Member { target, member, .. } => {
                debug_println!("          🎯 Member access: {}", member);
                let target_val = self.interpret_expression(target)?;
                debug_println!("          🎯 Target value: {:?}", std::mem::discriminant(&target_val));
                debug_println!("          🎯 Target value content: {:?}", target_val);
                self.interpret_member_access(&target_val, member)
            },
            Expression::Index { target, index, .. } => {
                let target_val = self.interpret_expression(target)?;
                let index_val = self.interpret_expression(index)?;
                self.interpret_index_access(&target_val, &index_val)
            },
            Expression::Lambda { params, body, .. } => {
                // Create a closure
                Ok(Value::Closure {
                    params: params.iter().map(|p| p.name.clone()).collect(),
                    body: *body.clone(),
                    closure: HashMap::new(),
                })
            },
            Expression::If { condition, then_expr, else_expr, .. } => {
                let condition_val = self.interpret_expression(condition)?;
                if self.is_truthy(&condition_val) {
                    self.interpret_expression(then_expr)
                } else {
                    self.interpret_expression(else_expr)
                }
            },
            Expression::Match { expression, arms, .. } => {
                let value_val = self.interpret_expression(expression)?;
                self.interpret_match(&value_val, arms)
            },
            Expression::StructLiteral { name, fields, .. } => {
                let mut field_map = HashMap::new();
                for field in fields {
                    let value = self.interpret_expression(&field.value)?;
                    field_map.insert(field.name.clone(), value);
                }
                Ok(Value::Struct {
                    name: name.clone(),
                    fields: field_map,
                })
            },
            Expression::VecLiteral { elements, .. } => {
                let mut vec = Vec::new();
                for element in elements {
                    vec.push(self.interpret_expression(element)?);
                }
                Ok(Value::Vec(vec))
            },
            _ => Err("Unsupported expression type".to_string()),
        }
    }
    
    /// Interpret a literal
    fn interpret_literal(&mut self, literal: &Literal) -> Result<Value, String> {
        match literal {
            Literal::Int(value) => Ok(Value::Int(*value)),
            Literal::Float(value) => Ok(Value::Float(*value)),
            Literal::Bool(value) => Ok(Value::Bool(*value)),
            Literal::String(value) => Ok(Value::String(value.clone())),
            Literal::FString(value) => Ok(Value::String(value.clone())),
            Literal::FStringInterpolation(parts) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        FStringPart::Text(text) => result.push_str(text),
                        FStringPart::Expression(expr) => {
                            let value = self.interpret_expression(expr)?;
                            result.push_str(&value.to_string());
                        }
                    }
                }
                Ok(Value::String(result))
            },
            Literal::None => Ok(Value::None),
        }
    }
    
    /// Interpret a binary operation
    fn interpret_binary_operation(&self, left: &Value, op: &BinaryOperator, right: &Value) -> Result<Value, String> {
        match op {
            BinaryOperator::Add => self.add_values(left, right),
            BinaryOperator::Subtract => self.subtract_values(left, right),
            BinaryOperator::Multiply => self.multiply_values(left, right),
            BinaryOperator::Divide => self.divide_values(left, right),
            BinaryOperator::Modulo => self.modulo_values(left, right),
            BinaryOperator::Equal => Ok(Value::Bool(left == right)),
            BinaryOperator::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOperator::LessThan => self.compare_values(left, right, |a, b| a < b),
            BinaryOperator::LessThanEqual => self.compare_values(left, right, |a, b| a <= b),
            BinaryOperator::GreaterThan => self.compare_values(left, right, |a, b| a > b),
            BinaryOperator::GreaterThanEqual => self.compare_values(left, right, |a, b| a >= b),
            BinaryOperator::And => Ok(Value::Bool(self.is_truthy(left) && self.is_truthy(right))),
            BinaryOperator::Or => Ok(Value::Bool(self.is_truthy(left) || self.is_truthy(right))),
            _ => Err("Unsupported binary operator".to_string()),
        }
    }
    
    /// Interpret a unary operation
    fn interpret_unary_operation(&self, op: &UnaryOperator, operand: &Value) -> Result<Value, String> {
        match op {
            UnaryOperator::Not => Ok(Value::Bool(!self.is_truthy(operand))),
            UnaryOperator::Negate => self.negate_value(operand),
            UnaryOperator::BitNot => Err("Bitwise operations not implemented".to_string()),
        }
    }
    
    /// Interpret a function call
    pub fn interpret_call(&mut self, callee: &Value, args: &[Value]) -> Result<Value, String> {
        debug_println!("            🔍 interpret_call: {:?}", std::mem::discriminant(callee));
        match callee {
            Value::Function { name, body, .. } => {
                debug_println!("            📞 Calling function: {}", name);
                debug_println!("            📋 Available stdlib functions: {:?}", self.stdlib.get_function_names());
                
                        // Check if it's a primitive method call
                        debug_println!("              🔍 Checking method call: name='{}', args.len()={}", name, args.len());
                        if name.starts_with("primitive_method::") {
                            debug_println!("              🔧 Handling primitive method call: {}", name);
                            // This is a primitive method call, execute using the primitive methods module
                            let method_name = name.strip_prefix("primitive_method::").unwrap_or(name);
                            let result = crate::primitive_methods::execute_primitive_method(&Value::None, method_name, args);
                            debug_println!("              ✅ Primitive method call result: {:?}", result);
                            return result;
                        }
                
                // First check if it's a stdlib function
                if let Some(native_func) = self.stdlib.get_function(name) {
                    debug_println!("              🚀 Calling stdlib function: {} with args: {:?}", name, args);
                    let result = native_func(self, args);
                    debug_println!("              ✅ Stdlib function result: {:?}", result);
                    return result;
                }
                
                debug_println!("              🔍 Looking up user function: {}", name);
                debug_println!("              📋 Available functions: {:?}", self.functions.keys().collect::<Vec<_>>());
                // Look up the actual function declaration
                if let Some(func_decl) = self.functions.get(name) {
                    debug_println!("              ✅ Found function declaration for: {}", name);
                    debug_println!("              📝 Function params: {:?}", func_decl.params);
                    debug_println!("              📝 Function body: {:?}", func_decl.body);
                    // Clone the function declaration to avoid borrow conflicts
                    let func_decl = func_decl.clone();
                    
                    debug_println!("              🔧 Creating new environment with {} params", func_decl.params.len());
                    // Create new environment with parameters that has access to global scope
                    let mut new_env = Environment::with_parent(self.environment.clone());
                    for (param, arg) in func_decl.params.iter().zip(args.iter()) {
                        debug_println!("                📝 Binding param {} = {:?}", param.name, arg);
                        new_env.define(param.name.clone(), arg.clone());
                    }
                    
                    debug_println!("              🏃 Executing function body...");
                    debug_println!("              🔍 Function body: {:?}", func_decl.body);
                    // Save current environment and switch to new one
                    let old_env = std::mem::replace(&mut self.environment, new_env);
                    let result = self.interpret_block(&func_decl.body);
                    self.environment = old_env;
                    debug_println!("              ✅ Function {} completed with result: {:?}", name, result);
                    result
                } else {
                    debug_println!("              ❌ Function '{}' not found", name);
                    debug_println!("              📋 Available functions: {:?}", self.functions.keys().collect::<Vec<_>>());
                    Err(format!("Function '{}' not found", name))
                }
            },
            Value::Closure { params, body, .. } => {
                debug_println!("            🔒 Calling closure with {} params", params.len());
                // Create new environment with parameters
                let mut new_env = Environment::new();
                for (param_name, arg) in params.iter().zip(args.iter()) {
                    new_env.define(param_name.clone(), arg.clone());
                }
                
                // Save current environment and switch to new one
                let old_env = std::mem::replace(&mut self.environment, new_env);
                let result = self.interpret_expression(body);
                self.environment = old_env;
                result
            },
            _ => {
                debug_println!("            ❌ Cannot call non-function value: {:?}", callee);
                Err("Cannot call non-function value".to_string())
            },
        }
    }
    
    /// Interpret member access
    fn interpret_member_access(&self, target: &Value, member: &str) -> Result<Value, String> {
        debug_println!("🔧 interpret_member_access called: target={:?}, member={}", std::mem::discriminant(target), member);
        match target {
            Value::Struct { fields, .. } => {
                debug_println!("🔧 Struct member access");
                fields.get(member)
                    .cloned()
                    .ok_or_else(|| format!("No field '{}' found", member))
            },
            // Handle method calls on primitive values
            _ => {
                debug_println!("🔧 Primitive method access");
                self.get_primitive_method(target, member)
            }
        }
    }
    
    /// Get a method for a primitive value
    fn get_primitive_method(&self, target: &Value, method: &str) -> Result<Value, String> {
        debug_println!("🔧 get_primitive_method called: target={:?}, method={}", std::mem::discriminant(target), method);
        
        // For methods that don't require arguments, execute them directly
        match method {
            "to_string" => Ok(Value::String(target.to_string())),
            "clone" => Ok(target.clone()),
            "type_name" => Ok(Value::String(crate::primitive_methods::get_type_name(target))),
            "is_null" => Ok(Value::Bool(matches!(target, Value::None))),
            "is_not_null" => Ok(Value::Bool(!matches!(target, Value::None))),
            "hash" => Ok(Value::Int(crate::primitive_methods::get_hash_code(target))),
            "is_int" => Ok(Value::Bool(matches!(target, Value::Int(_)))),
            "is_float" => Ok(Value::Bool(matches!(target, Value::Float(_)))),
            "is_bool" => Ok(Value::Bool(matches!(target, Value::Bool(_)))),
            "is_str" => Ok(Value::Bool(matches!(target, Value::String(_)))),
            "is_none" => Ok(Value::Bool(matches!(target, Value::None))),
            "debug_string" => Ok(Value::String(format!("{:?}", target))),
            "pretty_string" => Ok(Value::String(crate::primitive_methods::get_pretty_string(target))),
            // Type-specific methods
            _ => crate::primitive_methods::get_primitive_method(target, method),
        }
    }
    
    /// Get the type name of a value for error messages
    fn get_value_type_name(&self, value: &Value) -> &'static str {
        match value {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Struct { .. } => "struct",
            Value::Function { .. } => "function",
            Value::Closure { .. } => "closure",
            Value::None => "none",
            Value::Enum { .. } => "enum",
            Value::Tuple(_) => "tuple",
            Value::Vec(_) => "vec",
            Value::Set(_) => "set",
            Value::Map(_) => "map",
            Value::Channel { .. } => "channel",
            Value::Task { .. } => "task",
            Value::Reference(_) => "reference",
            Value::Type(_) => "type",
        }
    }
    
    /// Interpret index access
    fn interpret_index_access(&self, target: &Value, index: &Value) -> Result<Value, String> {
        match (target, index) {
            (Value::Vec(vec), Value::Int(idx)) => {
                if *idx >= 0 && (*idx as usize) < vec.len() {
                    Ok(vec[*idx as usize].clone())
                } else {
                    Err("Index out of bounds".to_string())
                }
            },
            _ => Err("Cannot index non-vector value or invalid index type".to_string()),
        }
    }
    
    /// Interpret a match expression
    fn interpret_match(&mut self, value: &Value, arms: &[MatchArm]) -> Result<Value, String> {
        for arm in arms {
            if self.pattern_matches(value, &arm.pattern) {
                return self.interpret_block(&arm.body);
            }
        }
        Err("No matching pattern found".to_string())
    }
    
    /// Check if a pattern matches a value
    fn pattern_matches(&self, value: &Value, pattern: &Pattern) -> bool {
        match (value, pattern) {
            (Value::Int(v), Pattern::Literal(Literal::Int(p))) => v == p,
            (Value::Float(v), Pattern::Literal(Literal::Float(p))) => (v - p).abs() < f64::EPSILON,
            (Value::Bool(v), Pattern::Literal(Literal::Bool(p))) => v == p,
            (Value::String(v), Pattern::Literal(Literal::String(p))) => v == p,
            (Value::Struct { name: v_name, fields: v_fields }, Pattern::Struct { name: p_name, fields: p_fields, .. }) => {
                v_name == p_name && p_fields.len() == v_fields.len() &&
                p_fields.iter().all(|(p_field, p_value)| {
                    v_fields.get(p_field)
                        .map(|v_value| self.pattern_matches(v_value, p_value))
                        .unwrap_or(false)
                })
            },
            (_, Pattern::Variable { .. }) => true, // Variables always match
            (_, Pattern::Wildcard(_)) => true,
            _ => false,
        }
    }
    
    /// Interpret a block
    fn interpret_block(&mut self, block: &Block) -> Result<Value, String> {
        debug_println!("                🔧 interpret_block: {} statements", block.statements.len());
        let mut result = Value::None;
        for (i, stmt) in block.statements.iter().enumerate() {
            debug_println!("                  📝 Statement {}: {:?}", i, std::mem::discriminant(stmt));
            debug_println!("                  🔍 Statement {} details: {:?}", i, stmt);
            result = self.interpret_statement(stmt)?;
            debug_println!("                  ✅ Statement {} result: {:?}", i, result);
        }
        debug_println!("                🎉 Block completed with result: {:?}", result);
        Ok(result)
    }
    
    /// Interpret a statement
    fn interpret_statement(&mut self, stmt: &Statement) -> Result<Value, String> {
        debug_println!("                    🔍 interpret_statement: {:?}", std::mem::discriminant(stmt));
        match stmt {
            Statement::Expression(expr) => {
                debug_println!("                      📝 Expression statement");
                self.interpret_expression(expr)
            },
            Statement::Variable(var) => {
                debug_println!("                      📝 Variable statement: {}", var.name);
                let value = self.interpret_expression(&var.value)?;
                self.environment.define(var.name.clone(), value.clone());
                Ok(value)
            },
            Statement::Return(ret_stmt) => {
                debug_println!("                      📝 Return statement");
                if let Some(expr) = &ret_stmt.value {
                    self.interpret_expression(expr)
                } else {
                    Ok(Value::None)
                }
            },
            Statement::If(if_stmt) => {
                let condition_val = self.interpret_expression(&if_stmt.condition)?;
                if self.is_truthy(&condition_val) {
                    self.interpret_block(&if_stmt.then_block)
                } else if let Some(else_block) = &if_stmt.else_block {
                    self.interpret_block(else_block)
                } else {
                    Ok(Value::None)
                }
            },
            Statement::While(while_stmt) => {
                loop {
                    let condition_val = self.interpret_expression(&while_stmt.condition)?;
                    if !self.is_truthy(&condition_val) {
                        break;
                    }
                    self.interpret_block(&while_stmt.body)?;
                }
                Ok(Value::None)
            },
            Statement::For(for_stmt) => {
                match for_stmt {
                    ForStatement::ForEach { var_name, iterable, body, .. } => {
                        let iter_val = self.interpret_expression(iterable)?;
                        if let Value::Vec(vec) = iter_val {
                            for item in vec {
                                self.environment.define(var_name.clone(), item);
                                self.interpret_block(body)?;
                            }
                        }
                    },
                    ForStatement::CStyle { .. } => {
                        // C-style for loops not implemented yet
                        return Err("C-style for loops not implemented".to_string());
                    }
                }
                Ok(Value::None)
            },
            Statement::Block(block) => self.interpret_block(block),
            _ => Ok(Value::None),
        }
    }
    
    /// Helper methods for operations
    fn add_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err("Cannot add these types".to_string()),
        }
    }
    
    fn subtract_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err("Cannot subtract these types".to_string()),
        }
    }
    
    fn multiply_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err("Cannot multiply these types".to_string()),
        }
    }
    
    fn divide_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Int(a / b))
                }
            },
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            },
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            },
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / *b as f64))
                }
            },
            _ => Err("Cannot divide these types".to_string()),
        }
    }
    
    fn modulo_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Int(a % b))
                }
            },
            _ => Err("Cannot modulo these types".to_string()),
        }
    }
    
    fn compare_values<F>(&self, left: &Value, right: &Value, cmp: F) -> Result<Value, String> 
    where F: FnOnce(f64, f64) -> bool 
    {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(cmp(*a as f64, *b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(cmp(*a, *b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(cmp(*a as f64, *b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(cmp(*a, *b as f64))),
            _ => Err("Cannot compare these types".to_string()),
        }
    }
    
    fn negate_value(&self, operand: &Value) -> Result<Value, String> {
        match operand {
            Value::Int(a) => Ok(Value::Int(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            _ => Err("Cannot negate this type".to_string()),
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Vec(v) => !v.is_empty(),
            Value::None => false,
            _ => true,
        }
    }
}

/// Closure for function values
#[derive(Debug, Clone)]
pub struct Closure {
    pub params: Vec<String>,
    pub body: Expression,
    pub environment: Environment,
}







