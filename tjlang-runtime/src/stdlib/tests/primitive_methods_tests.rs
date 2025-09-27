//! Tests for primitive methods functionality
//!
//! Comprehensive test suite for all primitive method implementations
//! covering integers, floats, booleans, strings, and Option types.

#[cfg(test)]
mod tests {
    use crate::primitive_methods::*;
    use crate::values::Value;

    // Helper function to create test values
    fn create_test_values() -> (Value, Value, Value, Value, Value) {
        (
            Value::Int(42),
            Value::Float(3.14),
            Value::Bool(true),
            Value::String("Hello World".to_string()),
            Value::None,
        )
    }

    // ===== CORE METHODS TESTS =====

    #[test]
    fn test_to_string_all_types() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test integer to_string
        let result = get_primitive_method(&int_val, "to_string").unwrap();
        assert!(matches!(result, Value::String(_)));
        if let Value::String(s) = result {
            assert_eq!(s, "42");
        }

        // Test float to_string
        let result = get_primitive_method(&float_val, "to_string").unwrap();
        assert!(matches!(result, Value::String(_)));
        if let Value::String(s) = result {
            assert_eq!(s, "3.14");
        }

        // Test boolean to_string
        let result = get_primitive_method(&bool_val, "to_string").unwrap();
        assert!(matches!(result, Value::String(_)));
        if let Value::String(s) = result {
            assert_eq!(s, "true");
        }

        // Test string to_string
        let result = get_primitive_method(&str_val, "to_string").unwrap();
        assert!(matches!(result, Value::String(_)));
        if let Value::String(s) = result {
            assert_eq!(s, "Hello World");
        }

        // Test None to_string
        let result = get_primitive_method(&none_val, "to_string").unwrap();
        assert!(matches!(result, Value::String(_)));
        if let Value::String(s) = result {
            assert_eq!(s, "None");
        }
    }

    #[test]
    fn test_clone_all_types() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test integer clone
        let result = get_primitive_method(&int_val, "clone").unwrap();
        assert_eq!(result, int_val);

        // Test float clone
        let result = get_primitive_method(&float_val, "clone").unwrap();
        assert_eq!(result, float_val);

        // Test boolean clone
        let result = get_primitive_method(&bool_val, "clone").unwrap();
        assert_eq!(result, bool_val);

        // Test string clone
        let result = get_primitive_method(&str_val, "clone").unwrap();
        assert_eq!(result, str_val);

        // Test None clone
        let result = get_primitive_method(&none_val, "clone").unwrap();
        assert_eq!(result, none_val);
    }

    #[test]
    fn test_type_name_all_types() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test integer type_name
        let result = get_primitive_method(&int_val, "type_name").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "int");
        }

        // Test float type_name
        let result = get_primitive_method(&float_val, "type_name").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "float");
        }

        // Test boolean type_name
        let result = get_primitive_method(&bool_val, "type_name").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "bool");
        }

        // Test string type_name
        let result = get_primitive_method(&str_val, "type_name").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "str");
        }

        // Test None type_name
        let result = get_primitive_method(&none_val, "type_name").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "None");
        }
    }

    #[test]
    fn test_is_null_is_not_null() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test non-null values
        let result = get_primitive_method(&int_val, "is_null").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }
        let result = get_primitive_method(&int_val, "is_not_null").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test None value
        let result = get_primitive_method(&none_val, "is_null").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
        let result = get_primitive_method(&none_val, "is_not_null").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }
    }

    #[test]
    fn test_hash_all_types() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test integer hash
        let result = get_primitive_method(&int_val, "hash").unwrap();
        if let Value::Int(h) = result {
            assert_eq!(h, 42);
        }

        // Test float hash
        let result = get_primitive_method(&float_val, "hash").unwrap();
        assert!(matches!(result, Value::Int(_)));

        // Test boolean hash
        let result = get_primitive_method(&bool_val, "hash").unwrap();
        if let Value::Int(h) = result {
            assert_eq!(h, 1);
        }

        // Test string hash
        let result = get_primitive_method(&str_val, "hash").unwrap();
        if let Value::Int(h) = result {
            assert_eq!(h, 11); // Length of "Hello World"
        }

        // Test None hash
        let result = get_primitive_method(&none_val, "hash").unwrap();
        if let Value::Int(h) = result {
            assert_eq!(h, 0);
        }
    }

    // ===== TYPE CHECKING TESTS =====

    #[test]
    fn test_type_checking_methods() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test is_int
        let result = get_primitive_method(&int_val, "is_int").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
        let result = get_primitive_method(&float_val, "is_int").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        // Test is_float
        let result = get_primitive_method(&float_val, "is_float").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
        let result = get_primitive_method(&int_val, "is_float").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        // Test is_bool
        let result = get_primitive_method(&bool_val, "is_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
        let result = get_primitive_method(&int_val, "is_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        // Test is_str
        let result = get_primitive_method(&str_val, "is_str").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
        let result = get_primitive_method(&int_val, "is_str").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        // Test is_none
        let result = get_primitive_method(&none_val, "is_none").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
        let result = get_primitive_method(&int_val, "is_none").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }
    }

    // ===== TYPE CONVERSION TESTS =====

    #[test]
    fn test_type_conversions() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test to_int conversions
        let result = get_primitive_method(&int_val, "to_int").unwrap();
        if let Value::Int(i) = result {
            assert_eq!(i, 42);
        }

        let result = get_primitive_method(&float_val, "to_int").unwrap();
        if let Value::Int(i) = result {
            assert_eq!(i, 3);
        }

        let result = get_primitive_method(&bool_val, "to_int").unwrap();
        if let Value::Int(i) = result {
            assert_eq!(i, 1);
        }

        let result = get_primitive_method(&none_val, "to_int").unwrap();
        if let Value::Int(i) = result {
            assert_eq!(i, 0);
        }

        // Test to_float conversions
        let result = get_primitive_method(&int_val, "to_float").unwrap();
        if let Value::Float(f) = result {
            assert_eq!(f, 42.0);
        }

        let result = get_primitive_method(&bool_val, "to_float").unwrap();
        if let Value::Float(f) = result {
            assert_eq!(f, 1.0);
        }

        // Test to_bool conversions
        let result = get_primitive_method(&int_val, "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        let result = get_primitive_method(&Value::Int(0), "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        let result = get_primitive_method(&float_val, "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        let result = get_primitive_method(&Value::Float(0.0), "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        let result = get_primitive_method(&str_val, "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        let result = get_primitive_method(&Value::String("".to_string()), "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        let result = get_primitive_method(&none_val, "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }
    }

    // ===== INTEGER-SPECIFIC METHOD TESTS =====

    #[test]
    fn test_integer_methods() {
        let int_val = Value::Int(42);
        let neg_int_val = Value::Int(-10);
        let zero_val = Value::Int(0);
        
        // Test abs
        let result = get_primitive_method(&neg_int_val, "abs").unwrap();
        if let Value::Int(i) = result {
            assert_eq!(i, 10);
        }

        // Test neg
        let result = get_primitive_method(&int_val, "neg").unwrap();
        if let Value::Int(i) = result {
            assert_eq!(i, -42);
        }

        // Test inc
        let result = get_primitive_method(&int_val, "inc").unwrap();
        if let Value::Int(i) = result {
            assert_eq!(i, 43);
        }

        // Test dec
        let result = get_primitive_method(&int_val, "dec").unwrap();
        if let Value::Int(i) = result {
            assert_eq!(i, 41);
        }

        // Test is_even
        let result = get_primitive_method(&int_val, "is_even").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_odd
        let result = get_primitive_method(&int_val, "is_odd").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        // Test is_positive
        let result = get_primitive_method(&int_val, "is_positive").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_negative
        let result = get_primitive_method(&neg_int_val, "is_negative").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_zero
        let result = get_primitive_method(&zero_val, "is_zero").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
    }

    // ===== FLOAT-SPECIFIC METHOD TESTS =====

    #[test]
    fn test_float_methods() {
        let float_val = Value::Float(3.14);
        let neg_float_val = Value::Float(-3.14);
        let zero_float_val = Value::Float(0.0);
        let nan_val = Value::Float(f64::NAN);
        let inf_val = Value::Float(f64::INFINITY);
        
        // Test ceil
        let result = get_primitive_method(&float_val, "ceil").unwrap();
        if let Value::Float(f) = result {
            assert_eq!(f, 4.0);
        }

        // Test floor
        let result = get_primitive_method(&float_val, "floor").unwrap();
        if let Value::Float(f) = result {
            assert_eq!(f, 3.0);
        }

        // Test round
        let result = get_primitive_method(&float_val, "round").unwrap();
        if let Value::Float(f) = result {
            assert_eq!(f, 3.0);
        }

        // Test trunc
        let result = get_primitive_method(&float_val, "trunc").unwrap();
        if let Value::Float(f) = result {
            assert_eq!(f, 3.0);
        }

        // Test is_finite
        let result = get_primitive_method(&float_val, "is_finite").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_infinite
        let result = get_primitive_method(&inf_val, "is_infinite").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_nan
        let result = get_primitive_method(&nan_val, "is_nan").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_positive
        let result = get_primitive_method(&float_val, "is_positive").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_negative
        let result = get_primitive_method(&neg_float_val, "is_negative").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_zero
        let result = get_primitive_method(&zero_float_val, "is_zero").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
    }

    // ===== BOOLEAN-SPECIFIC METHOD TESTS =====

    #[test]
    fn test_boolean_methods() {
        let true_val = Value::Bool(true);
        let false_val = Value::Bool(false);
        
        // Test not
        let result = get_primitive_method(&true_val, "not").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        let result = get_primitive_method(&false_val, "not").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }
    }

    // ===== STRING-SPECIFIC METHOD TESTS =====

    #[test]
    fn test_string_methods() {
        let str_val = Value::String("Hello World".to_string());
        let empty_str_val = Value::String("".to_string());
        let whitespace_str_val = Value::String("  hello  ".to_string());
        
        // Test length
        let result = get_primitive_method(&str_val, "length").unwrap();
        if let Value::Int(l) = result {
            assert_eq!(l, 11);
        }

        // Test is_empty
        let result = get_primitive_method(&empty_str_val, "is_empty").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_not_empty
        let result = get_primitive_method(&str_val, "is_not_empty").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test trim
        let result = get_primitive_method(&whitespace_str_val, "trim").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "hello");
        }

        // Test upper
        let result = get_primitive_method(&str_val, "upper").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "HELLO WORLD");
        }

        // Test lower
        let result = get_primitive_method(&str_val, "lower").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "hello world");
        }

        // Test capitalize
        let result = get_primitive_method(&Value::String("hello".to_string()), "capitalize").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "Hello");
        }

        // Test reverse
        let result = get_primitive_method(&Value::String("hello".to_string()), "reverse").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "olleh");
        }
    }

    // ===== NONE-SPECIFIC METHOD TESTS =====

    #[test]
    fn test_none_methods() {
        let none_val = Value::None;
        
        // Test is_none
        let result = get_primitive_method(&none_val, "is_none").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_not_none
        let result = get_primitive_method(&none_val, "is_not_none").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }
    }

    // ===== DEBUG STRING TESTS =====

    #[test]
    fn test_debug_strings() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test debug_string
        let result = get_primitive_method(&int_val, "debug_string").unwrap();
        if let Value::String(s) = result {
            assert!(s.contains("Int"));
            assert!(s.contains("42"));
        }

        // Test pretty_string
        let result = get_primitive_method(&int_val, "pretty_string").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "42");
        }

        let result = get_primitive_method(&float_val, "pretty_string").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "3.14");
        }

        let result = get_primitive_method(&str_val, "pretty_string").unwrap();
        if let Value::String(s) = result {
            assert_eq!(s, "\"Hello World\"");
        }
    }

    // ===== ERROR HANDLING TESTS =====

    #[test]
    fn test_invalid_methods() {
        let int_val = Value::Int(42);
        
        // Test invalid method name
        let result = get_primitive_method(&int_val, "invalid_method");
        assert!(result.is_err());
        
        // Test method that requires arguments on wrong type
        let result = get_primitive_method(&int_val, "starts_with");
        assert!(result.is_err());
    }

    #[test]
    fn test_string_conversion_errors() {
        // Test invalid string to int conversion
        let invalid_str = Value::String("not_a_number".to_string());
        let result = get_primitive_method(&invalid_str, "to_int");
        assert!(result.is_err());

        // Test invalid string to float conversion
        let invalid_str = Value::String("not_a_number".to_string());
        let result = get_primitive_method(&invalid_str, "to_float");
        assert!(result.is_err());
    }

    // ===== HELPER FUNCTION TESTS =====

    #[test]
    fn test_helper_functions() {
        let (int_val, float_val, bool_val, str_val, none_val) = create_test_values();
        
        // Test get_type_name
        assert_eq!(get_type_name(&int_val), "int");
        assert_eq!(get_type_name(&float_val), "float");
        assert_eq!(get_type_name(&bool_val), "bool");
        assert_eq!(get_type_name(&str_val), "str");
        assert_eq!(get_type_name(&none_val), "None");

        // Test get_hash_code
        assert_eq!(get_hash_code(&int_val), 42);
        assert_eq!(get_hash_code(&bool_val), 1);
        assert_eq!(get_hash_code(&str_val), 11);
        assert_eq!(get_hash_code(&none_val), 0);

        // Test get_pretty_string
        assert_eq!(get_pretty_string(&int_val), "42");
        assert_eq!(get_pretty_string(&float_val), "3.14");
        assert_eq!(get_pretty_string(&bool_val), "true");
        assert_eq!(get_pretty_string(&str_val), "\"Hello World\"");
        assert_eq!(get_pretty_string(&none_val), "None");
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_edge_cases() {
        // Test zero values
        let zero_int = Value::Int(0);
        let zero_float = Value::Float(0.0);
        let empty_string = Value::String("".to_string());
        let false_bool = Value::Bool(false);

        // Test is_zero for integers
        let result = get_primitive_method(&zero_int, "is_zero").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_zero for floats
        let result = get_primitive_method(&zero_float, "is_zero").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test is_empty for strings
        let result = get_primitive_method(&empty_string, "is_empty").unwrap();
        if let Value::Bool(b) = result {
            assert!(b);
        }

        // Test to_bool for false values
        let result = get_primitive_method(&false_bool, "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        let result = get_primitive_method(&zero_int, "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        let result = get_primitive_method(&zero_float, "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }

        let result = get_primitive_method(&empty_string, "to_bool").unwrap();
        if let Value::Bool(b) = result {
            assert!(!b);
        }
    }

    // ===== PERFORMANCE TESTS =====

    #[test]
    fn test_performance_basic_operations() {
        let int_val = Value::Int(42);
        
        // Test that basic operations are fast
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = get_primitive_method(&int_val, "to_string").unwrap();
            let _ = get_primitive_method(&int_val, "clone").unwrap();
            let _ = get_primitive_method(&int_val, "type_name").unwrap();
        }
        let duration = start.elapsed();
        
        // Should complete in under 1ms for 1000 operations
        assert!(duration.as_millis() < 10);
    }
}
