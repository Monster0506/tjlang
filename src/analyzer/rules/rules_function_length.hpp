#pragma once

#include "../analyzer.hpp"

namespace analyzer {

class FunctionLengthRule : public AnalysisRule {
public:
    void analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) override;

private:
    void checkFunctionLength(const ast::FunctionDecl& func, std::vector<Issue>& issues);
    int countStatements(const ast::Block& block);
    int countStatements(const ast::Stmt* stmt);
};

}  // namespace analyzer
