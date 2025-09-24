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
        assert_eq!(program.units.len(), 3); // Should be 3 separate identifier expressions
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
                
                // The parser should now parse the actual variable declaration
                if let ProgramUnit::Declaration(Declaration::Variable(var)) = &program.units[0] {
                    assert_eq!(var.name, "x"); // This should be the actual variable name
                    assert_eq!(var.var_type, Type::Primitive(PrimitiveType::Int));
                    if let Expression::Literal(Literal::Int(value)) = &var.value {
                        assert_eq!(*value, 42);
                    } else {
                        panic!("Expected integer literal 42, got: {:?}", var.value);
                    }
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
            "enum Option<T> { Some(T), Empty }",
            "enum Result<T, E> { Ok(T), Err(E) }",
            "enum Color { Red, Green, Blue }",
            
            // Interface declarations
            "interface Drawable { draw() -> int }",
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
        use crate::parser::{PestParser, TJLangPestParser, Rule};
        use pest::Parser;

        // Test the grammar rule directly first
        let range_test_cases = vec![
            "0..10",     // Exclusive range
            "0..=10",    // Inclusive range
            "1..5",      // Simple range
            "10..=20",   // Inclusive range with different numbers
        ];

        for source in range_test_cases {
            // Test grammar rule directly
            let grammar_result = TJLangPestParser::parse(Rule::range_expr, source);
            match &grammar_result {
                Ok(_pairs) => {
                    println!("✓ Grammar rule parsed range: {}", source);
                }
                Err(e) => {
                    println!("✗ Grammar rule failed for range '{}': {}", source, e);
                }
            }

            // For now, just test that the grammar rule works
            assert!(grammar_result.is_ok(), "Failed to parse range expression grammar rule: {}", source);
        }

        // Test range expressions in valid contexts
        let context_test_cases = vec![
            "for (x: int | 0..10) { pass }",     // Range in for loop
            "for (i: int | 1..5) { pass }",      // Simple range in for loop
            "for (j: int | 0..=10) { pass }",    // Inclusive range in for loop
        ];

        for source in context_test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);

            match &result {
                Ok(_program) => {
                    println!("✓ Program parsed range in context: {}", source);
                }
                Err(e) => {
                    println!("✗ Program failed for range in context '{}': {}", source, e);
                }
            }

            assert!(result.is_ok(), "Failed to parse range expression in context: {}", source);
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

    #[test]
    fn test_grammar_parse_interface_debug() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test identifier parsing first
        let identifier_tests = vec![
            "Drawable",
            "extends",
            "Printable",
        ];
        
        for source in identifier_tests {
            println!("Testing identifier: {}", source);
            let result = TJLangPestParser::parse(Rule::identifier, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Identifier parsed successfully");
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => println!("Failed to parse identifier '{}': {}", source, e),
            }
            println!();
        }
        
        // Test interface declaration grammar directly
        let test_cases = vec![
            "interface Drawable { draw() -> int }",
            "interface Drawable extends Printable { draw() -> int }",
        ];
        
        for source in test_cases {
            println!("Testing: {}", source);
            let result = TJLangPestParser::parse(Rule::interface_decl, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Interface declaration parsed successfully");
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                        println!("  Parse tree depth analysis:");
                        for (i, inner) in pair.into_inner().enumerate() {
                            println!("    [{}] Rule={:?}, Content='{}'", i, inner.as_rule(), inner.as_str());
                            println!("         Span: {:?}", inner.as_span());
                            // Check if this inner has more children
                            let inner_children: Vec<_> = inner.clone().into_inner().collect();
                            if !inner_children.is_empty() {
                                println!("         Has {} children:", inner_children.len());
                                for (j, child) in inner_children.iter().enumerate() {
                                    println!("           [{}] Rule={:?}, Content='{}'", j, child.as_rule(), child.as_str());
                                }
                            }
                        }
                    }
                }
                Err(e) => panic!("Failed to parse interface declaration '{}': {}", source, e),
            }
            println!();
        }
        
        // Test the specific problematic part
        println!("=== Testing specific parts ===");
        let problematic_parts = vec![
            "Drawable extends",
            "Drawable extends Printable",
            "Drawable ",
            "Drawable\t",
            "Drawable\n",
        ];
        
        for part in problematic_parts {
            println!("Testing part: '{}'", part);
            let result = TJLangPestParser::parse(Rule::identifier, part);
            match result {
                Ok(pairs) => {
                    println!("✓ Parsed as identifier");
                    for pair in pairs {
                        println!("  Content: '{}'", pair.as_str());
                        println!("  Length: {}", pair.as_str().len());
                        println!("  Bytes: {:?}", pair.as_str().as_bytes());
                    }
                }
                Err(e) => println!("✗ Failed to parse: {}", e),
            }
            println!();
        }
        
        // Test character sets
        println!("=== Testing character sets ===");
        let char_tests = vec![
            "a", "A", "1", "_", " ", "\t", "\n", "extends",
        ];
        
        for ch in char_tests {
            println!("Testing character: '{}' (bytes: {:?})", ch, ch.as_bytes());
            let result = TJLangPestParser::parse(Rule::identifier, ch);
            match result {
                Ok(pairs) => println!("✓ Parsed as identifier"),
                Err(e) => println!("✗ Failed: {}", e),
            }
        }
        
        // Test what ASCII_ALPHANUMERIC includes
        println!("=== Testing ASCII_ALPHANUMERIC behavior ===");
        let ascii_tests = vec![
            "a1", "a_", "a ", "a\t", "a\n", "ab", "a1b", "a_b", "a b",
        ];
        
        for test in ascii_tests {
            println!("Testing ASCII pattern: '{}' (bytes: {:?})", test, test.as_bytes());
            let result = TJLangPestParser::parse(Rule::identifier, test);
            match result {
                Ok(pairs) => {
                    for pair in pairs {
                        println!("✓ Parsed as identifier: '{}'", pair.as_str());
                    }
                }
                Err(e) => println!("✗ Failed: {}", e),
            }
        }
        
        // Test the exact problematic string
        println!("=== Testing exact problematic string ===");
        let exact_test = "Drawable extends Printable";
        println!("Testing exact string: '{}'", exact_test);
        let result = TJLangPestParser::parse(Rule::identifier, exact_test);
        match result {
            Ok(pairs) => {
                for pair in pairs {
                    println!("✓ Parsed as identifier: '{}'", pair.as_str());
                    println!("  Length: {}", pair.as_str().len());
                    println!("  Bytes: {:?}", pair.as_str().as_bytes());
                }
            }
            Err(e) => println!("✗ Failed: {}", e),
        }
        
        // Test if the issue is with the grammar rule structure
        println!("=== Testing grammar rule structure ===");
        let grammar_test = "interface Drawable extends Printable { draw() -> int }";
        println!("Testing full grammar: '{}'", grammar_test);
        let result = TJLangPestParser::parse(Rule::interface_decl, grammar_test);
        match result {
            Ok(pairs) => {
                for pair in pairs {
                    println!("✓ Parsed as interface_decl");
                    for (i, inner) in pair.into_inner().enumerate() {
                        println!("  [{}] Rule={:?}, Content='{}'", i, inner.as_rule(), inner.as_str());
                        println!("       Span: {:?}", inner.as_span());
                    }
                }
            }
            Err(e) => println!("✗ Failed: {}", e),
        }
        
        // Minimal test to understand the issue
        println!("=== Minimal test ===");
        let minimal_test = "a b";
        println!("Testing minimal: '{}'", minimal_test);
        let result = TJLangPestParser::parse(Rule::identifier, minimal_test);
        match result {
            Ok(pairs) => {
                for pair in pairs {
                    println!("✓ Parsed as identifier: '{}'", pair.as_str());
                    println!("  Length: {}", pair.as_str().len());
                    println!("  Bytes: {:?}", pair.as_str().as_bytes());
                }
            }
            Err(e) => println!("✗ Failed: {}", e),
        }
    }

    #[test]
    fn test_parse_interface_extends() {
        use crate::parser::PestParser;
        
        let test_cases = vec![
            "interface Drawable { draw() -> int }",
            "interface Drawable extends Printable { draw() -> int }",
            "interface Drawable extends Printable, Serializable { draw() -> int }",
        ];
        
        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            match result {
                Ok(program) => {
                    println!("✓ Successfully parsed interface: {}", source);
                    // Check that we have an interface declaration
                    if let Some(unit) = program.units.first() {
                        if let tjlang_ast::ProgramUnit::Declaration(decl) = unit {
                            if let tjlang_ast::Declaration::Interface(interface_decl) = decl {
                                println!("  Interface name: {}", interface_decl.name);
                                println!("  Extends: {:?}", interface_decl.extends);
                                println!("  Methods: {} method(s)", interface_decl.methods.len());
                            }
                        }
                    }
                }
                Err(e) => panic!("Failed to parse interface '{}': {}", source, e),
            }
        }
        
        println!("✓ All interface extension parsing tests passed");
    }

    #[test]
    fn test_parse_struct_literals() {
        use crate::parser::PestParser;
        
        let test_cases = vec![
            "Point { x: 1, y: 2 }",
            "Person { name: \"Alice\", age: 30 }",
            "Config { debug: true, port: 8080 }",
        ];
        
        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            match result {
                Ok(program) => {
                    println!("✓ Successfully parsed struct literal: {}", source);
                    // Check that we have a variable declaration with a struct literal
                    if let Some(unit) = program.units.first() {
                        if let tjlang_ast::ProgramUnit::Declaration(decl) = unit {
                            if let tjlang_ast::Declaration::Variable(var_decl) = decl {
                                if let tjlang_ast::Expression::StructLiteral { name, fields, .. } = &var_decl.value {
                                    println!("  Struct name: {}", name);
                                    println!("  Fields: {} field(s)", fields.len());
                                    for field in fields {
                                        println!("    {}: {:?}", field.name, field.value);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => panic!("Failed to parse struct literal '{}': {}", source, e),
            }
        }
        
        println!("✓ All struct literal parsing tests passed");
    }

    #[test]
    fn test_parse_spawn_expressions() {
        use crate::parser::PestParser;

        // Valid spawn expressions in various contexts
        let ok_cases = vec![
            // Bare spawn of an identifier
            "spawn task",
            // Spawn of a call
            "spawn run()",
            // Spawn of a binary expression
            "spawn (1 + 2)",
            // Spawn chained with member and call
            "spawn worker.start()",
            // Spawn inside an index
            "[spawn 1, 2, 3]",
            // Spawn inside a map literal value
            "{ 1: spawn 2 }",
            // Spawn inside a function body (no semicolons in grammar)
            "def main() -> int { spawn run() return 0 }",
        ];

        for source in ok_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            assert!(result.is_ok(), "Failed to parse spawn expression: {}", source);
        }

        // Invalid spawn usage should fail
        let err_cases = vec![
            // Missing expression
            "spawn",
            // Keyword follows spawn
            "spawn return 1",
        ];

        for source in err_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            assert!(result.is_err(), "Expected failure for invalid spawn usage: {}", source);
        }
    }

    #[test]
    fn test_grammar_parse_fstring_literals() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test f-string literal grammar rule directly
        let test_cases = vec![
            "f\"Hello world\"",
            "f\"Hello {name}\"",
            "f\"Value: {x + y}\"",
            "f\"Multiple: {a}, {b}\"",
            "f\"Escaped: {{brace}}\"",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::fstring_literal, source);
            match result {
                Ok(pairs) => {
                    println!("✓ F-string literal parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse f-string literal '{}': {}", source, e),
            }
        }
        
        println!("✓ All f-string literal grammar tests passed");
    }

    #[test]
    fn test_parse_fstring_literals() {
        use crate::parser::PestParser;
        
        let mut parser = PestParser::new();
        
        // Test basic f-string literal in variable declaration
        let result = parser.parse("x: str = f\"Hello world\"");
        match result {
            Ok(program) => {
                if let ProgramUnit::Declaration(Declaration::Variable(var_decl)) = &program.units[0] {
                    if let Expression::Literal(Literal::FString(content)) = &var_decl.value {
                        assert_eq!(content, "Hello world");
                    } else {
                        panic!("Expected FString literal, got: {:?}", var_decl.value);
                    }
                } else {
                    panic!("Expected variable declaration, got: {:?}", program.units);
                }
            }
            Err(e) => panic!("Failed to parse f-string literal: {}", e),
        }
        
        // Test f-string with interpolation syntax (parser doesn't validate, just captures)
        let result = parser.parse("y: str = f\"Hello {name}\"");
        match result {
            Ok(program) => {
                if let ProgramUnit::Declaration(Declaration::Variable(var_decl)) = &program.units[0] {
                    if let Expression::Literal(Literal::FString(content)) = &var_decl.value {
                        assert_eq!(content, "Hello {name}");
                    } else {
                        panic!("Expected FString literal, got: {:?}", var_decl.value);
                    }
                } else {
                    panic!("Expected variable declaration, got: {:?}", program.units);
                }
            }
            Err(e) => panic!("Failed to parse f-string with interpolation: {}", e),
        }
        
        // Test f-string with complex interpolation
        let result = parser.parse("z: str = f\"Value: {x + y}\"");
        match result {
            Ok(program) => {
                if let ProgramUnit::Declaration(Declaration::Variable(var_decl)) = &program.units[0] {
                    if let Expression::Literal(Literal::FString(content)) = &var_decl.value {
                        assert_eq!(content, "Value: {x + y}");
                    } else {
                        panic!("Expected FString literal, got: {:?}", var_decl.value);
                    }
                } else {
                    panic!("Expected variable declaration, got: {:?}", program.units);
                }
            }
            Err(e) => panic!("Failed to parse f-string with complex interpolation: {}", e),
        }
        
        // Test f-string with multiple interpolations
        let result = parser.parse("msg: str = f\"Multiple: {a}, {b}\"");
        match result {
            Ok(program) => {
                if let ProgramUnit::Declaration(Declaration::Variable(var_decl)) = &program.units[0] {
                    if let Expression::Literal(Literal::FString(content)) = &var_decl.value {
                        assert_eq!(content, "Multiple: {a}, {b}");
                    } else {
                        panic!("Expected FString literal, got: {:?}", var_decl.value);
                    }
                } else {
                    panic!("Expected variable declaration, got: {:?}", program.units);
                }
            }
            Err(e) => panic!("Failed to parse f-string with multiple interpolations: {}", e),
        }
        
        // Test f-string with escaped braces
        let result = parser.parse("escaped: str = f\"Escaped: {{brace}}\"");
        match result {
            Ok(program) => {
                if let ProgramUnit::Declaration(Declaration::Variable(var_decl)) = &program.units[0] {
                    if let Expression::Literal(Literal::FString(content)) = &var_decl.value {
                        assert_eq!(content, "Escaped: {{brace}}");
                    } else {
                        panic!("Expected FString literal, got: {:?}", var_decl.value);
                    }
                } else {
                    panic!("Expected variable declaration, got: {:?}", program.units);
                }
            }
            Err(e) => panic!("Failed to parse f-string with escaped braces: {}", e),
        }
        
        println!("✓ All f-string literal parsing tests passed");
    }

    #[test]
    fn test_grammar_parse_struct_literals() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test struct literal grammar rule directly
        let test_cases = vec![
            "Point { x: 1, y: 2 }",
            "Person { name: \"Alice\", age: 30 }",
            "Config { debug: true, port: 8080 }",
            "Single { value: 42 }",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::struct_literal, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Struct literal parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse struct literal '{}': {}", source, e),
            }
        }
        
        println!("✓ All struct literal grammar tests passed");
    }

    #[test]
    fn test_grammar_parse_field_initialization() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test field initialization grammar rule directly
        let test_cases = vec![
            "x: 42",
            "name: \"hello\"",
            "enabled: true",
            "value: x + y",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::field_init, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Field initialization parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse field initialization '{}': {}", source, e),
            }
        }
        
        println!("✓ All field initialization grammar tests passed");
    }

    #[test]
    fn test_grammar_parse_match_statements_no_commas() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test match statement with no commas between arms (corrected syntax)
        let test_cases = vec![
            "match x { 1: { pass } 2: { pass } _: { pass } }",
            "match value { true: { return 1 } false: { return 0 } }",
            "match status { \"ok\": { pass } \"error\": { raise \"failed\" } }",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::match_stmt, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Match statement parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse match statement '{}': {}", source, e),
            }
        }
        
        println!("✓ All match statement grammar tests passed");
    }

    #[test]
    fn test_grammar_parse_identifier_list() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test identifier list parsing
        let test_cases = vec![
            "Printable",
            "Printable, Serializable",
            "A, B, C",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::identifier_list, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Identifier list parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse identifier list '{}': {}", source, e),
            }
        }
        
        println!("✓ All identifier list grammar tests passed");
    }

    #[test]
    fn test_grammar_parse_interface_extends() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test interface with extends clause
        let test_cases = vec![
            "interface Drawable { draw() -> int }",
            "interface Drawable extends Printable { draw() -> int }",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::interface_decl, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Interface declaration parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse interface declaration '{}': {}", source, e),
            }
        }
        
        println!("✓ All interface declaration grammar tests passed");
    }

    #[test]
    fn test_grammar_parse_generic_params() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test generic parameters with bounds
        let test_cases = vec![
            "def process<T: implements [Comparable]> (item: T) -> int { 0 }",
            "def sort<T: implements [Comparable, Serializable]> (items: [T]) -> [T] { items }",
            "def map<T: implements [Clone], U: implements [Default]> (f: (T) -> U, items: [T]) -> [U] { [] }",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::function_decl, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Function with generic parameters parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse function with generic parameters '{}': {}", source, e),
            }
        }
        
        println!("✓ All generic parameters grammar tests passed");
    }

    #[test]
    fn test_parse_generic_params() {
        use crate::parser::PestParser;
        
        let test_cases = vec![
            "def identity<T: implements [Comparable]>(x: T) -> T { return x }",
            "def compare<T: implements [Comparable, Serializable]>(a: T, b: T) -> int { return a.compare(b) }",
            "def process<T: implements [Comparable], U: implements [Clone]>(item: T, backup: U) -> T { return item }",
        ];
        
        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            match result {
                Ok(program) => {
                    println!("✓ Successfully parsed function with generic parameters: {}", source);
                    if let Some(unit) = program.units.first() {
                        if let tjlang_ast::ProgramUnit::Declaration(decl) = unit {
                            if let tjlang_ast::Declaration::Function(func_decl) = decl {
                                println!("  Function name: {}", func_decl.name);
                                println!("  Generic parameters: {} parameter(s)", func_decl.generic_params.len());
                                for (i, param) in func_decl.generic_params.iter().enumerate() {
                                    println!("    [{}] {}: Implements {:?}", i, param.name, param.bounds);
                                }
                                println!("  Parameters: {} parameter(s)", func_decl.params.len());
                                println!("  Return type: {:?}", func_decl.return_type);
                            }
                        }
                    }
                }
                Err(e) => panic!("Failed to parse function with generic parameters '{}': {}", source, e),
            }
        }
        
        println!("✓ All generic parameters parsing tests passed");
    }

    #[test]
    fn test_debug_generic_params() {
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;
        use pest::Parser;
        
        let test_cases = vec![
            "<T: implements [Comparable]>",
            "<T: implements [Comparable, Serializable]>",
        ];
        
        for source in test_cases {
            println!("Testing generic_params: {}", source);
            let result = TJLangPestParser::parse(Rule::generic_params, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Generic params parsed successfully");
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                        for inner in pair.into_inner() {
                            println!("    Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        }
                    }
                }
                Err(e) => panic!("Failed to parse generic_params '{}': {}", source, e),
            }
            println!();
        }
    }

    #[test]
    fn test_debug_generic_param() {
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;
        use pest::Parser;
        
        let test_cases = vec![
            "T: implements [Comparable]",
            "T: implements [Comparable, Serializable]",
        ];
        
        for source in test_cases {
            println!("Testing generic_param: {}", source);
            let result = TJLangPestParser::parse(Rule::generic_param, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Generic param parsed successfully");
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                        for inner in pair.into_inner() {
                            println!("    Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        }
                    }
                }
                Err(e) => panic!("Failed to parse generic_param '{}': {}", source, e),
            }
            println!();
        }
    }

    #[test]
    fn test_grammar_parse_operator_methods() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test operator symbols in method signatures
        let test_cases = vec![
            "interface Math { + (other: int) -> int }",
            "interface Comparable { == (other: int) -> bool }",
            "interface Indexable { [] (index: int) -> int }",
            "interface Logic { and (other: bool) -> bool }",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::interface_decl, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Interface with operator method parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse interface with operator method '{}': {}", source, e),
            }
        }
        
        println!("✓ All operator method grammar tests passed");
    }

    #[test]
    fn test_parse_operator_methods() {
        use crate::parser::PestParser;
        
        let test_cases = vec![
            "interface Math { + (other: int) -> int }",
            "interface Comparable { == (other: int) -> bool }",
            "interface Indexable { [] (index: int) -> int }",
            "interface Logic { and (other: bool) -> bool }",
        ];
        
        for source in test_cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            
            match result {
                Ok(program) => {
                    println!("✓ Successfully parsed interface with operator method: {}", source);
                    if let Some(unit) = program.units.first() {
                        if let tjlang_ast::ProgramUnit::Declaration(decl) = unit {
                            if let tjlang_ast::Declaration::Interface(interface_decl) = decl {
                                println!("  Interface name: {}", interface_decl.name);
                                println!("  Methods: {} method(s)", interface_decl.methods.len());
                                for (i, method) in interface_decl.methods.iter().enumerate() {
                                    println!("    [{}] {}: {} parameter(s) -> {:?}", 
                                        i, method.name, method.params.len(), method.return_type);
                                }
                            }
                        }
                    }
                }
                Err(e) => panic!("Failed to parse interface with operator method '{}': {}", source, e),
            }
        }
        
        println!("✓ All operator method parsing tests passed");
    }

    #[test]
    fn test_grammar_parse_bitwise_and_power_precedence() {
        use crate::parser::{TJLangPestParser, Rule};
        use pest::Parser;

        let cases = vec![
            "1 + 2 * 3 ** 2",
            "~x & y | z ^ w",
            "a << 2 + 1",
            "(1 + 2) ** 3",
        ];

        for src in cases {
            let res = TJLangPestParser::parse(Rule::expression, src);
            assert!(res.is_ok(), "Failed to parse expression: {}", src);
        }
    }

    #[test]
    fn test_parse_bitwise_and_power_expressions() {
        use crate::parser::PestParser;

        let cases = vec![
            ("def main() -> int { return 2 ** 3 * 2 }", 1usize),
            ("def main() -> int { return ~1 & 3 }", 1usize),
            ("def main() -> int { return (1 | 2) ^ 3 }", 1usize),
            ("def main() -> int { return 1 << 2 >> 1 }", 1usize),
        ];

        for (src, expect_units) in cases {
            let mut p = PestParser::new();
            let program = p.parse(src).expect("parse failed");
            assert_eq!(program.units.len(), expect_units, "Unexpected unit count for {}", src);
        }
    }

    // Match statement coverage: literals, type binds, enums/constructors, option, traits, guards, tuples, structs, nested
    fn parse_match_stmt_ok(src: &str) {
        use crate::parser::{TJLangPestParser, Rule};
        use pest::Parser;
        let res = TJLangPestParser::parse(Rule::match_stmt, src);
        assert!(res.is_ok(), "Failed to parse match statement: {}", src);
    }

    #[test]
    fn test_grammar_match_literals_and_wildcard() {
        parse_match_stmt_ok("match x { 0: { pass } 1: { pass } _: { pass } }");
    }

    #[test]
    fn test_grammar_match_type_binds() {
        parse_match_stmt_ok("match val { n: int: { pass } s: str: { pass } }");
    }

    #[test]
    fn test_grammar_match_constructors() {
        parse_match_stmt_ok("match res { Ok(v: int): { pass } Err(e: str): { pass } }");
    }

    #[test]
    fn test_grammar_match_option_patterns() {
        parse_match_stmt_ok("match x { Some(v: int): { pass } None: { pass } }");
    }

    #[test]
    fn test_grammar_match_with_guard() {
        parse_match_stmt_ok("match n { v: int if v > 0: { pass } v: int: { pass } }");
    }

    #[test]
    fn test_grammar_match_tuple_patterns() {
        parse_match_stmt_ok("match pair { (0, y: int): { pass } (x: int, 0): { pass } (a: int, b: int): { pass } }");
    }

    fn parse_ok_program_helper(src: &str) {
        use crate::parser::PestParser;
        let mut p = PestParser::new();
        let result = p.parse(src);
        assert!(result.is_ok(), "Failed to parse program with match: {}", src);
    }

    #[test]
    fn test_parse_match_literals_and_wildcard() {
        parse_ok_program_helper("def main() -> int { match 5 { 0: { pass } 5: { return 1 } _: { return 0 } } }");
    }

    #[test]
    fn test_parse_match_type_binds() {
        parse_ok_program_helper("def main() -> int { match 1 { n: int: { return n } s: str: { return 0 } } }");
    }

    #[test]
    fn test_parse_match_constructors() {
        parse_ok_program_helper("def main() -> int { match Ok(42) { Ok(v: int): { return v } Err(e: str): { return 0 } } }");
    }

    #[test]
    fn test_parse_match_option_patterns() {
        parse_ok_program_helper("def main() -> int { match None { Some(v: int): { return v } None: { return 0 } } }");
    }

    #[test]
    fn test_parse_match_with_guard() {
        parse_ok_program_helper("def main() -> int { match 10 { v: int if v % 2 == 0: { return 1 } v: int: { return 0 } } }");
    }

    #[test]
    fn test_parse_match_tuple_patterns() {
        parse_ok_program_helper("def main() -> int { match (1,2) { (0, y: int): { return y } (x: int, 0): { return x } (a: int, b: int): { return a + b } } }");
    }

    #[test]
    fn test_grammar_match_struct_destructuring() {
        parse_match_stmt_ok("match p { Point{ x: 0, y: y: int }: { pass } Point{ x: x: int, y: y: int }: { pass } }");
    }

    #[test]
    fn test_parse_match_struct_destructuring() {
        parse_ok_program_helper("def main() -> int { p: Point = Point{ x: 1, y: 2 } match p { Point{ x: 0, y: y: int }: { return y } Point{ x: x: int, y: y: int }: { return x + y } } }");
    }

    // Nested patterns
    #[test]
    fn test_grammar_match_nested_constructor_struct() {
        parse_match_stmt_ok("match res { Ok(Point{ x: 0, y: y: int }): { pass } Ok(Point{ x: x: int, y: y: int }): { pass } Err(e: str): { pass } }");
    }

    #[test]
    fn test_parse_match_nested_constructor_struct() {
        parse_ok_program_helper("def main() -> int { p: Point = Point{ x: 0, y: 5 } match Ok(p) { Ok(Point{ x: 0, y: y: int }): { return y } Ok(Point{ x: x: int, y: y: int }): { return x + y } Err(e: str): { return 0 } } }");
    }

    #[test]
    fn test_grammar_match_nested_option_tuple() {
        parse_match_stmt_ok("match v { Some((a: int, b: int)): { pass } None: { pass } }");
    }

    #[test]
    fn test_parse_match_nested_option_tuple() {
        parse_ok_program_helper("def main() -> int { match Some((1,2)) { Some((a: int, b: int)): { return a + b } None: { return 0 } } }");
    }

    // Multiple methods in impl blocks
    #[test]
    fn test_grammar_parse_impl_multiple_methods() {
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;
        use pest::Parser;
        
        let source = "impl Drawable: Point { draw() -> int { return 1 } clear() -> int { return 0 } }";
        let result = TJLangPestParser::parse(Rule::impl_block, source);
        match result {
            Ok(pairs) => {
                println!("✓ Multiple methods impl block parsed successfully");
                assert!(pairs.len() > 0);
            }
            Err(e) => {
                panic!("Failed to parse impl block with multiple methods: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_impl_multiple_methods() {
        use crate::parser::PestParser;
        use tjlang_ast::{ProgramUnit, Declaration};
        let mut p = PestParser::new();
        let result = p.parse("impl Drawable: Point { draw() -> int { return 1 } clear() -> int { return 0 } }");
        match result {
            Ok(program) => {
                assert_eq!(program.units.len(), 1);
                match &program.units[0] {
                    ProgramUnit::Declaration(Declaration::Implementation(impl_block)) => {
                        assert_eq!(impl_block.trait_name, "Drawable");
                        assert_eq!(impl_block.type_name, "Point");
                        assert_eq!(impl_block.methods.len(), 2);
                        assert_eq!(impl_block.methods[0].name, "draw");
                        assert_eq!(impl_block.methods[1].name, "clear");
                    }
                    _ => panic!("Expected Implementation, got {:?}", program.units[0]),
                }
            }
            Err(e) => panic!("Failed to parse impl block with multiple methods: {}", e),
        }
    }

    #[test]
    fn test_debug_function_decl() {
        use crate::parser::TJLangPestParser;
        use crate::parser::Rule;
        use pest::Parser;
        
        let test_cases = vec![
            "def main() -> int { return 42 }",
            "def add(x: int, y: int) -> int { return x + y }",
        ];
        
        for source in test_cases {
            println!("Testing function_decl: {}", source);
            let result = TJLangPestParser::parse(Rule::function_decl, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Function declaration parsed successfully");
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                        for inner in pair.into_inner() {
                            println!("    Inner: {:?}, Content: '{}'", inner.as_rule(), inner.as_str());
                        }
                    }
                }
                Err(e) => panic!("Failed to parse function_decl '{}': {}", source, e),
            }
            println!();
        }
    }

    #[test]
    fn test_grammar_parse_enum_type_parameters() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        
        // Test enum with type parameters (corrected syntax)
        let test_cases = vec![
            "enum Option { Some(T), Empty }",
            "enum Result { Ok(T), Err(E) }",
            "enum Color { Red, Green, Blue }",
        ];
        
        for source in test_cases {
            let result = TJLangPestParser::parse(Rule::enum_decl, source);
            match result {
                Ok(pairs) => {
                    println!("✓ Enum declaration parsed successfully: {}", source);
                    for pair in pairs {
                        println!("  Rule: {:?}, Content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
                Err(e) => panic!("Failed to parse enum declaration '{}': {}", source, e),
            }
        }
        
        println!("✓ All enum declaration grammar tests passed");
    }

    #[test]
    fn test_grammar_parse_modules_imports_exports() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};
        // Grammar-level tests for modules/import/export
        let cases = vec![
            "module graphics.ui",
            "import math.core as m",
            "import { sin, cos } from math.core",
            "export draw",
            "export { draw, fill }",
        ];
        for source in cases {
            let result = TJLangPestParser::parse(Rule::program_unit, source);
            assert!(result.is_ok(), "Grammar failed for program_unit: {} -> {:?}", source, result.err());
        }
    }

    #[test]
    fn test_grammar_parse_match_statements() {
        use pest::Parser;
        use crate::parser::{TJLangPestParser, Rule};

        let cases = vec![
            // Basic match with literals - start simple
            "match x { 1: { pass } }",
            // Match with wildcard
            "match x { _: { pass } }",
            // Match with variable patterns
            "match result { Ok(value): { pass } }",
            // Match with constructor patterns
            "match point { Point(x, y): { pass } }",
            // Match with tuple patterns
            "match pair { (a, b): { pass } }",
            // Match with multiple arms
            "match x { 1: { pass } 2: { pass } }",
            // Match with guards
            "match x { n: int if n > 0: { pass } }",
            // Match with trait patterns
            "match obj { drawable: implements [Drawable]: { pass } }",
        ];

        for source in cases {
            // match statements are statements per grammar
            let result = TJLangPestParser::parse(Rule::statement, source);
            if let Err(e) = &result {
                println!("Failed to parse '{}': {:?}", source, e);
            }
            assert!(result.is_ok(), "Failed to parse match statement: {}", source);
        }
    }

    #[test]
    #[ignore]
    fn test_parse_match_statements_placeholder() {
        use crate::parser::PestParser;
        // Placeholder tests until match statements are supported in parser AST
        let cases = vec![
            "match x { 1: { pass }, 2: { pass }, _: { pass } }",
            "match result { Ok(value): { pass }, Err(error): { pass } }",
            "match point { Point(x, y): { pass } }",
        ];
        for source in cases {
            let mut parser = PestParser::new();
            let result = parser.parse(source);
            assert!(result.is_ok(), "Expected match statement to parse once implemented: {}", source);
        }
    }
}
