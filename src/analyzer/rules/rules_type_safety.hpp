#pragma once
#include "../analyzer.hpp"
#include <vector>

namespace analyzer {

class TypeSafetyRule : public AnalysisRule {
public:
    void analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) override;

private:
    void checkTypeSafety(const ast::FunctionDecl& func, std::vector<Issue>& issues);
    void validateExpressionTypes(const ast::Node& node, std::vector<Issue>& issues);
    bool areTypesCompatible(const ast::Type& left, const ast::Type& right);
};

} // namespace analyzer
