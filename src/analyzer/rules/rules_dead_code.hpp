#pragma once
#include <vector>

#include "../analyzer.hpp"

namespace analyzer {

class DeadCodeRule : public AnalysisRule {
   public:
    std::string getName() const override { return "dead-code"; }
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;

   private:
    void checkDeadCode(const ast::FunctionDecl& func,
                       std::vector<Issue>& issues);
    bool hasReturnStatement(const ast::Node& node);
    void findUnreachableStatements(const ast::Node& node,
                                   std::vector<Issue>& issues, int& lineNumber);
};

}  // namespace analyzer
