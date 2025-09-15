#pragma once

#include <algorithm>
#include <iomanip>
#include <sstream>
#include <string>
#include <vector>

#include "antlr4-runtime.h"

// Enhanced error listener with detailed error reporting
class ErrorListener : public antlr4::BaseErrorListener {
   public:
    struct SyntaxError {
        int line;
        int charPosition;
        std::string msg;
        std::string offendingText;
        std::string sourceLine;
    };

    std::vector<SyntaxError> errors;
    std::string sourceCode;
    std::vector<std::string> sourceLines;

    void setSourceCode(const std::string& code) {
        sourceCode = code;
        sourceLines.clear();
        std::istringstream stream(code);
        std::string line;
        while (std::getline(stream, line)) {
            sourceLines.push_back(line);
        }
    }

    void syntaxError(antlr4::Recognizer* recognizer,
                     antlr4::Token* offendingSymbol, size_t line,
                     size_t charPositionInLine, const std::string& msg,
                     std::exception_ptr e) override {
        std::string offendingText = "";
        if (offendingSymbol && offendingSymbol->getText() != "<EOF>") {
            offendingText = offendingSymbol->getText();
        }

        std::string sourceLine = "";
        if (line > 0 && line <= sourceLines.size()) {
            sourceLine = sourceLines[line - 1];
        }

        errors.push_back({static_cast<int>(line),
                          static_cast<int>(charPositionInLine), msg,
                          offendingText, sourceLine});
    }

    bool hasErrors() const { return !errors.empty(); }

    void printErrors(const std::string& filename) {
        for (const auto& error : errors) {
            printError(filename, error);
        }
    }

   private:
    void printError(const std::string& filename, const SyntaxError& error) {
        // Enhanced ANSI color codes
        const std::string RED = "\033[31m";
        const std::string BRIGHT_RED = "\033[91m";
        const std::string YELLOW = "\033[33m";
        const std::string BRIGHT_YELLOW = "\033[93m";
        const std::string BLUE = "\033[34m";
        const std::string BRIGHT_BLUE = "\033[94m";
        const std::string CYAN = "\033[36m";
        const std::string BRIGHT_CYAN = "\033[96m";
        const std::string GREEN = "\033[32m";
        const std::string MAGENTA = "\033[35m";
        const std::string BOLD = "\033[1m";
        const std::string DIM = "\033[2m";
        const std::string UNDERLINE = "\033[4m";
        const std::string RESET = "\033[0m";

        // Enhanced error header with box drawing
        std::cerr << "\n";
        std::cerr << BRIGHT_RED << BOLD << "+-- " << "ERROR" << RESET << " in "
                  << BRIGHT_CYAN << filename << RESET << " at line "
                  << BRIGHT_YELLOW << error.line << RESET << ", column "
                  << BRIGHT_YELLOW << (error.charPosition + 1) << RESET << "\n";

        // Error message with better formatting
        std::cerr << BRIGHT_RED << "|" << RESET << " " << BRIGHT_RED
                  << error.msg << RESET << "\n";

        if (!error.sourceLine.empty()) {
            std::cerr << BRIGHT_RED << "|" << RESET << "\n";

            // Show context lines (previous and next lines if available)
            int contextStart = std::max(1, error.line - 2);
            int contextEnd = std::min((int)sourceLines.size(), error.line + 2);

            for (int lineNum = contextStart; lineNum <= contextEnd; lineNum++) {
                std::string lineContent = sourceLines[lineNum - 1];

                if (lineNum == error.line) {
                    // Highlight the error line
                    std::cerr << BRIGHT_RED << "|" << RESET << " "
                              << BRIGHT_BLUE << std::setw(3) << std::right
                              << lineNum << " | " << RESET << lineContent
                              << "\n";

                    // Create enhanced visual indicator
                    std::string indicator =
                        BRIGHT_RED + "|" + RESET + "      " + RESET;
                    int spaces = error.charPosition;
                    int tabs = 0;

                    // Count tabs before the error position
                    for (int i = 0;
                         i < std::min(spaces, (int)lineContent.length()); i++) {
                        if (lineContent[i] == '\t') {
                            tabs++;
                            spaces -= 7;  // Adjust for tab width
                        }
                    }

                    // Add spaces and tabs
                    for (int i = 0; i < tabs; i++) {
                        indicator += "\t";
                    }
                    for (int i = 0; i < spaces; i++) {
                        indicator += " ";
                    }

                    // Add enhanced error indicator
                    if (!error.offendingText.empty() &&
                        error.offendingText != "<EOF>") {
                        indicator += BRIGHT_RED + BOLD + " ^ ";
                        for (size_t i = 1; i < error.offendingText.length();
                             i++) {
                            indicator += "~";
                        }
                        indicator += RESET;
                    } else {
                        indicator += BRIGHT_RED + BOLD + " ^ " + RESET;
                    }

                    std::cerr << indicator << "\n";
                } else {
                    // Show context lines
                    std::cerr << BRIGHT_RED << "|" << RESET << " " << DIM
                              << std::setw(3) << std::right << lineNum << " | "
                              << RESET << DIM << lineContent << RESET << "\n";
                }
            }
        }

        // Enhanced error details
        std::cerr << BRIGHT_RED << "+--" << RESET << " " << YELLOW
                  << "Details:" << RESET << "\n";

        if (!error.offendingText.empty() && error.offendingText != "<EOF>") {
            std::cerr << "   " << MAGENTA << "* " << RESET
                      << "Offending token: " << BRIGHT_CYAN << "'"
                      << error.offendingText << "'" << RESET << "\n";
        }

        // Add helpful suggestions based on error type
        std::string suggestion = getSuggestion(error.msg, error.offendingText);
        if (!suggestion.empty()) {
            std::cerr << "   " << GREEN << "* " << RESET
                      << "Suggestion: " << suggestion << "\n";
        }

        // Show error category
        std::string category = getErrorCategory(error.msg);
        if (!category.empty()) {
            std::cerr << "   " << BLUE << "* " << RESET
                      << "Category: " << BRIGHT_BLUE << category << RESET
                      << "\n";
        }

        std::cerr << "\n";
    }

    std::string getSuggestion(const std::string& msg,
                              const std::string& offendingText) {
        if (msg.find("missing ')'") != std::string::npos) {
            return "Add a closing parenthesis ')' to match the opening "
                   "parenthesis";
        }
        if (msg.find("missing '{'") != std::string::npos) {
            return "Add an opening brace '{'";
        }
        if (msg.find("missing '}'") != std::string::npos) {
            return "Add a closing brace '}'";
        }
        if (msg.find("missing ':'") != std::string::npos) {
            return "Add a colon ':'";
        }
        if (msg.find("missing '->'") != std::string::npos) {
            return "Add return type arrow '->' before the return type";
        }
        if (msg.find("missing 'def'") != std::string::npos) {
            return "Add 'def' keyword to declare a function";
        }
        if (msg.find("token recognition error") != std::string::npos) {
            return "Remove or replace the invalid character '" + offendingText +
                   "'";
        }
        if (msg.find("mismatched input") != std::string::npos) {
            return "Check syntax around this position - expected different "
                   "token";
        }
        return "";
    }

    std::string getErrorCategory(const std::string& msg) {
        if (msg.find("token recognition error") != std::string::npos) {
            return "Lexical Error";
        }
        if (msg.find("missing") != std::string::npos) {
            return "Syntax Error - Missing Token";
        }
        if (msg.find("mismatched input") != std::string::npos) {
            return "Syntax Error - Unexpected Token";
        }
        if (msg.find("no viable alternative") != std::string::npos) {
            return "Syntax Error - Invalid Grammar";
        }
        return "Parse Error";
    }
};
