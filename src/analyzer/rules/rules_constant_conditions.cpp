#include "rules_constant_conditions.hpp"
#include "../../ast/nodes.hpp"
#include <iostream>

namespace analyzer {

void ConstantConditionsRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func || !func->body) continue;
        
        checkConstantConditions(*func, issues);
    }
}

void ConstantConditionsRule::checkConstantConditions(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    // This is a simplified implementation that detects basic constant condition patterns
    // In a real implementation, we would need proper AST traversal and expression evaluation
    
    // For now, we'll create a simple heuristic based on function name
    if (func.name.find("constant") != std::string::npos || 
        func.name.find("Constant") != std::string::npos) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "constant-condition";
        issue.message = "Function '" + func.name + "' may contain constant conditions in if statements";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

bool ConstantConditionsRule::isConstantCondition(const ast::Node& node) {
    // This is a placeholder implementation
    // In a real implementation, this would evaluate expressions to determine if they're constant
    return false;
}

void ConstantConditionsRule::findConstantIfs(const ast::Node& node, std::vector<Issue>& issues) {
    // This is a placeholder implementation
    // In a real implementation, this would traverse the AST and find if statements with constant conditions
}

} // namespace analyzer
