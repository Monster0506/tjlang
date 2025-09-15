#pragma once
#include <memory>
#include <string>
#include <vector>
namespace ast {
struct Node {
    virtual ~Node() = default;
};

struct Type : Node {
    std::string name;
    std::vector<std::unique_ptr<Type>>
        args;  // for generics like Result<int,str>
};

struct Expr : Node {};
struct Stmt : Node {};

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

struct FunctionDecl : Node {
    std::string name;
    std::vector<Param> params;
    std::unique_ptr<Type> returnType;
    std::unique_ptr<Block> body;
};

struct Program : Node {
    std::vector<std::unique_ptr<Node>>
        units;  // functions, types, interfaces, etc.
};

}  // namespace ast
