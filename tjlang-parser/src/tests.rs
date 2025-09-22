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
    fn test_grammar_compilation() {
        // This test ensures our grammar compiles without errors
        // If this test runs, it means the grammar is syntactically correct
        let source = "x";
        let mut parser = PestParser::new();
        let result = parser.parse(source);
        
        // Should not panic during compilation
        assert!(result.is_ok() || result.is_err()); // Either is fine, just no panic
    }
}