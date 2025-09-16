#pragma once
#include <string>
#include <unordered_set>

#include "../analyzer.hpp"

namespace analyzer {

class UnusedVariablesRule : public AnalysisRule {
   public:
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;
    std::string getName() const override { return "unused-variables"; }

   private:
    void collectUsedIdentifiers(const ast::Node& node,
                                std::unordered_set<std::string>& used);
    void checkUnusedVariables(const ast::FunctionDecl& func,
                              std::vector<Issue>& issues);
};

}  // namespace analyzer
