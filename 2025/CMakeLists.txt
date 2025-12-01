cmake_minimum_required(VERSION 3.10)

project(AdventOfCode2025 LANGUAGES Swift)

set(ROOT_DIR "${CMAKE_SOURCE_DIR}")

# enable regex for all projects
add_compile_options(-enable-bare-slash-regex)

add_subdirectory(shared)

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "1")
    add_executable(day01 day01/main.swift)
    target_link_libraries(day01 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "2")
    add_executable(day02 day02/main.swift)
    target_link_libraries(day02 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "3")
    add_executable(day03 day03/main.swift)
    target_link_libraries(day03 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "4")
    add_executable(day04 day04/main.swift)
    target_link_libraries(day04 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "5")
    add_executable(day05 day05/main.swift)
    target_link_libraries(day05 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "6")
    add_executable(day06 day06/main.swift)
    target_link_libraries(day06 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "7")
    add_executable(day07 day07/main.swift)
    target_link_libraries(day07 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "8")
    add_executable(day08 day08/main.swift)
    target_link_libraries(day08 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "9")
    add_executable(day09 day09/main.swift)
    target_link_libraries(day09 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "10")
    add_executable(day10 day10/main.swift)
    target_link_libraries(day10 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "11")
    add_executable(day11 day11/main.swift)
    target_link_libraries(day11 util)
endif()

if (BUILD_DAY STREQUAL "all" OR BUILD_DAY STREQUAL "12")
    add_executable(day12 day12/main.swift)
    target_link_libraries(day12 util)
endif()