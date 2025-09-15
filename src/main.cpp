#include <iostream>

#include "ast/nodes.hpp"
#include "driver/driver.hpp"

int main(int argc, const char **argv) {
    if (argc < 2) {
        std::cerr << "Usage: mylang <source-file>\n";
        return 1;
    }

    Driver driver;
    auto ast = driver.parseFile(argv[1]);
    
    if (!ast) {
        std::cerr << "Failed to parse file: " << argv[1] << "\n";
        return 1;
    }

    std::cout << "Successfully parsed and built AST!\n";
    std::cout << "Program contains " << ast->units.size() << " units.\n";
    
    return 0;
}
