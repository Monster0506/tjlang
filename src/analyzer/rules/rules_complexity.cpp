#include "rules_complexity.hpp"
#include "../../ast/nodes.hpp"
#include <iostream>

namespace analyzer {

void ComplexityRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func || !func->body) continue;
        
        checkComplexity(*func, issues);
    }
}

void ComplexityRule::checkComplexity(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    // Check function length (simplified - count lines)
    checkFunctionLength(func, issues);
    
    // Check cyclomatic complexity
    int complexity = calculateCyclomaticComplexity(*func.body);
    if (complexity > 10) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "high-complexity";
        issue.message = "Function '" + func.name + "' has high cyclomatic complexity (" + std::to_string(complexity) + ")";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
    
    // Check nesting level
    int nesting = countNestingLevel(*func.body);
    if (nesting > 4) {
        Issue issue;
        issue.severity = Issue::Severity::Warning;
        issue.rule = "deep-nesting";
        issue.message = "Function '" + func.name + "' has deep nesting level (" + std::to_string(nesting) + ")";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

int ComplexityRule::calculateCyclomaticComplexity(const ast::Node& node) {
    // This is a placeholder implementation
    // In a real implementation, this would count decision points (if, while, for, etc.)
    // For now, return a simple placeholder value
    return 1;
}

int ComplexityRule::countNestingLevel(const ast::Node& node) {
    // This is a placeholder implementation
    // In a real implementation, this would count nested blocks
    // For now, return a simple placeholder value
    return 1;
}

void ComplexityRule::checkFunctionLength(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    // This is a simplified check - in a real implementation we'd count actual lines
    // For now, we'll just check if the function has a very long name as a placeholder
    if (func.name.length() > 20) {
        Issue issue;
        issue.severity = Issue::Severity::Info;
        issue.rule = "long-function-name";
        issue.message = "Function name '" + func.name + "' is quite long";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

} // namespace analyzer
