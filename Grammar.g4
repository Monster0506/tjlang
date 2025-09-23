grammar Language;

/*
 * ---------------- PARSER RULES ----------------
 */

program
    : program_unit* EOF
    ;

program_unit
    : moduleDecl
    | importDecl
    | exportDecl
    | decl
    ;

/* ----- Modules ----- */
moduleDecl
    : MOD IDENTIFIER
    ;

importDecl
    : IMPORT qualifiedName (AS IDENTIFIER)?
    | IMPORT qualifiedName DOT LBRACE identifierList RBRACE
    ;

exportDecl
    : EXPORT (functionDecl | typeDecl | interfaceDecl)
    ;

qualifiedName
    : IDENTIFIER (DOT IDENTIFIER)*
    ;

identifierList
    : IDENTIFIER (COMMA IDENTIFIER)*
    ;

/* ----- Declarations ----- */
decl
    : functionDecl
    | interfaceDecl
    | typeDecl
    | enumDecl
    | structDecl
    | varDecl
    | implBlock
    ;


implBlock
    : IDENTIFIER FOR IDENTIFIER LBRACE methodDecl+ RBRACE
    ;

methodDecl
    : (IDENTIFIER | operatorSymbol) LPAREN paramList? RPAREN ARROW type block
    ;

/* Type alias */
typeDecl
    : TYPE IDENTIFIER ASSIGN type
    ;

/* ----- Types ----- */
type
    : unionType
    ;

unionType
    : optionType (PIPE optionType)*
    ;

optionType
    : QUESTION? functionType
    ;

functionType
    : LPAREN typeList? RPAREN ARROW functionType
    | collectionType
    | primaryType
    ;

collectionType
    : vecType
    | setType
    | mapType
    | tupleType
    ;

vecType
    : LBRACK type RBRACK
    ;

setType
    : LBRACE type RBRACE
    ;

mapType
    : primaryType LT type COMMA type GT
    ;

tupleType
    : LPAREN type (COMMA type)+ RPAREN
    ;

primaryType
    : primitiveType
    | IDENTIFIER (typeParams)?
    | RESULT LT type COMMA type GT
    | OPTION LT type GT
    ;

primitiveType
    : INT | FLOAT | BOOL | STR | ANY
    ;

typeList
    : type (COMMA type)*
    ;

typeParams
    : LT type (COMMA type)* GT
    ;

// Generic type parameter list (for function declarations)
genericTypeParams
    : LT type (COMMA type)* GT
    ;

/* ----- Structs & Enums ----- */
structDecl
    : TYPE IDENTIFIER LBRACE fieldDecl (COMMA fieldDecl)* RBRACE
    ;

fieldDecl
    : IDENTIFIER COLON type
    ;

/* Enums with optional type parameters */
enumDecl
    : ENUM IDENTIFIER (LT typeParamList GT)? LBRACE enumVariant+ RBRACE
    ;

typeParamList
    : IDENTIFIER (COMMA IDENTIFIER)*
    ;

enumVariant
    : IDENTIFIER (LPAREN variantFields? RPAREN)?
    ;

variantFields
    : type (COMMA type)*
    ;

/* ----- Interfaces ----- */
interfaceDecl
    : INTERFACE IDENTIFIER ( EXTENDS identifierList )? LBRACE methodSig+ RBRACE
    ;

methodSig
    : (IDENTIFIER | operatorSymbol) LPAREN paramList? RPAREN ARROW type
    ;

operatorSymbol
    : PLUS | MINUS | STAR | SLASH | PERCENT
    | EQ | NEQ | LT | GT | LTE | GTE
    | LBRACK RBRACK // for indexers
    | OR | AND | BANG
    ;

/* ----- Functions ----- */
functionDecl
    : DEF IDENTIFIER (genericParams)? LPAREN paramList? RPAREN ARROW type block
    ;

genericParams
    : LT typeParamBound (COMMA typeParamBound)* GT
    ;

typeParamBound
    : IDENTIFIER COLON IMPLEMENTS LBRACK identifierList RBRACK
    ;

paramList
    : param (COMMA param)*
    ;

param
    : IDENTIFIER COLON type
    ;

// Simple type for function parameters (avoids left recursion)
simpleType
    : primitiveType
    | IDENTIFIER (simpleTypeParams)?
    | LBRACK simpleType RBRACK
    | LBRACE simpleType RBRACE
    | LPAREN simpleType (COMMA simpleType)+ RPAREN
    | QUESTION simpleType
    | RESULT LT simpleType COMMA simpleType GT
    ;

simpleTypeParams
    : LT simpleType (COMMA simpleType)* GT
    ;

/* ----- Variables ----- */
varDecl
    : IDENTIFIER COLON type ASSIGN expression
    ;

/* ----- Statements ----- */
statement
    : varDecl
    | expression
    | ifStmt
    | whileStmt
    | doWhileStmt
    | forStmt
    | matchStmt
    | returnStmt
    | breakStmt
    | continueStmt
    | passStmt
    | raiseStmt
    | block
    ;

returnStmt
    : RETURN expression?
    ;

breakStmt : BREAK ;
continueStmt : CONTINUE ;
passStmt : PASS ;
raiseStmt : RAISE expression ;

block
    : LBRACE statement* RBRACE
    ;

/* ----- Conditionals ----- */
ifStmt
    : IF expression block elifBranch* elseBranch?
    ;

elifBranch
    : ELIF expression block
    ;

elseBranch
    : ELSE block
    ;

/* ----- Loops ----- */
whileStmt
    : WHILE expression block
    ;

doWhileStmt
    : DO block WHILE expression
    ;

forStmt
    : FOR LPAREN IDENTIFIER COLON type PIPE expression RPAREN block
    ;

/* ----- Match ----- */
matchStmt
    : MATCH expression LBRACE matchArm+ RBRACE
    ;

matchArm
    : pattern ( IF expression )? COLON block
    ;

/* ----- Patterns ----- */
pattern
    : literal
    | IDENTIFIER COLON type
    | IDENTIFIER COLON IMPLEMENTS LBRACK IDENTIFIER RBRACK
    | constructorPattern
    | LPAREN pattern (COMMA pattern)* RPAREN
    | UNDERSCORE
    ;

constructorPattern
    : IDENTIFIER (LPAREN patternFields? RPAREN)?
    ;

patternFields
    : pattern (COMMA pattern)*
    ;

literal
    : INT_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | FSTRING_LITERAL
    | BOOL_LITERAL
    | NONE
    | collectionLiteral
    | tupleLiteral
    | structLiteral
    ;

collectionLiteral
    : vecLiteral
    | setLiteral
    | mapLiteral
    ;

vecLiteral
    : LBRACK (expression (COMMA expression)*)? RBRACK
    ;

setLiteral
    : LBRACE (expression (COMMA expression)*)? RBRACE
    ;

mapLiteral
    : LBRACE (mapEntry (COMMA mapEntry)*)? RBRACE
    ;

mapEntry
    : expression COLON expression
    ;

tupleLiteral
    : LPAREN expression (COMMA expression)+ RPAREN
    ;

structLiteral
    : IDENTIFIER LBRACE fieldInit (COMMA fieldInit)* RBRACE
    ;

/* ----- Expressions ----- */
expression
    : assignment
    ;

assignment
    : orExpr (ASSIGN expression)?
    ;

orExpr
    : andExpr ( OR andExpr )*
    ;

andExpr
    : equality ( AND equality )*
    ;

equality
    : relational ( ( EQ | NEQ ) relational )*
    ;

relational
    : addExpr ( ( LT | GT | LTE | GTE ) addExpr )*
    ;

addExpr
    : mulExpr ( ( PLUS | MINUS ) mulExpr )*
    ;

mulExpr
    : unary ( ( STAR | SLASH | PERCENT ) unary )*
    ;

unary
    : (MINUS | BANG) unary
    | postfixExpr
    ;

/* postfix expressions: calls, indexing, member access */
postfixExpr
    : primary ( callSuffix | indexSuffix | memberSuffix )*
    ;

primary
    : IDENTIFIER
    | literal
    | LPAREN expression RPAREN
    | lambdaExpr
    | rangeExpr
    | SPAWN expression
    ;

rangeExpr
    : INT_LITERAL DOT DOT (ASSIGN? INT_LITERAL)   // 0..10 or 0..=10
    ;
    
callSuffix
    : LPAREN argumentList? RPAREN          // normal call
    | LPAREN fieldInitList RPAREN          // struct literal with field initializers
    ;

fieldInitList
    : fieldInit (COMMA fieldInit)*
    ;

fieldInit
    : IDENTIFIER COLON expression
    ;

indexSuffix
    : LBRACK expression RBRACK
    ;

memberSuffix
    : DOT IDENTIFIER
    ;

lambdaExpr
    : LPAREN paramList? RPAREN ARROW expression
    ;

argumentList
    : expression (COMMA expression)*
    ;

/*
 * ---------------- LEXER RULES ----------------
 */

// Keywords
DEF         : 'def' ;
RETURN      : 'return' ;
TYPE        : 'type' ;
ENUM        : 'enum' ;
INTERFACE   : 'interface' ;
MOD         : 'mod' ;
IMPORT      : 'import' ;
EXPORT      : 'export' ;
IF          : 'if' ;
ELIF        : 'elif' ;
ELSE        : 'else' ;
WHILE       : 'while' ;
DO          : 'do' ;
FOR         : 'for' ;
MATCH       : 'match' ;
IMPLEMENTS  : 'Implements' ;
SPAWN       : 'spawn' ;
RAISE       : 'raise' ;
BREAK       : 'break' ;
CONTINUE    : 'continue' ;
PASS        : 'pass' ;
AS          : 'as' ;
EXTENDS     : 'extends' ;

// Primitive Types
INT         : 'int' ;
FLOAT       : 'float' ;
BOOL        : 'bool' ;
STR         : 'str' ;
ANY         : 'any' ;

// Built-in Types
RESULT      : 'Result' ;
OPTION      : 'Option' ;

// Symbols
ARROW       : '->' ;
ASSIGN      : '=' ;
COLON       : ':' ;
COMMA       : ',' ;
DOT         : '.' ;
LPAREN      : '(' ;
RPAREN      : ')' ;
LBRACE      : '{' ;
RBRACE      : '}' ;
LBRACK      : '[' ;
RBRACK      : ']' ;
PLUS        : '+' ;
MINUS       : '-' ;
STAR        : '*' ;
SLASH       : '/' ;
PERCENT     : '%' ;
LT          : '<' ;
GT          : '>' ;
LTE         : '<=' ;
GTE         : '>=' ;
EQ          : '==' ;
NEQ         : '!=' ;
PIPE        : '|' ;
QUESTION    : '?' ;
OR          : 'or' ;
AND         : 'and' ;
BANG        : '!' ;
UNDERSCORE  : '_' ;

// Literals
INT_LITERAL     : [0-9]+ ;
FLOAT_LITERAL   : [0-9]+ '.' [0-9]+ ;
STRING_LITERAL  : '"' (~["\r\n])* '"' ;
FSTRING_LITERAL : 'f"' ( ~["{}] | '{' ~["}]* '}' )* '"' ;
BOOL_LITERAL    : 'true' | 'false' ;
NONE            : 'None' ;

// Identifier
IDENTIFIER
    : [a-zA-Z_] [a-zA-Z0-9_]*
    ;

// Whitespace / Comments
WS
    : [ \t\r\n]+ -> skip
    ;

COMMENT
    : '#' ~[\r\n]* -> skip
    ;