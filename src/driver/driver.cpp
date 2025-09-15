#include <fstream>
#include <iostream>
#include <sstream>

#include "../ast/builder.hpp"
#include "../parser/errorListener.hpp"
#include "LanguageLexer.h"
#include "LanguageParser.h"
#include "driver.hpp"

std::string Driver::readFile(const std::string &path) {
    std::ifstream in(path, std::ios::binary);
    if (!in) throw std::runtime_error("Error: cannot open file '" + path + "'");
    std::ostringstream ss;
    ss << in.rdbuf();
    return ss.str();
}

std::unique_ptr<ast::Program> Driver::parseFile(const std::string &filePath, bool debug) {
    if (debug) {
        std::cout << "Reading file: " << filePath << std::endl;
    }
    std::string source = readFile(filePath);
    if (debug) {
        std::cout << "File content:\n" << source << std::endl;
    }

    antlr4::ANTLRInputStream input(source);
    LanguageLexer lexer(&input);
    antlr4::CommonTokenStream tokens(&lexer);
    LanguageParser parser(&tokens);

    // Add custom error listener to both lexer and parser
    ErrorListener errorListener;
    errorListener.setSourceCode(source);
    lexer.removeErrorListeners();
    lexer.addErrorListener(&errorListener);
    parser.removeErrorListeners();
    parser.addErrorListener(&errorListener);

    if (debug) {
        std::cout << "Tokens:" << std::endl;
        tokens.fill();
        for (auto t : tokens.getTokens()) {
            std::cout << "  [" << t->getType() << "] '" << t->getText() << "'";
            std::cout << " (line " << t->getLine() << ", col " << t->getCharPositionInLine() << ")" << std::endl;
        }
        std::cout << "Starting parse..." << std::endl;
    }
    // Run parse
    antlr4::tree::ParseTree *tree = parser.program();
    if (debug) {
        std::cout << "Parse completed." << std::endl;
    }

    if (errorListener.hasErrors()) {
        errorListener.printErrors(filePath);
        return nullptr;
    }

    if (debug) {
        std::cout << "Parse successful, building AST..." << std::endl;
    }
    // Build AST from parse tree
    ASTBuilder builder(debug);
    auto result = builder.build(tree);
    if (debug) {
        std::cout << "AST built successfully!" << std::endl;
    }
    return result;
}
