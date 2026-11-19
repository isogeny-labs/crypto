cmake_minimum_required(VERSION 3.16)
project(semaev_structure_lab LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_executable(semaev_lab src/semaev_lab.cpp)
target_compile_options(semaev_lab PRIVATE -O3 -march=native -Wall -Wextra -Wpedantic -Wconversion)

enable_testing()
add_test(NAME self_test COMMAND semaev_lab self-test)
