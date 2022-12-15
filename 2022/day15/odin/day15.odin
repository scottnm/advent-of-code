package main

import "core:fmt"
import "core:strings"
import "core:strconv"

vec2 :: struct {
    x: int,
    y: int,
}

sensor_data_t :: struct {
    sensor_pos: vec2,
    closest_beacon_pos: vec2,
}

// FIXME: I almost started with a grid but after looking at the real problem data a grid wouldn't be feasible
// e.g. the grid is nearly 4mil wide by 4mil tall which if I store at 1byte per cell is terabytes of data. haha no.
// cell_type_t :: enum u8 {
//     sensor = 'S',
//     beacon = 'B',
//     known_empty = '#',
//     unknown = '.',
// }
//
// grid_t :: struct {
//     cells: []cell_type_t,
//     width: int,
//     height: int,
//     origin: vec2,
// }


main :: proc() {
    simple_input_file_contents := string(#load("day15_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day15_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day15_solve("simple", simple_input_lines[:len(simple_input_lines)-1], 10)
    // day15_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day15_solve :: proc(title: string, input_lines: []string, row: int) {
    // FIXME: tack this onto the odin template
    // set the main allocator to be the temp allocator and just free all memory at the end of this function
    context.allocator = context.temp_allocator
    defer free_all(context.temp_allocator)

    sensor_readings := read_sensor_data_from_input(input_lines)
    non_beacon_scanned_spaces := calc_row_non_beacon_scanned_spaces(sensor_readings, row)
    fmt.printf("[{} pt1] non-beacon scanned spaces = {}\n", title, non_beacon_scanned_spaces)
    fmt.printf("[{}] TODO: impl pt2\n", title)
}

read_sensor_data_from_input :: proc(input_lines: []string) -> []sensor_data_t {
    return nil
}

calc_row_non_beacon_scanned_spaces :: proc(sensor_readings: []sensor_data_t, row: int) -> uint {
    return 0
}
