cmake_minimum_required(VERSION 3.10)
project(pros_package)

add_library(${PROJECT_NAME} src/main.cpp src/rust_exports.cpp)
target_include_directories(${PROJECT_NAME} PUBLIC include)
