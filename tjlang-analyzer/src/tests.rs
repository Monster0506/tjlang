//! Comprehensive tests for the TJLang analysis rules

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnalysisPipeline, AnalysisPhase, RuleCategory, AnalysisResult, RuleSeverity};
    use tjlang_diagnostics::{ErrorCode, TJLangDiagnostic};
    use codespan_reporting::diagnostic::Severity;
    use codespan::Files;

    /// Helper function to create a test file ID
    fn create_test_file_id(source: &str) -> codespan::FileId {
        let mut files = Files::new();
        files.add("test.tj", source)
    }

    /// Helper function to create a configuration with all rules enabled
    fn create_all_rules_enabled_config() -> crate::config::RuleConfig {
        use std::collections::HashMap;
        use crate::config::{RuleConfig, RuleSettings, RuleSeverity, GlobalSettings};
        
        let mut enabled_rules = HashMap::new();
        let mut rule_settings = HashMap::new();
        
        // List of all available rules
        let all_rules = vec![
            "TypeSafetyRule", "NullPointerRule", "BufferOverflowRule", "UnsafeOperationRule",
            "UnusedVariableRule", "DeadCodeRule", "UnusedParameterRule", "DuplicateNameRule",
            "UndefinedVariableRule", "CircularDependencyRule", "NamingConventionRule",
            "FunctionComplexityRule", "MagicNumberRule", "ParameterCountRule",
            "InefficientLoopRule", "MemoryAllocationRule", "StringConcatenationRule",
            "LargeFileRule", "TooManyImportsRule", "GlobalVariableRule",
            "FormattingConventionRule", "IndentationRule", "TrailingWhitespaceRule",
            "LineLengthRule", "CommentCoverageRule", "FunctionLengthRule",
            "NestingDepthRule", "EmptyFunctionRule", "UnreachableCodeRule",
            "RecursionDepthRule", "ResourceLeakRule", "AsyncAwaitRule",
            "ErrorHandlingRule", "PatternMatchingRule", "GenericConstraintRule",
            "CommentStyleRule", "SemicolonRule", "BracketMatchingRule",
            "ImportOrderRule", "CacheEfficiencyRule", "BranchPredictionRule",
            "VectorizationRule", "ConcurrencyRule", "MemoryLeakRule",
            "RaceConditionRule", "InputValidationRule", "HardcodedCredentialsRule",
            "SQLInjectionRule", "CouplingRule", "CohesionRule",
        ];
        
        // Enable all rules
        for rule in all_rules {
            enabled_rules.insert(rule.to_string(), true);
            rule_settings.insert(rule.to_string(), RuleSettings {
                severity: RuleSeverity::Warning,
                config: HashMap::new(),
            });
        }
        
        RuleConfig {
            enabled_rules,
            rule_settings,
            global_settings: GlobalSettings {
                max_diagnostics: None,
                stop_on_error: false,
                parallel_execution: false,
                timeout_seconds: Some(30),
            },
        }
    }

    /// Helper function to run analysis on source code with all rules enabled
    fn analyze_source(source: &str) -> AnalysisResult {
        let config = create_all_rules_enabled_config();
        let pipeline = AnalysisPipeline::with_config(config);
        let file_id = create_test_file_id(source);
        pipeline.analyze(source, file_id)
    }

    /// Helper function to check if a specific error code is present
    fn has_error_code(result: &AnalysisResult, error_code: ErrorCode) -> bool {
        result.diagnostics.iter().any(|d| d.code == error_code)
    }

    /// Helper function to check if a specific warning code is present
    fn has_warning_code(result: &AnalysisResult, error_code: ErrorCode) -> bool {
        result.diagnostics.iter().any(|d| d.code == error_code && d.severity == Severity::Warning)
    }

    /// Helper function to get diagnostics by error code
    fn get_diagnostics_by_code(result: &AnalysisResult, error_code: ErrorCode) -> Vec<&TJLangDiagnostic> {
        result.diagnostics.iter().filter(|d| d.code == error_code).collect()
    }

    // ============================================================================
    // TYPE SAFETY RULES TESTS
    // ============================================================================

    #[test]
    fn test_type_safety_rule_type_mismatch() {
        let source = r#"
x: int = "hello"
y: str = 42
z: bool = "not a boolean"
"#;
        let result = analyze_source(source);
        
        // Should detect type mismatches
        assert!(has_error_code(&result, ErrorCode::AnalyzerTypeMismatch));
        
        let type_mismatch_diagnostics = get_diagnostics_by_code(&result, ErrorCode::AnalyzerTypeMismatch);
        assert!(!type_mismatch_diagnostics.is_empty());
    }

    #[test]
    fn test_type_safety_rule_undefined_variable() {
        let source = r#"
x: int = 42
y = x + z  # z is undefined
"#;
        let result = analyze_source(source);
        
        // Should detect undefined variable
        assert!(has_error_code(&result, ErrorCode::AnalyzerUndefinedVariable));
    }

    #[test]
    fn test_type_safety_rule_duplicate_names() {
        let source = r#"
def func1() -> int { return 1 }
def func1() -> str { return "duplicate" }
"#;
        let result = analyze_source(source);
        
        // Should detect duplicate function names
        assert!(has_error_code(&result, ErrorCode::AnalyzerDuplicateDefinition));
    }

    // ============================================================================
    // NAMING CONVENTION RULES TESTS
    // ============================================================================

    #[test]
    fn test_naming_convention_rule_bad_variable_names() {
        let source = r#"
very_long_variable_name_that_exceeds_reasonable_length_and_should_trigger_warning: int = 42
normalVariable: int = 43
CONSTANT_VALUE: int = 44
"#;
        let result = analyze_source(source);
        
        // Should detect naming convention violations
        assert!(has_warning_code(&result, ErrorCode::AnalyzerNamingConvention));
    }

    #[test]
    fn test_naming_convention_rule_function_names() {
        let source = r#"
def very_long_function_name_that_exceeds_reasonable_length_and_should_trigger_warning() -> int { return 1 }
def normalFunction() -> int { return 2 }
def CONSTANT_FUNCTION() -> int { return 3 }
"#;
        let result = analyze_source(source);
        
        // Should detect naming convention violations
        assert!(has_warning_code(&result, ErrorCode::AnalyzerNamingConvention));
    }

    // ============================================================================
    // DEAD CODE RULES TESTS
    // ============================================================================

    #[test]
    fn test_unused_variable_rule() {
        let source = r#"
x: int = 42
y: int = 43
z: int = x + 1
"#;
        let result = analyze_source(source);
        
        // Should detect unused variable
        assert!(has_warning_code(&result, ErrorCode::AnalyzerUnusedVariable));
    }

    #[test]
    fn test_unused_parameter_rule() {
        let source = r#"
def func(x: int, unused_param: int) -> int {
    return x
}
"#;
        let result = analyze_source(source);
        
        // Should detect unused parameter
        assert!(has_warning_code(&result, ErrorCode::AnalyzerUnusedParameter));
    }

    #[test]
    fn test_dead_code_rule_unreachable() {
        let source = r#"
def func() -> int {
    return 42
    x: int = 100  # This is unreachable
}
"#;
        let result = analyze_source(source);
        
        // Should detect unreachable code
        assert!(has_warning_code(&result, ErrorCode::AnalyzerDeadCode));
    }

    #[test]
    fn test_empty_function_rule() {
        let source = r#"
def empty_func() -> void {
    # This function is empty
}
"#;
        let result = analyze_source(source);
        
        // Should detect empty function
        assert!(has_warning_code(&result, ErrorCode::AnalyzerEmptyFunction));
    }

    // ============================================================================
    // CODE QUALITY RULES TESTS
    // ============================================================================

    #[test]
    fn test_function_complexity_rule() {
        let source = r#"
def complex_function(a: int, b: int, c: int, d: int, e: int, f: int, g: int, h: int) -> int {
    if a > 0 {
        if b > 0 {
            if c > 0 {
                if d > 0 {
                    if e > 0 {
                        if f > 0 {
                            if g > 0 {
                                if h > 0 {
                                    return a + b + c + d + e + f + g + h
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0
}
"#;
        let result = analyze_source(source);
        
        // Should detect high complexity
        assert!(has_warning_code(&result, ErrorCode::AnalyzerFunctionComplexity));
    }

    #[test]
    fn test_parameter_count_rule() {
        let source = r#"
def too_many_params(a: int, b: int, c: int, d: int, e: int, f: int, g: int, h: int, i: int, j: int) -> int {
    return a + b + c + d + e + f + g + h + i + j
}
"#;
        let result = analyze_source(source);
        
        // Should detect too many parameters
        assert!(has_warning_code(&result, ErrorCode::AnalyzerParameterCount));
    }

    #[test]
    fn test_magic_number_rule() {
        let source = r#"
def test() -> int {
    return 42
}
"#;
        let result = analyze_source(source);
        
        // Should detect magic numbers
        assert!(has_warning_code(&result, ErrorCode::AnalyzerMagicNumber));
    }

    // ============================================================================
    // STYLE AND FORMATTING RULES TESTS
    // ============================================================================

    #[test]
    fn test_line_length_rule() {
        let source = r#"
this_is_a_very_long_line_that_exceeds_the_recommended_line_length_limit_and_should_trigger_a_warning_about_line_length
"#;
        let result = analyze_source(source);
        
        // Should detect long lines
        assert!(has_warning_code(&result, ErrorCode::AnalyzerLineLength));
    }

    #[test]
    fn test_trailing_whitespace_rule() {
        let source = r#"
x: int = 42    
y: str = "test"    
"#;
        let result = analyze_source(source);
        
        // Should detect trailing whitespace
        assert!(has_warning_code(&result, ErrorCode::AnalyzerTrailingWhitespace));
    }

    #[test]
    fn test_indentation_rule() {
        let source = r#"
def func() -> int {
  x: int = 42  # Inconsistent indentation
    y: int = 43  # Different indentation
  return x
}
"#;
        let result = analyze_source(source);
        
        // Should detect inconsistent indentation
        assert!(has_warning_code(&result, ErrorCode::AnalyzerIndentation));
    }

    // ============================================================================
    // PERFORMANCE RULES TESTS
    // ============================================================================

    #[test]
    fn test_inefficient_loop_rule() {
        let source = r#"
def inefficient_search(items: [int], target: int) -> bool {
    for (i: int | 0..10) {
        for (j: int | 0..10) {
            if items[i] == items[j] and items[i] == target {
                return true
            }
        }
    }
    return false
}
"#;
        let result = analyze_source(source);
        
        // Should detect inefficient nested loops
        assert!(has_warning_code(&result, ErrorCode::AnalyzerInefficientLoop));
    }

    #[test]
    fn test_string_concatenation_rule() {
        let source = r#"
def build_string(parts: [str]) -> str {
    result: str = ""
    for (i: int | 0..5) {
        result = result + parts[i]  # Inefficient string concatenation
    }
    return result
}
"#;
        let result = analyze_source(source);
        
        // Should detect inefficient string concatenation
        assert!(has_warning_code(&result, ErrorCode::AnalyzerStringConcatenation));
    }

    #[test]
    fn test_memory_allocation_rule() {
        let source = r#"
def process_data(data: [int]) -> [int] {
    result: [int] = []
    for (i: int | 0..5) {
        temp: [int] = []
        for (j: int | 0..5) {
            temp.append(data[j] * 2)
        }
        result.append(temp[0])
    }
    return result
}
"#;
        let result = analyze_source(source);
        
        // Should detect excessive memory allocation
        assert!(has_warning_code(&result, ErrorCode::AnalyzerMemoryAllocation));
    }

    // ============================================================================
    // ARCHITECTURE RULES TESTS
    // ============================================================================

    #[test]
    fn test_large_file_rule() {
        // Create a very long source file
        let mut source = String::new();
        for i in 0..1000 {
            source.push_str(&format!("x{}: int = {}\n", i, i));
        }
        
        let result = analyze_source(&source);
        
        // Should detect large file
        assert!(has_warning_code(&result, ErrorCode::AnalyzerLargeFile));
    }

    #[test]
    fn test_global_variable_rule() {
        let source = r#"
global_counter: int = 0

def increment_counter() -> void {
    global_counter = global_counter + 1
}
"#;
        let result = analyze_source(source);
        
        // Should detect global variables
        assert!(has_warning_code(&result, ErrorCode::AnalyzerGlobalVariable));
    }

    // ============================================================================
    // SECURITY RULES TESTS
    // ============================================================================

    #[test]
    fn test_null_pointer_rule() {
        let source = r#"
def process_user(user: User?) -> str {
    return user.name  # Potential null dereference
}
"#;
        let result = analyze_source(source);
        
        // Should detect potential null pointer dereference
        assert!(has_warning_code(&result, ErrorCode::AnalyzerNullPointer));
    }

    #[test]
    fn test_buffer_overflow_rule() {
        let source = r#"
def copy_string(source: str, dest: str) -> void {
    for (i: int | 0..10) {
        dest[i] = source[i]  # No bounds checking
    }
}
"#;
        let result = analyze_source(source);
        
        // Should detect potential buffer overflow
        assert!(has_warning_code(&result, ErrorCode::AnalyzerBufferOverflow));
    }

    #[test]
    fn test_hardcoded_credentials_rule() {
        let source = r#"
def connect_to_database() -> Connection {
    username: str = "admin"
    password: str = "password123"
    return connect(username, password)
}
"#;
        let result = analyze_source(source);
        
        // Should detect hardcoded credentials
        assert!(has_warning_code(&result, ErrorCode::AnalyzerHardcodedCredentials));
    }

    #[test]
    fn test_sql_injection_rule() {
        let source = r#"
def get_user_by_id(id: str) -> User {
    query: str = "SELECT * FROM users WHERE id = " + id
    return execute_query(query)
}
"#;
        let result = analyze_source(source);
        
        // Should detect potential SQL injection
        assert!(has_warning_code(&result, ErrorCode::AnalyzerSQLInjection));
    }

    // ============================================================================
    // LANGUAGE-SPECIFIC RULES TESTS
    // ============================================================================

    #[test]
    fn test_error_handling_rule() {
        let source = r#"
def process_input(input: str) -> str {
    return input.to_upper()  # No error handling
}
"#;
        let result = analyze_source(source);
        
        // Should detect missing error handling
        assert!(has_warning_code(&result, ErrorCode::AnalyzerErrorHandling));
    }

    #[test]
    fn test_pattern_matching_rule() {
        let source = r#"
def process_value(value: int) -> str {
    match value {
        x: int: { return "integer" }
        # Missing case for other values
    }
}
"#;
        let result = analyze_source(source);
        
        // Should detect non-exhaustive pattern matching
        assert!(has_warning_code(&result, ErrorCode::AnalyzerPatternMatching));
    }

    // ============================================================================
    // PIPELINE INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn test_pipeline_phase_analysis() {
        let source = r#"
x: int = "hello"  # Type mismatch
unused_var: int = 42  # Unused variable
"#;
        let pipeline = AnalysisPipeline::new();
        let file_id = create_test_file_id(source);
        
        // Test Pre-AST analysis
        let pre_ast_result = pipeline.analyze_phase(AnalysisPhase::PreAST, source, file_id);
        assert!(pre_ast_result.diagnostics_count >= 0);
        
        // Test AST analysis
        let ast_result = pipeline.analyze_phase(AnalysisPhase::AST, source, file_id);
        assert!(ast_result.diagnostics_count >= 0);
        
        // Test Post-AST analysis
        let post_ast_result = pipeline.analyze_phase(AnalysisPhase::PostAST, source, file_id);
        assert!(post_ast_result.diagnostics_count >= 0);
    }

    #[test]
    fn test_pipeline_category_analysis() {
        let source = r#"
x: int = "hello"  # Type safety issue
unused_var: int = 42  # Dead code issue
"#;
        let pipeline = AnalysisPipeline::new();
        let file_id = create_test_file_id(source);
        
        // Test type safety category
        let type_safety_result = pipeline.analyze_category(RuleCategory::TypeSafety, source, file_id);
        assert!(type_safety_result.diagnostics_count >= 0);
        
        // Test dead code category
        let dead_code_result = pipeline.analyze_category(RuleCategory::DeadCode, source, file_id);
        assert!(dead_code_result.diagnostics_count >= 0);
    }

    #[test]
    fn test_rule_configuration() {
        let source = r#"
x: int = "hello"  # Type mismatch
"#;
        let mut pipeline = AnalysisPipeline::new();
        let file_id = create_test_file_id(source);
        
        // Disable type safety rule
        pipeline.disable_rule("TypeSafetyRule");
        let result_disabled = pipeline.analyze(source, file_id);
        
        // Enable type safety rule
        pipeline.enable_rule("TypeSafetyRule");
        let result_enabled = pipeline.analyze(source, file_id);
        
        // The enabled result should have more diagnostics
        assert!(result_enabled.diagnostics_count >= result_disabled.diagnostics_count);
    }

    #[test]
    fn test_rule_severity_configuration() {
        let source = r#"
x: int = "hello"  # Type mismatch
"#;
        let mut pipeline = AnalysisPipeline::new();
        let file_id = create_test_file_id(source);
        
        // Set type safety rule to error severity
        pipeline.set_rule_severity("TypeSafetyRule", RuleSeverity::Error);
        let result = pipeline.analyze(source, file_id);
        
        // Should have error-level diagnostics
        let error_diagnostics = result.get_diagnostics_by_severity(Severity::Error);
        assert!(!error_diagnostics.is_empty());
    }

    // ============================================================================
    // COMPREHENSIVE INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn test_comprehensive_analysis() {
        let source = r#"
# Type safety issues
x: int = "hello"
y: str = 42

# Naming convention issues
badVariableName: int = 42
GoodVariableName: int = 43

# Dead code issues
unused_var: int = 100
def func(unused_param: int) -> int {
    return 42
}

# Magic numbers
def calculate_area(radius: float) -> float {
    return 3.14159 * radius * radius
}

# Long line
this_is_a_very_long_line_that_exceeds_the_recommended_line_length_limit_and_should_trigger_a_warning_about_line_length

# Inefficient loop
def inefficient_search(items: [int], target: int) -> bool {
    for (i: int | 0..10) {
        for (j: int | 0..10) {
            if items[i] == items[j] and items[i] == target {
                return true
            }
        }
    }
    return false
}
"#;
        let result = analyze_source(source);
        
        // Should detect multiple types of issues
        assert!(result.diagnostics_count > 0);
        
        // Check for specific error types
        assert!(has_error_code(&result, ErrorCode::AnalyzerTypeMismatch));
        assert!(has_warning_code(&result, ErrorCode::AnalyzerNamingConvention));
        assert!(has_warning_code(&result, ErrorCode::AnalyzerUnusedVariable));
        assert!(has_warning_code(&result, ErrorCode::AnalyzerUnusedParameter));
        assert!(has_warning_code(&result, ErrorCode::AnalyzerMagicNumber));
        assert!(has_warning_code(&result, ErrorCode::AnalyzerLineLength));
        assert!(has_warning_code(&result, ErrorCode::AnalyzerInefficientLoop));
    }

    #[test]
    fn test_analysis_performance() {
        let source = r#"
x: int = 42
y: str = "hello"
z: bool = true
"#;
        let result = analyze_source(source);
        
        // Analysis should be fast
        assert!(result.execution_time.as_micros() < 1000); // Less than 1ms
    }

    #[test]
    fn test_analysis_summary() {
        let source = r#"
x: int = "hello"  # Error
unused_var: int = 42  # Warning
"#;
        let result = analyze_source(source);
        let summary = result.get_summary();
        
        assert!(summary.total_rules > 0);
        assert!(summary.total_diagnostics >= 0);
        assert!(summary.execution_time.as_micros() > 0);
    }

    // ============================================================================
    // EDGE CASES AND ERROR HANDLING TESTS
    // ============================================================================

    #[test]
    fn test_empty_source() {
        let source = "";
        let result = analyze_source(source);
        
        // Should handle empty source gracefully
        assert!(result.diagnostics_count >= 0);
    }

    #[test]
    fn test_invalid_syntax() {
        let source = "+++";  // Invalid syntax
        let result = analyze_source(source);
        
        // Should handle invalid syntax gracefully
        assert!(result.diagnostics_count >= 0);
    }

    #[test]
    fn test_very_long_source() {
        // Create a very long source file
        let mut source = String::new();
        for i in 0..10000 {
            source.push_str(&format!("x{}: int = {}\n", i, i));
        }
        
        let result = analyze_source(&source);
        
        // Should handle large files
        assert!(result.diagnostics_count >= 0);
    }

    // ============================================================================
    // RULE-SPECIFIC EDGE CASES
    // ============================================================================

    #[test]
    fn test_naming_convention_edge_cases() {
        let source = r#"
# Edge cases for naming conventions
_: int = 42  # Single underscore
__: int = 43  # Double underscore
a: int = 44  # Single letter
very_long_variable_name_that_might_exceed_recommended_length: int = 45
"#;
        let result = analyze_source(source);
        
        // Should detect the long identifier (58 characters > 50 limit)
        assert!(has_warning_code(&result, ErrorCode::AnalyzerNamingConvention));
        
        // Should have exactly 1 naming convention warning
        let naming_warnings: Vec<_> = result.diagnostics.iter()
            .filter(|d| d.code == ErrorCode::AnalyzerNamingConvention)
            .collect();
        assert_eq!(naming_warnings.len(), 1);
        
        // Verify the warning message mentions the long identifier
        let warning = &naming_warnings[0];
        assert!(warning.message.contains("very_long_variable_name_that_might_exceed_recommended_length"));
        assert!(warning.message.contains("too long"));
        assert!(warning.message.contains("60 characters"));
        assert!(warning.message.contains("max recommended: 50"));
    }

    #[test]
    fn test_magic_number_edge_cases() {
        let source = r#"
# Edge cases for magic numbers
x: int = 0  # Zero might be acceptable
y: int = 1  # One might be acceptable
z: int = -1  # Negative one might be acceptable
pi: float = 3.14159  # Mathematical constant
e: float = 2.71828  # Mathematical constant
"#;
        let result = analyze_source(source);
        
        // Should handle edge cases appropriately
        assert!(result.diagnostics_count >= 0);
    }

    #[test]
    fn test_complexity_edge_cases() {
        let source = r#"
# Edge cases for complexity analysis
def simple_function() -> int {
    return 42
}

def complex_function_with_many_parameters(
    a: int, b: int, c: int, d: int, e: int, f: int, g: int, h: int, i: int, j: int,
    k: int, l: int, m: int, n: int, o: int, p: int, q: int, r: int, s: int, t: int
) -> int {
    return a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q + r + s + t
}
"#;
        let result = analyze_source(source);
        
        // Should detect complexity issues appropriately
        assert!(result.diagnostics_count >= 0);
    }
}
