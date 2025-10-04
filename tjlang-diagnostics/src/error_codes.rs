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

    // Code Quality Rules (A2100-A2199)
    AnalyzerNamingConvention,
    AnalyzerFunctionComplexity,
    AnalyzerMagicNumber,
    AnalyzerParameterCount,
    AnalyzerFunctionLength,
    AnalyzerNestingDepth,
    AnalyzerEmptyFunction,
    AnalyzerCommentCoverage,
    AnalyzerCommentStyle,
    AnalyzerImportOrder,

    // Dead Code & Usage Rules (A2200-A2299)
    AnalyzerUnusedVariable,
    AnalyzerUnusedParameter,
    AnalyzerDeadCode,
    AnalyzerUnreachableCode,
    AnalyzerRecursionDepth,
    AnalyzerResourceLeak,

    // Performance Rules (A2300-A2399)
    AnalyzerInefficientLoop,
    AnalyzerMemoryAllocation,
    AnalyzerStringConcatenation,
    AnalyzerCacheEfficiency,
    AnalyzerBranchPrediction,
    AnalyzerVectorization,

    // Architecture Rules (A2400-A2499)
    AnalyzerLargeFile,
    AnalyzerTooManyImports,
    AnalyzerGlobalVariable,
    AnalyzerCoupling,
    AnalyzerCohesion,

    // Style & Formatting Rules (A2500-A2599)
    AnalyzerFormattingConvention,
    AnalyzerIndentation,
    AnalyzerTrailingWhitespace,
    AnalyzerLineLength,
    AnalyzerSemicolon,
    AnalyzerBracketMatching,

    // Security & Safety Rules (A2600-A2699)
    AnalyzerNullPointer,
    AnalyzerBufferOverflow,
    AnalyzerUnsafeOperation,
    AnalyzerInputValidation,
    AnalyzerHardcodedCredentials,
    AnalyzerSQLInjection,
    AnalyzerConcurrency,
    AnalyzerMemoryLeak,
    AnalyzerRaceCondition,

    // Language-Specific Rules (A2700-A2799)
    AnalyzerAsyncAwait,
    AnalyzerErrorHandling,
    AnalyzerPatternMatching,
    AnalyzerGenericConstraint,

    // Static Analysis - Semantic Errors (A2800-A2899)
    AnalyzerIndexOutOfBoundsStatic,
    AnalyzerDivisionByZeroStatic,
    AnalyzerNullPointerStatic,
    AnalyzerWrongArgumentCount,
    AnalyzerMethodNotFoundStatic,
    AnalyzerInvalidCastStatic,

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

    // File system errors (F5000-F5999)
    FileNotFound,
    FilePermissionDenied,
    FileReadError,
    FileWriteError,
    FileInvalidPath,
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

            // Code Quality Rules (A2100-A2199)
            ErrorCode::AnalyzerNamingConvention => "A2100",
            ErrorCode::AnalyzerFunctionComplexity => "A2101",
            ErrorCode::AnalyzerMagicNumber => "A2102",
            ErrorCode::AnalyzerParameterCount => "A2103",
            ErrorCode::AnalyzerFunctionLength => "A2104",
            ErrorCode::AnalyzerNestingDepth => "A2105",
            ErrorCode::AnalyzerEmptyFunction => "A2106",
            ErrorCode::AnalyzerCommentCoverage => "A2107",
            ErrorCode::AnalyzerCommentStyle => "A2108",
            ErrorCode::AnalyzerImportOrder => "A2109",

            // Dead Code & Usage Rules (A2200-A2299)
            ErrorCode::AnalyzerUnusedVariable => "A2200",
            ErrorCode::AnalyzerUnusedParameter => "A2201",
            ErrorCode::AnalyzerDeadCode => "A2202",
            ErrorCode::AnalyzerUnreachableCode => "A2203",
            ErrorCode::AnalyzerRecursionDepth => "A2204",
            ErrorCode::AnalyzerResourceLeak => "A2205",

            // Performance Rules (A2300-A2399)
            ErrorCode::AnalyzerInefficientLoop => "A2300",
            ErrorCode::AnalyzerMemoryAllocation => "A2301",
            ErrorCode::AnalyzerStringConcatenation => "A2302",
            ErrorCode::AnalyzerCacheEfficiency => "A2303",
            ErrorCode::AnalyzerBranchPrediction => "A2304",
            ErrorCode::AnalyzerVectorization => "A2305",

            // Architecture Rules (A2400-A2499)
            ErrorCode::AnalyzerLargeFile => "A2400",
            ErrorCode::AnalyzerTooManyImports => "A2401",
            ErrorCode::AnalyzerGlobalVariable => "A2402",
            ErrorCode::AnalyzerCoupling => "A2403",
            ErrorCode::AnalyzerCohesion => "A2404",

            // Style & Formatting Rules (A2500-A2599)
            ErrorCode::AnalyzerFormattingConvention => "A2500",
            ErrorCode::AnalyzerIndentation => "A2501",
            ErrorCode::AnalyzerTrailingWhitespace => "A2502",
            ErrorCode::AnalyzerLineLength => "A2503",
            ErrorCode::AnalyzerSemicolon => "A2504",
            ErrorCode::AnalyzerBracketMatching => "A2505",

            // Security & Safety Rules (A2600-A2699)
            ErrorCode::AnalyzerNullPointer => "A2600",
            ErrorCode::AnalyzerBufferOverflow => "A2601",
            ErrorCode::AnalyzerUnsafeOperation => "A2602",
            ErrorCode::AnalyzerInputValidation => "A2603",
            ErrorCode::AnalyzerHardcodedCredentials => "A2604",
            ErrorCode::AnalyzerSQLInjection => "A2605",
            ErrorCode::AnalyzerConcurrency => "A2606",
            ErrorCode::AnalyzerMemoryLeak => "A2607",
            ErrorCode::AnalyzerRaceCondition => "A2608",

            // Language-Specific Rules (A2700-A2799)
            ErrorCode::AnalyzerAsyncAwait => "A2700",
            ErrorCode::AnalyzerErrorHandling => "A2701",
            ErrorCode::AnalyzerPatternMatching => "A2702",
            ErrorCode::AnalyzerGenericConstraint => "A2703",

            // Static Analysis - Semantic Errors (A2800-A2899)
            ErrorCode::AnalyzerIndexOutOfBoundsStatic => "A2800",
            ErrorCode::AnalyzerDivisionByZeroStatic => "A2801",
            ErrorCode::AnalyzerNullPointerStatic => "A2802",
            ErrorCode::AnalyzerWrongArgumentCount => "A2803",
            ErrorCode::AnalyzerMethodNotFoundStatic => "A2804",
            ErrorCode::AnalyzerInvalidCastStatic => "A2805",

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

            // File system errors
            ErrorCode::FileNotFound => "F5000",
            ErrorCode::FilePermissionDenied => "F5001",
            ErrorCode::FileReadError => "F5002",
            ErrorCode::FileWriteError => "F5003",
            ErrorCode::FileInvalidPath => "F5004",
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
            | ErrorCode::AnalyzerInvalidExport
            | ErrorCode::AnalyzerNamingConvention
            | ErrorCode::AnalyzerFunctionComplexity
            | ErrorCode::AnalyzerMagicNumber
            | ErrorCode::AnalyzerParameterCount
            | ErrorCode::AnalyzerFunctionLength
            | ErrorCode::AnalyzerNestingDepth
            | ErrorCode::AnalyzerEmptyFunction
            | ErrorCode::AnalyzerCommentCoverage
            | ErrorCode::AnalyzerCommentStyle
            | ErrorCode::AnalyzerImportOrder
            | ErrorCode::AnalyzerUnusedVariable
            | ErrorCode::AnalyzerUnusedParameter
            | ErrorCode::AnalyzerDeadCode
            | ErrorCode::AnalyzerUnreachableCode
            | ErrorCode::AnalyzerRecursionDepth
            | ErrorCode::AnalyzerResourceLeak
            | ErrorCode::AnalyzerInefficientLoop
            | ErrorCode::AnalyzerMemoryAllocation
            | ErrorCode::AnalyzerStringConcatenation
            | ErrorCode::AnalyzerCacheEfficiency
            | ErrorCode::AnalyzerBranchPrediction
            | ErrorCode::AnalyzerVectorization
            | ErrorCode::AnalyzerLargeFile
            | ErrorCode::AnalyzerTooManyImports
            | ErrorCode::AnalyzerGlobalVariable
            | ErrorCode::AnalyzerCoupling
            | ErrorCode::AnalyzerCohesion
            | ErrorCode::AnalyzerFormattingConvention
            | ErrorCode::AnalyzerIndentation
            | ErrorCode::AnalyzerTrailingWhitespace
            | ErrorCode::AnalyzerLineLength
            | ErrorCode::AnalyzerSemicolon
            | ErrorCode::AnalyzerBracketMatching
            | ErrorCode::AnalyzerNullPointer
            | ErrorCode::AnalyzerBufferOverflow
            | ErrorCode::AnalyzerUnsafeOperation
            | ErrorCode::AnalyzerInputValidation
            | ErrorCode::AnalyzerHardcodedCredentials
            | ErrorCode::AnalyzerSQLInjection
            | ErrorCode::AnalyzerConcurrency
            | ErrorCode::AnalyzerMemoryLeak
            | ErrorCode::AnalyzerRaceCondition
            | ErrorCode::AnalyzerAsyncAwait
            | ErrorCode::AnalyzerErrorHandling
            | ErrorCode::AnalyzerPatternMatching
            | ErrorCode::AnalyzerGenericConstraint
            | ErrorCode::AnalyzerIndexOutOfBoundsStatic
            | ErrorCode::AnalyzerDivisionByZeroStatic
            | ErrorCode::AnalyzerNullPointerStatic
            | ErrorCode::AnalyzerWrongArgumentCount
            | ErrorCode::AnalyzerMethodNotFoundStatic
            | ErrorCode::AnalyzerInvalidCastStatic => "Analyzer",

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

            ErrorCode::FileNotFound
            | ErrorCode::FilePermissionDenied
            | ErrorCode::FileReadError
            | ErrorCode::FileWriteError
            | ErrorCode::FileInvalidPath => "File System",
        }
    }
}
