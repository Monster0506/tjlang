#pragma once

#include "../analyzer.hpp"

namespace analyzer {

class NamingConventionsRule : public AnalysisRule {
   public:
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;
    std::string getName() const override { return "naming-conventions"; }

   private:
    void checkFunctionNaming(const ast::FunctionDecl& func,
                             std::vector<Issue>& issues);
    void checkParameterNaming(const ast::FunctionDecl& func,
                              std::vector<Issue>& issues);
    bool isValidFunctionName(const std::string& name);
    bool isValidParameterName(const std::string& name);
    bool isCamelCase(const std::string& name);
    bool isSnakeCase(const std::string& name);
    bool startsWithLower(const std::string& name);
};

}  // namespace analyzer
