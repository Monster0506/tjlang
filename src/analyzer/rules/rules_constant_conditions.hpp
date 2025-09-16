#pragma once
#include "../analyzer.hpp"
#include <vector>

namespace analyzer {

class ConstantConditionsRule : public AnalysisRule {
public:
    void analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) override;

private:
    void checkConstantConditions(const ast::FunctionDecl& func, std::vector<Issue>& issues);
    bool isConstantCondition(const ast::Node& node);
    void findConstantIfs(const ast::Node& node, std::vector<Issue>& issues);
};

} // namespace analyzer
