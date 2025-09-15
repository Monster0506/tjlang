#pragma once
#include <memory>
#include <string>
#include <vector>
#include <map>
#include <variant>

namespace ast {

struct Node {
    virtual ~Node() = default;
};

// Forward declarations
struct Type;
struct Expr;
struct Stmt;
struct Pattern;
struct Block;
struct FunctionDecl;

// Type system nodes
struct Type : Node {
    enum class Kind {
        PRIMITIVE,    // int, float, bool, str, any
        TUPLE,        // (T1, T2, T3)
        VEC,          // [T]
        SET,          // {T}
        MAP,          // T1<T2> (key-value map)
        FUNCTION,     // (T1, T2) -> T3
        UNION,        // T1 | T2
        OPTION,       // Option<T> or T?
        RESULT,       // Result<T, E>
        STRUCT,       // struct { field: T }
        ENUM,         // enum { Variant1(T1), Variant2(T2) }
        INTERFACE,    // interface { method: T }
        GENERIC,      // T with constraints
        NAMED         // User-defined type name
    };
    
    Kind kind;
    std::string name;
    std::vector<std::unique_ptr<Type>> args;
    std::vector<std::unique_ptr<Type>> constraints; // for generics
    
    // For tuples: args contains element types
    // For functions: args[0..n-2] are params, args[n-1] is return type
    // For unions: args contains unioned types
    // For structs: args contains field types, name contains field names
    // For enums: args contains variant types, name contains variant names
};

struct Expr : Node {};
struct Stmt : Node {};

// Literals
struct Identifier : Expr {
    std::string name;
};

struct IntLiteral : Expr {
    long long value;
};

struct FloatLiteral : Expr {
    double value;
};

struct StringLiteral : Expr {
    std::string value;
};

struct BoolLiteral : Expr {
    bool value;
};

// Collection literals
struct VecLiteral : Expr {
    std::vector<std::unique_ptr<Expr>> elements;
    std::unique_ptr<Type> elementType; // optional type annotation
};

struct SetLiteral : Expr {
    std::vector<std::unique_ptr<Expr>> elements;
    std::unique_ptr<Type> elementType; // optional type annotation
};

struct MapLiteral : Expr {
    std::vector<std::pair<std::unique_ptr<Expr>, std::unique_ptr<Expr>>> entries;
    std::unique_ptr<Type> keyType;     // optional type annotation
    std::unique_ptr<Type> valueType;   // optional type annotation
};

// Tuple literal
struct TupleLiteral : Expr {
    std::vector<std::unique_ptr<Expr>> elements;
    std::vector<std::unique_ptr<Type>> elementTypes; // optional type annotation
};

// Struct literal
struct StructLiteral : Expr {
    std::string typeName;
    std::vector<std::pair<std::string, std::unique_ptr<Expr>>> fields;
};

// Enum variant construction
struct EnumVariant : Expr {
    std::string enumName;
    std::string variantName;
    std::vector<std::unique_ptr<Expr>> args;
};

struct CallExpr : Expr {
    std::unique_ptr<Expr> callee;  // Identifier or member
    std::vector<std::unique_ptr<Expr>> args;
    // For struct-literal-style field inits Point(x: expr, y: expr)
    std::vector<std::pair<std::string, std::unique_ptr<Expr>>> namedArgs;
    bool hasNamed = false;
};

struct MemberExpr : Expr {
    std::unique_ptr<Expr> object;
    std::string member;
};

struct IndexExpr : Expr {
    std::unique_ptr<Expr> object;
    std::unique_ptr<Expr> index;
};

struct UnaryExpr : Expr {
    std::string op;  // "-", "!"
    std::unique_ptr<Expr> expr;
};

struct BinaryExpr : Expr {
    std::string op;  // "+", "-", "*", "/", "%", "==", ...
    std::unique_ptr<Expr> lhs;
    std::unique_ptr<Expr> rhs;
};

// Pattern matching
struct Pattern : Node {
    enum class Kind {
        LITERAL,        // 42, "hello", true
        VARIABLE,       // x
        TYPED_VARIABLE, // x: int
        TRAIT_GUARD,    // x: Implements[Display]
        TUPLE,          // (a, b, c)
        STRUCT,         // Point { x, y }
        ENUM_VARIANT,   // Some(x) or None
        WILDCARD        // _
    };
    
    Kind kind;
    std::string name;  // for variable patterns
    std::unique_ptr<Type> type;  // for typed patterns
    std::string traitName;  // for trait guard patterns
    std::vector<std::unique_ptr<Pattern>> subPatterns;  // for compound patterns
    std::unique_ptr<Expr> literal;  // for literal patterns
};

// Control flow statements
struct IfStmt : Stmt {
    std::unique_ptr<Expr> condition;
    std::unique_ptr<Block> thenBlock;
    std::vector<std::pair<std::unique_ptr<Expr>, std::unique_ptr<Block>>> elifBranches;
    std::unique_ptr<Block> elseBlock;
};

struct WhileStmt : Stmt {
    std::unique_ptr<Expr> condition;
    std::unique_ptr<Block> body;
};

struct ForStmt : Stmt {
    std::string variable;
    std::unique_ptr<Type> variableType;
    std::unique_ptr<Expr> iterable;
    std::unique_ptr<Block> body;
};

struct MatchStmt : Stmt {
    std::unique_ptr<Expr> expression;
    std::vector<std::pair<std::unique_ptr<Pattern>, std::unique_ptr<Block>>> arms;
    std::vector<std::unique_ptr<Expr>> guards;  // optional if conditions
};

struct AssignStmt : Stmt {
    std::string name;
    std::unique_ptr<Expr> value;
};

struct ReturnStmt : Stmt {
    std::unique_ptr<Expr> value;  // may be null
};

struct Block : Stmt {
    std::vector<std::unique_ptr<Stmt>> stmts;
};

struct Param {
    std::string name;
    std::unique_ptr<Type> type;
};

struct GenericParam {
    std::string name;
    std::vector<std::string> constraints;  // interface names
};

// Type declarations
struct StructDecl : Node {
    std::string name;
    std::vector<GenericParam> genericParams;
    std::vector<std::pair<std::string, std::unique_ptr<Type>>> fields;
};

struct EnumDecl : Node {
    std::string name;
    std::vector<GenericParam> genericParams;
    std::vector<std::pair<std::string, std::vector<std::unique_ptr<Type>>>> variants;
};

struct InterfaceDecl : Node {
    std::string name;
    std::vector<std::string> extends;  // parent interfaces
    std::vector<std::pair<std::string, std::unique_ptr<Type>>> methods;  // method name -> signature
};

struct TypeAlias : Node {
    std::string name;
    std::vector<GenericParam> genericParams;
    std::unique_ptr<Type> aliasedType;
};

// Implementation blocks
struct ImplBlock : Node {
    std::string typeName;
    std::string interfaceName;  // empty for inherent impls
    std::vector<std::unique_ptr<FunctionDecl>> methods;
};

struct FunctionDecl : Node {
    std::string name;
    std::vector<GenericParam> genericParams;
    std::vector<Param> params;
    std::unique_ptr<Type> returnType;
    std::unique_ptr<Block> body;
};

struct Program : Node {
    std::vector<std::unique_ptr<Node>>
        units;  // functions, types, interfaces, etc.
};

// Type checking context
struct TypeContext {
    std::map<std::string, std::unique_ptr<Type>> variables;
    std::map<std::string, std::unique_ptr<Type>> types;
    std::map<std::string, std::unique_ptr<InterfaceDecl>> interfaces;
    std::vector<std::map<std::string, std::unique_ptr<Type>>> scopes;
    
    void enterScope();
    void exitScope();
    void declareVariable(const std::string& name, std::unique_ptr<Type> type);
    void declareType(const std::string& name, std::unique_ptr<Type> type);
    void declareInterface(const std::string& name, std::unique_ptr<InterfaceDecl> interface);
    std::unique_ptr<Type> lookupVariable(const std::string& name);
    std::unique_ptr<Type> lookupType(const std::string& name);
    std::unique_ptr<InterfaceDecl> lookupInterface(const std::string& name);
};

}  // namespace ast
