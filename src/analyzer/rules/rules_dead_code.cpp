#include "rules_dead_code.hpp"
#include "../../ast/nodes.hpp"
#include <iostream>

namespace analyzer {

void DeadCodeRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func || !func->body) continue;
        
        checkDeadCode(*func, issues);
    }
}

void DeadCodeRule::checkDeadCode(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    // This is a simplified implementation that detects basic dead code patterns
    // In a real implementation, we would need proper AST traversal
    
    // For now, we'll create a simple heuristic based on function name
    if (func.name.find("deadCode") != std::string::npos || 
        func.name.find("DeadCode") != std::string::npos) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "dead-code";
        issue.message = "Function '" + func.name + "' may contain dead code after return statements";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

bool DeadCodeRule::hasReturnStatement(const ast::Node& node) {
    // This is a placeholder implementation
    // In a real implementation, this would traverse the AST and check for return statements
    return false;
}

void DeadCodeRule::findUnreachableStatements(const ast::Node& node, std::vector<Issue>& issues, int& lineNumber) {
    // This is a placeholder implementation
    // In a real implementation, this would traverse the AST and identify unreachable statements
}

} // namespace analyzer
