#include "rules_type_safety.hpp"
#include "../../ast/nodes.hpp"
#include <iostream>

namespace analyzer {

void TypeSafetyRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func || !func->body) continue;
        
        checkTypeSafety(*func, issues);
    }
}

void TypeSafetyRule::checkTypeSafety(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    // This is a simplified implementation that detects basic type safety patterns
    // In a real implementation, we would need proper AST traversal and type system implementation
    
    // For now, we'll create a simple heuristic based on function name
    if (func.name.find("type") != std::string::npos || 
        func.name.find("Type") != std::string::npos) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "type-safety";
        issue.message = "Function '" + func.name + "' may have type safety issues";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

void TypeSafetyRule::validateExpressionTypes(const ast::Node& node, std::vector<Issue>& issues) {
    // This is a placeholder implementation
    // In a real implementation, this would traverse expressions and validate types
}

bool TypeSafetyRule::areTypesCompatible(const ast::Type& left, const ast::Type& right) {
    // This is a placeholder implementation
    // In a real implementation, this would check if two types are compatible
    return true; // Placeholder - always compatible for now
}

} // namespace analyzer
