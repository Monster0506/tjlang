cmake_minimum_required(VERSION 3.20)
project(MyLang LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT DEFINED ANTLR_GEN_DIR)
  message(FATAL_ERROR "Pass -DANTLR_GEN_DIR when calling cmake")
endif()

# Gather generated sources
file(GLOB ANTLR_GEN_SOURCES "${ANTLR_GEN_DIR}/*.cpp")
file(GLOB ANTLR_GEN_HEADERS "${ANTLR_GEN_DIR}/*.h" "${ANTLR_GEN_DIR}/*.hpp")

# Gather user sources
set(MYLANG_SOURCES
    src/main.cpp
    src/driver/driver.cpp
    src/builder/builder.hpp
    src/serializer/serializer.cpp
    src/analyzer/analyzer.cpp
    src/analyzer/rules/rules_unused_params.cpp
    src/analyzer/rules/rules_unused_variables.cpp
    src/analyzer/rules/rules_duplicate_names.cpp
    src/analyzer/rules/rules_unreachable_code.cpp
    src/analyzer/rules/rules_dead_code.cpp
    src/analyzer/rules/rules_constant_conditions.cpp
    src/analyzer/rules/rules_type_safety.cpp
    src/analyzer/rules/rules_complexity.cpp
    src/analyzer/rules/rules_empty_functions.cpp
    src/analyzer/rules/rules_function_length.cpp
    src/analyzer/rules/rules_long_params.cpp
    src/analyzer/rules/rules_magic_numbers.cpp
    src/analyzer/rules/rules_naming_conventions.cpp
    src/parser/errorFormatter.cpp
    src/parser/suggestionEngine.cpp
    src/parser/errorCategories.cpp
    src/parser/unifiedErrorListener.cpp
)
set(MYLANG_HEADERS
    src/driver/driver.hpp
    src/parser/errorListener.hpp
    src/parser/unifiedErrorListener.hpp
    src/parser/errorFormatter.hpp
    src/parser/suggestionEngine.hpp
    src/parser/errorCategories.hpp
    src/builder/builder.cpp
    src/ast/nodes.hpp  
    src/serializer/serializer.hpp
    src/analyzer/analyzer.hpp
    src/analyzer/rules/rules_unused_params.hpp
    src/analyzer/rules/rules_unused_variables.hpp
    src/analyzer/rules/rules_duplicate_names.hpp
    src/analyzer/rules/rules_unreachable_code.hpp
    src/analyzer/rules/rules_dead_code.hpp
    src/analyzer/rules/rules_constant_conditions.hpp
    src/analyzer/rules/rules_type_safety.hpp
    src/analyzer/rules/rules_complexity.hpp
    src/analyzer/rules/rules_empty_functions.hpp
    src/analyzer/rules/rules_function_length.hpp
    src/analyzer/rules/rules_long_params.hpp
    src/analyzer/rules/rules_magic_numbers.hpp
    src/analyzer/rules/rules_naming_conventions.hpp
)

# Define executable
add_executable(tjlang
  ${MYLANG_SOURCES}
  ${MYLANG_HEADERS}
  ${ANTLR_GEN_SOURCES}
  ${ANTLR_GEN_HEADERS}
)

# Includes for our source and generated files
target_include_directories(tjlang PRIVATE
    "${ANTLR_GEN_DIR}"
    "${CMAKE_SOURCE_DIR}/src"
    C:/install/vcpkg/installed/x64-windows/include/antlr4-runtime
)

# Link against runtime library
target_link_libraries(tjlang PRIVATE
    C:/install/vcpkg/installed/x64-windows/lib/antlr4-runtime.lib
)

message(STATUS "ANTLR_GEN_DIR = ${ANTLR_GEN_DIR}")
message(STATUS "Added src include dir = ${CMAKE_SOURCE_DIR}/src")

