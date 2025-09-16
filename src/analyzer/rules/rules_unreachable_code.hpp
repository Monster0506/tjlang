#pragma once
#include "../analyzer.hpp"
#include <vector>

namespace analyzer {

class UnreachableCodeRule : public AnalysisRule {
public:
    void analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) override;

private:
    void checkUnreachableCode(const ast::FunctionDecl& func, std::vector<Issue>& issues);
    bool hasReturnStatement(const ast::Node& node);
};

} // namespace analyzer
