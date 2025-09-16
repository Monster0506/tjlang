// builder.hpp
#pragma once
#include "../ast/nodes.hpp"
#include "LanguageParser.h"

class ASTBuilder {
   public:
    ASTBuilder(bool debug = false) : debugMode(debug) {}

    std::unique_ptr<ast::Program> build(antlr4::tree::ParseTree* tree);

    // Direct visitor methods that return the actual types we need
    std::unique_ptr<ast::Program> visitProgram(
        LanguageParser::ProgramContext* ctx);
    std::unique_ptr<ast::Node> visitFunctionDecl(
        LanguageParser::FunctionDeclContext* ctx);
    std::unique_ptr<ast::Block> visitBlock(LanguageParser::BlockContext* ctx);
    std::unique_ptr<ast::Stmt> visitReturnStmt(
        LanguageParser::ReturnStmtContext* ctx);
    std::unique_ptr<ast::Stmt> visitVarDecl(
        LanguageParser::VarDeclContext* ctx);
    std::unique_ptr<ast::Expr> visitExpression(
        LanguageParser::ExpressionContext* ctx);
    std::unique_ptr<ast::Expr> visitAssignment(
        LanguageParser::AssignmentContext* ctx);
    std::unique_ptr<ast::Expr> visitOrExpr(LanguageParser::OrExprContext* ctx);
    std::unique_ptr<ast::Expr> visitAndExpr(
        LanguageParser::AndExprContext* ctx);
    std::unique_ptr<ast::Expr> visitEquality(
        LanguageParser::EqualityContext* ctx);
    std::unique_ptr<ast::Expr> visitRelational(
        LanguageParser::RelationalContext* ctx);
    std::unique_ptr<ast::Expr> visitAddExpr(
        LanguageParser::AddExprContext* ctx);
    std::unique_ptr<ast::Expr> visitMulExpr(
        LanguageParser::MulExprContext* ctx);
    std::unique_ptr<ast::Expr> visitUnary(LanguageParser::UnaryContext* ctx);
    std::unique_ptr<ast::Expr> visitPostfixExpr(
        LanguageParser::PostfixExprContext* ctx);
    std::unique_ptr<ast::Expr> visitPrimary(
        LanguageParser::PrimaryContext* ctx);

    // Helper methods
    std::unique_ptr<ast::Type> buildType(LanguageParser::TypeContext* ctx);
    std::unique_ptr<ast::Type> buildOptionType(
        LanguageParser::OptionTypeContext* ctx);
    std::unique_ptr<ast::Type> buildFunctionType(
        LanguageParser::FunctionTypeContext* ctx);
    std::unique_ptr<ast::Type> buildCollectionType(
        LanguageParser::CollectionTypeContext* ctx);
    std::unique_ptr<ast::Type> buildPrimaryType(
        LanguageParser::PrimaryTypeContext* ctx);
    std::unique_ptr<ast::Type> buildSimpleType(
        LanguageParser::SimpleTypeContext* ctx);

   private:
    bool debugMode = false;

    template <typename T, typename... Args>
    std::unique_ptr<T> make(Args&&... args) {
        return std::make_unique<T>(std::forward<Args>(args)...);
    }

    // Debug helpers
    void debug(const std::string& message) {
        if (debugMode) {
            std::cout << "[DEBUG] " << message << std::endl;
        }
    }

    void debug(const std::string& message, const std::string& context) {
        if (debugMode) {
            std::cout << "[DEBUG] " << message << " (" << context << ")"
                      << std::endl;
        }
    }

    void debugError(const std::string& message) {
        if (debugMode) {
            std::cerr << "[DEBUG ERROR] " << message << std::endl;
        }
    }

    void debugTree(antlr4::tree::ParseTree* tree, int depth = 0) {
        if (!debugMode) return;

        std::string indent(depth * 2, ' ');
        std::cout << indent << "Tree: " << tree->getText()
                  << " (type: " << typeid(*tree).name() << ")" << std::endl;

        for (int i = 0; i < tree->children.size(); ++i) {
            debugTree(tree->children[i], depth + 1);
        }
    }
};
