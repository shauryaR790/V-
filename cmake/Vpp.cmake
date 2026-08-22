# Vpp.cmake - CMake integration for the v++ language

if(NOT VPP_EXECUTABLE)
    message(FATAL_ERROR "Vpp.cmake: set VPP_EXECUTABLE or use find_package(Vpp)")
endif()

function(_vpp_build_env out_var)
    set(_env)
    if(VPP_HOME)
        list(APPEND _env "VPP_HOME=${VPP_HOME}")
    endif()
    if(DEFINED ENV{LLVM_SYS_221_PREFIX} AND NOT "$ENV{LLVM_SYS_221_PREFIX}" STREQUAL "")
        list(APPEND _env "LLVM_SYS_221_PREFIX=$ENV{LLVM_SYS_221_PREFIX}")
    endif()
    set(${out_var} "${_env}" PARENT_SCOPE)
endfunction()

function(vpp_add_executable target source)
    if(NOT target MATCHES "^[A-Za-z0-9_+-]+$")
        message(FATAL_ERROR "vpp_add_executable: invalid target name '${target}'")
    endif()

    cmake_parse_arguments(VPP "" "OUTPUT_NAME;WORKING_DIRECTORY" "DEPENDS" ${ARGN})

    get_filename_component(_source_abs "${source}" ABSOLUTE BASE_DIR "${CMAKE_CURRENT_SOURCE_DIR}")
    if(NOT EXISTS "${_source_abs}")
        message(FATAL_ERROR "vpp_add_executable: source not found: ${_source_abs}")
    endif()

    if(VPP_OUTPUT_NAME)
        set(_out_name "${VPP_OUTPUT_NAME}")
    else()
        set(_out_name "${target}")
    endif()

    if(WIN32)
        set(_out_file "${CMAKE_CURRENT_BINARY_DIR}/${_out_name}.exe")
    else()
        set(_out_file "${CMAKE_CURRENT_BINARY_DIR}/${_out_name}")
    endif()

    if(VPP_WORKING_DIRECTORY)
        set(_workdir "${VPP_WORKING_DIRECTORY}")
    else()
        get_filename_component(_workdir "${_source_abs}" DIRECTORY)
    endif()

    _vpp_build_env(_vpp_env)

    add_custom_command(
        OUTPUT "${_out_file}"
        COMMAND ${CMAKE_COMMAND} -E env ${_vpp_env}
            "${VPP_EXECUTABLE}" build "${_source_abs}" -o "${_out_file}"
        DEPENDS "${_source_abs}" ${VPP_DEPENDS}
        WORKING_DIRECTORY "${_workdir}"
        COMMENT "v++ build ${target}"
        VERBATIM
    )

    add_custom_target("${target}" ALL DEPENDS "${_out_file}")
    set_target_properties("${target}" PROPERTIES
        VPP_SOURCE "${_source_abs}"
        VPP_OUTPUT "${_out_file}"
    )
endfunction()

function(vpp_add_project target)
    cmake_parse_arguments(VPP "" "PROJECT_ROOT" "" ${ARGN})

    if(VPP_PROJECT_ROOT)
        set(_root "${VPP_PROJECT_ROOT}")
    else()
        set(_root "${CMAKE_CURRENT_SOURCE_DIR}")
    endif()

    get_filename_component(_root "${_root}" ABSOLUTE)
    set(_manifest "${_root}/vpp.toml")
    if(NOT EXISTS "${_manifest}")
        message(FATAL_ERROR "vpp_add_project: no vpp.toml in ${_root}")
    endif()

    file(READ "${_manifest}" _manifest_text)
    if(_manifest_text MATCHES "entry[ \t]*=[ \t]*\"([^\"]+)\"")
        set(_entry "${CMAKE_MATCH_1}")
    else()
        message(FATAL_ERROR "vpp_add_project: could not parse entry from ${_manifest}")
    endif()

    vpp_add_executable("${target}" "${_root}/${_entry}" WORKING_DIRECTORY "${_root}")
endfunction()
