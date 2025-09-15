// builder.hpp
#pragma once
#include "LanguageParser.h"
#include "nodes.hpp"

class ASTBuilder {
   public:
    std::unique_ptr<ast::Program> build(antlr4::tree::ParseTree* tree);

    // Direct visitor methods that return the actual types we need
    std::unique_ptr<ast::Program> visitProgram(LanguageParser::ProgramContext* ctx);
    std::unique_ptr<ast::Node> visitFunctionDecl(LanguageParser::FunctionDeclContext* ctx);
    std::unique_ptr<ast::Block> visitBlock(LanguageParser::BlockContext* ctx);
    std::unique_ptr<ast::Stmt> visitReturnStmt(LanguageParser::ReturnStmtContext* ctx);
    std::unique_ptr<ast::Stmt> visitVarDecl(LanguageParser::VarDeclContext* ctx);
    std::unique_ptr<ast::Expr> visitExpression(LanguageParser::ExpressionContext* ctx);
    std::unique_ptr<ast::Expr> visitAssignment(LanguageParser::AssignmentContext* ctx);
    std::unique_ptr<ast::Expr> visitOrExpr(LanguageParser::OrExprContext* ctx);
    std::unique_ptr<ast::Expr> visitAndExpr(LanguageParser::AndExprContext* ctx);
    std::unique_ptr<ast::Expr> visitEquality(LanguageParser::EqualityContext* ctx);
    std::unique_ptr<ast::Expr> visitRelational(LanguageParser::RelationalContext* ctx);
    std::unique_ptr<ast::Expr> visitAddExpr(LanguageParser::AddExprContext* ctx);
    std::unique_ptr<ast::Expr> visitMulExpr(LanguageParser::MulExprContext* ctx);
    std::unique_ptr<ast::Expr> visitUnary(LanguageParser::UnaryContext* ctx);
    std::unique_ptr<ast::Expr> visitPostfixExpr(LanguageParser::PostfixExprContext* ctx);
    std::unique_ptr<ast::Expr> visitPrimary(LanguageParser::PrimaryContext* ctx);

    // Helper methods
    std::unique_ptr<ast::Type> buildType(LanguageParser::TypeContext* ctx);

   private:
    template <typename T, typename... Args>
    std::unique_ptr<T> make(Args&&... args) {
        return std::make_unique<T>(std::forward<Args>(args)...);
    }
    
    // Generic visitor dispatcher
    template<typename T>
    T visit(antlr4::tree::ParseTree* tree) {
        // This will be specialized for each return type
        static_assert(sizeof(T) == 0, "visit must be specialized for each type");
    }
};