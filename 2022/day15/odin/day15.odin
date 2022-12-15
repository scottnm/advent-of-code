package main

import "core:fmt"
import "core:strings"
import "core:strconv"

main :: proc() {
    simple_input_file_contents := string(#load("day15_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day15_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day15_solve("simple", simple_input_lines[:len(simple_input_lines)-1])
    day15_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day15_solve :: proc(title: string, input_lines: []string) {
    fmt.printf("[{}] TODO: impl pt1\n", title)
    fmt.printf("[{}] TODO: impl pt2\n", title)
    // foo := parse_foo_from_input(input_lines)
    // defer delete(foo)
}

// parse_foo_from_input :: proc(input_lines: []string) -> ^foo_t {
//     return new(foo_t)
// }
