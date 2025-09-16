#pragma once

#include <memory>
#include <ostream>
#include <string>
#include <vector>

#include "../ast/nodes.hpp"

namespace analyzer {

struct Issue {
    enum class Severity { Info, Warning, Error };
    Severity severity = Severity::Warning;
    std::string rule;
    std::string message;
    std::string location;  // optional textual location
};

class AnalysisRule {
   public:
    virtual ~AnalysisRule() = default;
    virtual void analyzeProgram(const ast::Program& program,
                                std::vector<Issue>& issues) = 0;
    virtual std::string getName() const = 0;
};

class StaticAnalyzer {
   public:
    StaticAnalyzer();
    void setDebug(bool enabled) { debug_ = enabled; }
    void addRule(std::unique_ptr<AnalysisRule> rule);
    std::vector<Issue> analyze(const ast::Program& program);
    static void printIssues(const std::vector<Issue>& issues, std::ostream& os);

   private:
    std::vector<std::unique_ptr<AnalysisRule>> rules;
    bool debug_ = false;
};

}  // namespace analyzer
