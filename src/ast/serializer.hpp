#pragma once

#include <string>
#include "nodes.hpp"

namespace astio {

// Writes a JSON-like representation of the AST to the given file path.
// Returns true on success, false otherwise.
bool writeAstToFile(const ast::Program& program, const std::string& filePath);

} // namespace astio


