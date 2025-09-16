#pragma once

#include <string>
#include <vector>

#include "antlr4-runtime.h"
#include "../analyzer/analyzer.hpp"
#include "errorFormatter.hpp"
#include "suggestionEngine.hpp"
#include "errorCategories.hpp"

// Unified error listener that handles both syntax errors and static analysis issues
class UnifiedErrorListener : public antlr4::BaseErrorListener {
public:
    using SyntaxError = parser::SyntaxError;
    using AnalysisIssue = parser::AnalysisIssue;

    UnifiedErrorListener();
    
    void setSourceCode(const std::string& code);
    
    void syntaxError(antlr4::Recognizer* recognizer,
                     antlr4::Token* offendingSymbol, size_t line,
                     size_t charPositionInLine, const std::string& msg,
                     std::exception_ptr e) override;

    void addAnalysisIssue(const analyzer::Issue& issue, int line = 0, int charPosition = 0);

    bool hasErrors() const { 
        return !syntaxErrors.empty() || !analysisIssues.empty(); 
    }

    bool hasSyntaxErrors() const { 
        return !syntaxErrors.empty(); 
    }

    bool hasAnalysisIssues() const { 
        return !analysisIssues.empty(); 
    }

    void printAllErrors(const std::string& filename);
    void printSyntaxErrors(const std::string& filename);
    void printAnalysisIssues(const std::string& filename);

private:
    std::vector<SyntaxError> syntaxErrors;
    std::vector<AnalysisIssue> analysisIssues;
    std::string sourceCode;
    
    parser::ErrorFormatter formatter;
    parser::SuggestionEngine suggestionEngine;
    parser::ErrorCategories categories;
};