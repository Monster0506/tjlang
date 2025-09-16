#include "unifiedErrorListener.hpp"
#include <sstream>

UnifiedErrorListener::UnifiedErrorListener() = default;

void UnifiedErrorListener::setSourceCode(const std::string& code) {
    sourceCode = code;
    formatter.setSourceCode(code);
}

void UnifiedErrorListener::syntaxError(antlr4::Recognizer* recognizer,
                                      antlr4::Token* offendingSymbol, size_t line,
                                      size_t charPositionInLine, const std::string& msg,
                                      std::exception_ptr e) {
    std::string offendingText = "";
    if (offendingSymbol && offendingSymbol->getText() != "<EOF>") {
        offendingText = offendingSymbol->getText();
    }

    std::string sourceLine = "";
    if (line > 0) {
        std::istringstream stream(sourceCode);
        std::string currentLine;
        for (size_t i = 0; i < line && std::getline(stream, currentLine); ++i) {
            if (i == line - 1) {
                sourceLine = currentLine;
                break;
            }
        }
    }

    syntaxErrors.push_back({static_cast<int>(line),
                           static_cast<int>(charPositionInLine), msg,
                           offendingText, sourceLine});
}

void UnifiedErrorListener::addAnalysisIssue(const analyzer::Issue& issue, int line, int charPosition) {
    std::string sourceLine = "";
    if (line > 0) {
        std::istringstream stream(sourceCode);
        std::string currentLine;
        for (int i = 0; i < line && std::getline(stream, currentLine); ++i) {
            if (i == line - 1) {
                sourceLine = currentLine;
                break;
            }
        }
    }

    analysisIssues.push_back({issue, line, charPosition, sourceLine});
}

void UnifiedErrorListener::printAllErrors(const std::string& filename) {
    // Print syntax errors first
    for (const auto& error : syntaxErrors) {
        formatter.printSyntaxError(filename, error);
    }
    
    // Print analysis issues
    for (const auto& issue : analysisIssues) {
        formatter.printAnalysisIssue(filename, issue);
    }
}

void UnifiedErrorListener::printSyntaxErrors(const std::string& filename) {
    for (const auto& error : syntaxErrors) {
        formatter.printSyntaxError(filename, error);
    }
}

void UnifiedErrorListener::printAnalysisIssues(const std::string& filename) {
    for (const auto& issue : analysisIssues) {
        formatter.printAnalysisIssue(filename, issue);
    }
}
