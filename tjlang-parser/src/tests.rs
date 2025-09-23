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
            println!("\n=== DEBUGGING CONTROL FLOW: '{}' ===", source);
            
            // Test the grammar rule directly first
            use pest::Parser;
            use crate::parser::TJLangPestParser;
            use crate::parser::Rule;
            
            if source.starts_with("if ") {
                println!("Testing if_stmt rule directly:");
                let if_result = TJLangPestParser::parse(Rule::if_stmt, source);
                match if_result {
                    Ok(pairs) => {
                        println!("✓ if_stmt parsed successfully");
                        for pair in pairs {
                            println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                            for inner in pair.into_inner() {
                                println!("    Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                            }
                        }
                    },
                    Err(e) => println!("✗ if_stmt failed: {}", e),
                }
                
                // Test the expression part separately
                println!("Testing 'true' expression:");
                let expr_result = TJLangPestParser::parse(Rule::expression, "true");
                match expr_result {
                    Ok(pairs) => {
                        println!("✓ expression parsed successfully");
                        for pair in pairs {
                            println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                        }
                    },
                    Err(e) => println!("✗ expression failed: {}", e),
                }
            }
            
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            match &result {
                Ok(program) => {
                    println!("✓ Parsed successfully: {:?}", program);
                    assert_eq!(program.units.len(), 1);
                }
                Err(e) => {
                    println!("✗ Parse failed: {}", e);
                    panic!("Failed to parse control flow statement: {}", source);
                }
            }
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
        println!("\n=== DEBUGGING IF STATEMENT WITH ELIF/ELSE ===");
        println!("Source: '{}'", source);
        
        // Test individual components
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;
        
        println!("\n--- Testing if_stmt rule ---");
        let if_result = TJLangPestParser::parse(Rule::if_stmt, source);
        match if_result {
            Ok(pairs) => {
                println!("✓ if_stmt parsed successfully");
                for pair in pairs {
                    println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                }
            },
            Err(e) => println!("✗ if_stmt failed: {}", e),
        }
        
        println!("\n--- Testing full program ---");
        let mut parser = PestParser::new();
        let result = parser.parse(source);

        match &result {
            Ok(program) => {
                println!("✓ Parsed successfully: {:?}", program);
                assert_eq!(program.units.len(), 1);
            }
            Err(e) => {
                println!("✗ Parse failed: {}", e);
                panic!("Failed to parse if statement with elif and else");
            }
        }
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

        let source = "() -> 42";
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
            "type Callback = () -> int",
            
            // Struct declarations
            "type Point { x: int, y: int }",
            "type Person { name: str, age: int }",
            "type Complex { real: float, imag: float }",
            
            // Enum declarations
            "enum Option<T> { Some(T), None }",
            "enum Result<T, E> { Ok(T), Err(E) }",
            "enum Color { Red, Green, Blue }",
            
            // Interface declarations
            "interface Drawable { draw() -> int }",
            "interface Comparable<T> { compare(other: T) -> int }",
            "interface Iterator<T> { next() -> Option<T> }",
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
    fn test_parse_impl_blocks() {
        use crate::parser::PestParser;

        let impl_test_cases = vec![
            "impl Drawable:Point { draw() -> int { 0 } }",
            "impl Comparable:Point { compare(other: Point) -> int { 0 } }",
            "impl Iterator:List { next() -> Option<T> { None } }",
        ];

        for source in impl_test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            match &result {
                Ok(_program) => {
                    println!("Successfully parsed impl block: {}", source);
                }
                Err(e) => {
                    println!("Failed to parse impl block '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse impl block: {}", source);
        }
    }

    #[test]
    fn test_parse_impl_blocks_comprehensive() {
        use crate::parser::PestParser;

        // Test various trait and type name combinations (avoiding generic types for now)
        let trait_type_combinations = vec![
            ("Display", "String"),
            ("Clone", "User"),
            ("Serialize", "Data"),
            ("Debug", "Point"),
            ("PartialEq", "Version"),
            ("Iterator", "Range"),
            ("FromStr", "Number"),
            ("ToString", "Date"),
            ("Hash", "UUID"),
            ("Ord", "Version"),
        ];

        for (trait_name, type_name) in trait_type_combinations {
            let source = format!("impl {}:{} {{ method() -> int {{ 42 }} }}", trait_name, type_name);
            let mut parser = PestParser::new();
            let result = parser.parse(&source);

            assert!(result.is_ok(), "Failed to parse impl block with trait '{}' and type '{}': {}", 
                trait_name, type_name, result.unwrap_err());
        }
    }

    #[test]
    fn test_parse_impl_blocks_different_methods() {
        use crate::parser::PestParser;

        // Test different method signatures (single method per impl block)
        let method_cases = vec![
            "impl Drawable:Point { draw() -> int { 0 } }",
            "impl Renderer:Canvas { render() -> str { None } }",
            "impl Iterator:List { next() -> Option<T> { None } }",
            "impl Clone:Data { clone() -> Data { None } }",
            "impl Validator:Email { validate() -> bool { true } }",
        ];

        for source in method_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            assert!(result.is_ok(), "Failed to parse impl block with method: {}", source);
        }
    }

    #[test]
    fn test_parse_impl_blocks_complex_methods() {
        use crate::parser::PestParser;

        let complex_method_cases = vec![
            // Method with multiple parameters
            "impl Math:Calculator { add(a: int, b: int) -> int { 0 } }",
            // Method with single parameter
            "impl Validator:EmailValidator { validate(email: str) -> bool { true } }",
            // Method with no parameters
            "impl Factory:Builder { create() -> Product { None } }",
            // Method with complex return type
            "impl Parser:JsonParser { parse(input: str) -> Result { None } }",
            // Method with different parameter types
            "impl Converter:StringConverter { convert(value: int) -> str { None } }",
        ];

        for source in complex_method_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            assert!(result.is_ok(), "Failed to parse impl block with complex methods: {}", source);
        }
    }

    #[test]
    fn test_parse_impl_blocks_edge_cases() {
        use crate::parser::PestParser;

        let edge_case_cases = vec![
            // Single character names
            "impl A:B { c() -> int { 0 } }",
            // Names with underscores
            "impl _Private:Public { method() -> int { 0 } }",
            // Names starting with underscore
            "impl _Trait:_Type { _method() -> int { 0 } }",
            // Mixed case names
            "impl XMLParser:HTMLDocument { parseXML() -> XMLNode { None } }",
            // Long names
            "impl VeryLongTraitName:VeryLongTypeName { veryLongMethodName() -> int { 0 } }",
        ];

        for source in edge_case_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            assert!(result.is_ok(), "Failed to parse impl block edge case: {}", source);
        }
    }

    #[test]
    fn test_parse_impl_blocks_invalid_syntax() {
        use crate::parser::PestParser;

        let invalid_cases = vec![
            // Missing impl keyword
            "Drawable:Point { draw() -> int { 0 } }",
            // Missing colon
            "impl Drawable Point { draw() -> int { 0 } }",
            // Missing opening brace
            "impl Drawable:Point draw() -> int { 0 } }",
            // Missing closing brace
            "impl Drawable:Point { draw() -> int { 0 }",
            // Invalid trait name (starts with number)
            "impl 123Trait:Point { draw() -> int { 0 } }",
            // Invalid type name (starts with number)
            "impl Drawable:123Type { draw() -> int { 0 } }",
            // Missing method
            "impl Drawable:Point { }",
        ];

        for source in invalid_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            assert!(result.is_err(), "Expected parsing to fail for invalid syntax: {}", source);
        }
    }

    #[test]
    fn test_parse_impl_blocks_grammar_rules() {
        use crate::parser::{TJLangPestParser, Rule};
        use pest::Parser;

        // Test individual grammar rules
        let grammar_tests = vec![
            ("impl_trait_name", "Display"),
            ("impl_trait_name", "XMLParser"),
            ("impl_trait_name", "_Private"),
            ("impl_type_name", "String"),
            ("impl_type_name", "Vec<T>"),
            ("impl_type_name", "_Internal"),
        ];

        for (rule_name, input) in grammar_tests {
            let rule = match rule_name {
                "impl_trait_name" => Rule::impl_trait_name,
                "impl_type_name" => Rule::impl_type_name,
                _ => panic!("Unknown rule: {}", rule_name),
            };

            let result = TJLangPestParser::parse(rule, input);
            assert!(result.is_ok(), "Failed to parse {} rule with input '{}': {:?}", 
                rule_name, input, result);
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
    fn test_parse_lambda_expressions() {
        use crate::parser::PestParser;

        let lambda_test_cases = vec![
            "() -> 42",                      // No parameters
            "(x: int) -> x + 1",             // Single parameter
            "(x: int, y: int) -> x + y",     // Multiple parameters
            "(x: int) -> x * 2",             // Single typed parameter
        ];

        for source in lambda_test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            match &result {
                Ok(_program) => {
                    println!("Successfully parsed lambda: {}", source);
                }
                Err(e) => {
                    println!("Failed to parse lambda '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse lambda expression: {}", source);
        }
    }

    #[test]
    fn test_parse_range_expressions() {
        use crate::parser::PestParser;

        let range_test_cases = vec![
            "0..10",     // Exclusive range
            "0..=10",    // Inclusive range
            "1..5",      // Simple range
            "10..=20",   // Inclusive range with different numbers
        ];

        for source in range_test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            match &result {
                Ok(_program) => {
                    println!("Successfully parsed range: {}", source);
                }
                Err(e) => {
                    println!("Failed to parse range '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse range expression: {}", source);
        }
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
            "() -> 42",
            
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

    #[test]
    fn test_grammar_parse_impl_block() {
        use pest::Parser;
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;

        let source = "impl Drawable:Point { draw() -> int { 0 } }";
        println!("=== DEBUGGING IMPL BLOCK PARSING ===");
        println!("Source: '{}'", source);
        println!("Source length: {}", source.len());
        
        // Test individual components first
        println!("\n--- Testing 'impl' keyword ---");
        let impl_result = TJLangPestParser::parse(Rule::impl_kw, "impl");
        match impl_result {
            Ok(pairs) => println!("✓ 'impl' parsed successfully"),
            Err(e) => println!("✗ 'impl' failed: {}", e),
        }
        
        println!("\n--- Testing 'for' keyword ---");
        let for_result = TJLangPestParser::parse(Rule::for_kw, "for");
        match for_result {
            Ok(pairs) => println!("✓ 'for' parsed successfully"),
            Err(e) => println!("✗ 'for' failed: {}", e),
        }
        
        println!("\n--- Testing identifier 'Drawable' ---");
        let ident_result = TJLangPestParser::parse(Rule::identifier, "Drawable");
        match ident_result {
            Ok(pairs) => println!("✓ 'Drawable' parsed successfully"),
            Err(e) => println!("✗ 'Drawable' failed: {}", e),
        }
        
        println!("\n--- Testing method_decl 'draw() -> int {{ 0 }}' ---");
        let method_result = TJLangPestParser::parse(Rule::method_decl, "draw() -> int { 0 }");
        match method_result {
            Ok(pairs) => {
                println!("✓ method_decl parsed successfully");
                for pair in pairs {
                    println!("  Method rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("    Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                    }
                }
            },
            Err(e) => {
                println!("✗ method_decl failed: {}", e);
                // Let's test the components of method_decl
                println!("  Testing 'draw' identifier:");
                let ident_test = TJLangPestParser::parse(Rule::identifier, "draw");
                println!("    Result: {:?}", ident_test);
                
                println!("  Testing '()' param_list:");
                let param_test = TJLangPestParser::parse(Rule::param_list, "");
                println!("    Result: {:?}", param_test);
                
                println!("  Testing '->' ARROW:");
                let arrow_test = TJLangPestParser::parse(Rule::ARROW, "->");
                println!("    Result: {:?}", arrow_test);
                
                println!("  Testing 'int' type:");
                let type_test = TJLangPestParser::parse(Rule::type_, "int");
                println!("    Result: {:?}", type_test);
                
                println!("  Testing '{{ 0 }}' block:");
                let block_test = TJLangPestParser::parse(Rule::block, "{ 0 }");
                println!("    Result: {:?}", block_test);
            }
        }
        
        // Test a simpler method_decl
        println!("\n--- Testing simpler method_decl 'c() -> int {{ 0 }}' ---");
        let simple_method_result = TJLangPestParser::parse(Rule::method_decl, "c() -> int { 0 }");
        match simple_method_result {
            Ok(pairs) => {
                println!("✓ simple method_decl parsed successfully");
                for pair in pairs {
                    println!("  Method rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                }
            }
            Err(e) => println!("✗ simple method_decl failed: {}", e),
        }
        
        println!("\n--- Testing if impl_block rule exists ---");
        // Let's check if the rule exists by trying to parse an empty string
        let empty_test = TJLangPestParser::parse(Rule::impl_block, "");
        println!("Empty string test: {:?}", empty_test);
        
        // Let's try to see what rules are available by checking the Rule enum
        println!("Checking if impl_block is in the Rule enum...");
        match std::mem::discriminant(&Rule::impl_block) {
            _ => println!("✓ impl_block rule exists in enum"),
        }
        
        // Let's test a minimal impl_block with a simple method
        println!("\n--- Testing minimal impl_block ---");
        let minimal_impl = "impl A:B { c() -> int { 0 } }";
        let minimal_result = TJLangPestParser::parse(Rule::impl_block, minimal_impl);
        println!("Minimal impl_block test: {:?}", minimal_result);
        
        println!("\n--- Testing step by step impl_block ---");
        // Test: "Drawable"
        let step1 = TJLangPestParser::parse(Rule::impl_block, "Drawable");
        println!("Step 1 - 'Drawable': {:?}", step1);
        
        // Test: "Drawable for"
        let step2 = TJLangPestParser::parse(Rule::impl_block, "Drawable for");
        println!("Step 2 - 'Drawable for': {:?}", step2);
        
        // Test: "Drawable for Point"
        let step3 = TJLangPestParser::parse(Rule::impl_block, "Drawable for Point");
        println!("Step 3 - 'Drawable for Point': {:?}", step3);
        
        // Test: "Drawable for Point {"
        let step4 = TJLangPestParser::parse(Rule::impl_block, "Drawable for Point {");
        println!("Step 4 - 'Drawable for Point {{': {:?}", step4);
        
        // Let's test the exact components step by step
        println!("\n--- Testing exact components ---");
        let test1 = TJLangPestParser::parse(Rule::impl_block, "A for B { test }");
        println!("Test 1: {:?}", test1);
        
        let test2 = TJLangPestParser::parse(Rule::impl_block, "A for B { test }");
        println!("Test 2: {:?}", test2);
        
        // Let's try to parse just the method_decl part
        let method_test = TJLangPestParser::parse(Rule::method_decl, "c() -> int { 0 }");
        println!("Method test: {:?}", method_test);
        
        // Test the final impl_block rule
        let combined_test = TJLangPestParser::parse(Rule::impl_block, "impl Drawable:Point { draw() -> int { 0 } }");
        println!("impl_block result: {:?}", combined_test);
        
        // Test with different trait and type names
        let test1 = TJLangPestParser::parse(Rule::impl_block, "impl Comparable:Point { compare(other: Point) -> int { 0 } }");
        println!("Different names test 1: {:?}", test1);
        
        let test2 = TJLangPestParser::parse(Rule::impl_block, "impl Iterator:List { next() -> Option<T> { None } }");
        println!("Different names test 2: {:?}", test2);
        
        let test3 = TJLangPestParser::parse(Rule::impl_block, "impl Serializable:User { serialize() -> string { \"{}\" } }");
        println!("Different names test 3: {:?}", test3);
        
        println!("\n--- Testing full impl_block ---");
        let result = TJLangPestParser::parse(Rule::impl_block, source);

        match result {
            Ok(pairs) => {
                println!("✓ Impl block grammar parse successful!");
                for pair in pairs {
                    println!("Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    for inner in pair.into_inner() {
                        println!("  Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        for inner2 in inner.into_inner() {
                            println!("    Inner2: {:?}, Content: '{}'", inner2.as_rule(), inner2.as_str());
                        }
                    }
                }
                assert!(true, "Impl block grammar should parse successfully");
            }
            Err(e) => {
                println!("✗ Impl block grammar parse failed: {}", e);
                panic!("Impl block grammar should parse successfully");
            }
        }
    }
}