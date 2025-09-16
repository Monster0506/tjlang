#pragma once

#include <string>
#include <unordered_map>
#include <functional>

namespace parser {

class SuggestionEngine {
public:
    SuggestionEngine();
    
    std::string getSuggestion(const std::string& rule, const std::string& message, const std::string& context = "");
    std::string getSyntaxSuggestion(const std::string& msg, const std::string& offendingText);
    std::string getAnalysisSuggestion(const std::string& rule, const std::string& message);

private:
    std::unordered_map<std::string, std::function<std::string(const std::string&, const std::string&)>> suggestionMap;
    
    void initializeSuggestions();
    std::string getUnusedParameterSuggestion(const std::string& message, const std::string& context);
    std::string getDuplicateFunctionSuggestion(const std::string& message, const std::string& context);
    std::string getDuplicateParameterSuggestion(const std::string& message, const std::string& context);
    std::string getComplexitySuggestion(const std::string& message, const std::string& context);
    std::string getDeadCodeSuggestion(const std::string& message, const std::string& context);
    std::string getConstantConditionSuggestion(const std::string& message, const std::string& context);
    std::string getTypeSafetySuggestion(const std::string& message, const std::string& context);
};

} // namespace parser
