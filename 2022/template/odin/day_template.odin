package main

import "core:fmt"
import "core:strings"
import "core:strconv"

main :: proc() {
    simple_input_file_contents := string(#load("day_template_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day_template_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day_template_solve("simple", simple_input_lines[:len(simple_input_lines)-1])
    // day_template_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day_template_solve :: proc(title: string, input_lines: []string) {
    pt1_result := 0
    pt2_result := 0

    fmt.printf("[{} pt1] result = {}\n", title, pt1_result)
    fmt.printf("[{} pt2] result = {}\n", title, pt2_result)
}
