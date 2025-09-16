#pragma once
#include <string>
#include <unordered_map>

#include "../analyzer.hpp"

namespace analyzer {

class DuplicateNamesRule : public AnalysisRule {
   public:
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;
    std::string getName() const override { return "duplicate-names"; }

   private:
    void checkDuplicateFunctions(const ast::Program& program,
                                 std::vector<Issue>& issues);
    void checkDuplicateVariables(const ast::FunctionDecl& func,
                                 std::vector<Issue>& issues);
};

}  // namespace analyzer
