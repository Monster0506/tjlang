#pragma once
#include <vector>

#include "../analyzer.hpp"

namespace analyzer {

class UnreachableCodeRule : public AnalysisRule {
   public:
    std::string getName() const override { return "unreachable-code"; }
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;

   private:
    void checkUnreachableCode(const ast::FunctionDecl& func,
                              std::vector<Issue>& issues);
    bool hasReturnStatement(const ast::Node& node);
};

}  // namespace analyzer
