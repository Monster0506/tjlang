//! Primitive method implementations for TJLang
//!
//! This module provides methods that work on all primitive types (int, float, bool, str, None)

use crate::values::Value;
use std::collections::HashMap;
use tjlang_diagnostics::debug_println;

/// Get a method for a primitive value
pub fn get_primitive_method(target: &Value, method: &str) -> Result<Value, String> {
    debug_println!(
        "[DEBUG] get_primitive_method called: target={:?}, method={}",
        std::mem::discriminant(target),
        method
    );

    match method {
        // Core methods that work on all primitives
        "to_string" => Ok(Value::String(target.to_string())),
        "clone" => Ok(target.clone()),
        "equals" => {
            // For methods that need arguments, we need to return a function
            // But for now, let's handle simple methods directly
            Err(
                "equals method requires arguments - use execute_primitive_method instead"
                    .to_string(),
            )
        }
        "type_name" => Ok(Value::String(get_type_name(target))),
        "is_null" => Ok(Value::Bool(matches!(target, Value::None))),
        "is_not_null" => Ok(Value::Bool(!matches!(target, Value::None))),
        "hash" => Ok(Value::Int(get_hash_code(target))),

        // Type checking methods
        "is_int" => Ok(Value::Bool(matches!(target, Value::Int(_)))),
        "is_float" => Ok(Value::Bool(matches!(target, Value::Float(_)))),
        "is_bool" => Ok(Value::Bool(matches!(target, Value::Bool(_)))),
        "is_str" => Ok(Value::Bool(matches!(target, Value::String(_)))),
        "is_none" => Ok(Value::Bool(matches!(target, Value::None))),
        "is_tuple" => Ok(Value::Bool(matches!(target, Value::Tuple(_)))),

        // Conversion methods
        "to_int" => convert_to_int(target),
        "to_float" => convert_to_float(target),
        "to_bool" => convert_to_bool(target),
        "to_str" => Ok(Value::String(target.to_string())),

        // Utility methods
        "debug_string" => Ok(Value::String(format!("{:?}", target))),
        "pretty_string" => Ok(Value::String(get_pretty_string(target))),

        // Type-specific methods
        _ => get_type_specific_method(target, method),
    }
}

/// Execute a primitive method call
pub fn execute_primitive_method(
    target: &Value,
    method: &str,
    args: &[Value],
) -> Result<Value, String> {
    debug_println!(
        "[DEBUG] execute_primitive_method: method={}, args.len()={}",
        method,
        args.len()
    );

    // Handle collection methods with arguments
    if let Value::Vec(vec) = target {
        return execute_vec_method(vec, method, args);
    }
    if let Value::Set(set) = target {
        return execute_set_method(set, method, args);
    }
    if let Value::Map(map) = target {
        return execute_map_method(map, method, args);
    }

    match method {
        "equals" => {
            if args.len() != 1 {
                return Err("equals method requires exactly one argument".to_string());
            }
            Ok(Value::Bool(primitive_equals(target, &args[0])))
        }
        "to_int" => convert_to_int(target),
        "to_float" => convert_to_float(target),
        "to_bool" => convert_to_bool(target),
        "to_str" => Ok(Value::String(target.to_string())),
        "debug_string" => Ok(Value::String(format!("{:?}", target))),
        "pretty_string" => Ok(Value::String(get_pretty_string(target))),
        _ => get_type_specific_method(target, method),
    }
}

/// Get type-specific methods
fn get_type_specific_method(target: &Value, method: &str) -> Result<Value, String> {
    match target {
        Value::Int(_) => get_integer_method(target, method),
        Value::Float(_) => get_float_method(target, method),
        Value::Bool(_) => get_boolean_method(target, method),
        Value::String(_) => get_string_method(target, method),
        Value::None => get_none_method(target, method),
        Value::Tuple(_) => get_tuple_method(target, method),
        Value::Vec(_) => get_vec_method(target, method),
        Value::Set(_) => get_set_method(target, method),
        Value::Map(_) => get_map_method(target, method),
        _ => Err(format!(
            "No method '{}' found on {} value",
            method,
            get_type_name(target)
        )),
    }
}

/// Integer-specific methods
fn get_integer_method(target: &Value, method: &str) -> Result<Value, String> {
    if let Value::Int(value) = target {
        match method {
            "abs" => Ok(Value::Int(value.abs())),
            "neg" => Ok(Value::Int(-value)),
            "inc" => Ok(Value::Int(value + 1)),
            "dec" => Ok(Value::Int(value - 1)),
            "is_even" => Ok(Value::Bool(value % 2 == 0)),
            "is_odd" => Ok(Value::Bool(value % 2 != 0)),
            "is_positive" => Ok(Value::Bool(*value > 0)),
            "is_negative" => Ok(Value::Bool(*value < 0)),
            "is_zero" => Ok(Value::Bool(*value == 0)),
            _ => Err(format!("No method '{}' found on integer", method)),
        }
    } else {
        Err("Expected integer value".to_string())
    }
}

/// Float-specific methods
fn get_float_method(target: &Value, method: &str) -> Result<Value, String> {
    if let Value::Float(value) = target {
        match method {
            "abs" => Ok(Value::Float(value.abs())),
            "neg" => Ok(Value::Float(-value)),
            "ceil" => Ok(Value::Float(value.ceil())),
            "floor" => Ok(Value::Float(value.floor())),
            "round" => Ok(Value::Float(value.round())),
            "trunc" => Ok(Value::Float(value.trunc())),
            "is_finite" => Ok(Value::Bool(value.is_finite())),
            "is_infinite" => Ok(Value::Bool(value.is_infinite())),
            "is_nan" => Ok(Value::Bool(value.is_nan())),
            "is_positive" => Ok(Value::Bool(*value > 0.0)),
            "is_negative" => Ok(Value::Bool(*value < 0.0)),
            "is_zero" => Ok(Value::Bool(*value == 0.0)),
            _ => Err(format!("No method '{}' found on float", method)),
        }
    } else {
        Err("Expected float value".to_string())
    }
}

/// Boolean-specific methods
fn get_boolean_method(target: &Value, method: &str) -> Result<Value, String> {
    if let Value::Bool(value) = target {
        match method {
            "not" => Ok(Value::Bool(!value)),
            _ => Err(format!("No method '{}' found on boolean", method)),
        }
    } else {
        Err("Expected boolean value".to_string())
    }
}

/// String-specific methods
fn get_string_method(target: &Value, method: &str) -> Result<Value, String> {
    if let Value::String(value) = target {
        match method {
            "length" => Ok(Value::Int(value.len() as i64)),
            "is_empty" => Ok(Value::Bool(value.is_empty())),
            "is_not_empty" => Ok(Value::Bool(!value.is_empty())),
            "trim" => Ok(Value::String(value.trim().to_string())),
            "upper" => Ok(Value::String(value.to_uppercase())),
            "lower" => Ok(Value::String(value.to_lowercase())),
            "capitalize" => {
                let mut chars = value.chars();
                match chars.next() {
                    None => Ok(Value::String(String::new())),
                    Some(first) => Ok(Value::String(
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                    )),
                }
            }
            "reverse" => Ok(Value::String(value.chars().rev().collect())),
            _ => Err(format!("No method '{}' found on string", method)),
        }
    } else {
        Err("Expected string value".to_string())
    }
}

/// None-specific methods
fn get_none_method(_target: &Value, method: &str) -> Result<Value, String> {
    match method {
        "is_none" => Ok(Value::Bool(true)),
        "is_not_none" => Ok(Value::Bool(false)),
        _ => Err(format!("No method '{}' found on None", method)),
    }
}

/// Helper functions
pub fn get_type_name(value: &Value) -> String {
    match value {
        Value::Int(_) => "int".to_string(),
        Value::Float(_) => "float".to_string(),
        Value::Bool(_) => "bool".to_string(),
        Value::String(_) => "str".to_string(),
        Value::None => "None".to_string(),
        _ => "unknown".to_string(),
    }
}

pub fn get_hash_code(value: &Value) -> i64 {
    match value {
        Value::Int(i) => *i,
        Value::Float(f) => f.to_bits() as i64,
        Value::Bool(b) => {
            if *b {
                1
            } else {
                0
            }
        }
        Value::String(s) => s.len() as i64,
        Value::None => 0,
        _ => 0,
    }
}

pub fn get_pretty_string(value: &Value) -> String {
    match value {
        Value::Int(i) => i.to_string(),
        Value::Float(f) => {
            if f.fract() == 0.0 {
                format!("{:.0}", f)
            } else {
                f.to_string()
            }
        }
        Value::Bool(b) => b.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::None => "None".to_string(),
        _ => format!("{:?}", value),
    }
}

fn primitive_equals(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::None, Value::None) => true,
        _ => false,
    }
}

fn convert_to_int(value: &Value) -> Result<Value, String> {
    match value {
        Value::Int(i) => Ok(Value::Int(*i)),
        Value::Float(f) => Ok(Value::Int(*f as i64)),
        Value::Bool(b) => Ok(Value::Int(if *b { 1 } else { 0 })),
        Value::String(s) => s
            .parse::<i64>()
            .map(Value::Int)
            .map_err(|_| format!("Cannot convert string '{}' to integer", s)),
        Value::None => Ok(Value::Int(0)),
        _ => Err(format!(
            "Cannot convert {} to integer",
            get_type_name(value)
        )),
    }
}

fn convert_to_float(value: &Value) -> Result<Value, String> {
    match value {
        Value::Int(i) => Ok(Value::Float(*i as f64)),
        Value::Float(f) => Ok(Value::Float(*f)),
        Value::Bool(b) => Ok(Value::Float(if *b { 1.0 } else { 0.0 })),
        Value::String(s) => s
            .parse::<f64>()
            .map(Value::Float)
            .map_err(|_| format!("Cannot convert string '{}' to float", s)),
        Value::None => Ok(Value::Float(0.0)),
        _ => Err(format!("Cannot convert {} to float", get_type_name(value))),
    }
}

fn convert_to_bool(value: &Value) -> Result<Value, String> {
    match value {
        Value::Int(i) => Ok(Value::Bool(*i != 0)),
        Value::Float(f) => Ok(Value::Bool(*f != 0.0)),
        Value::Bool(b) => Ok(Value::Bool(*b)),
        Value::String(s) => Ok(Value::Bool(!s.is_empty())),
        Value::None => Ok(Value::Bool(false)),
        _ => Err(format!(
            "Cannot convert {} to boolean",
            get_type_name(value)
        )),
    }
}

/// Tuple-specific methods
fn get_tuple_method(target: &Value, method: &str) -> Result<Value, String> {
    if let Value::Tuple(tuple) = target {
        match method {
            "length" => Ok(Value::Int(tuple.len() as i64)),
            "is_empty" => Ok(Value::Bool(tuple.is_empty())),
            "is_not_empty" => Ok(Value::Bool(!tuple.is_empty())),
            _ => Err(format!("No method '{}' found on tuple", method)),
        }
    } else {
        Err("Expected tuple value".to_string())
    }
}

/// Vector-specific methods
fn get_vec_method(target: &Value, method: &str) -> Result<Value, String> {
    if let Value::Vec(vec) = target {
        match method {
            // Basic properties (no arguments needed)
            "length" => Ok(Value::Int(vec.len() as i64)),
            "len" => Ok(Value::Int(vec.len() as i64)),
            "capacity" => Ok(Value::Int(vec.capacity() as i64)),
            "is_empty" => Ok(Value::Bool(vec.is_empty())),
            "is_not_empty" => Ok(Value::Bool(!vec.is_empty())),
            "reverse" => {
                let mut new_vec = vec.clone();
                new_vec.reverse();
                Ok(Value::Vec(new_vec))
            }
            "sort" => {
                let mut new_vec = vec.clone();
                // Improved sorting that handles different types more effectively
                new_vec.sort_by(|a, b| {
                    match (a, b) {
                        // Integer comparison
                        (Value::Int(x), Value::Int(y)) => x.cmp(y),
                        // Float comparison
                        (Value::Float(x), Value::Float(y)) => {
                            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        // String comparison
                        (Value::String(x), Value::String(y)) => x.cmp(y),
                        // Boolean comparison (false < true)
                        (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
                        // Mixed types: convert to string for comparison
                        _ => a.to_string().cmp(&b.to_string()),
                    }
                });
                Ok(Value::Vec(new_vec))
            }

            // Methods that require arguments - these will be handled by the interpreter
            "push" | "pop" | "insert" | "remove" | "get" | "at" | "get_mut" | "set" | "slice"
            | "append" | "extend" | "sort_by" | "shuffle" | "unique" | "filter" | "map"
            | "reduce" | "fold" | "any" | "all" | "find" | "find_index" | "contains"
            | "index_of" | "last_index_of" => Err(format!(
                "{} method requires arguments - use execute_primitive_method instead",
                method
            )),

            _ => Err(format!("No method '{}' found on vector", method)),
        }
    } else {
        Err("Expected vector value".to_string())
    }
}

/// Execute vector methods with arguments
fn execute_vec_method(vec: &Vec<Value>, method: &str, args: &[Value]) -> Result<Value, String> {
    debug_println!(
        "[DEBUG] execute_vec_method called: method={}, args={:?}",
        method,
        args
    );

    match method {
        "push" => {
            if args.len() != 1 {
                return Err("push method requires exactly 1 argument".to_string());
            }
            let mut new_vec = vec.clone();
            new_vec.push(args[0].clone());
            Ok(Value::Vec(new_vec))
        }
        "get" | "at" => {
            if args.len() != 1 {
                return Err("get/at method requires exactly 1 argument".to_string());
            }
            if let Value::Int(index) = &args[0] {
                if *index >= 0 && (*index as usize) < vec.len() {
                    Ok(vec[*index as usize].clone())
                } else {
                    Err("Index out of bounds".to_string())
                }
            } else {
                Err("get/at method requires integer index".to_string())
            }
        }
        "set" => {
            if args.len() != 2 {
                return Err("set method requires exactly 2 arguments".to_string());
            }
            if let Value::Int(index) = &args[0] {
                if *index >= 0 && (*index as usize) < vec.len() {
                    let mut new_vec = vec.clone();
                    new_vec[*index as usize] = args[1].clone();
                    Ok(Value::Vec(new_vec))
                } else {
                    Err("Index out of bounds".to_string())
                }
            } else {
                Err("set method requires integer index".to_string())
            }
        }
        "pop" => {
            if args.len() != 0 {
                return Err("pop method takes no arguments".to_string());
            }
            let mut new_vec = vec.clone();
            if let Some(value) = new_vec.pop() {
                Ok(value)
            } else {
                Err("Cannot pop from empty vector".to_string())
            }
        }
        "insert" => {
            if args.len() != 2 {
                return Err("insert method requires exactly 2 arguments".to_string());
            }
            if let Value::Int(index) = &args[0] {
                if *index >= 0 && (*index as usize) <= vec.len() {
                    let mut new_vec = vec.clone();
                    new_vec.insert(*index as usize, args[1].clone());
                    Ok(Value::Vec(new_vec))
                } else {
                    Err("Index out of bounds".to_string())
                }
            } else {
                Err("insert method requires integer index".to_string())
            }
        }
        "remove" => {
            if args.len() != 1 {
                return Err("remove method requires exactly 1 argument".to_string());
            }
            if let Value::Int(index) = &args[0] {
                if *index >= 0 && (*index as usize) < vec.len() {
                    let mut new_vec = vec.clone();
                    let removed = new_vec.remove(*index as usize);
                    Ok(removed)
                } else {
                    Err("Index out of bounds".to_string())
                }
            } else {
                Err("remove method requires integer index".to_string())
            }
        }
        "slice" => {
            if args.len() != 2 {
                return Err("slice method requires exactly 2 arguments".to_string());
            }
            if let (Value::Int(start), Value::Int(end)) = (&args[0], &args[1]) {
                if *start >= 0
                    && *end >= 0
                    && *start as usize <= vec.len()
                    && *end as usize <= vec.len()
                    && *start <= *end
                {
                    let slice: Vec<Value> = vec[*start as usize..*end as usize].to_vec();
                    Ok(Value::Vec(slice))
                } else {
                    Err("Invalid slice bounds".to_string())
                }
            } else {
                Err("slice method requires integer start and end".to_string())
            }
        }
        "reverse" => {
            if args.len() != 0 {
                return Err("reverse method takes no arguments".to_string());
            }
            let mut new_vec = vec.clone();
            new_vec.reverse();
            Ok(Value::Vec(new_vec))
        }
        "sort" => {
            if args.len() != 0 {
                return Err("sort method takes no arguments".to_string());
            }
            let mut new_vec = vec.clone();
            // Improved sorting that handles different types more effectively
            new_vec.sort_by(|a, b| {
                match (a, b) {
                    // Integer comparison
                    (Value::Int(x), Value::Int(y)) => x.cmp(y),
                    // Float comparison
                    (Value::Float(x), Value::Float(y)) => {
                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    // String comparison
                    (Value::String(x), Value::String(y)) => x.cmp(y),
                    // Boolean comparison (false < true)
                    (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
                    // Mixed types: convert to string for comparison
                    _ => a.to_string().cmp(&b.to_string()),
                }
            });
            Ok(Value::Vec(new_vec))
        }
        "sort_by" => {
            if args.len() != 1 {
                return Err(
                    "sort_by method requires exactly 1 argument (comparison function)".to_string(),
                );
            }

            // Check if the argument is a closure (lambda function)
            match &args[0] {
                Value::Closure { params, body, .. } => {
                    if params.len() != 2 {
                        return Err("sort_by comparison function must take exactly 2 parameters"
                            .to_string());
                    }

                    let mut new_vec = vec.clone();
                    // For now, we'll use a simple comparison based on the closure
                    // In a full implementation, we would execute the closure for each comparison
                    new_vec.sort_by(|a, b| {
                        // For now, use string comparison as a fallback
                        // TODO: Execute the closure with parameters a and b
                        a.to_string().cmp(&b.to_string())
                    });
                    Ok(Value::Vec(new_vec))
                }
                _ => {
                    return Err("sort_by requires a lambda function as argument".to_string());
                }
            }
        }
        _ => Err(format!("No method '{}' found on vector", method)),
    }
}

/// Set-specific methods
fn get_set_method(target: &Value, method: &str) -> Result<Value, String> {
    if let Value::Set(set) = target {
        match method {
            // Basic properties (no arguments needed)
            "length" => Ok(Value::Int(set.len() as i64)),
            "len" => Ok(Value::Int(set.len() as i64)),
            "is_empty" => Ok(Value::Bool(set.is_empty())),
            "is_not_empty" => Ok(Value::Bool(!set.is_empty())),

            // Methods that require arguments - these will be handled by the interpreter
            "insert"
            | "remove"
            | "contains"
            | "union"
            | "intersection"
            | "difference"
            | "symmetric_difference"
            | "is_subset"
            | "is_superset"
            | "is_disjoint" => Err(format!(
                "{} method requires arguments - use execute_primitive_method instead",
                method
            )),

            _ => Err(format!("No method '{}' found on set", method)),
        }
    } else {
        Err("Expected set value".to_string())
    }
}

/// Execute set methods with arguments
fn execute_set_method(
    set: &std::collections::HashSet<Value>,
    method: &str,
    args: &[Value],
) -> Result<Value, String> {
    debug_println!(
        "[DEBUG] execute_set_method called: method={}, args={:?}",
        method,
        args
    );

    match method {
        "insert" => {
            if args.len() != 1 {
                return Err("insert method requires exactly 1 argument".to_string());
            }
            let mut new_set = set.clone();
            new_set.insert(args[0].clone());
            Ok(Value::Set(new_set))
        }
        "remove" => {
            if args.len() != 1 {
                return Err("remove method requires exactly 1 argument".to_string());
            }
            let mut new_set = set.clone();
            new_set.remove(&args[0]);
            Ok(Value::Set(new_set))
        }
        "contains" => {
            if args.len() != 1 {
                return Err("contains method requires exactly 1 argument".to_string());
            }
            Ok(Value::Bool(set.contains(&args[0])))
        }
        _ => Err(format!("No method '{}' found on set", method)),
    }
}

/// Map-specific methods
fn get_map_method(target: &Value, method: &str) -> Result<Value, String> {
    if let Value::Map(map) = target {
        match method {
            // Basic properties (no arguments needed)
            "length" => Ok(Value::Int(map.len() as i64)),
            "len" => Ok(Value::Int(map.len() as i64)),
            "is_empty" => Ok(Value::Bool(map.is_empty())),
            "is_not_empty" => Ok(Value::Bool(!map.is_empty())),

            // Methods that require arguments - these will be handled by the interpreter
            "insert" | "remove" | "get" | "set" | "contains_key" | "keys" | "values"
            | "entries" | "clear" => Err(format!(
                "{} method requires arguments - use execute_primitive_method instead",
                method
            )),

            _ => Err(format!("No method '{}' found on map", method)),
        }
    } else {
        Err("Expected map value".to_string())
    }
}

/// Execute map methods with arguments
fn execute_map_method(
    map: &HashMap<Value, Value>,
    method: &str,
    args: &[Value],
) -> Result<Value, String> {
    debug_println!(
        "[DEBUG] execute_map_method called: method={}, args={:?}",
        method,
        args
    );

    match method {
        "insert" => {
            if args.len() != 2 {
                return Err("insert method requires exactly 2 arguments".to_string());
            }
            let mut new_map = map.clone();
            new_map.insert(args[0].clone(), args[1].clone());
            Ok(Value::Map(new_map))
        }
        "set" => {
            if args.len() != 2 {
                return Err("set method requires exactly 2 arguments".to_string());
            }
            let mut new_map = map.clone();
            new_map.insert(args[0].clone(), args[1].clone());
            Ok(Value::Map(new_map))
        }
        "get" => {
            if args.len() != 1 {
                return Err("get method requires exactly 1 argument".to_string());
            }
            if let Some(value) = map.get(&args[0]) {
                Ok(value.clone())
            } else {
                Ok(Value::None)
            }
        }
        "remove" => {
            if args.len() != 1 {
                return Err("remove method requires exactly 1 argument".to_string());
            }
            let mut new_map = map.clone();
            new_map.remove(&args[0]);
            Ok(Value::Map(new_map))
        }
        "contains_key" => {
            if args.len() != 1 {
                return Err("contains_key method requires exactly 1 argument".to_string());
            }
            Ok(Value::Bool(map.contains_key(&args[0])))
        }
        "keys" => {
            if args.len() != 0 {
                return Err("keys method takes no arguments".to_string());
            }
            let keys: Vec<Value> = map.keys().cloned().collect();
            Ok(Value::Vec(keys))
        }
        "values" => {
            if args.len() != 0 {
                return Err("values method takes no arguments".to_string());
            }
            let values: Vec<Value> = map.values().cloned().collect();
            Ok(Value::Vec(values))
        }
        "entries" => {
            if args.len() != 0 {
                return Err("entries method takes no arguments".to_string());
            }
            let entries: Vec<Value> = map
                .iter()
                .map(|(k, v)| Value::Tuple(vec![k.clone(), v.clone()]))
                .collect();
            Ok(Value::Vec(entries))
        }
        "clear" => {
            if args.len() != 0 {
                return Err("clear method takes no arguments".to_string());
            }
            Ok(Value::Map(HashMap::new()))
        }
        _ => Err(format!("No method '{}' found on map", method)),
    }
}
