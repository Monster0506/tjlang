#include <iostream>
#include <exception>
#include <csignal>

#include "ast/nodes.hpp"
#include "ast/serializer.hpp"
#include "driver/driver.hpp"

static void signalHandler(int sig) {
    std::cerr << "Signal caught: " << sig << "\n";
    std::cerr.flush();
    std::abort();
}

static void setupCrashHandlers() {
    std::set_terminate([](){
        std::cerr << "std::terminate called (unhandled exception).\n";
        try { throw; }
        catch (const std::exception& ex) { std::cerr << "Exception: " << ex.what() << "\n"; }
        catch (...) { std::cerr << "Unknown exception.\n"; }
        std::abort();
    });
    std::signal(SIGABRT, signalHandler);
    std::signal(SIGSEGV, signalHandler);
}

int main(int argc, const char **argv) {
    setupCrashHandlers();
    if (argc < 2) {
        std::cerr << "Usage: mylang <source-file> [--debug] [--emit-ast[=<path>]]\n";
        std::cerr << "  --debug: Enable debug output\n";
        std::cerr << "  --emit-ast: Write AST to <source-file>.tjast (or to <path> if provided)\n";
        return 1;
    }


    bool debug = false;
    bool emitAst = false;
    std::string emitAstPath;
    std::string filename = argv[1];
    
    // Parse flags (very simple)
    for (int i = 2; i < argc; ++i) {
        std::string arg = argv[i];
        if (arg == "--debug") {
            debug = true;
        } else if (arg.rfind("--emit-ast", 0) == 0) {
            emitAst = true;
            auto eq = arg.find('=');
            if (eq != std::string::npos && eq + 1 < arg.size()) {
                emitAstPath = arg.substr(eq + 1);
            }
        }
    }

    try {
        Driver driver;
        auto ast = driver.parseFile(filename, debug);
        if (!ast) {
            std::cerr << "Failed to parse file: " << filename << "\n";
            return 1;
        }

        std::cout << "Successfully parsed and built AST!\n";
        std::cout << "Program contains " << ast->units.size() << " units.\n";

        if (emitAst) {
            std::string outPath = emitAstPath;
            if (outPath.empty()) {
                outPath = filename + ".tjast"; // TJ AST JSON
            }
            std::cout << "Emitting AST to: " << outPath << "\n";
            if (!astio::writeAstToFile(*ast, outPath)) {
                std::cerr << "Failed to write AST to '" << outPath << "'\n";
                return 1;
            }
            std::cout << "AST written to: " << outPath << "\n";
        }
    } catch (const std::exception& ex) {
        std::cerr << "Unhandled error: " << ex.what() << "\n";
        return 1;
    } catch (...) {
        std::cerr << "Unhandled unknown error.\n";
        return 1;
    }

    return 0;
}
