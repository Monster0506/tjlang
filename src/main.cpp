#include <iostream>

#include "driver/driver.hpp"

int main(int argc, const char **argv) {
    if (argc < 2) {
        std::cerr << "Usage: mylang <source-file>\n";
        return 1;
    }

    Driver driver;
    bool ok = driver.parseFile(argv[1]);
    return ok ? 0 : 1;
}
