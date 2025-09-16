#pragma once

#include <string>
#include <vector>
#include <iomanip>
#include <sstream>

#include "antlr4-runtime.h"
#include "../analyzer/analyzer.hpp"

namespace parser {

struct SyntaxError {
    int line;
    int charPosition;
    std::string msg;
    std::string offendingText;
    std::string sourceLine;
};

struct AnalysisIssue {
    analyzer::Issue issue;
    int line;
    int charPosition;
    std::string sourceLine;
};

class ErrorFormatter {
public:
    ErrorFormatter();
    
    void setSourceCode(const std::string& code);
    void printSyntaxError(const std::string& filename, const SyntaxError& error);
    void printAnalysisIssue(const std::string& filename, const AnalysisIssue& issue);
    void printSourceContext(int line, int charPosition, const std::string& offendingText);

private:
    std::vector<std::string> sourceLines;
    
    // Color constants
    static const std::string RED;
    static const std::string BRIGHT_RED;
    static const std::string YELLOW;
    static const std::string BRIGHT_YELLOW;
    static const std::string BLUE;
    static const std::string BRIGHT_BLUE;
    static const std::string CYAN;
    static const std::string BRIGHT_CYAN;
    static const std::string GREEN;
    static const std::string MAGENTA;
    static const std::string BOLD;
    static const std::string DIM;
    static const std::string RESET;
    
    void printErrorHeader(const std::string& filename, const std::string& errorType, 
                         int line, int charPosition, const std::string& color);
    void printErrorDetails(const std::string& message, const std::string& color);
    void printSuggestion(const std::string& suggestion);
    void printRuleInfo(const std::string& rule, const std::string& location);
};

} // namespace parser
