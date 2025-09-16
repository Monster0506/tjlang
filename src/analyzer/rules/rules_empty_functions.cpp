#include "rules_empty_functions.hpp"

#include <iostream>

namespace analyzer {

void EmptyFunctionsRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func) continue;

        checkFunctionForEmptyness(*func, issues);
    }
}

void EmptyFunctionsRule::checkFunctionForEmptyness(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    if (!func.body) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "empty-function";
        issue.message = "Function '" + func.name + "' has no body";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
        return;
    }

    if (isEmptyBlock(*func.body)) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "empty-function";
        issue.message = "Function '" + func.name + "' has an empty body";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

bool EmptyFunctionsRule::isEmptyBlock(const ast::Block& block) {
    return block.stmts.empty();
}

}  // namespace analyzer
