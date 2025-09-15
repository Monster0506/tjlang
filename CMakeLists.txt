cmake_minimum_required(VERSION 3.20)
project(MyLang LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT DEFINED ANTLR_GEN_DIR)
  message(FATAL_ERROR "Pass -DANTLR_GEN_DIR")
endif()

file(GLOB ANTLR_GEN_SOURCES "${ANTLR_GEN_DIR}/*.cpp")
file(GLOB ANTLR_GEN_HEADERS "${ANTLR_GEN_DIR}/*.h" "${ANTLR_GEN_DIR}/*.hpp")

add_executable(tjlang
  src/main.cpp
  ${ANTLR_GEN_SOURCES}
  ${ANTLR_GEN_HEADERS}
)

target_include_directories(tjlang PRIVATE "${ANTLR_GEN_DIR}")
# Runtime headers + lib
target_include_directories(tjlang PRIVATE C:/install/vcpkg/installed/x64-windows/include/antlr4-runtime)
target_link_libraries(tjlang PRIVATE C:/install/vcpkg/installed/x64-windows/lib/antlr4-runtime.lib)

message(STATUS "ANTLR_GEN_DIR = ${ANTLR_GEN_DIR}")
message(STATUS "RUNTIME INC   = C:/install/vcpkg/installed/x64-windows/include/antlr4-runtime")
message(STATUS "RUNTIME LIB   = C:/install/vcpkg/installed/x64-windows/lib/antlr4-runtime.lib")
