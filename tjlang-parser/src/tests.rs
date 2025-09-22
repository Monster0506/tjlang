//! Parser tests

#[cfg(test)]
mod tests {
    use super::*;
    use codespan::Files;

    fn create_test_file_id() -> codespan::FileId {
        let mut files = Files::new();
        files.add("test.tj", "test content")
    }

    #[test]
    fn test_parse_simple_function() {
        let source = r#"
def add(x: int, y: int) -> int {
    return x + y
}
"#;
        let file_id = create_test_file_id();
        let result = parse(source, file_id);
        
        assert!(result.is_ok());
        let (program, diagnostics) = result.unwrap();
        assert!(diagnostics.is_empty());
        assert_eq!(program.units.len(), 1);
        
        if let ProgramUnit::Declaration(Declaration::Function(func)) = &program.units[0] {
            assert_eq!(func.name, "add");
            assert_eq!(func.params.len(), 2);
            assert_eq!(func.params[0].name, "x");
            assert_eq!(func.params[1].name, "y");
        } else {
            panic!("Expected function declaration");
        }
    }

    #[test]
    fn test_parse_variable_declaration() {
        let source = r#"
x: int = 42
name: str = "hello"
"#;
        let file_id = create_test_file_id();
        let result = parse(source, file_id);
        
        assert!(result.is_ok());
        let (program, diagnostics) = result.unwrap();
        assert!(diagnostics.is_empty());
        assert_eq!(program.units.len(), 2);
        
        if let ProgramUnit::Declaration(Declaration::Variable(var)) = &program.units[0] {
            assert_eq!(var.name, "x");
            assert_eq!(var.var_type, Type::Primitive(PrimitiveType::Int));
        } else {
            panic!("Expected variable declaration");
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let source = r#"
if x > 0 {
    print("positive")
} elif x == 0 {
    print("zero")
} else {
    print("negative")
}
"#;
        let file_id = create_test_file_id();
        let result = parse(source, file_id);
        
        assert!(result.is_ok());
        let (program, diagnostics) = result.unwrap();
        assert!(diagnostics.is_empty());
        assert_eq!(program.units.len(), 1);
        
        if let ProgramUnit::Declaration(Declaration::Variable(var)) = &program.units[0] {
            if let Expression::If(if_expr) = &var.value {
                assert_eq!(if_expr.elif_branches.len(), 1);
                assert!(if_expr.else_block.is_some());
            } else {
                panic!("Expected if expression");
            }
        } else {
            panic!("Expected variable declaration");
        }
    }

    #[test]
    fn test_parse_enum() {
        let source = r#"
enum Result<T, E> {
    Ok(T),
    Err(E)
}
"#;
        let file_id = create_test_file_id();
        let result = parse(source, file_id);
        
        assert!(result.is_ok());
        let (program, diagnostics) = result.unwrap();
        assert!(diagnostics.is_empty());
        assert_eq!(program.units.len(), 1);
        
        if let ProgramUnit::Declaration(Declaration::Enum(enum_decl)) = &program.units[0] {
            assert_eq!(enum_decl.name, "Result");
            assert_eq!(enum_decl.type_params.len(), 2);
            assert_eq!(enum_decl.variants.len(), 2);
            assert_eq!(enum_decl.variants[0].name, "Ok");
            assert_eq!(enum_decl.variants[1].name, "Err");
        } else {
            panic!("Expected enum declaration");
        }
    }

    #[test]
    fn test_parse_struct() {
        let source = r#"
type Point {
    x: int,
    y: int
}
"#;
        let file_id = create_test_file_id();
        let result = parse(source, file_id);
        
        assert!(result.is_ok());
        let (program, diagnostics) = result.unwrap();
        assert!(diagnostics.is_empty());
        assert_eq!(program.units.len(), 1);
        
        if let ProgramUnit::Declaration(Declaration::Struct(struct_decl)) = &program.units[0] {
            assert_eq!(struct_decl.name, "Point");
            assert_eq!(struct_decl.fields.len(), 2);
            assert_eq!(struct_decl.fields[0].name, "x");
            assert_eq!(struct_decl.fields[1].name, "y");
        } else {
            panic!("Expected struct declaration");
        }
    }

    #[test]
    fn test_parse_interface() {
        let source = r#"
interface Display {
    to_str(self) -> str
}
"#;
        let file_id = create_test_file_id();
        let result = parse(source, file_id);
        
        assert!(result.is_ok());
        let (program, diagnostics) = result.unwrap();
        assert!(diagnostics.is_empty());
        assert_eq!(program.units.len(), 1);
        
        if let ProgramUnit::Declaration(Declaration::Interface(interface_decl)) = &program.units[0] {
            assert_eq!(interface_decl.name, "Display");
            assert_eq!(interface_decl.methods.len(), 1);
            assert_eq!(interface_decl.methods[0].name, "to_str");
        } else {
            panic!("Expected interface declaration");
        }
    }
}
