#pragma once

#include <memory>
#include <string>

namespace ast {
class Program;
}

class Driver {
   public:
    std::unique_ptr<ast::Program> parseFile(const std::string &filePath);

   private:
    std::string readFile(const std::string &path);
};
