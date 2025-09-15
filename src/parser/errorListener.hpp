#pragma once

#include <string>
#include <vector>

#include "antlr4-runtime.h"

// Collects syntax errors instead of letting Antlr dump to stderr
class ErrorListener : public antlr4::BaseErrorListener {
   public:
    struct SyntaxError {
        int line;
        int charPosition;
        std::string msg;
    };

    std::vector<SyntaxError> errors;

    void syntaxError(antlr4::Recognizer *recognizer,
                     antlr4::Token *offendingSymbol, size_t line,
                     size_t charPositionInLine, const std::string &msg,
                     std::exception_ptr e) override {
        errors.push_back({static_cast<int>(line),
                          static_cast<int>(charPositionInLine), msg});
    }

    bool hasErrors() const { return !errors.empty(); }
};
