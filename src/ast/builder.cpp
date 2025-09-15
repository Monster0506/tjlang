#include <cassert>
#include "LanguageParser.h"
#include "builder.hpp"

using namespace antlr4;

std::unique_ptr<ast::Program> ASTBuilder::build(tree::ParseTree* tree) {
    return visitProgram(dynamic_cast<LanguageParser::ProgramContext*>(tree));
}

std::unique_ptr<ast::Program> ASTBuilder::visitProgram(LanguageParser::ProgramContext* ctx) {
    auto prog = make<ast::Program>();
    for (auto unit : ctx->program_unit()) {
        if (auto decl = unit->decl()) {
            auto declNode = visitFunctionDecl(dynamic_cast<LanguageParser::FunctionDeclContext*>(decl));
            if (declNode) {
                prog->units.push_back(std::move(declNode));
            }
        }
    }
    return prog;
}

std::unique_ptr<ast::Node> ASTBuilder::visitFunctionDecl(LanguageParser::FunctionDeclContext* ctx) {
    auto fn = make<ast::FunctionDecl>();
    fn->name = ctx->IDENTIFIER()->getText();

    // Build parameters
    if (auto plist = ctx->paramList()) {
        for (auto p : plist->param()) {
            ast::Param param;
            param.name = p->IDENTIFIER()->getText();
            param.type = buildType(p->type());
            fn->params.push_back(std::move(param));
        }
    }
    
    // Build return type
    fn->returnType = buildType(ctx->type());
    
    // Build body
    fn->body = visitBlock(ctx->block());
    
    return std::unique_ptr<ast::Node>(std::move(fn));
}

std::unique_ptr<ast::Block> ASTBuilder::visitBlock(LanguageParser::BlockContext* ctx) {
    auto block = make<ast::Block>();
    for (auto stmt : ctx->statement()) {
        if (auto returnStmt = stmt->returnStmt()) {
            auto stmtNode = visitReturnStmt(returnStmt);
            if (stmtNode) {
                block->stmts.push_back(std::move(stmtNode));
            }
        } else if (auto varDecl = stmt->varDecl()) {
            auto stmtNode = visitVarDecl(varDecl);
            if (stmtNode) {
                block->stmts.push_back(std::move(stmtNode));
            }
        }
    }
    return block;
}

std::unique_ptr<ast::Stmt> ASTBuilder::visitReturnStmt(LanguageParser::ReturnStmtContext* ctx) {
    auto ret = make<ast::ReturnStmt>();
    if (ctx->expression()) {
        ret->value = visitExpression(ctx->expression());
    }
    return std::unique_ptr<ast::Stmt>(std::move(ret));
}

std::unique_ptr<ast::Stmt> ASTBuilder::visitVarDecl(LanguageParser::VarDeclContext* ctx) {
    auto assign = make<ast::AssignStmt>();
    assign->name = ctx->IDENTIFIER()->getText();
    assign->value = visitExpression(ctx->expression());
    return std::unique_ptr<ast::Stmt>(std::move(assign));
}

std::unique_ptr<ast::Expr> ASTBuilder::visitExpression(LanguageParser::ExpressionContext* ctx) {
    if (ctx->assignment()) {
        return visitAssignment(ctx->assignment());
    }
    return nullptr;
}

std::unique_ptr<ast::Expr> ASTBuilder::visitAssignment(LanguageParser::AssignmentContext* ctx) {
    if (ctx->ASSIGN()) {
        auto lhs = visitOrExpr(ctx->orExpr());
        auto rhs = visitExpression(ctx->expression());
        
        auto bin = make<ast::BinaryExpr>();
        bin->op = "=";
        bin->lhs = std::move(lhs);
        bin->rhs = std::move(rhs);
        
        return std::unique_ptr<ast::Expr>(std::move(bin));
    }
    return visitOrExpr(ctx->orExpr());
}

// Helper function to build binary expressions
static std::unique_ptr<ast::Expr> buildBinaryExpr(
    std::vector<std::unique_ptr<ast::Expr>>& terms,
    const std::vector<std::string>& ops) {
    if (terms.empty()) return nullptr;
    if (terms.size() == 1) return std::move(terms[0]);
    
    auto result = std::move(terms[0]);
    for (size_t i = 0; i < ops.size(); ++i) {
        auto bin = std::make_unique<ast::BinaryExpr>();
        bin->op = ops[i];
        bin->lhs = std::move(result);
        bin->rhs = std::move(terms[i + 1]);
        result = std::move(bin);
    }
    return result;
}

std::unique_ptr<ast::Expr> ASTBuilder::visitOrExpr(LanguageParser::OrExprContext* ctx) {
    std::vector<std::unique_ptr<ast::Expr>> terms;
    std::vector<std::string> ops;
    
    for (auto expr : ctx->andExpr()) {
        terms.push_back(visitAndExpr(expr));
    }
    
    for (size_t i = 1; i < terms.size(); ++i) {
        ops.push_back("or");
    }
    
    return buildBinaryExpr(terms, ops);
}

std::unique_ptr<ast::Expr> ASTBuilder::visitAndExpr(LanguageParser::AndExprContext* ctx) {
    std::vector<std::unique_ptr<ast::Expr>> terms;
    std::vector<std::string> ops;
    
    for (auto expr : ctx->equality()) {
        terms.push_back(visitEquality(expr));
    }
    
    for (size_t i = 1; i < terms.size(); ++i) {
        ops.push_back("and");
    }
    
    return buildBinaryExpr(terms, ops);
}

std::unique_ptr<ast::Expr> ASTBuilder::visitEquality(LanguageParser::EqualityContext* ctx) {
    std::vector<std::unique_ptr<ast::Expr>> terms;
    std::vector<std::string> ops;
    
    for (auto expr : ctx->relational()) {
        terms.push_back(visitRelational(expr));
    }
    
    // Extract operators
    for (auto child : ctx->children) {
        if (auto token = dynamic_cast<antlr4::tree::TerminalNode*>(child)) {
            std::string op = token->getText();
            if (op == "==" || op == "!=") {
                ops.push_back(op);
            }
        }
    }
    
    return buildBinaryExpr(terms, ops);
}

std::unique_ptr<ast::Expr> ASTBuilder::visitRelational(LanguageParser::RelationalContext* ctx) {
    std::vector<std::unique_ptr<ast::Expr>> terms;
    std::vector<std::string> ops;
    
    for (auto expr : ctx->addExpr()) {
        terms.push_back(visitAddExpr(expr));
    }
    
    // Extract operators
    for (auto child : ctx->children) {
        if (auto token = dynamic_cast<antlr4::tree::TerminalNode*>(child)) {
            std::string op = token->getText();
            if (op == "<" || op == ">" || op == "<=" || op == ">=") {
                ops.push_back(op);
            }
        }
    }
    
    return buildBinaryExpr(terms, ops);
}

std::unique_ptr<ast::Expr> ASTBuilder::visitAddExpr(LanguageParser::AddExprContext* ctx) {
    std::vector<std::unique_ptr<ast::Expr>> terms;
    std::vector<std::string> ops;
    
    for (auto expr : ctx->mulExpr()) {
        terms.push_back(visitMulExpr(expr));
    }
    
    // Extract operators
    for (auto child : ctx->children) {
        if (auto token = dynamic_cast<antlr4::tree::TerminalNode*>(child)) {
            std::string op = token->getText();
            if (op == "+" || op == "-") {
                ops.push_back(op);
            }
        }
    }
    
    return buildBinaryExpr(terms, ops);
}

std::unique_ptr<ast::Expr> ASTBuilder::visitMulExpr(LanguageParser::MulExprContext* ctx) {
    std::vector<std::unique_ptr<ast::Expr>> terms;
    std::vector<std::string> ops;
    
    for (auto expr : ctx->unary()) {
        terms.push_back(visitUnary(expr));
    }
    
    // Extract operators
    for (auto child : ctx->children) {
        if (auto token = dynamic_cast<antlr4::tree::TerminalNode*>(child)) {
            std::string op = token->getText();
            if (op == "*" || op == "/" || op == "%") {
                ops.push_back(op);
            }
        }
    }
    
    return buildBinaryExpr(terms, ops);
}

std::unique_ptr<ast::Expr> ASTBuilder::visitUnary(LanguageParser::UnaryContext* ctx) {
    if (auto unary = ctx->unary()) {
        auto unaryExpr = make<ast::UnaryExpr>();
        unaryExpr->op = ctx->children.front()->getText();
        unaryExpr->expr = visitUnary(unary);
        return std::unique_ptr<ast::Expr>(std::move(unaryExpr));
    }
    return visitPostfixExpr(ctx->postfixExpr());
}

std::unique_ptr<ast::Expr> ASTBuilder::visitPostfixExpr(LanguageParser::PostfixExprContext* ctx) {
    auto base = visitPrimary(ctx->primary());
    
    size_t idx = 1;
    while (idx < ctx->children.size()) {
        auto child = ctx->children[idx];
        std::string text = child->getText();
        
        if (text == "(") {
            // Function call
            auto call = make<ast::CallExpr>();
            call->callee = std::move(base);
            base = std::unique_ptr<ast::Expr>(std::move(call));
        } else if (text == "[") {
            // Array indexing
            auto index = make<ast::IndexExpr>();
            index->object = std::move(base);
            base = std::unique_ptr<ast::Expr>(std::move(index));
        } else if (text == ".") {
            // Member access
            auto member = make<ast::MemberExpr>();
            member->object = std::move(base);
            
            if (idx + 1 < ctx->children.size()) {
                auto idToken = dynamic_cast<antlr4::tree::TerminalNode*>(ctx->children[idx + 1]);
                member->member = idToken ? idToken->getText() : "";
                idx++; // Skip the identifier
            }
            base = std::unique_ptr<ast::Expr>(std::move(member));
        }
        idx++;
    }
    
    return base;
}

std::unique_ptr<ast::Expr> ASTBuilder::visitPrimary(LanguageParser::PrimaryContext* ctx) {
    if (auto id = ctx->IDENTIFIER()) {
        auto identifier = make<ast::Identifier>();
        identifier->name = id->getText();
        return std::unique_ptr<ast::Expr>(std::move(identifier));
    }
    
    if (auto lit = ctx->literal()) {
        if (auto intLit = lit->INT_LITERAL()) {
            auto intExpr = make<ast::IntLiteral>();
            intExpr->value = std::stoll(intLit->getText());
            return std::unique_ptr<ast::Expr>(std::move(intExpr));
        }
        
        if (auto floatLit = lit->FLOAT_LITERAL()) {
            auto floatExpr = make<ast::FloatLiteral>();
            floatExpr->value = std::stod(floatLit->getText());
            return std::unique_ptr<ast::Expr>(std::move(floatExpr));
        }
        
        if (auto strLit = lit->STRING_LITERAL()) {
            auto strExpr = make<ast::StringLiteral>();
            std::string text = strLit->getText();
            strExpr->value = text.substr(1, text.size() - 2); // Remove quotes
            return std::unique_ptr<ast::Expr>(std::move(strExpr));
        }
    }
    
    if (ctx->expression()) {
        return visitExpression(ctx->expression());
    }
    
    return nullptr;
}

std::unique_ptr<ast::Type> ASTBuilder::buildType(LanguageParser::TypeContext* ctx) {
    auto type = make<ast::Type>();
    
    if (auto primitive = ctx->primitiveType()) {
        type->name = primitive->getText();
        return type;
    }
    
    if (auto id = ctx->IDENTIFIER()) {
        type->name = id->getText();
        
        if (auto typeParams = ctx->typeParams()) {
            for (auto param : typeParams->type()) {
                type->args.push_back(buildType(param));
            }
        }
        return type;
    }
    
    if (ctx->QUESTION()) {
        // Optional type: ?Type becomes Option<Type>
        auto optionType = make<ast::Type>();
        optionType->name = "Option";
        optionType->args.push_back(buildType(ctx->type(0)));
        return optionType;
    }
    
    if (ctx->PIPE()) {
        // Union type: Type1 | Type2 becomes Union<Type1, Type2>
        auto unionType = make<ast::Type>();
        unionType->name = "Union";
        unionType->args.push_back(buildType(ctx->type(0)));
        unionType->args.push_back(buildType(ctx->type(1)));
        return unionType;
    }
    
    // Fallback
    type->name = ctx->getText();
    return type;
}