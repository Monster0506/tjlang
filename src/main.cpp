#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

#include "LanguageLexer.h"
#include "LanguageParser.h"

static std::string readFile(const std::string &path) {
    std::ifstream in(path, std::ios::binary);
    if (!in) {
        throw std::runtime_error("Error: could not open file '" + path + "'");
    }
    std::ostringstream ss;
    ss << in.rdbuf();
    return ss.str();
}

int main(int argc, const char **argv) {
    if (argc < 2) {
        std::cerr << "Usage: mylang <source-file>\n";
        return 1;
    }

    std::string filePath = argv[1];
    std::string inputText;
    try {
        inputText = readFile(filePath);
    } catch (const std::exception &ex) {
        std::cerr << ex.what() << "\n";
        return 1;
    }

    try {
        antlr4::ANTLRInputStream input(inputText);
        LanguageLexer lexer(&input);
        antlr4::CommonTokenStream tokens(&lexer);
        LanguageParser parser(&tokens);

        antlr4::tree::ParseTree *tree = parser.program();

        std::cout << tree->toStringTree(&parser) << "\n";
    } catch (const std::exception &ex) {
        std::cerr << "Parser error: " << ex.what() << "\n";
        return 1;
    }

    return 0;
}
