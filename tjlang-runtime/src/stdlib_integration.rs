//! TJLang Standard Library Integration
//!
//! This module provides integration between the TJLang standard library
//! and the runtime interpreter, allowing TJLang code to call stdlib functions.

use crate::values::Value;
use crate::interpreter::Interpreter;
use std::collections::HashMap;
use tjlang_diagnostics::debug_println;
use tjlang_ast::{Expression, Literal};

/// Native function registry for standard library functions
pub struct StdlibRegistry {
    functions: HashMap<String, NativeFunction>,
    structs: HashMap<String, Value>,
}

/// Native function type that can be called from TJLang code
pub type NativeFunction = fn(&mut Interpreter, &[Value]) -> Result<Value, String>;

impl StdlibRegistry {
    pub fn new() -> Self {
        debug_println!("🔧 Creating stdlib registry...");
        let mut registry = Self {
            functions: HashMap::new(),
            structs: HashMap::new(),
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
        // Basic output functions
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

        self.functions.insert("IO::printf".to_string(), |_interpreter, args| {
            if args.len() < 1 {
                return Err("IO::printf expects at least 1 argument".to_string());
            }
            let format = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::printf expects string format".to_string()),
            };
            let format_args = &args[1..];
            crate::stdlib::io::IO::printf(format, format_args)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        // Input functions
        self.functions.insert("IO::read_line".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::read_line()
                .map(|s| Value::String(s))
                .map_err(|e| e.to_string())
        });
        
        self.functions.insert("IO::read_char".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::read_char()
                .map(|c| Value::String(c.to_string()))
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

        // Colored output functions
        self.functions.insert("IO::print_color".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("IO::print_color expects 2 arguments".to_string());
            }
            let text = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::print_color expects string text".to_string()),
            };
            let color = match &args[1] {
                Value::String(s) => s,
                _ => return Err("IO::print_color expects string color".to_string()),
            };
            // Convert string color to Color enum
            let color_enum = match color.as_str() {
                "red" => crate::stdlib::io::Color::Red,
                "green" => crate::stdlib::io::Color::Green,
                "yellow" => crate::stdlib::io::Color::Yellow,
                "blue" => crate::stdlib::io::Color::Blue,
                "magenta" => crate::stdlib::io::Color::Magenta,
                "cyan" => crate::stdlib::io::Color::Cyan,
                "white" => crate::stdlib::io::Color::White,
                "black" => crate::stdlib::io::Color::Black,
                "reset" => crate::stdlib::io::Color::Reset,
                _ => return Err("Invalid color name".to_string()),
            };
            crate::stdlib::io::IO::print_color(text, color_enum)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::print_error".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::print_error expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::print_error expects string message".to_string()),
            };
            crate::stdlib::io::IO::print_error(message)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::print_warning".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::print_warning expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::print_warning expects string message".to_string()),
            };
            crate::stdlib::io::IO::print_warning(message)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::print_success".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::print_success expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::print_success expects string message".to_string()),
            };
            crate::stdlib::io::IO::print_success(message)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::print_info".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::print_info expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::print_info expects string message".to_string()),
            };
            crate::stdlib::io::IO::print_info(message)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::print_debug".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::print_debug expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::print_debug expects string message".to_string()),
            };
            crate::stdlib::io::IO::print_debug(message)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        // Terminal control functions
        debug_println!("🔧 Registering IO::clear_screen");
        self.functions.insert("IO::clear_screen".to_string(), |_interpreter, _args| {
            debug_println!("🔧 IO::clear_screen called");
            crate::stdlib::io::IO::clear_screen()
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::move_cursor".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("IO::move_cursor expects 2 arguments".to_string());
            }
            let row = match &args[0] {
                Value::Int(i) => *i as u16,
                _ => return Err("IO::move_cursor expects int row".to_string()),
            };
            let col = match &args[1] {
                Value::Int(i) => *i as u16,
                _ => return Err("IO::move_cursor expects int col".to_string()),
            };
            crate::stdlib::io::IO::move_cursor(row, col)
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::hide_cursor".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::hide_cursor()
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::show_cursor".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::show_cursor()
                .map(|_| Value::None)
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::get_terminal_size".to_string(), |_interpreter, _args| {
            crate::stdlib::io::IO::get_terminal_size()
                .map(|(w, h)| Value::Tuple(vec![Value::Int(w as i64), Value::Int(h as i64)]))
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::is_terminal".to_string(), |_interpreter, _args| {
            Ok(Value::Bool(crate::stdlib::io::IO::is_terminal()))
        });

        self.functions.insert("IO::is_input_terminal".to_string(), |_interpreter, _args| {
            Ok(Value::Bool(crate::stdlib::io::IO::is_input_terminal()))
        });

        // User interaction functions
        self.functions.insert("IO::prompt".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::prompt expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::prompt expects string message".to_string()),
            };
            crate::stdlib::io::IO::prompt(message)
                .map(|s| Value::String(s))
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::prompt_with_default".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("IO::prompt_with_default expects 2 arguments".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::prompt_with_default expects string message".to_string()),
            };
            let default = match &args[1] {
                Value::String(s) => s,
                _ => return Err("IO::prompt_with_default expects string default".to_string()),
            };
            crate::stdlib::io::IO::prompt_with_default(message, default)
                .map(|s| Value::String(s))
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::confirm".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::confirm expects 1 argument".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::confirm expects string message".to_string()),
            };
            crate::stdlib::io::IO::confirm(message)
                .map(|b| Value::Bool(b))
                .map_err(|e| e.to_string())
        });

        // Progress bar and spinner functions
        self.functions.insert("IO::create_progress_bar".to_string(), |_interpreter, args| {
            if args.len() != 1 {
                return Err("IO::create_progress_bar expects 1 argument".to_string());
            }
            let total = match &args[0] {
                Value::Int(i) => *i as u64,
                _ => return Err("IO::create_progress_bar expects int total".to_string()),
            };
            // Note: ProgressBar is a struct that would need to be stored in the interpreter
            // For now, we'll return a placeholder value
            Ok(Value::String(format!("ProgressBar(total={})", total)))
        });

        self.functions.insert("IO::create_spinner".to_string(), |_interpreter, _args| {
            // Note: Spinner is a struct that would need to be stored in the interpreter
            // For now, we'll return a placeholder value
            Ok(Value::String("Spinner".to_string()))
        });

        // Select functions - these are complex because they need to handle arrays
        self.functions.insert("IO::select".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("IO::select expects 2 arguments".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::select expects string message".to_string()),
            };
            let options = match &args[1] {
                Value::Vec(v) => {
                    let mut string_options = Vec::new();
                    for option in v {
                        match option {
                            Value::String(s) => string_options.push(s.clone()),
                            _ => return Err("IO::select options must be strings".to_string()),
                        }
                    }
                    string_options
                },
                _ => return Err("IO::select expects array of strings".to_string()),
            };
            crate::stdlib::io::IO::select(message, &options)
                .map(|i| Value::Int(i as i64))
                .map_err(|e| e.to_string())
        });

        self.functions.insert("IO::multi_select".to_string(), |_interpreter, args| {
            if args.len() != 2 {
                return Err("IO::multi_select expects 2 arguments".to_string());
            }
            let message = match &args[0] {
                Value::String(s) => s,
                _ => return Err("IO::multi_select expects string message".to_string()),
            };
            let options = match &args[1] {
                Value::Vec(v) => {
                    let mut string_options = Vec::new();
                    for option in v {
                        match option {
                            Value::String(s) => string_options.push(s.clone()),
                            _ => return Err("IO::multi_select options must be strings".to_string()),
                        }
                    }
                    string_options
                },
                _ => return Err("IO::multi_select expects array of strings".to_string()),
            };
            crate::stdlib::io::IO::multi_select(message, &options)
                .map(|indices| Value::Vec(indices.into_iter().map(|i| Value::Int(i as i64)).collect()))
                .map_err(|e| e.to_string())
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
        
        // Set creation functions
        self.functions.insert("COLLECTIONS::set_new".to_string(), |_interpreter, _args| {
            Ok(Value::Set(std::collections::HashSet::new()))
        });
        
        // Map creation functions  
        self.functions.insert("COLLECTIONS::map_new".to_string(), |_interpreter, _args| {
            Ok(Value::Map(std::collections::HashMap::new()))
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


