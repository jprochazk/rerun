cmake_minimum_required(VERSION 3.16)

# project(RerunExample)

# ------------------------------------------------------------------------------

# Arrow requires a C++17 compliant compiler
if(NOT DEFINED CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 17)
endif()

add_executable(rerun_example main.cpp)

set_default_warning_settings(rerun_example)
target_link_libraries(rerun_example PRIVATE loguru::loguru rerun_sdk)
