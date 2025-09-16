#include "suggestionEngine.hpp"
#include <iostream>

namespace parser {

SuggestionEngine::SuggestionEngine() {
    initializeSuggestions();
}

void SuggestionEngine::initializeSuggestions() {
    // Analysis rule suggestions
    suggestionMap["unused-parameter"] = [this](const std::string& msg, const std::string& ctx) {
        return getUnusedParameterSuggestion(msg, ctx);
    };
    
    suggestionMap["duplicate-function"] = [this](const std::string& msg, const std::string& ctx) {
        return getDuplicateFunctionSuggestion(msg, ctx);
    };
    
    suggestionMap["duplicate-parameter"] = [this](const std::string& msg, const std::string& ctx) {
        return getDuplicateParameterSuggestion(msg, ctx);
    };
    
    suggestionMap["high-complexity"] = [this](const std::string& msg, const std::string& ctx) {
        return getComplexitySuggestion(msg, ctx);
    };
    
    suggestionMap["deep-nesting"] = [this](const std::string& msg, const std::string& ctx) {
        return getComplexitySuggestion(msg, ctx);
    };
    
    suggestionMap["long-function-name"] = [this](const std::string& msg, const std::string& ctx) {
        return "Consider using a shorter, more descriptive function name";
    };
    
    suggestionMap["dead-code"] = [this](const std::string& msg, const std::string& ctx) {
        return getDeadCodeSuggestion(msg, ctx);
    };
    
    suggestionMap["constant-condition"] = [this](const std::string& msg, const std::string& ctx) {
        return getConstantConditionSuggestion(msg, ctx);
    };
    
    suggestionMap["type-mismatch"] = [this](const std::string& msg, const std::string& ctx) {
        return getTypeSafetySuggestion(msg, ctx);
    };
}

std::string SuggestionEngine::getSuggestion(const std::string& rule, const std::string& message, const std::string& context) {
    auto it = suggestionMap.find(rule);
    if (it != suggestionMap.end()) {
        return it->second(message, context);
    }
    return ""; // No suggestion available
}

std::string SuggestionEngine::getSyntaxSuggestion(const std::string& msg, const std::string& offendingText) {
    if (msg.find("missing ')'") != std::string::npos) {
        return "Add a closing parenthesis ')' to match the opening parenthesis";
    }
    if (msg.find("missing '{'") != std::string::npos) {
        return "Add an opening brace '{'";
    }
    if (msg.find("missing '}'") != std::string::npos) {
        return "Add a closing brace '}'";
    }
    if (msg.find("missing ':'") != std::string::npos) {
        return "Add a colon ':'";
    }
    if (msg.find("missing '->'") != std::string::npos) {
        return "Add return type arrow '->' before the return type";
    }
    if (msg.find("missing 'def'") != std::string::npos) {
        return "Add 'def' keyword to declare a function";
    }
    if (msg.find("token recognition error") != std::string::npos) {
        return "Remove or replace the invalid character '" + offendingText + "'";
    }
    if (msg.find("mismatched input") != std::string::npos) {
        return "Check syntax around this position - expected different token";
    }
    return "";
}

std::string SuggestionEngine::getAnalysisSuggestion(const std::string& rule, const std::string& message) {
    return getSuggestion(rule, message, "");
}

std::string SuggestionEngine::getUnusedParameterSuggestion(const std::string& message, const std::string& context) {
    return "Remove the unused parameter or use it in the function body. Consider prefixing with underscore if intentionally unused.";
}

std::string SuggestionEngine::getDuplicateFunctionSuggestion(const std::string& message, const std::string& context) {
    return "Rename one of the functions to have a unique name. Consider using more descriptive names that reflect their different purposes.";
}

std::string SuggestionEngine::getDuplicateParameterSuggestion(const std::string& message, const std::string& context) {
    return "Rename one of the duplicate parameters to have a unique name. Each parameter should have a distinct identifier.";
}

std::string SuggestionEngine::getComplexitySuggestion(const std::string& message, const std::string& context) {
    if (message.find("high cyclomatic complexity") != std::string::npos) {
        return "Break down the function into smaller, more focused functions. Consider extracting complex logic into helper functions.";
    }
    if (message.find("deep nesting") != std::string::npos) {
        return "Reduce nesting by using early returns, guard clauses, or extracting nested logic into separate functions.";
    }
    return "Consider simplifying the function structure to improve readability and maintainability.";
}

std::string SuggestionEngine::getDeadCodeSuggestion(const std::string& message, const std::string& context) {
    return "Remove the unreachable code after the return statement. This code will never be executed and may indicate a logic error.";
}

std::string SuggestionEngine::getConstantConditionSuggestion(const std::string& message, const std::string& context) {
    if (message.find("always true") != std::string::npos) {
        return "The condition always evaluates to true. Consider removing the if statement or using a more dynamic condition.";
    }
    if (message.find("always false") != std::string::npos) {
        return "The condition always evaluates to false. This code block will never execute and should be removed.";
    }
    return "The condition has a constant value. Consider using a more dynamic expression or removing the conditional.";
}

std::string SuggestionEngine::getTypeSafetySuggestion(const std::string& message, const std::string& context) {
    return "Ensure type compatibility between operands. Consider explicit type conversion or using compatible types.";
}

} // namespace parser
