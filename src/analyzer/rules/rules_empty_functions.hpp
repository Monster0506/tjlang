#pragma once

#include "../analyzer.hpp"

namespace analyzer {

class EmptyFunctionsRule : public AnalysisRule {
   public:
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;
    std::string getName() const override { return "empty-functions"; }

   private:
    void checkFunctionForEmptyness(const ast::FunctionDecl& func,
                                   std::vector<Issue>& issues);
    bool isEmptyBlock(const ast::Block& block);
};

}  // namespace analyzer
