#pragma once

#include "../analyzer.hpp"

namespace analyzer {

class UnusedParamsRule : public AnalysisRule {
   public:
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;
    std::string getName() const override { return "unused-params"; }
};

}  // namespace analyzer
