# FindVpp.cmake
# Usage:
#   list(APPEND CMAKE_MODULE_PATH "${VPP_ROOT}/cmake")
#   find_package(Vpp REQUIRED)
#
# Or set VPP_EXECUTABLE directly and include(Vpp).

if(NOT VPP_EXECUTABLE)
    find_program(
        VPP_EXECUTABLE
        NAMES vpp vpp.exe
        HINTS
            "$ENV{VPP_HOME}/bin"
            "$ENV{LOCALAPPDATA}/Programs/vpp"
            "${CMAKE_SOURCE_DIR}/../target/release"
            "${CMAKE_SOURCE_DIR}/../../target/release"
        DOC "v++ compiler (vpp build)"
    )
endif()

if(NOT VPP_EXECUTABLE)
    set(Vpp_FOUND FALSE)
    if(Vpp_FIND_REQUIRED)
        message(FATAL_ERROR "Could not find vpp. Install v++ or set VPP_EXECUTABLE.")
    endif()
    return()
endif()

get_filename_component(_vpp_real "${VPP_EXECUTABLE}" REALPATH)
get_filename_component(_vpp_bin_dir "${_vpp_real}" DIRECTORY)

if(NOT VPP_HOME)
    if(DEFINED ENV{VPP_HOME} AND NOT "$ENV{VPP_HOME}" STREQUAL "")
        set(VPP_HOME "$ENV{VPP_HOME}")
    elseif(EXISTS "${_vpp_bin_dir}/../std")
        get_filename_component(VPP_HOME "${_vpp_bin_dir}/.." ABSOLUTE)
    elseif(EXISTS "${_vpp_bin_dir}/std")
        set(VPP_HOME "${_vpp_bin_dir}")
    endif()
endif()

include("${CMAKE_CURRENT_LIST_DIR}/Vpp.cmake")

set(Vpp_FOUND TRUE)
set(Vpp_VERSION "1.0.3")

mark_as_advanced(VPP_EXECUTABLE VPP_HOME)
