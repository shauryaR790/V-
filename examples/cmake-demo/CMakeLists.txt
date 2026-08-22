cmake_minimum_required(VERSION 3.16)
project(cmake_demo LANGUAGES NONE)

# Point at the repo cmake module (or install path).
get_filename_component(VPP_REPO_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
list(APPEND CMAKE_MODULE_PATH "${VPP_REPO_ROOT}/cmake")

find_package(Vpp REQUIRED)

# Option A: single entry file
vpp_add_executable(cmake_hello src/main.vpp)

# Option B: vpp.toml project (same result if entry = src/main.vpp)
# vpp_add_project(cmake_hello)

message(STATUS "vpp: ${VPP_EXECUTABLE}")
message(STATUS "VPP_HOME: ${VPP_HOME}")
