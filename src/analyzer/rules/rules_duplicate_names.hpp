#pragma once
#include "../analyzer.hpp"
#include <unordered_map>
#include <string>

namespace analyzer {

class DuplicateNamesRule : public AnalysisRule {
public:
    void analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) override;

private:
    void checkDuplicateFunctions(const ast::Program& program, std::vector<Issue>& issues);
    void checkDuplicateVariables(const ast::FunctionDecl& func, std::vector<Issue>& issues);
};

} // namespace analyzer
