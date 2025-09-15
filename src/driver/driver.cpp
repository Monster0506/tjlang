#include <fstream>
#include <iostream>
#include <sstream>

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

bool Driver::parseFile(const std::string &filePath) {
    std::string source = readFile(filePath);

    antlr4::ANTLRInputStream input(source);
    LanguageLexer lexer(&input);
    antlr4::CommonTokenStream tokens(&lexer);
    LanguageParser parser(&tokens);

    // Add custom error listener
    ErrorListener errorListener;
    parser.removeErrorListeners();
    parser.addErrorListener(&errorListener);

    // Run parse
    antlr4::tree::ParseTree *tree = parser.program();

    if (errorListener.hasErrors()) {
        for (auto &err : errorListener.errors) {
            std::cerr << filePath << ":" << err.line << ":" << err.charPosition
                      << ": error: " << err.msg << "\n";
        }
        return false;
    }

    // Print debug parse tree (optional)
    std::cout << tree->toStringTree(&parser) << "\n";
    return true;
}
