#include "rules_duplicate_names.hpp"
#include "../../ast/nodes.hpp"
#include <iostream>

namespace analyzer {

void DuplicateNamesRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    checkDuplicateFunctions(program, issues);
    
    // Check for duplicate variable names within each function
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func) continue;
        
        checkDuplicateVariables(*func, issues);
    }
}

void DuplicateNamesRule::checkDuplicateFunctions(const ast::Program& program, std::vector<Issue>& issues) {
    std::unordered_map<std::string, int> functionNames;
    
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func) continue;
        
        auto it = functionNames.find(func->name);
        if (it != functionNames.end()) {
            Issue issue;
            issue.severity = Issue::Severity::Error;
            issue.rule = "duplicate-function";
            issue.message = "Duplicate function name: '" + func->name + "'";
            issue.location = "Function '" + func->name + "' (declared multiple times)";
            issues.push_back(issue);
        } else {
            functionNames[func->name] = 1;
        }
    }
}

void DuplicateNamesRule::checkDuplicateVariables(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    // This is a placeholder implementation
    // In a real implementation, we would:
    // 1. Parse the function body to find variable declarations
    // 2. Track variable names within the function scope
    // 3. Report duplicates
    
    // For now, we'll check parameter names for duplicates
    std::unordered_map<std::string, int> paramNames;
    for (const auto& param : func.params) {
        auto it = paramNames.find(param.name);
        if (it != paramNames.end()) {
            Issue issue;
            issue.severity = Issue::Severity::Error;
            issue.rule = "duplicate-parameter";
            issue.message = "Duplicate parameter name: '" + param.name + "'";
            issue.location = "Function '" + func.name + "' parameter '" + param.name + "'";
            issues.push_back(issue);
        } else {
            paramNames[param.name] = 1;
        }
    }
}

} // namespace analyzer
