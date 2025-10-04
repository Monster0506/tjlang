//! TJLang Standard Library Registry
//!
//! Shared registry of all standard library functions that can be used by both
//! the runtime and static analyzer to ensure consistency.

use std::collections::HashSet;

/// Get all available stdlib function names
/// This provides a single source of truth for all stdlib functions
/// 
/// Note: This should ideally be generated from the actual runtime registry,
/// but for now we maintain it here to avoid circular dependencies.
pub fn get_stdlib_function_names() -> HashSet<String> {
    // Create a temporary registry to get the function names
    // This mirrors the functions registered in tjlang-runtime/src/stdlib_integration.rs
    let mut functions = HashSet::new();
    
    // IO Module functions
    let io_functions = vec![
        "print", "println", "printf", "read_line", "read_char", "read_int", "read_float", "read_bool",
        "print_color", "print_error", "print_warning", "print_success", "print_info", "print_debug",
        "clear_screen", "move_cursor", "hide_cursor", "show_cursor", "get_terminal_size",
        "is_terminal", "is_input_terminal", "prompt", "prompt_with_default", "confirm",
        "create_progress_bar", "create_spinner", "select", "multi_select",
    ];
    
    for func in io_functions {
        functions.insert(format!("IO::{}", func));
    }
    
    // FILE Module functions
    let file_functions = vec![
        "read_to_string", "write_string", "exists", "is_file", "is_dir",
    ];
    
    for func in file_functions {
        functions.insert(format!("FILE::{}", func));
    }
    
    // MATH Module functions
    let math_functions = vec![
        "add", "subtract", "multiply", "divide", "sqrt", "sin", "cos", "PI", "E",
    ];
    
    for func in math_functions {
        functions.insert(format!("MATH::{}", func));
    }
    
    // STRING Module functions
    let string_functions = vec![
        "length", "to_uppercase", "to_lowercase", "contains", "replace",
    ];
    
    for func in string_functions {
        functions.insert(format!("STRING::{}", func));
    }
    
    // COLLECTIONS Module functions
    let collections_functions = vec![
        "array_new", "array_push", "array_get", "set_new", "map_new", "queue_new",
        "priority_queue_new", "btree_map_new", "btree_set_new", "counter_new",
        "default_dict_new", "chain_map_new", "named_tuple_new", "ordered_dict_new",
        "deque_new", "heap_new",
    ];
    
    for func in collections_functions {
        functions.insert(format!("COLLECTIONS::{}", func));
    }
    
    // TIME Module functions
    let time_functions = vec!["now", "now_string", "sleep"];
    
    for func in time_functions {
        functions.insert(format!("TIME::{}", func));
    }
    
    // ERROR Module functions
    let error_functions = vec!["new", "log"];
    
    for func in error_functions {
        functions.insert(format!("ERROR::{}", func));
    }
    
    // TESTING Module functions
    let testing_functions = vec!["assert_true", "assert_equal"];
    
    for func in testing_functions {
        functions.insert(format!("TESTING::{}", func));
    }
    
    functions
}

/// Get all stdlib module names
pub fn get_stdlib_module_names() -> HashSet<String> {
    let mut modules = HashSet::new();
    modules.insert("IO".to_string());
    modules.insert("FILE".to_string());
    modules.insert("MATH".to_string());
    modules.insert("STRING".to_string());
    modules.insert("COLLECTIONS".to_string());
    modules.insert("TIME".to_string());
    modules.insert("ERROR".to_string());
    modules.insert("TESTING".to_string());
    modules
}

/// Check if a method name is a known primitive method
/// Primitive methods are dynamically dispatched at runtime based on the type
pub fn is_primitive_method(method_name: &str) -> bool {
    let primitive_methods = vec![
        "to_string", "to_int", "to_float", "to_bool",
        "at", "get", "push", "pop", "len", "is_empty",
        "contains", "insert", "remove", "clear",
        "keys", "values", "entries",
    ];
    
    primitive_methods.contains(&method_name)
}
