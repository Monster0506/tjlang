#pragma once

#include <string>

class Driver {
   public:
    bool parseFile(const std::string &filePath);

   private:
    std::string readFile(const std::string &path);
};
