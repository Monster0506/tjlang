#include "errorFormatter.hpp"
#include <algorithm>

namespace parser {

// Color constants
const std::string ErrorFormatter::RED = "\033[31m";
const std::string ErrorFormatter::BRIGHT_RED = "\033[91m";
const std::string ErrorFormatter::YELLOW = "\033[33m";
const std::string ErrorFormatter::BRIGHT_YELLOW = "\033[93m";
const std::string ErrorFormatter::BLUE = "\033[34m";
const std::string ErrorFormatter::BRIGHT_BLUE = "\033[94m";
const std::string ErrorFormatter::CYAN = "\033[36m";
const std::string ErrorFormatter::BRIGHT_CYAN = "\033[96m";
const std::string ErrorFormatter::GREEN = "\033[32m";
const std::string ErrorFormatter::MAGENTA = "\033[35m";
const std::string ErrorFormatter::BOLD = "\033[1m";
const std::string ErrorFormatter::DIM = "\033[2m";
const std::string ErrorFormatter::RESET = "\033[0m";

ErrorFormatter::ErrorFormatter() = default;

void ErrorFormatter::setSourceCode(const std::string& code) {
    sourceLines.clear();
    std::istringstream stream(code);
    std::string line;
    while (std::getline(stream, line)) {
        sourceLines.push_back(line);
    }
}

void ErrorFormatter::printSyntaxError(const std::string& filename, const SyntaxError& error) {
    printErrorHeader(filename, "SYNTAX ERROR", error.line, error.charPosition, BRIGHT_RED);
    printErrorDetails(error.msg, BRIGHT_RED);
    
    if (!error.sourceLine.empty()) {
        std::cerr << BRIGHT_RED << "|" << RESET << "\n";
        printSourceContext(error.line, error.charPosition, error.offendingText);
    }
    
    std::cerr << BRIGHT_RED << "+--" << RESET << " " << YELLOW << "Details:" << RESET << "\n";
    
    if (!error.offendingText.empty() && error.offendingText != "<EOF>") {
        std::cerr << "   " << MAGENTA << "* " << RESET
                  << "Offending token: " << BRIGHT_CYAN << "'"
                  << error.offendingText << "'" << RESET << "\n";
    }
    
    std::cerr << "\n";
}

void ErrorFormatter::printAnalysisIssue(const std::string& filename, const AnalysisIssue& analysisIssue) {
    const auto& issue = analysisIssue.issue;
    
    // Choose color based on severity
    std::string severityColor = YELLOW;
    std::string severityText = "WARNING";
    if (issue.severity == analyzer::Issue::Severity::Error) {
        severityColor = BRIGHT_RED;
        severityText = "ERROR";
    } else if (issue.severity == analyzer::Issue::Severity::Info) {
        severityColor = BLUE;
        severityText = "INFO";
    }
    
    printErrorHeader(filename, severityText, analysisIssue.line, analysisIssue.charPosition, severityColor);
    printErrorDetails("[" + issue.rule + "] " + issue.message, severityColor);
    
    if (!analysisIssue.sourceLine.empty()) {
        std::cerr << severityColor << "|" << RESET << "\n";
        printSourceContext(analysisIssue.line, analysisIssue.charPosition, "");
    }
    
    std::cerr << severityColor << "+--" << RESET << " " << YELLOW << "Details:" << RESET << "\n";
    printRuleInfo(issue.rule, issue.location);
    std::cerr << "\n";
}

void ErrorFormatter::printSourceContext(int line, int charPosition, const std::string& offendingText) {
    // Show context lines (previous and next lines if available)
    int contextStart = std::max(1, line - 2);
    int contextEnd = std::min((int)sourceLines.size(), line + 2);

    for (int lineNum = contextStart; lineNum <= contextEnd; lineNum++) {
        std::string lineContent = sourceLines[lineNum - 1];

        if (lineNum == line) {
            // Highlight the error line
            std::cerr << BRIGHT_RED << "|" << RESET << " "
                      << BRIGHT_BLUE << std::setw(3) << std::right
                      << lineNum << " | " << RESET << lineContent
                      << "\n";

            // Create enhanced visual indicator
            std::string indicator = BRIGHT_RED + "|" + RESET + "      " + RESET;
            int spaces = charPosition;
            int tabs = 0;

            // Count tabs before the error position
            for (int i = 0; i < std::min(spaces, (int)lineContent.length()); i++) {
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
            if (!offendingText.empty() && offendingText != "<EOF>") {
                indicator += BRIGHT_RED + BOLD + " ^ ";
                for (size_t i = 1; i < offendingText.length(); i++) {
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

void ErrorFormatter::printErrorHeader(const std::string& filename, const std::string& errorType, 
                                     int line, int charPosition, const std::string& color) {
    std::cerr << "\n";
    std::cerr << color << BOLD << "+-- " << errorType << RESET << " in "
              << BRIGHT_CYAN << filename << RESET;
    
    if (line > 0) {
        std::cerr << " at line " << BRIGHT_YELLOW << line << RESET;
        if (charPosition > 0) {
            std::cerr << ", column " << BRIGHT_YELLOW << (charPosition + 1) << RESET;
        }
    }
    std::cerr << "\n";
}

void ErrorFormatter::printErrorDetails(const std::string& message, const std::string& color) {
    std::cerr << color << "|" << RESET << " " << color << message << RESET << "\n";
}

void ErrorFormatter::printSuggestion(const std::string& suggestion) {
    if (!suggestion.empty()) {
        std::cerr << "   " << GREEN << "* " << RESET << "Suggestion: " << suggestion << "\n";
    }
}

void ErrorFormatter::printRuleInfo(const std::string& rule, const std::string& location) {
    std::cerr << "   " << MAGENTA << "* " << RESET << "Rule: " << BRIGHT_CYAN << rule << RESET << "\n";
    if (!location.empty()) {
        std::cerr << "   " << GREEN << "* " << RESET << "Location: " << location << "\n";
    }
}

} // namespace parser
