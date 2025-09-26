//! TJLang Bytecode
//! 
//! Advanced bytecode instruction set for the TJLang VM.

use crate::values::Value;
use tjlang_ast::{Expression, Type, SourceSpan};

/// Bytecode instruction set
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack operations
    Push(Value),
    Pop,
    Dup,
    Swap,
    
    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    
    // Comparison operations
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    
    // Logical operations
    And,
    Or,
    Not,
    
    // Bitwise operations
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    ShiftLeft,
    ShiftRight,
    
    // Variable operations
    Load(String),
    Store(String),
    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(String),
    StoreGlobal(String),
    
    // Function operations
    Call(String, usize), // function name, arg count
    CallClosure(usize), // arg count
    Return,
    TailCall(String, usize),
    
    // Control flow
    Jump(usize), // absolute address
    JumpIf(usize), // jump if top of stack is truthy
    JumpIfNot(usize), // jump if top of stack is falsy
    
    // Pattern matching
    Match(usize), // number of arms
    MatchArm(usize), // arm index
    MatchGuard,
    MatchBind(String),
    MatchDestruct(String, usize), // struct name, field count
    
    // Concurrency
    Spawn,
    Send,
    Receive,
    Yield,
    Join,
    
    // Data structure operations
    NewStruct(String, usize), // struct name, field count
    GetField(String),
    SetField(String),
    NewEnum(String, String, usize), // enum name, variant, field count
    NewTuple(usize),
    NewVec(usize),
    NewSet(usize),
    NewMap(usize),
    VecPush,
    VecPop,
    VecGet(usize),
    VecSet(usize),
    SetAdd,
    SetRemove,
    SetContains,
    MapGet,
    MapSet,
    MapRemove,
    MapContains,
    
    // Type operations
    TypeCheck(Type),
    TypeCast(Type),
    IsType(Type),
    
    // Memory operations
    Alloc(usize), // size in bytes
    Dealloc,
    GcMark,
    GcSweep,
    
    // Debug operations
    DebugPrint,
    DebugStack,
    DebugHeap,
    Breakpoint,
    
    // Special operations
    Halt,
    Nop,
}

/// A complete bytecode program
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub functions: Vec<Function>,
    pub globals: Vec<String>,
}

/// Function definition in bytecode
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub param_types: Vec<Type>,
    pub return_type: Type,
    pub start_address: usize,
    pub local_count: usize,
    pub is_closure: bool,
}

impl Bytecode {
    /// Create a new bytecode program
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            functions: Vec::new(),
            globals: Vec::new(),
        }
    }
    
    /// Add an instruction to the program
    pub fn add_instruction(&mut self, instruction: Instruction) -> usize {
        let address = self.instructions.len();
        self.instructions.push(instruction);
        address
    }
    
    /// Add a constant value
    pub fn add_constant(&mut self, value: Value) -> usize {
        let index = self.constants.len();
        self.constants.push(value);
        index
    }
    
    /// Add a function definition
    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }
    
    /// Get an instruction at a specific address
    pub fn get_instruction(&self, address: usize) -> Option<&Instruction> {
        self.instructions.get(address)
    }
    
    /// Get a constant by index
    pub fn get_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }
    
    /// Find a function by name
    pub fn find_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }
    
    /// Get the total instruction count
    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }
}

impl Default for Bytecode {
    fn default() -> Self {
        Self::new()
    }
}

/// Bytecode compiler that converts AST to bytecode
pub struct BytecodeCompiler {
    bytecode: Bytecode,
    current_function: Option<String>,
    local_vars: Vec<String>,
    label_counter: usize,
}

impl BytecodeCompiler {
    /// Create a new bytecode compiler
    pub fn new() -> Self {
        Self {
            bytecode: Bytecode::new(),
            current_function: None,
            local_vars: Vec::new(),
            label_counter: 0,
        }
    }
    
    /// Compile an expression to bytecode
    pub fn compile_expression(&mut self, expr: &Expression) -> Result<(), String> {
        match expr {
            Expression::Literal(literal) => {
                let value = self.compile_literal(literal);
                self.bytecode.add_instruction(Instruction::Push(value));
            },
            Expression::Variable(name) => {
                if let Some(index) = self.local_vars.iter().position(|v| v == name) {
                    self.bytecode.add_instruction(Instruction::LoadLocal(index));
                } else {
                    self.bytecode.add_instruction(Instruction::Load(name.clone()));
                }
            },
            Expression::Binary { left, operator, right, .. } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                self.compile_binary_operator(operator);
            },
            Expression::Unary { operator, operand, .. } => {
                self.compile_expression(operand)?;
                self.compile_unary_operator(operator);
            },
            Expression::Call { callee, args, .. } => {
                for arg in args {
                    self.compile_expression(arg)?;
                }
                match callee.as_ref() {
                    Expression::Variable(name) => {
                        self.bytecode.add_instruction(Instruction::Call(name.clone(), args.len()));
                    },
                    _ => {
                        self.bytecode.add_instruction(Instruction::CallClosure(args.len()));
                    }
                }
            },
            Expression::If { condition, then_expr, else_expr, .. } => {
                self.compile_expression(condition)?;
                let jump_if_false = self.bytecode.add_instruction(Instruction::JumpIfNot(0)); // placeholder
                self.compile_expression(then_expr)?;
                let jump_end = self.bytecode.add_instruction(Instruction::Jump(0)); // placeholder
                
                // Update the jump address
                let else_start = self.bytecode.instruction_count();
                self.bytecode.instructions[jump_if_false] = Instruction::JumpIfNot(else_start);
                
                self.compile_expression(else_expr)?;
                let end_address = self.bytecode.instruction_count();
                self.bytecode.instructions[jump_end] = Instruction::Jump(end_address);
            },
            Expression::Match { expression, arms, .. } => {
                self.compile_expression(expression)?;
                self.bytecode.add_instruction(Instruction::Match(arms.len()));
                
                for (i, arm) in arms.iter().enumerate() {
                    self.compile_pattern(&arm.pattern)?;
                    if let Some(guard) = &arm.guard {
                        self.compile_expression(guard)?;
                        self.bytecode.add_instruction(Instruction::MatchGuard);
                    }
                    self.bytecode.add_instruction(Instruction::MatchArm(i));
                }
            },
            Expression::Spawn { expression, .. } => {
                self.compile_expression(expression)?;
                self.bytecode.add_instruction(Instruction::Spawn);
            },
            Expression::StructLiteral { name, fields, .. } => {
                for field in fields {
                    self.compile_expression(&field.value)?;
                }
                self.bytecode.add_instruction(Instruction::NewStruct(name.clone(), fields.len()));
            },
            Expression::TupleLiteral { elements, .. } => {
                for element in elements {
                    self.compile_expression(element)?;
                }
                self.bytecode.add_instruction(Instruction::NewTuple(elements.len()));
            },
            Expression::VecLiteral { elements, .. } => {
                for element in elements {
                    self.compile_expression(element)?;
                }
                self.bytecode.add_instruction(Instruction::NewVec(elements.len()));
            },
            Expression::SetLiteral { elements, .. } => {
                for element in elements {
                    self.compile_expression(element)?;
                }
                self.bytecode.add_instruction(Instruction::NewSet(elements.len()));
            },
            Expression::MapLiteral { entries, .. } => {
                for entry in entries {
                    self.compile_expression(&entry.key)?;
                    self.compile_expression(&entry.value)?;
                }
                self.bytecode.add_instruction(Instruction::NewMap(entries.len()));
            },
            Expression::Lambda { params, body, .. } => {
                // TODO: Implement closure compilation
                self.compile_expression(body)?;
            },
            Expression::Range { start, end, inclusive, .. } => {
                self.compile_expression(start)?;
                self.compile_expression(end)?;
                if *inclusive {
                    self.bytecode.add_instruction(Instruction::Push(Value::Bool(true)));
                } else {
                    self.bytecode.add_instruction(Instruction::Push(Value::Bool(false)));
                }
                // TODO: Implement range creation
            },
            Expression::Index { target, index, .. } => {
                self.compile_expression(target)?;
                self.compile_expression(index)?;
                // TODO: Implement indexing
            },
            Expression::Member { target, member, .. } => {
                self.compile_expression(target)?;
                self.bytecode.add_instruction(Instruction::GetField(member.clone()));
            },
        }
        Ok(())
    }
    
    fn compile_literal(&self, literal: &tjlang_ast::Literal) -> Value {
        match literal {
            tjlang_ast::Literal::Int(i) => Value::Int(*i),
            tjlang_ast::Literal::Float(f) => Value::Float(*f),
            tjlang_ast::Literal::String(s) => Value::String(s.clone()),
            tjlang_ast::Literal::FString(s) => Value::String(s.clone()),
            tjlang_ast::Literal::FStringInterpolation(parts) => {
                // TODO: Implement f-string interpolation
                Value::String("".to_string())
            },
            tjlang_ast::Literal::Bool(b) => Value::Bool(*b),
            tjlang_ast::Literal::None => Value::None,
        }
    }
    
    fn compile_binary_operator(&mut self, operator: &tjlang_ast::BinaryOperator) -> usize {
        match operator {
            tjlang_ast::BinaryOperator::Add => self.bytecode.add_instruction(Instruction::Add),
            tjlang_ast::BinaryOperator::Subtract => self.bytecode.add_instruction(Instruction::Subtract),
            tjlang_ast::BinaryOperator::Multiply => self.bytecode.add_instruction(Instruction::Multiply),
            tjlang_ast::BinaryOperator::Divide => self.bytecode.add_instruction(Instruction::Divide),
            tjlang_ast::BinaryOperator::Modulo => self.bytecode.add_instruction(Instruction::Modulo),
            tjlang_ast::BinaryOperator::Power => self.bytecode.add_instruction(Instruction::Power),
            tjlang_ast::BinaryOperator::Equal => self.bytecode.add_instruction(Instruction::Equal),
            tjlang_ast::BinaryOperator::NotEqual => self.bytecode.add_instruction(Instruction::NotEqual),
            tjlang_ast::BinaryOperator::LessThan => self.bytecode.add_instruction(Instruction::LessThan),
            tjlang_ast::BinaryOperator::GreaterThan => self.bytecode.add_instruction(Instruction::GreaterThan),
            tjlang_ast::BinaryOperator::LessThanEqual => self.bytecode.add_instruction(Instruction::LessThanEqual),
            tjlang_ast::BinaryOperator::GreaterThanEqual => self.bytecode.add_instruction(Instruction::GreaterThanEqual),
            tjlang_ast::BinaryOperator::And => self.bytecode.add_instruction(Instruction::And),
            tjlang_ast::BinaryOperator::Or => self.bytecode.add_instruction(Instruction::Or),
            tjlang_ast::BinaryOperator::Assign => {
                // TODO: Implement assignment
                0
            },
            _ => {
                // TODO: Implement other operators
                0
            }
        }
    }
    
    fn compile_unary_operator(&mut self, operator: &tjlang_ast::UnaryOperator) -> usize {
        match operator {
            tjlang_ast::UnaryOperator::Negate => {
                self.bytecode.add_instruction(Instruction::Push(Value::Int(0)));
                self.bytecode.add_instruction(Instruction::Subtract);
                0
            },
            tjlang_ast::UnaryOperator::Not => self.bytecode.add_instruction(Instruction::Not),
            tjlang_ast::UnaryOperator::BitNot => self.bytecode.add_instruction(Instruction::BitNot),
        }
    }
    
    fn compile_pattern(&mut self, pattern: &tjlang_ast::Pattern) -> Result<(), String> {
        match pattern {
            tjlang_ast::Pattern::Literal(literal) => {
                let value = self.compile_literal(literal);
                self.bytecode.add_instruction(Instruction::Push(value));
                self.bytecode.add_instruction(Instruction::Equal);
            },
            tjlang_ast::Pattern::Variable { name, .. } => {
                self.bytecode.add_instruction(Instruction::MatchBind(name.clone()));
            },
            tjlang_ast::Pattern::Constructor { name, fields, .. } => {
                for field in fields {
                    self.compile_pattern(field)?;
                }
                self.bytecode.add_instruction(Instruction::NewEnum(name.clone(), "".to_string(), fields.len()));
            },
            tjlang_ast::Pattern::Struct { name, fields, .. } => {
                for (field_name, field_pattern) in fields {
                    self.compile_pattern(field_pattern)?;
                }
                self.bytecode.add_instruction(Instruction::NewStruct(name.clone(), fields.len()));
            },
            tjlang_ast::Pattern::Tuple { patterns, .. } => {
                for pattern in patterns {
                    self.compile_pattern(pattern)?;
                }
                self.bytecode.add_instruction(Instruction::NewTuple(patterns.len()));
            },
            tjlang_ast::Pattern::Wildcard(_) => {
                // Wildcard matches anything, no instruction needed
            },
            _ => {
                // TODO: Implement other pattern types
            }
        }
        Ok(())
    }
    
    /// Get the compiled bytecode
    pub fn into_bytecode(self) -> Bytecode {
        self.bytecode
    }
}
