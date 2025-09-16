#pragma once

#include "../analyzer.hpp"

namespace analyzer {

class LongParamsRule : public AnalysisRule {
   public:
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;
    std::string getName() const override { return "long-params"; }

   private:
    void checkFunctionParams(const ast::FunctionDecl& func,
                             std::vector<Issue>& issues);
};

}  // namespace analyzer
