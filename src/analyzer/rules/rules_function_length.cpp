#include "rules_function_length.hpp"

#include <iostream>

namespace analyzer {

void FunctionLengthRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func) continue;

        checkFunctionLength(*func, issues);
    }
}

void FunctionLengthRule::checkFunctionLength(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    if (!func.body) return;

    int statementCount = countStatements(*func.body);
    
    if (statementCount > 50) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "long-function";
        issue.message = "Function '" + func.name + "' is very long (" + std::to_string(statementCount) + " statements). Consider breaking it into smaller functions";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    } else if (statementCount > 30) {
        Issue issue;
        issue.severity = Issue::Severity::Info;
        issue.rule = "long-function";
        issue.message = "Function '" + func.name + "' is quite long (" + std::to_string(statementCount) + " statements). Consider if it could be refactored";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

int FunctionLengthRule::countStatements(const ast::Block& block) {
    int count = 0;
    for (const auto& stmt : block.stmts) {
        count += countStatements(stmt.get());
    }
    return count;
}

int FunctionLengthRule::countStatements(const ast::Stmt* stmt) {
    if (!stmt) return 0;

    // Each statement counts as 1, plus any nested statements
    int count = 1;

    if (auto blk = dynamic_cast<const ast::Block*>(stmt)) {
        count += countStatements(*blk) - 1; // -1 because we already counted the block itself
    } else if (auto iff = dynamic_cast<const ast::IfStmt*>(stmt)) {
        count += countStatements(*iff->thenBlock);
        for (auto& p : iff->elifBranches) {
            count += countStatements(*p.second);
        }
        if (iff->elseBlock) {
            count += countStatements(*iff->elseBlock);
        }
    } else if (auto wh = dynamic_cast<const ast::WhileStmt*>(stmt)) {
        count += countStatements(*wh->body);
    } else if (auto fr = dynamic_cast<const ast::ForStmt*>(stmt)) {
        count += countStatements(*fr->body);
    } else if (auto mt = dynamic_cast<const ast::MatchStmt*>(stmt)) {
        for (auto& a : mt->arms) {
            count += countStatements(*a.second);
        }
    }

    return count;
}

}  // namespace analyzer
