cmake_minimum_required(VERSION 3.20)
project(MyLang LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
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
    # src/parser/errorListener.cpp
)
set(MYLANG_HEADERS
    src/driver/driver.hpp
    src/parser/errorListener.hpp
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
