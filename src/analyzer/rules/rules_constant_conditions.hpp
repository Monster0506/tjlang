#pragma once
#include <vector>

#include "../analyzer.hpp"

namespace analyzer {

class ConstantConditionsRule : public AnalysisRule {
   public:
    std::string getName() const override { return "constant-conditions"; }

    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;

   private:
    void checkConstantConditions(const ast::FunctionDecl& func,
                                 std::vector<Issue>& issues);
    bool isConstantCondition(const ast::Node& node);
    void findConstantIfs(const ast::Node& node, std::vector<Issue>& issues);
};

}  // namespace analyzer
