#include "rules_naming_conventions.hpp"

#include <iostream>
#include <cctype>

namespace analyzer {

void NamingConventionsRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func) continue;

        checkFunctionNaming(*func, issues);
        checkParameterNaming(*func, issues);
    }
}

void NamingConventionsRule::checkFunctionNaming(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    if (!isValidFunctionName(func.name)) {
        Issue issue;
        issue.severity = Issue::Severity::Info;
        issue.rule = "naming-convention";
        issue.message = "Function '" + func.name + "' should use camelCase naming (e.g., 'calculateTotal')";
        issue.location = "Function '" + func.name + "'";
        issues.push_back(issue);
    }
}

void NamingConventionsRule::checkParameterNaming(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    for (const auto& param : func.params) {
        if (!isValidParameterName(param.name)) {
            Issue issue;
            issue.severity = Issue::Severity::Info;
            issue.rule = "naming-convention";
            issue.message = "Parameter '" + param.name + "' should use camelCase naming (e.g., 'userName')";
            issue.location = "Function '" + func.name + "' parameter '" + param.name + "'";
            issues.push_back(issue);
        }
    }
}

bool NamingConventionsRule::isValidFunctionName(const std::string& name) {
    if (name.empty()) return false;
    
    // Functions should start with lowercase and use camelCase
    return startsWithLower(name) && isCamelCase(name);
}

bool NamingConventionsRule::isValidParameterName(const std::string& name) {
    if (name.empty()) return false;
    
    // Parameters should start with lowercase and use camelCase
    return startsWithLower(name) && isCamelCase(name);
}

bool NamingConventionsRule::isCamelCase(const std::string& name) {
    if (name.empty()) return false;
    
    // Check that it contains only letters and digits, with no underscores
    for (char c : name) {
        if (!std::isalnum(c)) {
            return false;
        }
    }
    
    // Check that it doesn't have consecutive uppercase letters (except at start)
    bool prevUpper = false;
    for (size_t i = 1; i < name.length(); ++i) {
        bool currentUpper = std::isupper(name[i]);
        if (prevUpper && currentUpper) {
            return false; // Consecutive uppercase letters
        }
        prevUpper = currentUpper;
    }
    
    return true;
}

bool NamingConventionsRule::isSnakeCase(const std::string& name) {
    if (name.empty()) return false;
    
    // Check that it contains only lowercase letters, digits, and underscores
    for (char c : name) {
        if (!std::islower(c) && !std::isdigit(c) && c != '_') {
            return false;
        }
    }
    
    return true;
}

bool NamingConventionsRule::startsWithLower(const std::string& name) {
    if (name.empty()) return false;
    return std::islower(name[0]);
}

}  // namespace analyzer
