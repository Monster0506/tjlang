#pragma once

#include <memory>
#include <string>
#include <vector>

namespace ast {
class Program;
}

namespace analyzer {
class StaticAnalyzer;
}

class Driver {
   public:
    std::unique_ptr<ast::Program> parseFile(const std::string &filePath, bool debug = false);
    std::unique_ptr<ast::Program> parseAndAnalyze(const std::string &filePath, bool debug = false, bool runAnalysis = false);

   private:
    std::string readFile(const std::string &path);
};
