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

std::unique_ptr<ast::Program> Driver::parseFile(const std::string &filePath) {
    std::cout << "Reading file: " << filePath << std::endl;
    std::string source = readFile(filePath);
    std::cout << "File content:\n" << source << std::endl;

    antlr4::ANTLRInputStream input(source);
    LanguageLexer lexer(&input);
    antlr4::CommonTokenStream tokens(&lexer);
    LanguageParser parser(&tokens);

    // Add custom error listener
    ErrorListener errorListener;
    parser.removeErrorListeners();
    parser.addErrorListener(&errorListener);

    std::cout << "Starting parse..." << std::endl;
    // Run parse
    antlr4::tree::ParseTree *tree = parser.program();
    std::cout << "Parse completed." << std::endl;

    if (errorListener.hasErrors()) {
        std::cout << "Parse errors found:" << std::endl;
        for (auto &err : errorListener.errors) {
            std::cerr << filePath << ":" << err.line << ":" << err.charPosition
                      << ": error: " << err.msg << "\n";
        }
        return nullptr;
    }

    std::cout << "Parse successful, building AST..." << std::endl;
    // Build AST from parse tree
    ASTBuilder builder;
    auto result = builder.build(tree);
    std::cout << "AST built successfully!" << std::endl;
    return result;
}
