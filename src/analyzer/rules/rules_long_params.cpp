#include "rules_long_params.hpp"

#include <iostream>

namespace analyzer {

void LongParamsRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func) continue;

        checkFunctionParams(*func, issues);
    }
}

void LongParamsRule::checkFunctionParams(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    int paramCount = func.params.size();
    
    if (paramCount > 7) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "long-parameter-list";
        issue.message = "Function '" + func.name + "' has too many parameters (" + std::to_string(paramCount) + "). Consider using a struct or object to group related parameters";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    } else if (paramCount > 5) {
        Issue issue;
        issue.severity = Issue::Severity::Info;
        issue.rule = "long-parameter-list";
        issue.message = "Function '" + func.name + "' has many parameters (" + std::to_string(paramCount) + "). Consider if some could be grouped together";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

}  // namespace analyzer
