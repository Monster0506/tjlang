//! TJLang Standard Library Integration
//!
//! This module provides integration between the TJLang standard library
//! and the runtime interpreter, allowing TJLang code to call stdlib functions.

use crate::values::Value;
use crate::interpreter::Interpreter;
use std::collections::HashMap;
use tjlang_diagnostics::debug_println;

/// Native function registry for standard library functions
pub struct StdlibRegistry {
    functions: HashMap<String, NativeFunction>,
}

/// Native function type that can be called from TJLang code
pub type NativeFunction = fn(&mut Interpreter, &[Value]) -> Result<Value, String>;

impl StdlibRegistry {
    pub fn new() -> Self {
        debug_println!("🔧 Creating stdlib registry...");
        let mut registry = Self {
            functions: HashMap::new(),
        };
        registry.register_stdlib_functions();
        debug_println!("🔧 Stdlib registry created (functions enabled)");
        registry
    }
    
    /// Register all standard library functions
    fn register_stdlib_functions(&mut self) {
        // IO Module functions
        self.register_io_functions();
        
        // FILE Module functions
        self.register_file_functions();
        
        // MATH Module functions
        self.register_math_functions();
        
        // STRING Module functions
        self.register_string_functions();
        
        // COLLECTIONS Module functions
        self.register_collections_functions();
        
        // TIME Module functions
        self.register_time_functions();
        
        // ERROR Module functions
        self.register_error_functions();
        
        // TESTING Module functions
        self.register_testing_functions();
    }
    
    /// Register IO module functions
    fn register_io_functions(&mut self) {
        self.functions.insert("IO::print".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::print expects 1 argument".to_string());
            }
            crate::stdlib::io::IO::print(&args[0])
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("IO::println".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::println expects 1 argument".to_string());
            }
            crate::stdlib::io::IO::println(&args[0])
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("IO::read_line".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::read_line()
                .map(|s| Value::String(s))
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("IO::read_int".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::read_int()
                .map(|i| Value::Int(i))
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("IO::read_float".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::read_float()
                .map(|f| Value::Float(f))
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("IO::read_bool".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::read_bool()
                .map(|b| Value::Bool(b))
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("IO::clear_screen".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::clear_screen()
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("IO::is_terminal".to_string(), |_interpreter, _args| {
            Ok(Value::Bool(crate::stdlib::io::IO::is_terminal()))
        });
    }
    
    /// Register FILE module functions
    fn register_file_functions(&mut self) {
        self.functions.insert("FILE::read_to_string".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("FILE::read_to_string expects 1 argument".to_string());
            }
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err("FILE::read_to_string expects string argument".to_string()),
            };
            crate::stdlib::file::FILE::read_to_string(path)
                .map(|s| Value::String(s))
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("FILE::write_string".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("FILE::write_string expects 2 arguments".to_string());
            }
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err("FILE::write_string expects string path".to_string()),
            };
            let content = match &args[1] {
                Value::String(s) => s,
                _ => return Err("FILE::write_string expects string content".to_string()),
            };
            crate::stdlib::file::FILE::write_string(path, content)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("FILE::exists".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("FILE::exists expects 1 argument".to_string());
            }
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err("FILE::exists expects string argument".to_string()),
            };
            Ok(Value::Bool(crate::stdlib::file::FILE::exists(path)))
        });
        
        self.functions.insert("FILE::is_file".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("FILE::is_file expects 1 argument".to_string());
            }
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err("FILE::is_file expects string argument".to_string()),
            };
            Ok(Value::Bool(crate::stdlib::file::FILE::is_file(path)))
        });
        
        self.functions.insert("FILE::is_dir".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("FILE::is_dir expects 1 argument".to_string());
            }
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err("FILE::is_dir expects string argument".to_string()),
            };
            Ok(Value::Bool(crate::stdlib::file::FILE::is_dir(path)))
        });
    }
    
    /// Register MATH module functions
    fn register_math_functions(&mut self) {
        // Basic arithmetic
        self.functions.insert("MATH::add".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("MATH::add expects 2 arguments".to_string());
            }
            let a = match &args[0] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::add expects numeric arguments".to_string()),
            };
            let b = match &args[1] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::add expects numeric arguments".to_string()),
            };
            Ok(Value::Float(crate::stdlib::math::MATH::add(a, b)))
        });
        
        self.functions.insert("MATH::subtract".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("MATH::subtract expects 2 arguments".to_string());
            }
            let a = match &args[0] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::subtract expects numeric arguments".to_string()),
            };
            let b = match &args[1] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::subtract expects numeric arguments".to_string()),
            };
            Ok(Value::Float(crate::stdlib::math::MATH::subtract(a, b)))
        });
        
        self.functions.insert("MATH::multiply".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("MATH::multiply expects 2 arguments".to_string());
            }
            let a = match &args[0] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::multiply expects numeric arguments".to_string()),
            };
            let b = match &args[1] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::multiply expects numeric arguments".to_string()),
            };
            Ok(Value::Float(crate::stdlib::math::MATH::multiply(a, b)))
        });
        
        self.functions.insert("MATH::divide".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("MATH::divide expects 2 arguments".to_string());
            }
            let a = match &args[0] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::divide expects numeric arguments".to_string()),
            };
            let b = match &args[1] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::divide expects numeric arguments".to_string()),
            };
            Ok(Value::Float(crate::stdlib::math::MATH::divide(a, b)))
        });
        
        self.functions.insert("MATH::sqrt".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("MATH::sqrt expects 1 argument".to_string());
            }
            let a = match &args[0] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::sqrt expects numeric argument".to_string()),
            };
            Ok(Value::Float(crate::stdlib::math::MATH::sqrt(a)))
        });
        
        self.functions.insert("MATH::sin".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("MATH::sin expects 1 argument".to_string());
            }
            let a = match &args[0] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::sin expects numeric argument".to_string()),
            };
            Ok(Value::Float(crate::stdlib::math::MATH::sin(a)))
        });
        
        self.functions.insert("MATH::cos".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("MATH::cos expects 1 argument".to_string());
            }
            let a = match &args[0] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("MATH::cos expects numeric argument".to_string()),
            };
            Ok(Value::Float(crate::stdlib::math::MATH::cos(a)))
        });
        
        // Constants
        self.functions.insert("MATH::PI".to_string(), |_interpreter, _args| {
            Ok(Value::Float(crate::stdlib::math::MATH::PI))
        });
        
        self.functions.insert("MATH::E".to_string(), |_interpreter, _args| {
            Ok(Value::Float(crate::stdlib::math::MATH::E))
        });
    }
    
    /// Register STRING module functions
    fn register_string_functions(&mut self) {
        self.functions.insert("STRING::length".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("STRING::length expects 1 argument".to_string());
            }
            let s = match &args[0] {
                Value::String(s) => s,
                _ => return Err("STRING::length expects string argument".to_string()),
            };
            Ok(Value::Int(crate::stdlib::string::STRING::length(s) as i64))
        });
        
        self.functions.insert("STRING::to_uppercase".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("STRING::to_uppercase expects 1 argument".to_string());
            }
            let s = match &args[0] {
                Value::String(s) => s,
                _ => return Err("STRING::to_uppercase expects string argument".to_string()),
            };
            Ok(Value::String(crate::stdlib::string::STRING::to_uppercase(s)))
        });
        
        self.functions.insert("STRING::to_lowercase".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("STRING::to_lowercase expects 1 argument".to_string());
            }
            let s = match &args[0] {
                Value::String(s) => s,
                _ => return Err("STRING::to_lowercase expects string argument".to_string()),
            };
            Ok(Value::String(crate::stdlib::string::STRING::to_lowercase(s)))
        });
        
        self.functions.insert("STRING::contains".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("STRING::contains expects 2 arguments".to_string());
            }
            let s = match &args[0] {
                Value::String(s) => s,
                _ => return Err("STRING::contains expects string argument".to_string()),
            };
            let pattern = match &args[1] {
                Value::String(s) => s,
                _ => return Err("STRING::contains expects string pattern".to_string()),
            };
            Ok(Value::Bool(crate::stdlib::string::STRING::contains(s, pattern)))
        });
        
        self.functions.insert("STRING::replace".to_string(), |_interpreter, args| {
            if args.len() != 3 {
                return Err("STRING::replace expects 3 arguments".to_string());
            }
            let s = match &args[0] {
                Value::String(s) => s,
                _ => return Err("STRING::replace expects string argument".to_string()),
            };
            let from = match &args[1] {
                Value::String(s) => s,
                _ => return Err("STRING::replace expects string from".to_string()),
            };
            let to = match &args[2] {
                Value::String(s) => s,
                _ => return Err("STRING::replace expects string to".to_string()),
            };
            Ok(Value::String(crate::stdlib::string::STRING::replace(s, from, to)))
        });
    }
    
    /// Register COLLECTIONS module functions
    fn register_collections_functions(&mut self) {
        self.functions.insert("COLLECTIONS::array_new".to_string(), |_interpreter, _args| {
            Ok(Value::Vec(Vec::new()))
        });
        
        self.functions.insert("COLLECTIONS::array_push".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("COLLECTIONS::array_push expects 2 arguments".to_string());
            }
            let mut vec = match &args[0] {
                Value::Vec(v) => v.clone(),
                _ => return Err("COLLECTIONS::array_push expects vector argument".to_string()),
            };
            vec.push(args[1].clone());
            Ok(Value::Vec(vec))
        });
        
        self.functions.insert("COLLECTIONS::array_get".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("COLLECTIONS::array_get expects 2 arguments".to_string());
            }
            let vec = match &args[0] {
                Value::Vec(v) => v,
                _ => return Err("COLLECTIONS::array_get expects vector argument".to_string()),
            };
            let index = match &args[1] {
                Value::Int(i) => *i as usize,
                _ => return Err("COLLECTIONS::array_get expects integer index".to_string()),
            };
            if index >= vec.len() {
                return Err("Index out of bounds".to_string());
            }
            Ok(vec[index].clone())
        });
    }
    
    /// Register TIME module functions
    fn register_time_functions(&mut self) {
        self.functions.insert("TIME::now".to_string(), |_interpreter, _args| {
            Ok(Value::Float(crate::stdlib::time::TIME::now() as f64))
        });
        
        self.functions.insert("TIME::now_string".to_string(), |_interpreter, _args| {
            Ok(Value::String(crate::stdlib::time::TIME::now_string()))
        });
        
        self.functions.insert("TIME::sleep".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("TIME::sleep expects 1 argument".to_string());
            }
            let seconds = match &args[0] {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => return Err("TIME::sleep expects numeric argument".to_string()),
            };
            crate::stdlib::time::TIME::sleep(seconds);
            Ok(Value::None)
        });
    }
    
    /// Register ERROR module functions
    fn register_error_functions(&mut self) {
        self.functions.insert("ERROR::new".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("ERROR::new expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("ERROR::new expects string argument".to_string()),
            };
            Ok(Value::String(crate::stdlib::error::ERROR::new(message)))
        });
        
        self.functions.insert("ERROR::log".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("ERROR::log expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("ERROR::log expects string argument".to_string()),
            };
            crate::stdlib::error::ERROR::log(message);
            Ok(Value::None)
        });
    }
    
    /// Register TESTING module functions
    fn register_testing_functions(&mut self) {
        self.functions.insert("TESTING::assert_true".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("TESTING::assert_true expects 2 arguments".to_string());
            }
            let condition = match &args[0] {
                Value::Bool(b) => *b,
                _ => return Err("TESTING::assert_true expects boolean condition".to_string()),
            };
            let message = match &args[1] {
                Value::String(s) => s,
                _ => return Err("TESTING::assert_true expects string message".to_string()),
            };
            crate::stdlib::testing::TESTING::assert_true(condition, message)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("TESTING::assert_equal".to_string(), |_interpreter, args| {
            if args.len() != 3 {
                return Err("TESTING::assert_equal expects 3 arguments".to_string());
            }
            let actual = &args[0];
            let expected = &args[1];
            let message = match &args[2] {
                Value::String(s) => s,
                _ => return Err("TESTING::assert_equal expects string message".to_string()),
            };
            crate::stdlib::testing::TESTING::assert_equal(actual, expected, message)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });
    }
    
    /// Get a native function by name
    pub fn get_function(&self, name: &str) -> Option<&NativeFunction> {
        self.functions.get(name)
    }
    
    /// Check if a function exists
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
    
    /// Get all available function names
    pub fn get_function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

impl Default for StdlibRegistry {
    fn default() -> Self {
        Self::new()
    }
}


