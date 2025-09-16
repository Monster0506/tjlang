#include "rules_magic_numbers.hpp"

#include <iostream>

namespace analyzer {

void MagicNumbersRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func || !func->body) continue;

        checkBlockForMagicNumbers(*func->body, issues, "Function '" + func->name + "'");
    }
}

void MagicNumbersRule::checkExpressionForMagicNumbers(const ast::Expr* expr, std::vector<Issue>& issues, const std::string& context) {
    if (!expr) return;

    if (auto intLit = dynamic_cast<const ast::IntLiteral*>(expr)) {
        if (isMagicNumber(*intLit)) {
            Issue issue;
            issue.severity = Issue::Severity::Info;
            issue.rule = "magic-number";
            issue.message = "Magic number " + std::to_string(intLit->value) + " should be a named constant";
            issue.location = context;
            issues.push_back(issue);
        }
        return;
    }

    if (auto floatLit = dynamic_cast<const ast::FloatLiteral*>(expr)) {
        if (isMagicNumber(*floatLit)) {
            Issue issue;
            issue.severity = Issue::Severity::Info;
            issue.rule = "magic-number";
            issue.message = "Magic number " + std::to_string(floatLit->value) + " should be a named constant";
            issue.location = context;
            issues.push_back(issue);
        }
        return;
    }

    // Check binary expressions
    if (auto bin = dynamic_cast<const ast::BinaryExpr*>(expr)) {
        checkExpressionForMagicNumbers(bin->lhs.get(), issues, context);
        checkExpressionForMagicNumbers(bin->rhs.get(), issues, context);
        return;
    }

    // Check unary expressions
    if (auto un = dynamic_cast<const ast::UnaryExpr*>(expr)) {
        checkExpressionForMagicNumbers(un->expr.get(), issues, context);
        return;
    }

    // Check call expressions
    if (auto call = dynamic_cast<const ast::CallExpr*>(expr)) {
        checkExpressionForMagicNumbers(call->callee.get(), issues, context);
        for (auto& arg : call->args) {
            checkExpressionForMagicNumbers(arg.get(), issues, context);
        }
        return;
    }

    // Check member expressions
    if (auto mem = dynamic_cast<const ast::MemberExpr*>(expr)) {
        checkExpressionForMagicNumbers(mem->object.get(), issues, context);
        return;
    }

    // Check index expressions
    if (auto idx = dynamic_cast<const ast::IndexExpr*>(expr)) {
        checkExpressionForMagicNumbers(idx->object.get(), issues, context);
        checkExpressionForMagicNumbers(idx->index.get(), issues, context);
        return;
    }

    // Check collection literals
    if (auto vec = dynamic_cast<const ast::VecLiteral*>(expr)) {
        for (auto& elem : vec->elements) {
            checkExpressionForMagicNumbers(elem.get(), issues, context);
        }
        return;
    }

    if (auto set = dynamic_cast<const ast::SetLiteral*>(expr)) {
        for (auto& elem : set->elements) {
            checkExpressionForMagicNumbers(elem.get(), issues, context);
        }
        return;
    }

    if (auto map = dynamic_cast<const ast::MapLiteral*>(expr)) {
        for (auto& kv : map->entries) {
            checkExpressionForMagicNumbers(kv.first.get(), issues, context);
            checkExpressionForMagicNumbers(kv.second.get(), issues, context);
        }
        return;
    }

    if (auto tup = dynamic_cast<const ast::TupleLiteral*>(expr)) {
        for (auto& elem : tup->elements) {
            checkExpressionForMagicNumbers(elem.get(), issues, context);
        }
        return;
    }
}

void MagicNumbersRule::checkStatementForMagicNumbers(const ast::Stmt* stmt, std::vector<Issue>& issues, const std::string& context) {
    if (!stmt) return;

    if (auto ret = dynamic_cast<const ast::ReturnStmt*>(stmt)) {
        checkExpressionForMagicNumbers(ret->value.get(), issues, context);
        return;
    }

    if (auto asg = dynamic_cast<const ast::AssignStmt*>(stmt)) {
        checkExpressionForMagicNumbers(asg->value.get(), issues, context);
        return;
    }

    if (auto blk = dynamic_cast<const ast::Block*>(stmt)) {
        checkBlockForMagicNumbers(*blk, issues, context);
        return;
    }

    if (auto iff = dynamic_cast<const ast::IfStmt*>(stmt)) {
        checkExpressionForMagicNumbers(iff->condition.get(), issues, context);
        checkBlockForMagicNumbers(*iff->thenBlock, issues, context);
        for (auto& p : iff->elifBranches) {
            checkExpressionForMagicNumbers(p.first.get(), issues, context);
            checkBlockForMagicNumbers(*p.second, issues, context);
        }
        if (iff->elseBlock) {
            checkBlockForMagicNumbers(*iff->elseBlock, issues, context);
        }
        return;
    }

    if (auto wh = dynamic_cast<const ast::WhileStmt*>(stmt)) {
        checkExpressionForMagicNumbers(wh->condition.get(), issues, context);
        checkBlockForMagicNumbers(*wh->body, issues, context);
        return;
    }

    if (auto fr = dynamic_cast<const ast::ForStmt*>(stmt)) {
        checkExpressionForMagicNumbers(fr->iterable.get(), issues, context);
        checkBlockForMagicNumbers(*fr->body, issues, context);
        return;
    }

    if (auto mt = dynamic_cast<const ast::MatchStmt*>(stmt)) {
        checkExpressionForMagicNumbers(mt->expression.get(), issues, context);
        for (auto& g : mt->guards) {
            checkExpressionForMagicNumbers(g.get(), issues, context);
        }
        for (auto& a : mt->arms) {
            checkBlockForMagicNumbers(*a.second, issues, context);
        }
        return;
    }
}

void MagicNumbersRule::checkBlockForMagicNumbers(const ast::Block& block, std::vector<Issue>& issues, const std::string& context) {
    for (const auto& stmt : block.stmts) {
        checkStatementForMagicNumbers(stmt.get(), issues, context);
    }
}

bool MagicNumbersRule::isMagicNumber(const ast::IntLiteral& literal) {
    // Consider numbers other than 0, 1, 2 as magic numbers
    // 0 and 1 are often used for initialization and indexing
    // 2 is common for binary operations
    return literal.value != 0 && literal.value != 1 && literal.value != 2;
}

bool MagicNumbersRule::isMagicNumber(const ast::FloatLiteral& literal) {
    // Consider non-zero floats as magic numbers
    return literal.value != 0.0;
}

}  // namespace analyzer
