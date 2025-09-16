#pragma once
#include "../analyzer.hpp"
#include <vector>

namespace analyzer {

class ComplexityRule : public AnalysisRule {
public:
    void analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) override;

private:
    void checkComplexity(const ast::FunctionDecl& func, std::vector<Issue>& issues);
    int calculateCyclomaticComplexity(const ast::Node& node);
    int countNestingLevel(const ast::Node& node);
    void checkFunctionLength(const ast::FunctionDecl& func, std::vector<Issue>& issues);
};

} // namespace analyzer
