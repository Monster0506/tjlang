#include "rules_unused_variables.hpp"
#include "../../ast/nodes.hpp"
#include <iostream>

namespace analyzer {

void UnusedVariablesRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    for (const auto& unit : program.units) {
        auto func = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!func || !func->body) continue;
        
        checkUnusedVariables(*func, issues);
    }
}

void UnusedVariablesRule::collectUsedIdentifiers(const ast::Node& node, std::unordered_set<std::string>& used) {
    // This is a placeholder implementation
    // In a real implementation, this would traverse the AST and collect all identifier references
    // For now, we'll just check if it's an Identifier node
    if (auto ident = dynamic_cast<const ast::Identifier*>(&node)) {
        used.insert(ident->name);
    }
    
    // TODO: Add proper AST traversal for all node types
    // This would involve checking children of Block, Expression, etc.
}

void UnusedVariablesRule::checkUnusedVariables(const ast::FunctionDecl& func, std::vector<Issue>& issues) {
    // This is a simplified implementation
    // In a real implementation, we would:
    // 1. Parse the function body to find variable declarations
    // 2. Collect all identifier references in the function
    // 3. Check which declared variables are never referenced
    
    // For now, this is a placeholder that doesn't actually detect unused variables
    // because we need proper AST traversal and variable declaration parsing
}

} // namespace analyzer
