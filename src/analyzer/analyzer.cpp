#include <iostream>

#include "analyzer.hpp"
#include "rules/rules_duplicate_names.hpp"
#include "rules/rules_unreachable_code.hpp"
#include "rules/rules_unused_params.hpp"
#include "rules/rules_unused_variables.hpp"
#include "rules/rules_dead_code.hpp"
#include "rules/rules_constant_conditions.hpp"
#include "rules/rules_type_safety.hpp"
#include "rules/rules_complexity.hpp"
#include "rules/rules_empty_functions.hpp"
#include "rules/rules_function_length.hpp"
#include "rules/rules_long_params.hpp"
#include "rules/rules_magic_numbers.hpp"
#include "rules/rules_naming_conventions.hpp"

namespace analyzer {

StaticAnalyzer::StaticAnalyzer() {
    // Register all analysis rules
    addRule(std::make_unique<UnusedParamsRule>());
    addRule(std::make_unique<UnusedVariablesRule>());
    addRule(std::make_unique<DuplicateNamesRule>());
    addRule(std::make_unique<UnreachableCodeRule>());
    addRule(std::make_unique<DeadCodeRule>());
    addRule(std::make_unique<ConstantConditionsRule>());
    addRule(std::make_unique<TypeSafetyRule>());
    addRule(std::make_unique<ComplexityRule>());
    
    // Code quality rules
    addRule(std::make_unique<EmptyFunctionsRule>());
    addRule(std::make_unique<FunctionLengthRule>());
    addRule(std::make_unique<LongParamsRule>());
    addRule(std::make_unique<MagicNumbersRule>());
    addRule(std::make_unique<NamingConventionsRule>());
}

void StaticAnalyzer::addRule(std::unique_ptr<AnalysisRule> rule) {
    rules.push_back(std::move(rule));
}

std::vector<Issue> StaticAnalyzer::analyze(const ast::Program& program) {
    std::vector<Issue> issues;
    if (debug_) {
        std::cout << "[ANALYZE] Starting analysis with " << rules.size()
                  << " rule(s)\n";
        std::cout << "[ANALYZE] Program units: " << program.units.size()
                  << "\n";
    }
    for (size_t i = 0; i < rules.size(); ++i) {
        if (debug_) {
            std::cout << "[ANALYZE] Running rule " << i << "...\n";
        }
        rules[i]->analyzeProgram(program, issues);
    }
    if (debug_) {
        std::cout << "[ANALYZE] Completed. Issues: " << issues.size() << "\n";
    }
    return issues;
}

void StaticAnalyzer::printIssues(const std::vector<Issue>& issues,
                                 std::ostream& os) {
    if (issues.empty()) {
        os << "No issues found.\n";
        return;
    }
    for (const auto& i : issues) {
        const char* sev =
            i.severity == Issue::Severity::Error
                ? "error"
                : (i.severity == Issue::Severity::Warning ? "warning" : "info");
        os << sev << ": [" << i.rule << "] " << i.message;
        if (!i.location.empty()) os << " (" << i.location << ")";
        os << "\n";
    }
}

}  // namespace analyzer
