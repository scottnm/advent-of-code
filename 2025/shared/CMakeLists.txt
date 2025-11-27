cmake_minimum_required(VERSION 3.20)

project(AdventOfCode2025_Shared LANGUAGES Swift)

cmake_path(SET ROOT_DIR NORMALIZE "${CMAKE_SOURCE_DIR}../")

add_library(util util.swift)