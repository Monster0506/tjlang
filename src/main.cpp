#include <iostream>

#include "ast/nodes.hpp"
#include "driver/driver.hpp"

int main(int argc, const char **argv) {
    if (argc < 2) {
        std::cerr << "Usage: mylang <source-file> [--debug]\n";
        std::cerr << "  --debug: Enable debug output\n";
        return 1;
    }

    bool debug = false;
    std::string filename = argv[1];
    
    // Check for debug flag
    if (argc > 2 && std::string(argv[2]) == "--debug") {
        debug = true;
    }

    Driver driver;
    auto ast = driver.parseFile(filename, debug);

    if (!ast) {
        std::cerr << "Failed to parse file: " << filename << "\n";
        return 1;
    }

    std::cout << "Successfully parsed and built AST!\n";
    std::cout << "Program contains " << ast->units.size() << " units.\n";

    return 0;
}
