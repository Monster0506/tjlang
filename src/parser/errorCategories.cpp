#include "errorCategories.hpp"
#include <algorithm>

namespace parser {

std::string ErrorCategories::getSyntaxErrorCategory(const std::string& msg) {
    if (contains(msg, "token recognition error")) {
        return "Lexical Error";
    }
    if (contains(msg, "missing")) {
        return "Syntax Error - Missing Token";
    }
    if (contains(msg, "mismatched input")) {
        return "Syntax Error - Unexpected Token";
    }
    if (contains(msg, "no viable alternative")) {
        return "Syntax Error - Invalid Grammar";
    }
    return "Parse Error";
}

std::string ErrorCategories::getAnalysisErrorCategory(const std::string& rule) {
    if (rule == "unused-parameter" || rule == "unused-variable") {
        return "Code Quality - Unused Code";
    }
    if (rule == "duplicate-function" || rule == "duplicate-parameter") {
        return "Code Quality - Naming Issues";
    }
    if (rule == "high-complexity" || rule == "deep-nesting" || rule == "long-function-name") {
        return "Code Quality - Complexity";
    }
    if (rule == "dead-code" || rule == "unreachable-code") {
        return "Code Quality - Dead Code";
    }
    if (rule == "constant-condition") {
        return "Code Quality - Logic Issues";
    }
    if (rule == "type-mismatch" || rule == "type-safety") {
        return "Type Safety";
    }
    return "Static Analysis";
}

std::string ErrorCategories::getSeverityDescription(const std::string& severity) {
    if (severity == "ERROR") {
        return "This is a critical issue that must be fixed";
    }
    if (severity == "WARNING") {
        return "This is a potential issue that should be addressed";
    }
    if (severity == "INFO") {
        return "This is an informational message about code style or best practices";
    }
    return "Unknown severity level";
}

bool ErrorCategories::contains(const std::string& str, const std::string& substr) {
    return str.find(substr) != std::string::npos;
}

} // namespace parser
