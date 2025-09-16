#pragma once
#include "../analyzer.hpp"
#include <unordered_set>
#include <string>

namespace analyzer {

class UnusedVariablesRule : public AnalysisRule {
public:
    void analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) override;

private:
    void collectUsedIdentifiers(const ast::Node& node, std::unordered_set<std::string>& used);
    void checkUnusedVariables(const ast::FunctionDecl& func, std::vector<Issue>& issues);
};

} // namespace analyzer
