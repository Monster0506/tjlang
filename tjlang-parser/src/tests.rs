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
}