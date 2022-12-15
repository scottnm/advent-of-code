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
    fmt.println()
    day15_solve("real", real_input_lines[:len(real_input_lines)-1], 2_000_000)


}

day15_solve :: proc(title: string, input_lines: []string, row: int) {
    // FIXME: tack this onto the odin template
    // set the main allocator to be the temp allocator and just free all memory at the end of this function

    // FIXME: WOOOOAHHHHHHH this seemed to be the main bug here. I guess I was exhausting all of the memory in the temp allocator.
    // I thinkt there's probably a lesson worth learning about the temp allocator which I don't quite want to think about just yet. Something something, you can't just assign the temp allocator and be on your way. You probably still need to periodically clean things up.
    // context.allocator = context.temp_allocator
    defer free_all(context.allocator)

    sensor_readings := read_sensor_data_from_lines(input_lines)
    // FIXME: fmt.println("SENSOR READINGS [before]:")
    // FIXME: print_sensor_data(sensor_readings, false)
    non_beacon_scanned_spaces := calc_row_non_beacon_scanned_spaces(sensor_readings, row)
    // FIXME: fmt.println("SENSOR READINGS [after]:")
    // FIXME: print_sensor_data(sensor_readings, false)
    fmt.printf("[{} pt1] non-beacon scanned spaces = {}\n", title, non_beacon_scanned_spaces)
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
    // FIXME: a map of ints to ints is suboptimal. If I had more time to think about this I could make this likely
    // much more performant by tracking these counts as an expanding array. The tricky part that I failed at the first
    // time is being able to expand on both the left and right sides of the array. If this was C I'd just do some manually
    // mallocs + memcpys and keep track of where the buffer starts and ends but here I'm not ready to jump to that

    row_count_map := make(map[int]bool)

    for s,i in sensor_readings {
        max_dist := s.manhattan_dist

        // FIXME: set_read_pos := false
        start_scan_dist := calc_manhattan_dist(s.sensor_pos, vec2{s.sensor_pos.x, row})
        for offset := 0;
            (start_scan_dist + offset) <= max_dist;
            offset += 1
        {
            // FIXME: set_read_pos = true
            row_count_map[s.sensor_pos.x + offset] = true
            row_count_map[s.sensor_pos.x - offset] = true
        }

        // FIXME:
        // if set_read_pos {
        //     // fmt.printf("sensor {} set read pos\n", i)
        // } else {
        //     fmt.printf("sensor {} did NOT set read pos! max_dist was {} start_scan_dist was {}\n", i, max_dist, start_scan_dist)
        // }
    }

    //FIXME: fmt.println("Old row count", len(row_count_map))

    // we need to ignore any beacons already registered in this row so delete their count values
    for s in sensor_readings {
        if s.closest_beacon_pos.y == row {
            if s.closest_beacon_pos.x in row_count_map {
                // FIXME: fmt.println("Deleting...", s.closest_beacon_pos.x)
                delete_key(&row_count_map, s.closest_beacon_pos.x)
            }
        }
    }

    return len(row_count_map)
}

calc_manhattan_dist :: proc(v1, v2: vec2) -> int {
    // manhattan distance is the "sum of the absolute differences of two points' cartesian coordinates"
    xdelta := v1.x - v2.x
    ydelta := v1.y - v2.y
    return abs(xdelta) + abs(ydelta)
}

print_sensor_data :: proc(sensor_readings: []sensor_data_t, original_format: bool = false) {
    if (original_format) {
        for s in sensor_readings {
            fmt.printf("Sensor at x={}, y={}: closest beacon is at x={}, y={}\n",
                s.sensor_pos.x, s.sensor_pos.y, s.closest_beacon_pos.x, s.closest_beacon_pos.y)
        }
    } else {
        for s,i in sensor_readings {
            can_reach_2mil := abs(s.sensor_pos.y - 2_000_000) <= s.manhattan_dist
            fmt.printf("{}: sensor@({},{}); beacon@({},{}); dist={}; can reach 2mil: {}\n",
                i,
                s.sensor_pos.x,
                s.sensor_pos.y,
                s.closest_beacon_pos.x,
                s.closest_beacon_pos.y,
                s.manhattan_dist,
                can_reach_2mil)
        }
    }
}
