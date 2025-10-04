//! TJLang Interpreter
//!
//! A real interpreter that works with the TJLang AST.
use crate::stdlib_integration::StdlibRegistry;
use crate::values::Value;
use std::collections::HashMap;
use tjlang_ast::*;
use tjlang_diagnostics::debug_println;

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
        debug_println!(" Registering stdlib functions...");
        debug_println!("[DEBUG] DEBUG: Interpreter::new() called");
        let environment = Environment::new();
        let functions = HashMap::new();
        let stdlib = StdlibRegistry::new();

        let mut interpreter = Self {
            environment,
            functions,
            stdlib,
        };
        interpreter.register_stdlib_functions();
        debug_println!("[DEBUG] Interpreter created successfully (stdlib enabled)");

        interpreter
    }

    /// Register all stdlib functions in the environment
    fn register_stdlib_functions(&mut self) {
        debug_println!("[DEBUG] DEBUG: register_stdlib_functions called");
        let function_names = self.stdlib.get_function_names();
        debug_println!(
            " Found {} stdlib functions to register",
            function_names.len()
        );

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
                modules
                    .entry(module.to_string())
                    .or_insert_with(HashMap::new)
                    .insert(func.to_string(), func_value);
            }
        }

        // Register modules as structs
        for (module_name, functions) in modules {
            debug_println!("[DEBUG] DEBUG: Registering module: {}", module_name);
            debug_println!(
                "[DEBUG] DEBUG: Functions in {}: {:?}",
                module_name,
                functions.keys().collect::<Vec<_>>()
            );
            let module_value = Value::Struct {
                name: module_name.clone(),
                fields: functions,
            };
            self.environment.define(module_name.clone(), module_value);
            debug_println!(
                "[DEBUG] DEBUG: Module {} registered successfully",
                module_name
            );
        }

        debug_println!(" All stdlib functions registered successfully");
    }

    /// Interpret a complete program
    pub fn interpret_program(&mut self, program: &Program) -> Result<Value, String> {
        debug_println!("[DEBUG] Starting program interpretation...");
        debug_println!(" Program has {} units", program.units.len());

        // First pass: collect all function declarations
        debug_println!(" First pass: collecting function declarations...");
        for (i, unit) in program.units.iter().enumerate() {
            debug_println!("  Unit {}: {:?}", i, std::mem::discriminant(unit));
            if let ProgramUnit::Declaration(Declaration::Function(func)) = unit {
                debug_println!("     Registering function: {}", func.name);
                self.functions.insert(func.name.clone(), func.clone());
            }
        }
        debug_println!("[DEBUG] Registered {} functions", self.functions.len());

        // Second pass: execute the program
        debug_println!(" Second pass: executing program...");
        let mut result = Value::None;
        for (i, unit) in program.units.iter().enumerate() {
            debug_println!("  Executing unit {}: {:?}", i, std::mem::discriminant(unit));
            match unit {
                ProgramUnit::Declaration(decl) => {
                    debug_println!(
                        "     Interpreting declaration: {:?}",
                        std::mem::discriminant(decl)
                    );
                    result = self.interpret_declaration(decl)?;
                    debug_println!("    [DEBUG] Declaration result: {:?}", result);
                }
                ProgramUnit::Expression(expr) => {
                    debug_println!(
                        "    [DEBUG] Interpreting expression: {:?}",
                        std::mem::discriminant(expr)
                    );
                    result = self.interpret_expression(expr)?;
                    debug_println!("    [DEBUG] Expression result: {:?}", result);
                }
                ProgramUnit::Statement(stmt) => {
                    debug_println!(
                        "    [DEBUG] Interpreting statement: {:?}",
                        std::mem::discriminant(stmt)
                    );
                    result = self.interpret_statement(stmt)?;
                    debug_println!("    [DEBUG] Statement result: {:?}", result);
                }
                _ => {
                    debug_println!("     Skipping unknown unit type");
                }
            }
        }

        debug_println!(" Program interpretation completed successfully");
        Ok(result)
    }

    /// Interpret a declaration
    fn interpret_declaration(&mut self, decl: &Declaration) -> Result<Value, String> {
        debug_println!(
            "      [DEBUG] Interpreting declaration: {:?}",
            std::mem::discriminant(decl)
        );
        match decl {
            Declaration::Function(func) => {
                debug_println!("         Processing function: {}", func.name);
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
                debug_println!(
                    "        [DEBUG] Function {} stored in environment",
                    func.name
                );
                Ok(Value::None)
            }
            Declaration::Variable(var) => {
                debug_println!("         Processing variable: {}", var.name);
                let value = self.interpret_expression(&var.value)?;
                self.environment.define(var.name.clone(), value.clone());
                debug_println!(
                    "        [DEBUG] Variable {} stored with value: {:?}",
                    var.name,
                    value
                );
                Ok(value)
            }
            _ => {
                debug_println!("         Skipping unknown declaration type");
                Ok(Value::None)
            }
        }
    }

    /// Interpret an expression
    pub fn interpret_expression(&mut self, expr: &Expression) -> Result<Value, String> {
        debug_println!(
            "        [DEBUG] Interpreting expression: {:?}",
            std::mem::discriminant(expr)
        );
        match expr {
            Expression::Literal(literal) => {
                debug_println!("           Literal: {:?}", literal);
                self.interpret_literal(literal)
            }
            Expression::Variable(name) => {
                debug_println!("          [DEBUG] Variable: {}", name);
                self.environment
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                debug_println!("           Binary operation: {:?}", operator);
                debug_println!("           Left operand: {:?}", left);
                debug_println!("           Right operand: {:?}", right);

                // Special handling for assignment
                if *operator == BinaryOperator::Assign {
                    if let Expression::Variable(var_name) = left.as_ref() {
                        let value = self.interpret_expression(right)?;
                        self.environment.define(var_name.clone(), value.clone());
                        return Ok(value);
                    } else {
                        return Err("Left side of assignment must be a variable".to_string());
                    }
                }

                let left_val = self.interpret_expression(left)?;
                debug_println!("           Left value: {:?}", left_val);
                let right_val = self.interpret_expression(right)?;
                debug_println!("           Right value: {:?}", right_val);
                let result = self.interpret_binary_operation(&left_val, operator, &right_val);
                debug_println!("           Binary result: {:?}", result);
                result
            }
            Expression::Unary {
                operator, operand, ..
            } => {
                debug_println!("[DEBUG] [UNARY] Unary operation: {:?}", operator);
                let operand_val = self.interpret_expression(operand)?;
                debug_println!("[DEBUG] [UNARY] Operand value: {:?}", operand_val);
                let result = self.interpret_unary_operation(operator, &operand_val);
                debug_println!("[DEBUG] [UNARY] Result: {:?}", result);
                result
            }
            Expression::Call { callee, args, .. } => {
                debug_println!("           Function call with {} args", args.len());
                debug_println!("          [DEBUG] Callee expression: {:?}", callee);

                // Special handling for method calls on primitives
                if let Expression::Member { target, member, .. } = callee.as_ref() {
                    debug_println!("          [DEBUG] Method call: {}.{}()", "target", member);
                    let target_val = self.interpret_expression(target)?;
                    debug_println!(
                        "          [DEBUG] Target value: {:?}",
                        std::mem::discriminant(&target_val)
                    );

                    // Only apply primitive method handling to actual primitive types, not structs
                    let is_primitive = matches!(
                        target_val,
                        Value::Int(_)
                            | Value::Float(_)
                            | Value::Bool(_)
                            | Value::String(_)
                            | Value::None
                            | Value::Tuple(_)
                            | Value::Vec(_)
                            | Value::Set(_)
                            | Value::Map(_)
                    );

                    if is_primitive && args.is_empty() {
                        match member.as_str() {
                            "to_string" => return Ok(Value::String(target_val.to_string())),
                            "clone" => return Ok(target_val.clone()),
                            "type_name" => {
                                return Ok(Value::String(crate::primitive_methods::get_type_name(
                                    &target_val,
                                )))
                            }
                            "is_null" => return Ok(Value::Bool(matches!(target_val, Value::None))),
                            "is_not_null" => {
                                return Ok(Value::Bool(!matches!(target_val, Value::None)))
                            }
                            "hash" => {
                                return Ok(Value::Int(crate::primitive_methods::get_hash_code(
                                    &target_val,
                                )))
                            }
                            "is_int" => {
                                return Ok(Value::Bool(matches!(target_val, Value::Int(_))))
                            }
                            "is_float" => {
                                return Ok(Value::Bool(matches!(target_val, Value::Float(_))))
                            }
                            "is_bool" => {
                                return Ok(Value::Bool(matches!(target_val, Value::Bool(_))))
                            }
                            "is_str" => {
                                return Ok(Value::Bool(matches!(target_val, Value::String(_))))
                            }
                            "is_none" => return Ok(Value::Bool(matches!(target_val, Value::None))),
                            "debug_string" => {
                                return Ok(Value::String(format!("{:?}", target_val)))
                            }
                            "pretty_string" => {
                                return Ok(Value::String(
                                    crate::primitive_methods::get_pretty_string(&target_val),
                                ))
                            }
                            "reverse" => {
                                // Special handling for reverse as in-place modification
                                if matches!(target_val, Value::Vec(_)) {
                                    let result = crate::primitive_methods::get_primitive_method(
                                        &target_val,
                                        member,
                                    )?;
                                    // If the target is a simple variable reference, update it in the environment
                                    if let Expression::Variable(var_name) = &**target {
                                        debug_println!(
                                            "           Updating variable {} in-place",
                                            var_name
                                        );
                                        self.environment.define(var_name.clone(), result.clone());
                                    }
                                    return Ok(result);
                                } else {
                                    return crate::primitive_methods::get_primitive_method(
                                        &target_val,
                                        member,
                                    );
                                }
                            }
                            "sort" => {
                                // Special handling for sort as in-place modification
                                if matches!(target_val, Value::Vec(_)) {
                                    let result = crate::primitive_methods::get_primitive_method(
                                        &target_val,
                                        member,
                                    )?;
                                    // If the target is a simple variable reference, update it in the environment
                                    if let Expression::Variable(var_name) = &**target {
                                        debug_println!(
                                            "           Updating variable {} in-place",
                                            var_name
                                        );
                                        self.environment.define(var_name.clone(), result.clone());
                                    }
                                    return Ok(result);
                                } else {
                                    return crate::primitive_methods::get_primitive_method(
                                        &target_val,
                                        member,
                                    );
                                }
                            }
                            _ => {
                                // Try type-specific methods
                                return crate::primitive_methods::get_primitive_method(
                                    &target_val,
                                    member,
                                );
                            }
                        }
                    } else if is_primitive {
                        // For primitive methods that do take arguments
                        let mut arg_values = Vec::new();
                        for arg in args {
                            arg_values.push(self.interpret_expression(arg)?);
                        }

                        // Special handling for collection methods that should modify in-place
                        if matches!(target_val, Value::Vec(_))
                            && matches!(
                                member.as_str(),
                                "push"
                                    | "pop"
                                    | "insert"
                                    | "remove"
                                    | "set"
                                    | "reverse"
                                    | "sort"
                                    | "sort_by"
                            )
                        {
                            // For vector methods that modify in-place, we need to update the original variable
                            let result = crate::primitive_methods::execute_primitive_method(
                                &target_val,
                                member,
                                &arg_values,
                            )?;

                            // If the target is a simple variable reference, update it in the environment
                            if let Expression::Variable(var_name) = &**target {
                                // Fixed: &**target to dereference Box<Expression>
                                debug_println!(
                                    "           Updating variable {} in-place",
                                    var_name
                                );
                                self.environment.define(var_name.clone(), result.clone());
                            }

                            return Ok(result);
                        } else if matches!(target_val, Value::Set(_))
                            && matches!(member.as_str(), "insert" | "remove")
                        {
                            // For set methods that modify in-place
                            let result = crate::primitive_methods::execute_primitive_method(
                                &target_val,
                                member,
                                &arg_values,
                            )?;

                            // If the target is a simple variable reference, update it in the environment
                            if let Expression::Variable(var_name) = &**target {
                                debug_println!(
                                    "           Updating variable {} in-place",
                                    var_name
                                );
                                self.environment.define(var_name.clone(), result.clone());
                            }

                            return Ok(result);
                        } else if matches!(target_val, Value::Map(_))
                            && matches!(member.as_str(), "insert" | "set" | "remove" | "clear")
                        {
                            // For map methods that modify in-place
                            let result = crate::primitive_methods::execute_primitive_method(
                                &target_val,
                                member,
                                &arg_values,
                            )?;

                            // If the target is a simple variable reference, update it in the environment
                            if let Expression::Variable(var_name) = &**target {
                                debug_println!(
                                    "           Updating variable {} in-place",
                                    var_name
                                );
                                self.environment.define(var_name.clone(), result.clone());
                            }

                            return Ok(result);
                        }

                        return crate::primitive_methods::execute_primitive_method(
                            &target_val,
                            member,
                            &arg_values,
                        );
                    }
                }

                let callee_val = self.interpret_expression(callee)?;
                debug_println!("          [DEBUG] Callee resolved to: {:?}", callee_val);
                let mut arg_values = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    debug_println!(
                        "             Evaluating arg {}: {:?}",
                        i,
                        std::mem::discriminant(arg)
                    );
                    let arg_val = self.interpret_expression(arg)?;
                    debug_println!("             Arg {} resolved to: {:?}", i, arg_val);
                    arg_values.push(arg_val);
                }
                debug_println!(
                    "           Calling function: {:?} with args: {:?}",
                    callee_val,
                    arg_values
                );
                let result = self.interpret_call(&callee_val, &arg_values);
                debug_println!("          [DEBUG] Function call result: {:?}", result);
                result
            }
            Expression::Member { target, member, .. } => {
                debug_println!("          [DEBUG] Member access: {}", member);
                let target_val = self.interpret_expression(target)?;
                debug_println!(
                    "          [DEBUG] Target value: {:?}",
                    std::mem::discriminant(&target_val)
                );
                debug_println!("          [DEBUG] Target value content: {:?}", target_val);
                self.interpret_member_access(&target_val, member)
            }
            Expression::Index { target, index, .. } => {
                let target_val = self.interpret_expression(target)?;
                let index_val = self.interpret_expression(index)?;
                self.interpret_index_access(&target_val, &index_val)
            }
            Expression::Lambda { params, body, .. } => {
                // Create a closure
                Ok(Value::Closure {
                    params: params.iter().map(|p| p.name.clone()).collect(),
                    body: *body.clone(),
                    closure: HashMap::new(),
                })
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                let condition_val = self.interpret_expression(condition)?;
                if self.is_truthy(&condition_val) {
                    self.interpret_expression(then_expr)
                } else {
                    self.interpret_expression(else_expr)
                }
            }
            Expression::Match {
                expression, arms, ..
            } => {
                let value_val = self.interpret_expression(expression)?;
                self.interpret_match(&value_val, arms)
            }
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
            }
            Expression::VecLiteral { elements, .. } => {
                let mut vec = Vec::new();
                for element in elements {
                    vec.push(self.interpret_expression(element)?);
                }
                Ok(Value::Vec(vec))
            }
            Expression::Range {
                start,
                end,
                inclusive,
                ..
            } => {
                debug_println!("           Range expression");
                let start_val = self.interpret_expression(start)?;
                let end_val = self.interpret_expression(end)?;

                if let (Value::Int(start_int), Value::Int(end_int)) = (start_val, end_val) {
                    let vec: Vec<Value> = if *inclusive {
                        (start_int..=end_int).map(|i| Value::Int(i)).collect()
                    } else {
                        (start_int..end_int).map(|i| Value::Int(i)).collect()
                    };
                    Ok(Value::Vec(vec))
                } else {
                    Err("Range bounds must be integers".to_string())
                }
            }
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
            }
            Literal::None => Ok(Value::None),
        }
    }

    /// Interpret a binary operation
    fn interpret_binary_operation(
        &self,
        left: &Value,
        op: &BinaryOperator,
        right: &Value,
    ) -> Result<Value, String> {
        debug_println!(
            "[DEBUG] interpret_binary_operation: left={:?}, op={:?}, right={:?}",
            left,
            op,
            right
        );
        let result = match op {
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
        };
        debug_println!("[DEBUG] interpret_binary_operation result: {:?}", result);
        result
    }

    /// Interpret a unary operation
    fn interpret_unary_operation(
        &self,
        op: &UnaryOperator,
        operand: &Value,
    ) -> Result<Value, String> {
        match op {
            UnaryOperator::Not => Ok(Value::Bool(!self.is_truthy(operand))),
            UnaryOperator::Negate => self.negate_value(operand),
            UnaryOperator::BitNot => Err("Bitwise operations not implemented".to_string()),
        }
    }

    /// Interpret a function call
    pub fn interpret_call(&mut self, callee: &Value, args: &[Value]) -> Result<Value, String> {
        debug_println!(
            "            [DEBUG] interpret_call: {:?}",
            std::mem::discriminant(callee)
        );
        match callee {
            Value::Function { name, body, .. } => {
                debug_println!("             Calling function: {}", name);

                // Check if it's a primitive method call
                debug_println!(
                    "              [DEBUG] Checking method call: name='{}', args.len()={}",
                    name,
                    args.len()
                );
                if name.starts_with("primitive_method::") {
                    debug_println!("               Handling primitive method call: {}", name);
                    // This is a primitive method call, execute using the primitive methods module
                    let method_name = name.strip_prefix("primitive_method::").unwrap_or(name);
                    let result = crate::primitive_methods::execute_primitive_method(
                        &Value::None,
                        method_name,
                        args,
                    );
                    debug_println!(
                        "              [DEBUG] Primitive method call result: {:?}",
                        result
                    );
                    return result;
                }

                // First check if it's a stdlib function
                if let Some(native_func) = self.stdlib.get_function(name) {
                    debug_println!(
                        "               Calling stdlib function: {} with args: {:?}",
                        name,
                        args
                    );
                    let result = native_func(self, args);
                    debug_println!("              [DEBUG] Stdlib function result: {:?}", result);
                    return result;
                }

                debug_println!("              [DEBUG] Looking up user function: {}", name);
                debug_println!(
                    "               Available functions: {:?}",
                    self.functions.keys().collect::<Vec<_>>()
                );
                // Look up the actual function declaration
                if let Some(func_decl) = self.functions.get(name) {
                    debug_println!(
                        "              [DEBUG] Found function declaration for: {}",
                        name
                    );
                    debug_println!("               Function params: {:?}", func_decl.params);
                    debug_println!("               Function body: {:?}", func_decl.body);
                    // Clone the function declaration to avoid borrow conflicts
                    let func_decl = func_decl.clone();

                    debug_println!(
                        "               Creating new environment with {} params",
                        func_decl.params.len()
                    );
                    // Create new environment with parameters that has access to global scope
                    let mut new_env = Environment::with_parent(self.environment.clone());
                    for (param, arg) in func_decl.params.iter().zip(args.iter()) {
                        debug_println!("                 Binding param {} = {:?}", param.name, arg);
                        new_env.define(param.name.clone(), arg.clone());
                    }

                    debug_println!("               Executing function body...");
                    debug_println!("              [DEBUG] Function body: {:?}", func_decl.body);
                    // Save current environment and switch to new one
                    let old_env = std::mem::replace(&mut self.environment, new_env);
                    let result = self.interpret_block(&func_decl.body);
                    self.environment = old_env;
                    debug_println!(
                        "              [DEBUG] Function {} completed with result: {:?}",
                        name,
                        result
                    );
                    result
                } else {
                    debug_println!("               Function '{}' not found", name);
                    debug_println!(
                        "               Available functions: {:?}",
                        self.functions.keys().collect::<Vec<_>>()
                    );
                    Err(format!("Function '{}' not found", name))
                }
            }
            Value::Closure { params, body, .. } => {
                debug_println!("             Calling closure with {} params", params.len());
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
            }
            _ => {
                debug_println!("             Cannot call non-function value: {:?}", callee);
                Err("Cannot call non-function value".to_string())
            }
        }
    }

    /// Interpret member access
    fn interpret_member_access(&self, target: &Value, member: &str) -> Result<Value, String> {
        debug_println!(
            "[DEBUG] DEBUG: interpret_member_access called: target={:?}, member={}",
            target,
            member
        );
        match target {
            Value::Struct { fields, name } => {
                debug_println!(
                    "[DEBUG] DEBUG: Found struct '{}' with fields: {:?}",
                    name,
                    fields.keys().collect::<Vec<_>>()
                );
                if let Some(field_value) = fields.get(member) {
                    debug_println!(
                        "[DEBUG] DEBUG: Found field '{}' in struct '{}'",
                        member,
                        name
                    );
                    Ok(field_value.clone())
                } else {
                    debug_println!(
                        "[DEBUG] DEBUG: Field '{}' not found in struct '{}'",
                        member,
                        name
                    );
                    Err(format!("No field '{}' found", member))
                }
            }
            // Handle method calls on primitive values
            _ => {
                debug_println!("[DEBUG] DEBUG: Object is not a struct, trying primitive methods");
                self.get_primitive_method(target, member)
            }
        }
    }

    /// Get a method for a primitive value
    fn get_primitive_method(&self, target: &Value, method: &str) -> Result<Value, String> {
        debug_println!(
            " get_primitive_method called: target={:?}, method={}",
            std::mem::discriminant(target),
            method
        );

        // For methods that don't require arguments, execute them directly
        match method {
            "to_string" => Ok(Value::String(target.to_string())),
            "clone" => Ok(target.clone()),
            "type_name" => Ok(Value::String(crate::primitive_methods::get_type_name(
                target,
            ))),
            "is_null" => Ok(Value::Bool(matches!(target, Value::None))),
            "is_not_null" => Ok(Value::Bool(!matches!(target, Value::None))),
            "hash" => Ok(Value::Int(crate::primitive_methods::get_hash_code(target))),
            "is_int" => Ok(Value::Bool(matches!(target, Value::Int(_)))),
            "is_float" => Ok(Value::Bool(matches!(target, Value::Float(_)))),
            "is_bool" => Ok(Value::Bool(matches!(target, Value::Bool(_)))),
            "is_str" => Ok(Value::Bool(matches!(target, Value::String(_)))),
            "is_none" => Ok(Value::Bool(matches!(target, Value::None))),
            "is_tuple" => Ok(Value::Bool(matches!(target, Value::Tuple(_)))),
            "debug_string" => Ok(Value::String(format!("{:?}", target))),
            "pretty_string" => Ok(Value::String(crate::primitive_methods::get_pretty_string(
                target,
            ))),
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
            Value::Union { .. } => "union",
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
            }
            (Value::Tuple(tuple), Value::Int(idx)) => {
                if *idx >= 0 && (*idx as usize) < tuple.len() {
                    Ok(tuple[*idx as usize].clone())
                } else {
                    Err("Index out of bounds".to_string())
                }
            }
            _ => Err("Cannot index non-vector/tuple value or invalid index type".to_string()),
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
            (
                Value::Struct {
                    name: v_name,
                    fields: v_fields,
                },
                Pattern::Struct {
                    name: p_name,
                    fields: p_fields,
                    ..
                },
            ) => {
                v_name == p_name
                    && p_fields.len() == v_fields.len()
                    && p_fields.iter().all(|(p_field, p_value)| {
                        v_fields
                            .get(p_field)
                            .map(|v_value| self.pattern_matches(v_value, p_value))
                            .unwrap_or(false)
                    })
            }
            (_, Pattern::Variable { .. }) => true, // Variables always match
            (_, Pattern::Wildcard(_)) => true,
            _ => false,
        }
    }

    /// Interpret a block
    fn interpret_block(&mut self, block: &Block) -> Result<Value, String> {
        debug_println!(
            "                 interpret_block: {} statements",
            block.statements.len()
        );
        let mut result = Value::None;
        for (i, stmt) in block.statements.iter().enumerate() {
            debug_println!(
                "                   Statement {}: {:?}",
                i,
                std::mem::discriminant(stmt)
            );
            debug_println!(
                "                  [DEBUG] Statement {} details: {:?}",
                i,
                stmt
            );
            result = self.interpret_statement(stmt)?;
            debug_println!(
                "                  [DEBUG] Statement {} result: {:?}",
                i,
                result
            );
        }
        debug_println!("                 Block completed with result: {:?}", result);
        Ok(result)
    }

    /// Interpret a statement
    fn interpret_statement(&mut self, stmt: &Statement) -> Result<Value, String> {
        debug_println!(
            "                    [DEBUG] interpret_statement: {:?}",
            std::mem::discriminant(stmt)
        );
        match stmt {
            Statement::Expression(expr) => {
                debug_println!("                       Expression statement");
                self.interpret_expression(expr)
            }
            Statement::Variable(var) => {
                debug_println!("                       Variable statement: {}", var.name);
                let value = self.interpret_expression(&var.value)?;
                self.environment.define(var.name.clone(), value.clone());
                Ok(value)
            }
            Statement::Return(ret_stmt) => {
                debug_println!("                       Return statement");
                if let Some(expr) = &ret_stmt.value {
                    self.interpret_expression(expr)
                } else {
                    Ok(Value::None)
                }
            }
            Statement::If(if_stmt) => {
                let condition_val = self.interpret_expression(&if_stmt.condition)?;
                debug_println!(
                    "[DEBUG] IF: condition = {:?}, is_truthy = {}",
                    condition_val,
                    self.is_truthy(&condition_val)
                );
                debug_println!(
                    "[DEBUG] IF: elif_branches = {}, else_block = {}",
                    if_stmt.elif_branches.len(),
                    if_stmt.else_block.is_some()
                );

                if self.is_truthy(&condition_val) {
                    debug_println!("[DEBUG] IF: executing then_block");
                    self.interpret_block(&if_stmt.then_block)
                } else {
                    debug_println!("[DEBUG] IF: condition false, checking elif branches");
                    // Check elif branches
                    let mut executed = false;
                    for elif_branch in &if_stmt.elif_branches {
                        let elif_condition_val =
                            self.interpret_expression(&elif_branch.condition)?;
                        debug_println!(
                            "[DEBUG] IF: elif condition = {:?}, is_truthy = {}",
                            elif_condition_val,
                            self.is_truthy(&elif_condition_val)
                        );
                        if self.is_truthy(&elif_condition_val) {
                            executed = true;
                            debug_println!("[DEBUG] IF: executing elif block");
                            return self.interpret_block(&elif_branch.block);
                        }
                    }

                    // If no elif branch was executed, check else block
                    if !executed {
                        if let Some(else_block) = &if_stmt.else_block {
                            debug_println!("[DEBUG] IF: executing else block");
                            self.interpret_block(else_block)
                        } else {
                            debug_println!("[DEBUG] IF: no else block, returning None");
                            Ok(Value::None)
                        }
                    } else {
                        debug_println!("[DEBUG] IF: elif executed, returning None");
                        Ok(Value::None)
                    }
                }
            }
            Statement::While(while_stmt) => {
                loop {
                    let condition_val = self.interpret_expression(&while_stmt.condition)?;
                    if !self.is_truthy(&condition_val) {
                        break;
                    }
                    self.interpret_block(&while_stmt.body)?;
                }
                Ok(Value::None)
            }
            Statement::DoWhile(do_while_stmt) => {
                loop {
                    self.interpret_block(&do_while_stmt.body)?;
                    let condition_val = self.interpret_expression(&do_while_stmt.condition)?;
                    if !self.is_truthy(&condition_val) {
                        break;
                    }
                }
                Ok(Value::None)
            }
            Statement::For(for_stmt) => {
                match for_stmt {
                    ForStatement::ForEach {
                        var_name,
                        iterable,
                        body,
                        ..
                    } => {
                        debug_println!("[DEBUG] FOR_LOOP: iterable expression = {:?}", iterable);
                        // First, check if it's a Range expression and convert to vector
                        let iter_val = if let Expression::Range {
                            start,
                            end,
                            inclusive,
                            ..
                        } = iterable
                        {
                            debug_println!("[DEBUG] FOR_LOOP: Detected Range expression, start={:?}, end={:?}, inclusive={}", start, end, inclusive);
                            let start_val = self.interpret_expression(start)?;
                            let end_val = self.interpret_expression(end)?;

                            if let (Value::Int(start_int), Value::Int(end_int)) =
                                (start_val, end_val)
                            {
                                let vec: Vec<Value> = if *inclusive {
                                    (start_int..=end_int).map(|i| Value::Int(i)).collect()
                                } else {
                                    (start_int..end_int).map(|i| Value::Int(i)).collect()
                                };
                                Value::Vec(vec)
                            } else {
                                return Err("Range bounds must be integers".to_string());
                            }
                        } else {
                            debug_println!(
                                "[DEBUG] FOR_LOOP: NOT a Range expression, evaluating iterable"
                            );
                            // Evaluate the iterable expression
                            self.interpret_expression(iterable)?
                        };

                        debug_println!("[DEBUG] FOR_LOOP: iter_val = {:?}", iter_val);
                        // Now iterate over the value
                        if let Value::Vec(vec) = iter_val {
                            debug_println!(
                                "[DEBUG] FOR_LOOP: Iterating over vector with {} items",
                                vec.len()
                            );
                            for item in vec {
                                self.environment.define(var_name.clone(), item);
                                self.interpret_block(body)?;
                            }
                        } else {
                            return Err(format!(
                                "Cannot iterate over value of type: {:?}",
                                iter_val
                            ));
                        }
                    }
                    ForStatement::CStyle {
                        initializer,
                        condition,
                        increment,
                        body,
                        ..
                    } => {
                        debug_println!("[DEBUG] FOR_LOOP: C-style loop");

                        // Execute initializer if present
                        if let Some(init_stmt) = initializer {
                            self.interpret_statement(init_stmt)?;
                        }

                        // Loop while condition is true (or forever if no condition)
                        loop {
                            // Check condition if present
                            if let Some(cond_expr) = condition {
                                let cond_val = self.interpret_expression(cond_expr)?;
                                if !self.is_truthy(&cond_val) {
                                    break;
                                }
                            }

                            // Execute body
                            self.interpret_block(body)?;

                            // Execute increment if present
                            if let Some(inc_expr) = increment {
                                self.interpret_expression(inc_expr)?;
                            }
                        }
                    }
                }
                Ok(Value::None)
            }
            Statement::Block(block) => self.interpret_block(block),
            _ => Ok(Value::None),
        }
    }

    /// Helper methods for operations
    fn add_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        // Handle union types by unwrapping them
        let left_unwrapped = left.unwrap_union();
        let right_unwrapped = right.unwrap_union();
        
        match (left_unwrapped, right_unwrapped) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err("Cannot add these types".to_string()),
        }
    }

    fn subtract_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        // Handle union types by unwrapping them
        let left_unwrapped = left.unwrap_union();
        let right_unwrapped = right.unwrap_union();
        
        match (left_unwrapped, right_unwrapped) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err("Cannot subtract these types".to_string()),
        }
    }

    fn multiply_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        // Handle union types by unwrapping them
        let left_unwrapped = left.unwrap_union();
        let right_unwrapped = right.unwrap_union();
        
        match (left_unwrapped, right_unwrapped) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err("Cannot multiply these types".to_string()),
        }
    }

    fn divide_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        // Handle union types by unwrapping them
        let left_unwrapped = left.unwrap_union();
        let right_unwrapped = right.unwrap_union();
        
        match (left_unwrapped, right_unwrapped) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / *b as f64))
                }
            }
            _ => Err("Cannot divide these types".to_string()),
        }
    }

    fn modulo_values(&self, left: &Value, right: &Value) -> Result<Value, String> {
        // Handle union types by unwrapping them
        let left_unwrapped = left.unwrap_union();
        let right_unwrapped = right.unwrap_union();
        
        match (left_unwrapped, right_unwrapped) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            _ => Err("Cannot modulo these types".to_string()),
        }
    }

    fn compare_values<F>(&self, left: &Value, right: &Value, cmp: F) -> Result<Value, String>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        // Handle union types by unwrapping them
        let left_unwrapped = left.unwrap_union();
        let right_unwrapped = right.unwrap_union();
        
        match (left_unwrapped, right_unwrapped) {
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
