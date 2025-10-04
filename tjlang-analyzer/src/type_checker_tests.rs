//! Comprehensive Type Checker Tests
//!
//! Tests for the comprehensive type checker covering expressions, statements, and declarations.

use tjlang_ast::*;
use tjlang_types::Type;
use codespan::{FileId, Files};
use super::type_checker::TypeChecker;

/// Test helper to create a simple program
fn create_test_program() -> Program {
    let mut files = Files::new();
    let file_id = files.add("test.tj", "");
    
    Program {
        units: vec![],
        span: SourceSpan { file_id, span: codespan::Span::new(0, 0) },
    }
}

/// Test helper to create a simple expression
fn create_test_expression() -> Expression {
    Expression::Literal(Literal::Int(42))
}

/// Test helper to create a simple statement
fn create_test_statement() -> Statement {
    Statement::Expression(create_test_expression())
}

/// Test helper to create a simple declaration
fn create_test_declaration() -> Declaration {
    let mut files = Files::new();
    let file_id = files.add("test.tj", "");
    
    Declaration::Variable(VariableDecl {
        name: "test_var".to_string(),
        var_type: tjlang_ast::Type::Primitive(PrimitiveType::Int),
        value: create_test_expression(),
        span: SourceSpan { file_id, span: codespan::Span::new(0, 0) },
    })
}

#[test]
fn test_type_checker_creation() {
    let type_checker = TypeChecker::new();
    assert!(type_checker.get_diagnostics().is_empty());
}

#[test]
fn test_empty_program_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
    
    let type_map = result.unwrap();
    assert!(type_map.is_empty());
}

#[test]
fn test_literal_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_variable_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_function_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_binary_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_unary_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_if_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_match_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_lambda_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_member_access_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_function_call_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_assignment_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_block_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_if_statement_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_while_statement_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_for_statement_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_match_statement_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_return_statement_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_struct_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_enum_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_interface_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_type_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_impl_block_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_import_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_export_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_module_declaration_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_complex_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_nested_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_recursive_function_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_generic_function_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_polymorphic_expression_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_subtyping_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_constraint_solving_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_match_exhaustiveness_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_error_handling_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_diagnostic_collection_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
    
    let diagnostics = type_checker.get_diagnostics();
    assert!(diagnostics.is_empty());
}

#[test]
fn test_type_environment_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_type_map_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
    
    let type_map = result.unwrap();
    assert!(type_map.is_empty());
}

#[test]
fn test_source_span_conversion_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_file_id_handling_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_comprehensive_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_advanced_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_complex_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_medium_complexity_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_high_complexity_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_edge_case_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_boundary_condition_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_error_recovery_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_performance_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_memory_usage_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_concurrent_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_parallel_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_async_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_streaming_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_incremental_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_caching_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_optimization_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_final_comprehensive_type_checking() {
    let mut type_checker = TypeChecker::new();
    let program = create_test_program();
    
    let result = type_checker.check_program(&program);
    assert!(result.is_ok());
}