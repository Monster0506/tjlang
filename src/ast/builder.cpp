#include <cassert>

#include "LanguageParser.h"
#include "builder.hpp"

using namespace antlr4;

std::unique_ptr<ast::Program> ASTBuilder::build(tree::ParseTree* tree) {
    debug("Starting AST build");
    debugTree(tree);
    
    auto programCtx = dynamic_cast<LanguageParser::ProgramContext*>(tree);
    if (!programCtx) {
        debugError("Failed to cast tree to ProgramContext");
        return nullptr;
    }
    
    debug("Calling visitProgram");
    auto result = visitProgram(programCtx);
    
    if (result) {
        debug("AST build completed successfully");
    } else {
        debugError("AST build failed - visitProgram returned nullptr");
    }
    
    return result;
}

std::unique_ptr<ast::Program> ASTBuilder::visitProgram(
    LanguageParser::ProgramContext* ctx) {
    debug("visitProgram: Processing program with " + std::to_string(ctx->program_unit().size()) + " units");
    
    auto prog = make<ast::Program>();
    for (size_t i = 0; i < ctx->program_unit().size(); ++i) {
        auto unit = ctx->program_unit(i);
        debug("visitProgram: Processing unit " + std::to_string(i));
        
        if (auto decl = unit->decl()) {
            debug("visitProgram: Found declaration");
            debug("visitProgram: Declaration type = " + std::string(typeid(*decl).name()));
            
            // Check if this is a function declaration
            if (auto funcDecl = decl->functionDecl()) {
                debug("visitProgram: Found function declaration within decl");
                auto declNode = visitFunctionDecl(funcDecl);
                if (declNode) {
                    debug("visitProgram: Successfully created declaration node");
                    prog->units.push_back(std::move(declNode));
                } else {
                    debugError("visitProgram: Failed to create declaration node");
                }
            } else {
                debug("visitProgram: No function declaration found in decl");
            }
        } else {
            debug("visitProgram: No declaration found in unit");
        }
    }
    
    debug("visitProgram: Created program with " + std::to_string(prog->units.size()) + " units");
    return prog;
}

std::unique_ptr<ast::Node> ASTBuilder::visitFunctionDecl(
    LanguageParser::FunctionDeclContext* ctx) {
    try {
        debug("visitFunctionDecl: Starting function declaration");
        
        if (!ctx) {
            debugError("visitFunctionDecl: Context is null");
            return nullptr;
        }
        
        debug("visitFunctionDecl: Creating FunctionDecl node");
        auto fn = make<ast::FunctionDecl>();
        
        debug("visitFunctionDecl: Getting function identifier");
        debug("visitFunctionDecl: About to get children count");
        debug("visitFunctionDecl: Context children count = " + std::to_string(ctx->children.size()));
        
        // Try to find the identifier in the children
        for (size_t i = 0; i < ctx->children.size(); ++i) {
            auto child = ctx->children[i];
            debug("visitFunctionDecl: Child " + std::to_string(i) + " type = " + typeid(*child).name());
            if (auto terminal = dynamic_cast<antlr4::tree::TerminalNode*>(child)) {
                debug("visitFunctionDecl: Child " + std::to_string(i) + " text = '" + terminal->getText() + "'");
            }
        }
        
        if (!ctx->IDENTIFIER()) {
            debugError("visitFunctionDecl: No IDENTIFIER found in function declaration");
            return nullptr;
        }
        
        fn->name = ctx->IDENTIFIER()->getText();
        debug("visitFunctionDecl: Function name = " + fn->name);

    // Build parameters
    debug("visitFunctionDecl: Checking for parameter list...");
    if (auto plist = ctx->paramList()) {
        debug("visitFunctionDecl: Found parameter list with " + std::to_string(plist->param().size()) + " parameters");
        for (size_t i = 0; i < plist->param().size(); ++i) {
            auto p = plist->param(i);
            ast::Param param;
            param.name = p->IDENTIFIER()->getText();
            debug("visitFunctionDecl: Parameter " + std::to_string(i) + " name = " + param.name);
            debug("visitFunctionDecl: Building parameter type...");
            param.type = buildType(p->type());
            if (!param.type) {
                debugError("visitFunctionDecl: Failed to build parameter type for " + param.name);
                return nullptr;
            }
            fn->params.push_back(std::move(param));
        }
    } else {
        debug("visitFunctionDecl: No parameter list found");
    }
    
    // Build return type
    debug("visitFunctionDecl: Building return type");
    fn->returnType = buildType(ctx->type());
    if (!fn->returnType) {
        debugError("visitFunctionDecl: Failed to build return type");
        return nullptr;
    }
    
    // Build body
    debug("visitFunctionDecl: Building function body");
    fn->body = visitBlock(ctx->block());
    if (!fn->body) {
        debugError("visitFunctionDecl: Failed to build function body");
        return nullptr;
    }
    
    debug("visitFunctionDecl: Successfully created function declaration");
    return std::unique_ptr<ast::Node>(std::move(fn));
    
    } catch (const std::exception& e) {
        debugError("visitFunctionDecl: Exception caught: " + std::string(e.what()));
        return nullptr;
    } catch (...) {
        debugError("visitFunctionDecl: Unknown exception caught");
        return nullptr;
    }
}

std::unique_ptr<ast::Block> ASTBuilder::visitBlock(
    LanguageParser::BlockContext* ctx) {
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

std::unique_ptr<ast::Stmt> ASTBuilder::visitReturnStmt(
    LanguageParser::ReturnStmtContext* ctx) {
    auto ret = make<ast::ReturnStmt>();
    if (ctx->expression()) {
        ret->value = visitExpression(ctx->expression());
    }
    return std::unique_ptr<ast::Stmt>(std::move(ret));
}

std::unique_ptr<ast::Stmt> ASTBuilder::visitVarDecl(
    LanguageParser::VarDeclContext* ctx) {
    auto assign = make<ast::AssignStmt>();
    assign->name = ctx->IDENTIFIER()->getText();
    assign->value = visitExpression(ctx->expression());
    return std::unique_ptr<ast::Stmt>(std::move(assign));
}

std::unique_ptr<ast::Expr> ASTBuilder::visitExpression(
    LanguageParser::ExpressionContext* ctx) {
    if (ctx->assignment()) {
        return visitAssignment(ctx->assignment());
    }
    return nullptr;
}

std::unique_ptr<ast::Expr> ASTBuilder::visitAssignment(
    LanguageParser::AssignmentContext* ctx) {
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

std::unique_ptr<ast::Expr> ASTBuilder::visitOrExpr(
    LanguageParser::OrExprContext* ctx) {
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

std::unique_ptr<ast::Expr> ASTBuilder::visitAndExpr(
    LanguageParser::AndExprContext* ctx) {
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

std::unique_ptr<ast::Expr> ASTBuilder::visitEquality(
    LanguageParser::EqualityContext* ctx) {
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

std::unique_ptr<ast::Expr> ASTBuilder::visitRelational(
    LanguageParser::RelationalContext* ctx) {
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

std::unique_ptr<ast::Expr> ASTBuilder::visitAddExpr(
    LanguageParser::AddExprContext* ctx) {
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

std::unique_ptr<ast::Expr> ASTBuilder::visitMulExpr(
    LanguageParser::MulExprContext* ctx) {
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

std::unique_ptr<ast::Expr> ASTBuilder::visitUnary(
    LanguageParser::UnaryContext* ctx) {
    if (auto unary = ctx->unary()) {
        auto unaryExpr = make<ast::UnaryExpr>();
        unaryExpr->op = ctx->children.front()->getText();
        unaryExpr->expr = visitUnary(unary);
        return std::unique_ptr<ast::Expr>(std::move(unaryExpr));
    }
    return visitPostfixExpr(ctx->postfixExpr());
}

std::unique_ptr<ast::Expr> ASTBuilder::visitPostfixExpr(
    LanguageParser::PostfixExprContext* ctx) {
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
                auto idToken = dynamic_cast<antlr4::tree::TerminalNode*>(
                    ctx->children[idx + 1]);
                member->member = idToken ? idToken->getText() : "";
                idx++;  // Skip the identifier
            }
            base = std::unique_ptr<ast::Expr>(std::move(member));
        }
        idx++;
    }

    return base;
}

std::unique_ptr<ast::Expr> ASTBuilder::visitPrimary(
    LanguageParser::PrimaryContext* ctx) {
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
            strExpr->value = text.substr(1, text.size() - 2);  // Remove quotes
            return std::unique_ptr<ast::Expr>(std::move(strExpr));
        }
    }

    if (ctx->expression()) {
        return visitExpression(ctx->expression());
    }

    return nullptr;
}

std::unique_ptr<ast::Type> ASTBuilder::buildType(
    LanguageParser::TypeContext* ctx) {
    debug("buildType: Starting type building");
    debug("buildType: Type text = '" + ctx->getText() + "'");
    
    auto type = make<ast::Type>();

    if (auto primitive = ctx->primitiveType()) {
        type->name = primitive->getText();
        debug("buildType: Primitive type = " + type->name);
        return type;
    }

    if (auto id = ctx->IDENTIFIER()) {
        type->name = id->getText();
        debug("buildType: Identifier type = " + type->name);

        if (auto typeParams = ctx->typeParams()) {
            debug("buildType: Found type parameters with " + std::to_string(typeParams->type().size()) + " args");
            for (size_t i = 0; i < typeParams->type().size(); ++i) {
                auto param = typeParams->type(i);
                debug("buildType: Building type parameter " + std::to_string(i));
                auto paramType = buildType(param);
                if (paramType) {
                    type->args.push_back(std::move(paramType));
                } else {
                    debugError("buildType: Failed to build type parameter " + std::to_string(i));
                    return nullptr;
                }
            }
        } else {
            debug("buildType: No type parameters found");
        }
        return type;
    }

    if (ctx->QUESTION()) {
        debug("buildType: Optional type detected");
        // Optional type: ?Type becomes Option<Type>
        auto optionType = make<ast::Type>();
        optionType->name = "Option";
        auto innerType = buildType(ctx->type(0));
        if (innerType) {
            optionType->args.push_back(std::move(innerType));
        } else {
            debugError("buildType: Failed to build inner type for optional");
            return nullptr;
        }
        return optionType;
    }

    if (ctx->PIPE()) {
        debug("buildType: Union type detected");
        // Union type: Type1 | Type2 becomes Union<Type1, Type2>
        auto unionType = make<ast::Type>();
        unionType->name = "Union";
        auto type1 = buildType(ctx->type(0));
        auto type2 = buildType(ctx->type(1));
        if (type1 && type2) {
            unionType->args.push_back(std::move(type1));
            unionType->args.push_back(std::move(type2));
        } else {
            debugError("buildType: Failed to build union type components");
            return nullptr;
        }
        return unionType;
    }

    // Fallback
    debug("buildType: Using fallback - type text = '" + ctx->getText() + "'");
    type->name = ctx->getText();
    return type;
}
