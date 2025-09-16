#pragma once

#include <string>

namespace parser {

class ErrorCategories {
public:
    static std::string getSyntaxErrorCategory(const std::string& msg);
    static std::string getAnalysisErrorCategory(const std::string& rule);
    static std::string getSeverityDescription(const std::string& severity);

private:
    static bool contains(const std::string& str, const std::string& substr);
};

} // namespace parser
