//! TJLang Virtual Machine
//! 
//! Advanced VM with stack, heap, and concurrency support.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::values::Value;
use crate::bytecode::{Bytecode, Instruction, Function};
use crate::gc::GarbageCollector;
use crate::concurrency::ConcurrencyRuntime;

/// Virtual machine state
pub struct VirtualMachine {
    /// Stack for local variables and temporary values
    stack: Vec<Value>,
    
    /// Local variables in current scope
    locals: Vec<Value>,
    
    /// Global variables
    globals: HashMap<String, Value>,
    
    /// Function definitions
    functions: HashMap<String, Function>,
    
    /// Program counter
    pc: usize,
    
    /// Call stack for function calls
    call_stack: Vec<CallFrame>,
    
    /// Garbage collector
    gc: GarbageCollector,
    
    /// Concurrency runtime
    concurrency: ConcurrencyRuntime,
    
    /// Bytecode program
    bytecode: Arc<Bytecode>,
    
    /// Running flag
    running: bool,
}

/// Call frame for function execution
#[derive(Debug, Clone)]
struct CallFrame {
    pub function_name: String,
    pub return_address: usize,
    pub locals: Vec<Value>,
    pub stack_base: usize,
}

impl VirtualMachine {
    /// Create a new virtual machine
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            stack: Vec::new(),
            locals: Vec::new(),
            globals: HashMap::new(),
            functions: HashMap::new(),
            pc: 0,
            call_stack: Vec::new(),
            gc: GarbageCollector::new(),
            concurrency: ConcurrencyRuntime::new(),
            bytecode: Arc::new(bytecode),
            running: false,
        }
    }
    
    /// Run the virtual machine
    pub fn run(&mut self) -> Result<Value, String> {
        self.running = true;
        
        // Initialize functions
        for function in &self.bytecode.functions {
            self.functions.insert(function.name.clone(), function.clone());
        }
        
        while self.running && self.pc < self.bytecode.instructions.len() {
            let instruction = self.bytecode.instructions[self.pc].clone();
            self.execute_instruction(&instruction)?;
            self.pc += 1;
        }
        
        // Return the top of the stack
        Ok(self.stack.pop().unwrap_or(Value::None))
    }
    
    /// Execute a single instruction
    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), String> {
        match instruction {
            // Stack operations
            Instruction::Push(value) => {
                self.stack.push(value.clone());
            },
            Instruction::Pop => {
                self.stack.pop();
            },
            Instruction::Dup => {
                if let Some(value) = self.stack.last() {
                    self.stack.push(value.clone());
                }
            },
            Instruction::Swap => {
                if self.stack.len() >= 2 {
                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                }
            },
            
            // Arithmetic operations
            Instruction::Add => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.add_values(a, b)?;
                self.stack.push(result);
            },
            Instruction::Subtract => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.subtract_values(a, b)?;
                self.stack.push(result);
            },
            Instruction::Multiply => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.multiply_values(a, b)?;
                self.stack.push(result);
            },
            Instruction::Divide => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.divide_values(a, b)?;
                self.stack.push(result);
            },
            Instruction::Modulo => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.modulo_values(a, b)?;
                self.stack.push(result);
            },
            Instruction::Power => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.power_values(a, b)?;
                self.stack.push(result);
            },
            
            // Comparison operations
            Instruction::Equal => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                self.stack.push(Value::Bool(a == b));
            },
            Instruction::NotEqual => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                self.stack.push(Value::Bool(a != b));
            },
            Instruction::LessThan => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.compare_values(a, b, |a, b| a < b)?;
                self.stack.push(Value::Bool(result));
            },
            Instruction::GreaterThan => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.compare_values(a, b, |a, b| a > b)?;
                self.stack.push(Value::Bool(result));
            },
            Instruction::LessThanEqual => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.compare_values(a, b, |a, b| a <= b)?;
                self.stack.push(Value::Bool(result));
            },
            Instruction::GreaterThanEqual => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = self.compare_values(a, b, |a, b| a >= b)?;
                self.stack.push(Value::Bool(result));
            },
            
            // Logical operations
            Instruction::And => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = a.is_truthy() && b.is_truthy();
                self.stack.push(Value::Bool(result));
            },
            Instruction::Or => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = a.is_truthy() || b.is_truthy();
                self.stack.push(Value::Bool(result));
            },
            Instruction::Not => {
                let a = self.pop_value()?;
                self.stack.push(Value::Bool(!a.is_truthy()));
            },
            
            // Variable operations
            Instruction::Load(name) => {
                if let Some(value) = self.globals.get(name) {
                    self.stack.push(value.clone());
                } else {
                    return Err(format!("Undefined variable: {}", name));
                }
            },
            Instruction::Store(name) => {
                if let Some(value) = self.stack.pop() {
                    self.globals.insert(name.clone(), value);
                }
            },
            Instruction::LoadLocal(index) => {
                if let Some(value) = self.locals.get(*index) {
                    self.stack.push(value.clone());
                } else {
                    return Err(format!("Undefined local variable at index {}", index));
                }
            },
            Instruction::StoreLocal(index) => {
                if let Some(value) = self.stack.pop() {
                    while self.locals.len() <= *index {
                        self.locals.push(Value::None);
                    }
                    self.locals[*index] = value;
                }
            },
            
            // Function operations
            Instruction::Call(name, arg_count) => {
                if let Some(function) = self.functions.get(name).cloned() {
                    self.call_function(&function, *arg_count)?;
                } else {
                    return Err(format!("Undefined function: {}", name));
                }
            },
            Instruction::CallClosure(arg_count) => {
                // TODO: Implement closure calls
                return Err("Closure calls not yet implemented".to_string());
            },
            Instruction::Return => {
                if let Some(frame) = self.call_stack.pop() {
                    self.pc = frame.return_address;
                    self.locals = frame.locals;
                    // Restore stack to base
                    while self.stack.len() > frame.stack_base {
                        self.stack.pop();
                    }
                } else {
                    self.running = false;
                }
            },
            
            // Control flow
            Instruction::Jump(address) => {
                self.pc = *address - 1; // -1 because pc will be incremented
            },
            Instruction::JumpIf(address) => {
                if let Some(value) = self.stack.pop() {
                    if value.is_truthy() {
                        self.pc = *address - 1;
                    }
                }
            },
            Instruction::JumpIfNot(address) => {
                if let Some(value) = self.stack.pop() {
                    if !value.is_truthy() {
                        self.pc = *address - 1;
                    }
                }
            },
            
            // Concurrency
            Instruction::Spawn => {
                let task = self.concurrency.spawn_task()?;
                self.stack.push(Value::Task { id: task.id, handle: std::thread::spawn(|| Value::None) });
            },
            Instruction::Send => {
                // TODO: Implement channel send
            },
            Instruction::Receive => {
                // TODO: Implement channel receive
            },
            Instruction::Yield => {
                thread::yield_now();
            },
            Instruction::Join => {
                if let Some(Value::Task { handle, .. }) = self.stack.pop() {
                    match handle.join() {
                        Ok(result) => self.stack.push(result),
                        Err(_) => return Err("Task join failed".to_string()),
                    }
                }
            },
            
            // Debug operations
            Instruction::DebugPrint => {
                if let Some(value) = self.stack.last() {
                    println!("{}", value.to_string());
                }
            },
            Instruction::DebugStack => {
                println!("Stack: {:?}", self.stack);
            },
            Instruction::DebugHeap => {
                println!("Heap: {} objects", self.gc.object_count());
            },
            Instruction::Breakpoint => {
                println!("Breakpoint hit at PC: {}", self.pc);
            },
            
            // Special operations
            Instruction::Halt => {
                self.running = false;
            },
            Instruction::Nop => {
                // Do nothing
            },
            
            _ => {
                return Err(format!("Unimplemented instruction: {:?}", instruction));
            }
        }
        
        Ok(())
    }
    
    /// Pop a value from the stack
    fn pop_value(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or("Stack underflow".to_string())
    }
    
    /// Call a function
    fn call_function(&mut self, function: &Function, arg_count: usize) -> Result<(), String> {
        if arg_count != function.params.len() {
            return Err(format!(
                "Function {} expects {} arguments, got {}",
                function.name, function.params.len(), arg_count
            ));
        }
        
        // Save current state
        let return_address = self.pc + 1;
        let stack_base = self.stack.len() - arg_count;
        let current_locals = std::mem::take(&mut self.locals);
        
        // Set up new locals
        self.locals = vec![Value::None; function.local_count];
        
        // Copy arguments to locals
        for (i, param) in function.params.iter().enumerate() {
            if let Some(value) = self.stack.get(stack_base + i) {
                self.locals[i] = value.clone();
            }
        }
        
        // Push call frame
        self.call_stack.push(CallFrame {
            function_name: function.name.clone(),
            return_address,
            locals: current_locals,
            stack_base,
        });
        
        // Jump to function
        self.pc = function.start_address - 1; // -1 because pc will be incremented
        
        Ok(())
    }
    
    /// Add two values
    fn add_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            _ => Err("Cannot add these types".to_string()),
        }
    }
    
    /// Subtract two values
    fn subtract_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
            _ => Err("Cannot subtract these types".to_string()),
        }
    }
    
    /// Multiply two values
    fn multiply_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
            _ => Err("Cannot multiply these types".to_string()),
        }
    }
    
    /// Divide two values
    fn divide_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Int(a / b))
                }
            },
            (Value::Float(a), Value::Float(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            },
            (Value::Int(a), Value::Float(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a as f64 / b))
                }
            },
            (Value::Float(a), Value::Int(b)) => {
                if b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b as f64))
                }
            },
            _ => Err("Cannot divide these types".to_string()),
        }
    }
    
    /// Modulo two values
    fn modulo_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Int(a % b))
                }
            },
            _ => Err("Cannot modulo these types".to_string()),
        }
    }
    
    /// Power of two values
    fn power_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.pow(b as u32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f64).powf(b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powi(b as i32))),
            _ => Err("Cannot compute power of these types".to_string()),
        }
    }
    
    /// Compare two values
    fn compare_values<F>(&self, a: Value, b: Value, cmp: F) -> Result<bool, String>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(cmp(a as f64, b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(cmp(a, b)),
            (Value::Int(a), Value::Float(b)) => Ok(cmp(a as f64, b)),
            (Value::Float(a), Value::Int(b)) => Ok(cmp(a, b as f64)),
            _ => Err("Cannot compare these types".to_string()),
        }
    }
}
