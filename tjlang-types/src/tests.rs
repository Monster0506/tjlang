//! Comprehensive tests for the TJLang type system
//! 
//! Tests type checking, type inference, subtyping, and type environment functionality.

#[cfg(test)]
mod tests {
    use crate::types::{Type, TypeEnvironment};
    use crate::checker::TypeChecker;

    // ============================================================================
    // Basic Type Tests
    // ============================================================================

    #[test]
    fn test_primitive_types() {
        let types = vec![
            Type::Int,
            Type::Float,
            Type::Bool,
            Type::Str,
        ];

        for type_ in types {
            assert!(TypeChecker::new().check_type(&type_));
        }
    }

    #[test]
    fn test_type_to_string() {
        assert_eq!(Type::Int.to_string(), "int");
        assert_eq!(Type::Float.to_string(), "float");
        assert_eq!(Type::Bool.to_string(), "bool");
        assert_eq!(Type::Str.to_string(), "str");
    }

    #[test]
    fn test_type_variable_to_string() {
        let var_type = Type::Variable("T".to_string());
        assert_eq!(var_type.to_string(), "T");
    }

    // ============================================================================
    // Subtyping Tests
    // ============================================================================

    #[test]
    fn test_primitive_type_relationships() {
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        let str_type = Type::Str;

        // Primitive types should be subtypes of themselves
        assert!(int.is_subtype_of(&int));
        assert!(float.is_subtype_of(&float));
        assert!(bool_type.is_subtype_of(&bool_type));
        assert!(str_type.is_subtype_of(&str_type));
        
        // Different primitive types should not be subtypes of each other
        assert!(!int.is_subtype_of(&float));
        assert!(!float.is_subtype_of(&int));
        assert!(!bool_type.is_subtype_of(&str_type));
        assert!(!str_type.is_subtype_of(&bool_type));
    }

    #[test]
    fn test_reflexive_subtyping() {
        let types = vec![
            Type::Int,
            Type::Float,
            Type::Bool,
            Type::Str,
        ];

        for type_ in types {
            assert!(type_.is_subtype_of(&type_));
        }
    }

    #[test]
    fn test_union_type_subtyping() {
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        let union = Type::Sum(vec![int.clone(), float.clone()]);
        let union_all = Type::Sum(vec![int.clone(), float.clone(), bool_type.clone()]);

        // Individual types should be subtypes of their union
        assert!(int.is_subtype_of(&union));
        assert!(float.is_subtype_of(&union));
        
        // Individual types should be subtypes of larger union
        assert!(int.is_subtype_of(&union_all));
        assert!(float.is_subtype_of(&union_all));
        assert!(bool_type.is_subtype_of(&union_all));
        
        // Test that different unions are not subtypes of each other
        let union_bool = Type::Sum(vec![bool_type.clone()]);
        assert!(!union.is_subtype_of(&union_bool));
        assert!(!union_bool.is_subtype_of(&union));
    }

    #[test]
    fn test_product_type_subtyping() {
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        let product1 = Type::Product(vec![int.clone(), float.clone()]);
        let product2 = Type::Product(vec![int.clone(), float.clone()]);
        let product3 = Type::Product(vec![int.clone(), float.clone(), bool_type.clone()]);

        // Identical product types should be subtypes of each other
        assert!(product1.is_subtype_of(&product2));
        assert!(product2.is_subtype_of(&product1));
        
        // Product types with different lengths should not be subtypes
        assert!(!product1.is_subtype_of(&product3));
        assert!(!product3.is_subtype_of(&product1));
    }

    #[test]
    fn test_function_type_subtyping() {
        let int = Type::Int;
        let float = Type::Float;
        let func1 = Type::Function(vec![int.clone()], Box::new(float.clone()));
        let func2 = Type::Function(vec![int.clone()], Box::new(float.clone()));

        assert!(func1.is_subtype_of(&func2));
    }

    #[test]
    fn test_contravariant_function_parameters() {
        let int = Type::Int;
        let float = Type::Float;
        
        // Function accepting Int should be subtype of function accepting Int
        let func_int = Type::Function(vec![int.clone()], Box::new(int.clone()));
        let func_int2 = Type::Function(vec![int.clone()], Box::new(int.clone()));
        
        assert!(func_int.is_subtype_of(&func_int2));
        
        // Function accepting Float should not be subtype of function accepting Int
        let func_float = Type::Function(vec![float.clone()], Box::new(int.clone()));
        assert!(!func_float.is_subtype_of(&func_int));
    }

    #[test]
    fn test_covariant_function_return() {
        let int = Type::Int;
        let float = Type::Float;
        
        // Function returning Int should be subtype of function returning Int
        let func_int = Type::Function(vec![int.clone()], Box::new(int.clone()));
        let func_int2 = Type::Function(vec![int.clone()], Box::new(int.clone()));
        
        assert!(func_int.is_subtype_of(&func_int2));
        
        // Function returning Int should not be subtype of function returning Float
        let func_float = Type::Function(vec![int.clone()], Box::new(float.clone()));
        assert!(!func_int.is_subtype_of(&func_float));
    }

    // ============================================================================
    // Container Type Tests
    // ============================================================================

    #[test]
    fn test_vec_type() {
        let int = Type::Int;
        let vec_int = Type::Vec(Box::new(int));
        
        assert!(TypeChecker::new().check_type(&vec_int));
        assert_eq!(vec_int.to_string(), "Vec<int>");
    }

    #[test]
    fn test_option_type() {
        let int = Type::Int;
        let option_int = Type::Option(Box::new(int));
        
        assert!(TypeChecker::new().check_type(&option_int));
        assert_eq!(option_int.to_string(), "Option<int>");
    }

    #[test]
    fn test_result_type() {
        let int = Type::Int;
        let str_type = Type::Str;
        let result = Type::Result(Box::new(int), Box::new(str_type));
        
        assert!(TypeChecker::new().check_type(&result));
        assert_eq!(result.to_string(), "Result<int, str>");
    }

    #[test]
    fn test_tuple_type() {
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        let tuple = Type::Tuple(vec![int, float, bool_type]);
        
        assert!(TypeChecker::new().check_type(&tuple));
    }

    #[test]
    fn test_set_type() {
        let int = Type::Int;
        let set_int = Type::Set(Box::new(int));
        
        assert!(TypeChecker::new().check_type(&set_int));
    }

    #[test]
    fn test_map_type() {
        let str_type = Type::Str;
        let int = Type::Int;
        let map = Type::Map(Box::new(str_type), Box::new(int));
        
        assert!(TypeChecker::new().check_type(&map));
    }

    #[test]
    fn test_task_type() {
        let int = Type::Int;
        let task = Type::Task(Box::new(int));
        
        assert!(TypeChecker::new().check_type(&task));
    }

    // ============================================================================
    // Generic Type Tests
    // ============================================================================

    #[test]
    fn test_generic_type() {
        let int = Type::Int;
        let generic = Type::Generic("T".to_string(), vec![int]);
        
        assert!(TypeChecker::new().check_type(&generic));
        assert_eq!(generic.to_string(), "T<int>");
    }

    #[test]
    fn test_generic_type_no_args() {
        let generic = Type::Generic("T".to_string(), vec![]);
        
        assert!(TypeChecker::new().check_type(&generic));
        assert_eq!(generic.to_string(), "T");
    }

    #[test]
    fn test_complex_generic_type() {
        let int = Type::Int;
        let str_type = Type::Str;
        let generic = Type::Generic("Map".to_string(), vec![int, str_type]);
        
        assert!(TypeChecker::new().check_type(&generic));
        assert_eq!(generic.to_string(), "Map<int, str>");
    }

    // ============================================================================
    // Type Environment Tests
    // ============================================================================

    #[test]
    fn test_empty_environment() {
        let env = TypeEnvironment::new();
        assert!(env.variables.is_empty());
        assert!(env.functions.is_empty());
        assert!(env.scopes.is_empty());
    }

    #[test]
    fn test_variable_lookup() {
        let mut env = TypeEnvironment::new();
        let int = Type::Int;
        
        env.bind_variable("x".to_string(), int.clone());
        
        assert_eq!(env.lookup_variable("x"), Some(int));
        assert_eq!(env.lookup_variable("y"), None);
    }

    #[test]
    fn test_function_lookup() {
        let mut env = TypeEnvironment::new();
        let int = Type::Int;
        let float = Type::Float;
        let func_type = Type::Function(vec![int], Box::new(float));
        
        env.bind_function("add".to_string(), func_type.clone());
        
        assert_eq!(env.lookup_function("add"), Some(func_type));
        assert_eq!(env.lookup_function("sub"), None);
    }

    #[test]
    fn test_scope_management() {
        let mut env = TypeEnvironment::new();
        let int = Type::Int;
        let float = Type::Float;
        
        // Global scope
        env.bind_variable("global".to_string(), int.clone());
        
        // Enter local scope
        env.enter_scope();
        env.bind_variable("local".to_string(), float.clone());
        
        // Should find local variable
        assert_eq!(env.lookup_variable("local"), Some(float.clone()));
        // Should still find global variable
        assert_eq!(env.lookup_variable("global"), Some(int.clone()));
        
        // Exit scope
        env.exit_scope();
        
        // Local variable should be gone
        assert_eq!(env.lookup_variable("local"), None);
        // Global variable should still be there
        assert_eq!(env.lookup_variable("global"), Some(int));
    }

    #[test]
    fn test_nested_scopes() {
        let mut env = TypeEnvironment::new();
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        
        // Global scope
        env.bind_variable("global".to_string(), int.clone());
        
        // First local scope
        env.enter_scope();
        env.bind_variable("level1".to_string(), float.clone());
        
        // Second local scope
        env.enter_scope();
        env.bind_variable("level2".to_string(), bool_type.clone());
        
        // Should find all variables
        assert_eq!(env.lookup_variable("global"), Some(int.clone()));
        assert_eq!(env.lookup_variable("level1"), Some(float.clone()));
        assert_eq!(env.lookup_variable("level2"), Some(bool_type.clone()));
        
        // Exit first scope
        env.exit_scope();
        assert_eq!(env.lookup_variable("level2"), None);
        assert_eq!(env.lookup_variable("level1"), Some(float));
        
        // Exit second scope
        env.exit_scope();
        assert_eq!(env.lookup_variable("level1"), None);
        assert_eq!(env.lookup_variable("global"), Some(int));
    }

    // ============================================================================
    // Type Checker Tests
    // ============================================================================

    #[test]
    fn test_type_checker_creation() {
        let checker = TypeChecker::new();
        assert!(checker.environment().variables.is_empty());
        assert!(checker.environment().functions.is_empty());
    }

    #[test]
    fn test_type_compatibility() {
        let checker = TypeChecker::new();
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        let str_type = Type::Str;
        
        // Same types are compatible
        assert!(checker.check_compatibility(&int, &int));
        assert!(checker.check_compatibility(&float, &float));
        assert!(checker.check_compatibility(&bool_type, &bool_type));
        assert!(checker.check_compatibility(&str_type, &str_type));
        
        // Different primitive types are not compatible
        assert!(!checker.check_compatibility(&int, &float));
        assert!(!checker.check_compatibility(&int, &bool_type));
        assert!(!checker.check_compatibility(&int, &str_type));
        assert!(!checker.check_compatibility(&float, &bool_type));
        assert!(!checker.check_compatibility(&float, &str_type));
        assert!(!checker.check_compatibility(&bool_type, &str_type));
    }

    #[test]
    fn test_complex_type_validation() {
        let checker = TypeChecker::new();
        
        // Valid complex type
        let int = Type::Int;
        let float = Type::Float;
        let func_type = Type::Function(vec![int], Box::new(float));
        let vec_type = Type::Vec(Box::new(func_type));
        
        assert!(checker.check_type(&vec_type));
    }

    #[test]
    fn test_specific_type_validation() {
        let checker = TypeChecker::new();
        
        // Test specific primitive types
        assert!(checker.check_type(&Type::Int));
        assert!(checker.check_type(&Type::Float));
        assert!(checker.check_type(&Type::Bool));
        assert!(checker.check_type(&Type::Str));
        
        // Test specific container types
        let vec_int = Type::Vec(Box::new(Type::Int));
        let option_float = Type::Option(Box::new(Type::Float));
        let result_bool_str = Type::Result(Box::new(Type::Bool), Box::new(Type::Str));
        
        assert!(checker.check_type(&vec_int));
        assert!(checker.check_type(&option_float));
        assert!(checker.check_type(&result_bool_str));
    }

    // ============================================================================
    // Complex Type Construction Tests
    // ============================================================================

    #[test]
    fn test_nested_container_types() {
        let int = Type::Int;
        let vec_int = Type::Vec(Box::new(int));
        let vec_vec_int = Type::Vec(Box::new(vec_int));
        
        assert!(TypeChecker::new().check_type(&vec_vec_int));
        assert_eq!(vec_vec_int.to_string(), "Vec<Vec<int>>");
    }

    #[test]
    fn test_function_with_multiple_parameters() {
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        let func_type = Type::Function(
            vec![int, float], 
            Box::new(bool_type)
        );
        
        assert!(TypeChecker::new().check_type(&func_type));
        assert_eq!(func_type.to_string(), "(int, float) -> bool");
    }

    #[test]
    fn test_sum_type_with_multiple_variants() {
        let int = Type::Int;
        let float = Type::Float;
        let str_type = Type::Str;
        let sum_type = Type::Sum(vec![int, float, str_type]);
        
        assert!(TypeChecker::new().check_type(&sum_type));
    }

    #[test]
    fn test_product_type_with_multiple_fields() {
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        let str_type = Type::Str;
        let product_type = Type::Product(vec![int, float, bool_type, str_type]);
        
        assert!(TypeChecker::new().check_type(&product_type));
    }

    #[test]
    fn test_mixed_container_types() {
        let int = Type::Int;
        let str_type = Type::Str;
        
        // Map<String, Vec<Int>>
        let vec_int = Type::Vec(Box::new(int));
        let map_type = Type::Map(Box::new(str_type), Box::new(vec_int));
        
        assert!(TypeChecker::new().check_type(&map_type));
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[test]
    fn test_empty_sum_type() {
        let empty_sum = Type::Sum(vec![]);
        assert!(TypeChecker::new().check_type(&empty_sum));
    }

    #[test]
    fn test_empty_product_type() {
        let empty_product = Type::Product(vec![]);
        assert!(TypeChecker::new().check_type(&empty_product));
    }

    #[test]
    fn test_empty_tuple_type() {
        let empty_tuple = Type::Tuple(vec![]);
        assert!(TypeChecker::new().check_type(&empty_tuple));
    }

    #[test]
    fn test_recursive_type_definition() {
        // This would be a recursive type like List<T> = Nil | Cons(T, List<T>)
        // For now, we just test that we can create complex nested types
        let int = Type::Int;
        let option_int = Type::Option(Box::new(int));
        let vec_option = Type::Vec(Box::new(option_int));
        
        assert!(TypeChecker::new().check_type(&vec_option));
    }

    #[test]
    fn test_type_variable_in_environment() {
        let mut env = TypeEnvironment::new();
        let type_var = Type::Variable("T".to_string());
        
        env.bind_variable("generic_var".to_string(), type_var.clone());
        
        assert_eq!(env.lookup_variable("generic_var"), Some(type_var));
    }

    #[test]
    fn test_function_type_in_environment() {
        let mut env = TypeEnvironment::new();
        let int = Type::Int;
        let float = Type::Float;
        let func_type = Type::Function(vec![int], Box::new(float));
        
        env.bind_function("square".to_string(), func_type.clone());
        
        assert_eq!(env.lookup_function("square"), Some(func_type));
    }

    // ============================================================================
    // Performance and Stress Tests
    // ============================================================================

    #[test]
    fn test_large_type_construction() {
        let mut types = Vec::new();
        
        // Create a large nested type structure
        for i in 0..100 {
            let int = Type::Int;
            let vec_type = Type::Vec(Box::new(int));
            types.push(vec_type);
        }
        
        let large_product = Type::Product(types);
        assert!(TypeChecker::new().check_type(&large_product));
    }

    #[test]
    fn test_deep_nesting() {
        let mut current_type = Type::Int;
        
        // Create deeply nested Vec types
        for _ in 0..50 {
            current_type = Type::Vec(Box::new(current_type));
        }
        
        assert!(TypeChecker::new().check_type(&current_type));
    }

    #[test]
    fn test_many_scopes() {
        let mut env = TypeEnvironment::new();
        let int = Type::Int;
        
        // Create many nested scopes
        for _i in 0..100 {
            env.enter_scope();
            env.bind_variable(format!("var_{}", _i), int.clone());
        }
        
        // Check that we can still find variables in the innermost scope
        assert_eq!(env.lookup_variable("var_99"), Some(int));
        
        // Exit all scopes
        for _ in 0..100 {
            env.exit_scope();
        }
        
        // All variables should be gone
        assert_eq!(env.lookup_variable("var_0"), None);
        assert_eq!(env.lookup_variable("var_99"), None);
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_complete_type_checking_workflow() {
        let mut checker = TypeChecker::new();
        let env = checker.environment_mut();
        
        // Define some variables with specific types
        env.bind_variable("x".to_string(), Type::Int);
        env.bind_variable("y".to_string(), Type::Float);
        env.bind_variable("name".to_string(), Type::Str);
        env.bind_variable("is_valid".to_string(), Type::Bool);
        
        // Define specific functions
        let add_func = Type::Function(
            vec![Type::Int, Type::Int], 
            Box::new(Type::Int)
        );
        let compare_func = Type::Function(
            vec![Type::Int, Type::Float], 
            Box::new(Type::Bool)
        );
        let format_func = Type::Function(
            vec![Type::Str, Type::Int], 
            Box::new(Type::Str)
        );
        
        env.bind_function("add".to_string(), add_func.clone());
        env.bind_function("compare".to_string(), compare_func.clone());
        env.bind_function("format".to_string(), format_func.clone());
        
        // Check that everything is accessible
        assert_eq!(env.lookup_variable("x"), Some(Type::Int));
        assert_eq!(env.lookup_variable("y"), Some(Type::Float));
        assert_eq!(env.lookup_variable("name"), Some(Type::Str));
        assert_eq!(env.lookup_variable("is_valid"), Some(Type::Bool));
        assert_eq!(env.lookup_function("add"), Some(add_func));
        assert_eq!(env.lookup_function("compare"), Some(compare_func));
        assert_eq!(env.lookup_function("format"), Some(format_func));
    }

    #[test]
    fn test_type_inference_simulation() {
        // Simulate type inference by checking compatibility
        let checker = TypeChecker::new();
        
        let int = Type::Int;
        let float = Type::Float;
        let bool_type = Type::Bool;
        let str_type = Type::Str;
        
        // Test various compatibility scenarios
        assert!(checker.check_compatibility(&int, &int));
        assert!(checker.check_compatibility(&float, &float));
        assert!(checker.check_compatibility(&bool_type, &bool_type));
        assert!(checker.check_compatibility(&str_type, &str_type));
        
        // Different primitive types should not be compatible
        assert!(!checker.check_compatibility(&int, &float));
        assert!(!checker.check_compatibility(&int, &bool_type));
        assert!(!checker.check_compatibility(&int, &str_type));
        
        // Test function compatibility - same function types should be compatible
        let func1 = Type::Function(vec![int.clone()], Box::new(int.clone()));
        let func2 = Type::Function(vec![int.clone()], Box::new(int.clone()));
        
        assert!(checker.check_compatibility(&func1, &func2));
        
        // Test function compatibility - different function types should not be compatible
        let func3 = Type::Function(vec![float.clone()], Box::new(float.clone()));
        assert!(!checker.check_compatibility(&func1, &func3));
    }
}
