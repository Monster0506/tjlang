//! Error codes for different categories of errors


/// Error codes for different categories of errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Lexer errors (L0001-L0999)
    LexerInvalidCharacter,
    LexerUnterminatedString,
    LexerUnterminatedComment,
    LexerInvalidNumber,
    LexerInvalidEscape,
    
    // Parser errors (P1000-P1999)
    ParserUnexpectedToken,
    ParserExpectedToken,
    ParserUnexpectedEof,
    ParserInvalidExpression,
    ParserInvalidStatement,
    ParserInvalidType,
    ParserInvalidPattern,
    ParserInvalidFunction,
    ParserInvalidStruct,
    ParserInvalidEnum,
    ParserInvalidInterface,
    ParserInvalidModule,
    ParserInvalidImport,
    ParserInvalidExport,
    
    // Analyzer errors (A2000-A2999)
    AnalyzerUndefinedVariable,
    AnalyzerUndefinedFunction,
    AnalyzerUndefinedType,
    AnalyzerTypeMismatch,
    AnalyzerTraitNotImplemented,
    AnalyzerNonExhaustiveMatch,
    AnalyzerDuplicateDefinition,
    AnalyzerCircularDependency,
    AnalyzerInvalidGeneric,
    AnalyzerInvalidTraitBound,
    AnalyzerInvalidInterface,
    AnalyzerInvalidImplementation,
    AnalyzerInvalidModule,
    AnalyzerInvalidImport,
    AnalyzerInvalidExport,
    
    // Codegen errors (C3000-C3999)
    CodegenInvalidType,
    CodegenInvalidExpression,
    CodegenInvalidFunction,
    CodegenInvalidStruct,
    CodegenInvalidEnum,
    CodegenInvalidInterface,
    CodegenInvalidModule,
    CodegenInvalidImport,
    CodegenInvalidExport,
    
    // Runtime errors (R4000-R4999)
    RuntimePanic,
    RuntimeTaskError,
    RuntimeMemoryError,
    RuntimeTypeError,
    RuntimeValueError,
}

impl ErrorCode {
    /// Get the error code as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            // Lexer errors
            ErrorCode::LexerInvalidCharacter => "L0001",
            ErrorCode::LexerUnterminatedString => "L0002",
            ErrorCode::LexerUnterminatedComment => "L0003",
            ErrorCode::LexerInvalidNumber => "L0004",
            ErrorCode::LexerInvalidEscape => "L0005",
            
            // Parser errors
            ErrorCode::ParserUnexpectedToken => "P1000",
            ErrorCode::ParserExpectedToken => "P1001",
            ErrorCode::ParserUnexpectedEof => "P1002",
            ErrorCode::ParserInvalidExpression => "P1003",
            ErrorCode::ParserInvalidStatement => "P1004",
            ErrorCode::ParserInvalidType => "P1005",
            ErrorCode::ParserInvalidPattern => "P1006",
            ErrorCode::ParserInvalidFunction => "P1007",
            ErrorCode::ParserInvalidStruct => "P1008",
            ErrorCode::ParserInvalidEnum => "P1009",
            ErrorCode::ParserInvalidInterface => "P1010",
            ErrorCode::ParserInvalidModule => "P1011",
            ErrorCode::ParserInvalidImport => "P1012",
            ErrorCode::ParserInvalidExport => "P1013",
            
            // Analyzer errors
            ErrorCode::AnalyzerUndefinedVariable => "A2000",
            ErrorCode::AnalyzerUndefinedFunction => "A2001",
            ErrorCode::AnalyzerUndefinedType => "A2002",
            ErrorCode::AnalyzerTypeMismatch => "A2003",
            ErrorCode::AnalyzerTraitNotImplemented => "A2004",
            ErrorCode::AnalyzerNonExhaustiveMatch => "A2005",
            ErrorCode::AnalyzerDuplicateDefinition => "A2006",
            ErrorCode::AnalyzerCircularDependency => "A2007",
            ErrorCode::AnalyzerInvalidGeneric => "A2008",
            ErrorCode::AnalyzerInvalidTraitBound => "A2009",
            ErrorCode::AnalyzerInvalidInterface => "A2010",
            ErrorCode::AnalyzerInvalidImplementation => "A2011",
            ErrorCode::AnalyzerInvalidModule => "A2012",
            ErrorCode::AnalyzerInvalidImport => "A2013",
            ErrorCode::AnalyzerInvalidExport => "A2014",
            
            // Codegen errors
            ErrorCode::CodegenInvalidType => "C3000",
            ErrorCode::CodegenInvalidExpression => "C3001",
            ErrorCode::CodegenInvalidFunction => "C3002",
            ErrorCode::CodegenInvalidStruct => "C3003",
            ErrorCode::CodegenInvalidEnum => "C3004",
            ErrorCode::CodegenInvalidInterface => "C3005",
            ErrorCode::CodegenInvalidModule => "C3006",
            ErrorCode::CodegenInvalidImport => "C3007",
            ErrorCode::CodegenInvalidExport => "C3008",
            
            // Runtime errors
            ErrorCode::RuntimePanic => "R4000",
            ErrorCode::RuntimeTaskError => "R4001",
            ErrorCode::RuntimeMemoryError => "R4002",
            ErrorCode::RuntimeTypeError => "R4003",
            ErrorCode::RuntimeValueError => "R4004",
        }
    }
    
    /// Get the error category
    pub fn category(&self) -> &'static str {
        match self {
            ErrorCode::LexerInvalidCharacter
            | ErrorCode::LexerUnterminatedString
            | ErrorCode::LexerUnterminatedComment
            | ErrorCode::LexerInvalidNumber
            | ErrorCode::LexerInvalidEscape => "Lexer",
            
            ErrorCode::ParserUnexpectedToken
            | ErrorCode::ParserExpectedToken
            | ErrorCode::ParserUnexpectedEof
            | ErrorCode::ParserInvalidExpression
            | ErrorCode::ParserInvalidStatement
            | ErrorCode::ParserInvalidType
            | ErrorCode::ParserInvalidPattern
            | ErrorCode::ParserInvalidFunction
            | ErrorCode::ParserInvalidStruct
            | ErrorCode::ParserInvalidEnum
            | ErrorCode::ParserInvalidInterface
            | ErrorCode::ParserInvalidModule
            | ErrorCode::ParserInvalidImport
            | ErrorCode::ParserInvalidExport => "Parser",
            
            ErrorCode::AnalyzerUndefinedVariable
            | ErrorCode::AnalyzerUndefinedFunction
            | ErrorCode::AnalyzerUndefinedType
            | ErrorCode::AnalyzerTypeMismatch
            | ErrorCode::AnalyzerTraitNotImplemented
            | ErrorCode::AnalyzerNonExhaustiveMatch
            | ErrorCode::AnalyzerDuplicateDefinition
            | ErrorCode::AnalyzerCircularDependency
            | ErrorCode::AnalyzerInvalidGeneric
            | ErrorCode::AnalyzerInvalidTraitBound
            | ErrorCode::AnalyzerInvalidInterface
            | ErrorCode::AnalyzerInvalidImplementation
            | ErrorCode::AnalyzerInvalidModule
            | ErrorCode::AnalyzerInvalidImport
            | ErrorCode::AnalyzerInvalidExport => "Analyzer",
            
            ErrorCode::CodegenInvalidType
            | ErrorCode::CodegenInvalidExpression
            | ErrorCode::CodegenInvalidFunction
            | ErrorCode::CodegenInvalidStruct
            | ErrorCode::CodegenInvalidEnum
            | ErrorCode::CodegenInvalidInterface
            | ErrorCode::CodegenInvalidModule
            | ErrorCode::CodegenInvalidImport
            | ErrorCode::CodegenInvalidExport => "Codegen",
            
            ErrorCode::RuntimePanic
            | ErrorCode::RuntimeTaskError
            | ErrorCode::RuntimeMemoryError
            | ErrorCode::RuntimeTypeError
            | ErrorCode::RuntimeValueError => "Runtime",
        }
    }
}
