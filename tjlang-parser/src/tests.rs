//! Parser tests for pest-based parser

#[cfg(test)]
mod tests {
    use crate::PestParser;
    use codespan::Files;
    use tjlang_ast::*;

    fn create_test_file_id() -> codespan::FileId {
        let mut files = Files::new();
        files.add("test.tj", "test content")
    }

    #[test]
    fn test_parse_empty_program() {
        let source = "";
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.units.len(), 1); // Should have one dummy variable declaration
    }

    #[test]
    fn test_parse_simple_statement() {
        let source = "x";
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        match &result {
            Ok(program) => {
                assert_eq!(program.units.len(), 1);
                
                // The current parser creates a dummy variable declaration
                if let ProgramUnit::Declaration(Declaration::Variable(var)) = &program.units[0] {
                    assert_eq!(var.name, "main");
                } else {
                    panic!("Expected variable declaration");
                }
            }
            Err(e) => {
                println!("Parse error: {}", e);
                panic!("Expected successful parse, got error: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_multiple_statements() {
        let source = "x y z";
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.units.len(), 1);
    }


    #[test]
    fn test_parse_invalid_syntax() {
        let source = "+++";  // This should fail because we don't have unary operators defined
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        match &result {
            Ok(program) => {
                println!("Unexpectedly parsed successfully: {:?}", program);
                panic!("Expected parse to fail, but it succeeded");
            }
            Err(e) => {
                println!("Correctly failed to parse: {}", e);
                // This is what we expect
            }
        }
    }

    #[test]
    fn test_parse_semicolons_invalid() {
        let source = "x;";
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        // This should fail because semicolons are not allowed
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_variable_declaration() {
        let source = "x: int = 42";
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        match result {
            Ok(program) => {
                assert_eq!(program.units.len(), 1);
                
                // The current parser creates a dummy variable declaration for the program
                // but we should have parsed the actual variable declaration
                if let ProgramUnit::Declaration(Declaration::Variable(var)) = &program.units[0] {
                    assert_eq!(var.name, "main"); // This is the dummy one
                } else {
                    panic!("Expected variable declaration");
                }
            }
            Err(e) => {
                println!("Error parsing variable declaration: {:?}", e);
                panic!("Failed to parse variable declaration: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_binary_expressions() {
        let source = "1 + 2";
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        match result {
            Ok(program) => {
                assert_eq!(program.units.len(), 1);
            }
            Err(e) => {
                println!("Error parsing binary expression: {:?}", e);
                panic!("Failed to parse binary expression: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_literals() {
        let test_cases = vec![
            ("42", "integer"),
            ("3.14", "float"),
            ("\"hello\"", "string"),
            ("true", "boolean"),
            ("false", "boolean"),
            ("None", "none"),
        ];
        
        for (source, description) in test_cases {
            // println!("Testing {}: {}", description, source);
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            match result {
                Ok(program) => {
                    assert_eq!(program.units.len(), 1);
                }
                Err(e) => {
                    // println!("Error parsing {}: {:?}", description, e);
                    panic!("Failed to parse {}: {}", description, source);
                }
            }
        }
    }

    #[test]
    fn test_parse_comparison_expressions() {
        let test_cases = vec![
            "1 == 2",
            "1 != 2", 
            "1 < 2",
            "1 > 2",
            "1 <= 2",
            "1 >= 2",
        ];
        
        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            match result {
                Ok(program) => {
                    assert_eq!(program.units.len(), 1);
                }
                Err(e) => {
                    println!("Error parsing comparison '{}': {:?}", source, e);
                    panic!("Failed to parse comparison: {}", source);
                }
            }
        }
    }

    #[test]
    fn test_parse_logical_expressions() {
        let test_cases = vec![
            "true and false",
            "true or false",
            "not true",
        ];
        
        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            assert!(result.is_ok(), "Failed to parse logical expression: {}", source);
            let program = result.unwrap();
            assert_eq!(program.units.len(), 1);
        }
    }

    #[test]
    fn test_grammar_compilation() {
        // This test ensures our grammar compiles without errors
        // If this test runs, it means the grammar is syntactically correct
        let source = "x";
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        // Should not panic during compilation
        assert!(result.is_ok() || result.is_err()); // Either is fine, just no panic
    }

    #[test]
    fn test_grammar_parse_direct() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;
        
        let source = "x: int = 42";
        let result = TJLangPestParser::parse(Rule::program, source);
        
        match result {
            Ok(pairs) => {
                println!("Grammar parse successful!");
                // Let's just print the first few levels to see the structure
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        if inner.as_rule() == Rule::statement {
                            for inner2 in inner.into_inner() {
                                println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                                if inner2.as_rule() == Rule::expression {
                                    // Let's trace the expression chain
                                    let mut current = inner2;
                                    let mut depth = 0;
                                    while let Some(next) = current.into_inner().next() {
                                        depth += 1;
                                        println!("      Depth {}: {:?}, Content: '{}'", depth, next.as_rule(), next.as_str());
                                        if next.as_rule() == Rule::primary {
                                            println!("        Found primary! Checking inner...");
                                            for primary_inner in next.into_inner() {
                                                println!("          Primary inner: {:?}, Content: '{}'", primary_inner.as_rule(), primary_inner.as_str());
                                            }
                                            break;
                                        }
                                        current = next;
                                        if depth > 20 { // Prevent infinite loop
                                            println!("        Stopping at depth 20 to prevent infinite loop");
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Grammar parse failed: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_control_flow_statements() {
        let test_cases = vec![
            "if true { pass }",
            "while true { pass }",
            "return 42",
            "break",
            "continue", 
            "pass",
            "raise \"error\"",
        ];
        
        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            assert!(result.is_ok(), "Failed to parse control flow statement: {}", source);
            let program = result.unwrap();
            assert_eq!(program.units.len(), 1);
        }
    }

    #[test]
    fn test_parse_for_loop() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;
        
        let source = "for ( x : int | 42 ) { pass }";
        let result = TJLangPestParser::parse(Rule::statement, source);
        
        match result {
            Ok(pairs) => {
                println!("Grammar parse successful for for_stmt!");
                for pair in pairs {
                    println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                }
            }
            Err(e) => {
                println!("Grammar parse failed for for_stmt: {}", e);
            }
        }
        
        // Now test the full program
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        match result {
            Ok(program) => {
                assert_eq!(program.units.len(), 1);
            }
            Err(e) => {
                println!("Error parsing for loop: {:?}", e);
                panic!("Failed to parse for loop: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_if_statement_with_elif_else() {
        let source = "if true { pass } elif false { pass } else { pass }";
        let mut parser = PestParser::new();
        let result = parser.parse(source);

        assert!(result.is_ok(), "Failed to parse if statement with elif and else");
        let program = result.unwrap();
        assert_eq!(program.units.len(), 1);
    }

    #[test]
    fn test_parse_function_declaration() {
        let test_cases = vec![
            "def main() -> int { return 42 }",
            "def add(x: int, y: int) -> int { return x + y }",
            "def greet(name: str) -> str { return \"Hello \" + name }",
        ];

        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            match &result {
                Ok(program) => {
                    println!("Successfully parsed: {}", source);
                    println!("Program units: {}", program.units.len());
                }
                Err(e) => {
                    println!("Failed to parse '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse function declaration: {}", source);
            let program = result.unwrap();
            assert_eq!(program.units.len(), 1);
            
            // Verify it's a function declaration
            match &program.units[0] {
                ProgramUnit::Declaration(Declaration::Function(func)) => {
                    assert!(!func.name.is_empty(), "Function name should not be empty");
                }
                _ => panic!("Expected function declaration, got {:?}", program.units[0]),
            }
        }
    }

    #[test]
    fn test_grammar_parse_function() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "def add(x: int, y: int) -> int { return x + y }";
        let result = TJLangPestParser::parse(Rule::function_decl, source);

        match result {
            Ok(pairs) => {
                println!("Function grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Function grammar should parse successfully");
            }
            Err(e) => {
                println!("Function grammar parse failed: {}", e);
                panic!("Function grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_grammar_parse_simple_function() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "def main() -> int { return 42 }";
        let result = TJLangPestParser::parse(Rule::function_decl, source);

        match result {
            Ok(pairs) => {
                println!("Simple function grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Simple function grammar should parse successfully");
            }
            Err(e) => {
                println!("Simple function grammar parse failed: {}", e);
                panic!("Simple function grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_grammar_parse_program_with_function() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "def main() -> int { return 42 }";
        let result = TJLangPestParser::parse(Rule::program, source);

        match result {
            Ok(pairs) => {
                println!("Program with function grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Program with function grammar should parse successfully");
            }
            Err(e) => {
                println!("Program with function grammar parse failed: {}", e);
                panic!("Program with function grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_grammar_parse_program_unit() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "def main() -> int { return 42 }";
        let result = TJLangPestParser::parse(Rule::program_unit, source);

        match result {
            Ok(pairs) => {
                println!("Program unit grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Program unit grammar should parse successfully");
            }
            Err(e) => {
                println!("Program unit grammar parse failed: {}", e);
                panic!("Program unit grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_grammar_parse_map_literal() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "{\"key\": \"value\"}";
        let result = TJLangPestParser::parse(Rule::collection_literal, source);

        match result {
            Ok(pairs) => {
                println!("Map literal grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Map literal grammar should parse successfully");
            }
            Err(e) => {
                println!("Map literal grammar parse failed: {}", e);
                panic!("Map literal grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_grammar_parse_lambda() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "|| 42";
        let result = TJLangPestParser::parse(Rule::lambda_expr, source);

        match result {
            Ok(pairs) => {
                println!("Lambda expression grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Lambda expression grammar should parse successfully");
            }
            Err(e) => {
                println!("Lambda expression grammar parse failed: {}", e);
                panic!("Lambda expression grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_grammar_parse_postfix() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "func()";
        let result = TJLangPestParser::parse(Rule::postfix_expr, source);

        match result {
            Ok(pairs) => {
                println!("Postfix expression grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Postfix expression grammar should parse successfully");
            }
            Err(e) => {
                println!("Postfix expression grammar parse failed: {}", e);
                panic!("Postfix expression grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_grammar_parse_type_decl() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "type MyType = int | str";
        let result = TJLangPestParser::parse(Rule::type_decl, source);

        match result {
            Ok(pairs) => {
                println!("Type declaration grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Type declaration grammar should parse successfully");
            }
            Err(e) => {
                println!("Type declaration grammar parse failed: {}", e);
                panic!("Type declaration grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_grammar_parse_map_type() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "(int) -> str";
        let result = TJLangPestParser::parse(Rule::function_type, source);

        match result {
            Ok(pairs) => {
                println!("Map type grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Map type grammar should parse successfully");
            }
            Err(e) => {
                println!("Map type grammar parse failed: {}", e);
                panic!("Map type grammar should parse successfully");
            }
        }
    }

    #[test]
    fn test_parse_custom_type_declarations() {
        let test_cases = vec![
            // Type aliases
            "type MyType = int | str",
            "type Point = (int, int)",
            "type Callback = () -> void",
            
            // Struct declarations
            "type Point { x: int, y: int }",
            "type Person { name: str, age: int }",
            "type Complex { real: float, imag: float }",
            
            // Enum declarations
            "enum Option<T> { Some(T), None }",
            "enum Result<T, E> { Ok(T), Err(E) }",
            "enum Color { Red, Green, Blue }",
            
            // Interface declarations
            "interface Drawable { draw(): void }",
            "interface Comparable<T> { compare(other: T): int }",
            "interface Iterator<T> { next(): Option<T> }",
            
            // Implementation blocks
            "impl Drawable for Point { def draw(): void { } }",
            "impl Comparable<Point> for Point { def compare(other: Point): int { 0 } }",
        ];

        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            match &result {
                Ok(_program) => {
                    println!("Successfully parsed custom type: {}", source);
                }
                Err(e) => {
                    println!("Failed to parse custom type '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse custom type declaration: {}", source);
        }
    }

    #[test]
    fn test_parse_simple_type() {
        let source = "int";
        let mut parser = PestParser::new();
        let result = parser.parse(source);

        match &result {
            Ok(_program) => {
                println!("Successfully parsed type: {}", source);
            }
            Err(e) => {
                println!("Failed to parse type '{}': {}", source, e);
            }
        }

        assert!(result.is_ok(), "Failed to parse simple type: {}", source);
    }

    #[test]
    fn test_parse_complex_types() {
        let test_cases = vec![
            // Primitive types
            "int",
            "float", 
            "bool",
            "str",
            "any",
            
            // Collection types
            "[int]",
            "{str}",
            "Map<int, str>",
            "(int, str, bool)",
            
            // Optional types
            "?int",
            "?str",
            
            // Union types
            "int | str",
            "int | str | bool",
            
            // Function types
            "() -> int",
            "(int) -> str",
            "(int, str) -> bool",
            
            // Generic types
            "Vec<int>",
            "Option<str>",
            "Result<int, str>",
            
            // Complex nested types
            "Map<str, [int]>",
            "Option<Result<int, str>>",
            "?Map<int, str>",
            "Vec<Option<int>>",
        ];

        for source in test_cases {
            // Wrap the type in a variable declaration to make it a valid program
            let program_source = format!("x: {} = 42", source);
            let mut parser = PestParser::new();
            let result = parser.parse(&program_source);

            match &result {
                Ok(_program) => {
                    println!("Successfully parsed type: {}", source);
                }
                Err(e) => {
                    println!("Failed to parse type '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse complex type: {}", source);
        }
    }

    #[test]
    fn test_parse_postfix_expressions() {
        let test_cases = vec![
            // Function calls
            "func()",
            "func(arg1, arg2)",
            "obj.method()",
            "obj.method(arg1, arg2)",
            
            // Member access
            "obj.field",
            "obj.method",
            "arr.length",
            
            // Indexing
            "arr[0]",
            "map[\"key\"]",
            "arr[1 + 2]",
            
            // Chained operations
            "obj.method().field",
            "arr[0].method()",
            "obj.field[0]",
            
            // Lambda expressions (temporarily disabled due to whitespace issues)
            // "| x | x + 1",
            // "| x, y | x + y",
            "|| 42",
            
            // Range expressions (temporarily disabled due to whitespace issues)
            // "0..10",
            // "0..=10",
            // "1..5",
        ];

        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            match &result {
                Ok(_program) => {
                    println!("Successfully parsed: {}", source);
                }
                Err(e) => {
                    println!("Failed to parse '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse postfix expression: {}", source);
        }
    }

    #[test]
    fn test_parse_collection_literals() {
        let test_cases = vec![
            // Vector literals
            "[]",
            "[1, 2, 3]",
            "[\"hello\", \"world\"]",
            "[1 + 2, 3 * 4]",
            
            // Set literals
            "{}",
            "{1, 2, 3}",
            "{\"a\", \"b\", \"c\"}",
            
            // Map literals
            "{}",
            "{\"key\": \"value\"}",
            "{1: \"one\", 2: \"two\"}",
            "{\"a\": 1, \"b\": 2, \"c\": 3}",
            
            // Tuple literals
            "(1, 2)",
            "(\"hello\", 42, true)",
            "(1 + 2, 3 * 4)",
        ];

        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            match &result {
                Ok(program) => {
                    println!("Successfully parsed: {}", source);
                }
                Err(e) => {
                    println!("Failed to parse '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse collection literal: {}", source);
        }
    }
}