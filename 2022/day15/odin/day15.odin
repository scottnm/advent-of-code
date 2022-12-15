package main

import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:c/libc"

vec2 :: struct {
    x: int,
    y: int,
}

sensor_data_t :: struct {
    sensor_pos: vec2,
    closest_beacon_pos: vec2,
    manhattan_dist: int,
}

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

    sensor_readings := read_sensor_data_from_lines(input_lines)
    non_beacon_scanned_spaces := calc_row_non_beacon_scanned_spaces(sensor_readings, row)
    fmt.printf("[{} pt1] non-beacon scanned spaces = {}\n", title, non_beacon_scanned_spaces)
    fmt.println("DEBUG SENSOR READINGS:")
    print_sensor_data(sensor_readings)
    fmt.printf("[{}] TODO: impl pt2\n", title)
}

read_sensor_data_from_lines :: proc(input_lines: []string) -> []sensor_data_t {
    sensor_readings := make([]sensor_data_t, len(input_lines))
    for line,i in input_lines {
        sensor_readings[i] = read_sensor_data_from_line(line)
    }
    return sensor_readings
}

read_sensor_data_from_line :: proc(line: string) -> sensor_data_t {
    // FIXME: fuckkkkk I should really just write an sscanf wrapper that gives me back odin strings
    line_cstr := strings.clone_to_cstring(line)

    // N.B. odin's default int seems to be 64-bit. Passing that naively to sscanf across the FFI boundary seems to just
    // fill the first 32-bits without any sign extension so negative numbers get messed up
    sensor_x: i32
    sensor_y: i32
    beacon_x: i32
    beacon_y: i32

    libc.sscanf(line_cstr, "Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d",
        &sensor_x, &sensor_y, &beacon_x, &beacon_y)

    sensor_pos := vec2{ cast(int)sensor_x, cast(int)sensor_y, }
    beacon_pos := vec2{ cast(int)beacon_x, cast(int)beacon_y, }
    manhattan_dist := calc_manhattan_dist(sensor_pos, beacon_pos)
    return sensor_data_t { sensor_pos, beacon_pos, manhattan_dist, }
}

calc_row_non_beacon_scanned_spaces :: proc(sensor_readings: []sensor_data_t, row: int) -> uint {
    return 0
}

calc_manhattan_dist :: proc(v1, v2: vec2) -> int {
    // manhattan distance is the "sum of the absolute differences of two points' cartesian coordinates"
    xdelta := v1.x - v2.x
    ydelta := v1.y - v2.y
    return abs(xdelta) + abs(ydelta)
}

print_sensor_data :: proc(sensor_readings: []sensor_data_t) {
    for s,i in sensor_readings {
        fmt.printf("{}: sensor@({},{}); beacon@({},{}); dist={}\n",
            i, s.sensor_pos.x, s.sensor_pos.y, s.closest_beacon_pos.x, s.closest_beacon_pos.y, s.manhattan_dist)
    }
}
