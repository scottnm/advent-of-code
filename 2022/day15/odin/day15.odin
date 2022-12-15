package main

import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:c/libc"
import "core:testing"
import "core:sort"

vec2 :: struct {
    x: int,
    y: int,
}

row_span :: struct {
    start: int,
    end: int,
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

    day15_solve("simple", simple_input_lines[:len(simple_input_lines)-1], 10, {20,20})
    fmt.println()
    day15_solve("real", real_input_lines[:len(real_input_lines)-1], 2_000_000, {4_000_000,4_000_000})
}

day15_solve :: proc(title: string, input_lines: []string, pt1_row: int, grid_size: vec2) {
    // FIXME: WOOOOAHHHHHHH this seemed to be the main bug here. I guess I was exhausting all of the memory in the temp allocator.
    // I thinkt there's probably a lesson worth learning about the temp allocator which I don't quite want to think about just yet. Something something, you can't just assign the temp allocator and be on your way. You probably still need to periodically clean things up.
    // context.allocator = context.temp_allocator
    defer free_all(context.allocator)

    sensor_readings := read_sensor_data_from_lines(input_lines)
    coverage_spans_pt1 := build_row_sensor_coverage(sensor_readings, pt1_row)
    defer delete(coverage_spans_pt1)

    non_beacon_scanned_spaces_pt1 := calc_row_non_beacon_scanned_spaces(sensor_readings, pt1_row, coverage_spans_pt1)
    fmt.printf("[{} pt1] non-beacon scanned spaces @ {} = {}\n", title, pt1_row, non_beacon_scanned_spaces_pt1)

    for row in 0 ..= grid_size.y {
        coverage_spans := build_row_sensor_coverage(sensor_readings, row)
        defer delete(coverage_spans)

        uncovered_cells := collect_uncovered_cells(coverage_spans, grid_size.x)
        defer delete(uncovered_cells)

        if len(uncovered_cells) == 1 {
            uncovered_cell_pos := vec2{uncovered_cells[0], row}
            frequency :=  uncovered_cell_pos.x * 4_000_000 + uncovered_cell_pos.y
            fmt.printf("[{} pt2] frequent @ {},{} = {}\n", title, uncovered_cell_pos.x, uncovered_cell_pos.y, frequency)
        }
    }
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
    defer delete(line_cstr)

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

calc_row_non_beacon_scanned_spaces :: proc(sensor_readings: []sensor_data_t, row: int, coverage_spans: []row_span) -> uint {
    // count all of the cells covered by coverage spans
    cells_covered := 0
    for span in coverage_spans {
        cells_covered += (span.end - span.start + 1)
    }

    // discount all of the known beacons
    row_beacon_map := make(map[int]bool)
    for s in sensor_readings {
        if s.closest_beacon_pos.y == row {
            row_beacon_map[s.closest_beacon_pos.x] = true
        }
    }

    cells_covered -= len(row_beacon_map)

    return cast(uint)cells_covered
}

collect_uncovered_cells :: proc(coverage_spans: []row_span, grid_width: int) -> []int {
    uncovered_cells := make([dynamic]int)

    last_coverage_end := -1 // our grid starts at 0
    for coverage_span in coverage_spans {
        for i := last_coverage_end + 1; i < coverage_span.start; i += 1 {
            append(&uncovered_cells, i)
        }
        last_coverage_end = coverage_span.end
    }

    for i := last_coverage_end + 1; i < grid_width; i += 1 {
        append(&uncovered_cells, i)
    }

    return uncovered_cells[:]
}

build_row_sensor_coverage :: proc(sensor_readings: []sensor_data_t, row: int) -> []row_span {
    coverage_spans := make([dynamic]row_span)

    for s in sensor_readings {
        max_dist := s.manhattan_dist
        min_dist := calc_manhattan_dist(s.sensor_pos, vec2{s.sensor_pos.x, row})

        // if this sensor's nearest beacon is closer than the distance from this sensor to the row, then this sensor
        // can't see any positions on this row. Skip.
        if min_dist > max_dist do continue

        remaining_sensor_range := max_dist - min_dist
        sensor_row_coverage_span := row_span { s.sensor_pos.x - remaining_sensor_range, s.sensor_pos.x + remaining_sensor_range }
        append(&coverage_spans, sensor_row_coverage_span)
    }

    // combine any overlapping sensor ranges

    next_source_span_index := 0
    for next_source_span_index < len(coverage_spans) {
        source_span := coverage_spans[next_source_span_index]

        merged := false

        for next_merge_span_index := 0; next_merge_span_index < len(coverage_spans); next_merge_span_index += 1 {
            if next_source_span_index == next_merge_span_index do continue

            merge_span := coverage_spans[next_merge_span_index]
            if are_row_spans_mergable(source_span, merge_span) {
                coverage_spans[next_source_span_index] = merge_row_spans(source_span, merge_span)
                unordered_remove(&coverage_spans, next_merge_span_index)
                merged = true
                break
            }
        }

        if !merged {
            next_source_span_index += 1
        }
    }

    sort.merge_sort_proc(coverage_spans[:], cmp_row_spans)

    return coverage_spans[:]
}

are_row_spans_mergable :: proc(v1, v2: row_span) -> bool {
    return (v1.start <= v2.start && v1.end >= v2.start-1) ||
           (v2.start <= v1.start && v2.end >= v1.start-1)
}

merge_row_spans :: proc(v1, v2: row_span) -> row_span {
    return row_span {
        start=min(v1.start,v2.start),
        end=max(v1.end,v2.end),
    }
}

cmp_row_spans :: proc(v1, v2: row_span) -> int {
    if (v1.start < v2.start) {
        return -1
    } else if (v1.start == v2.start) {
        return 0
    } else {
        return 1
    }
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

@test
test_row_span_merges :: proc(t: ^testing.T) {
    {
        r1 := row_span{1, 1}
        r2 := row_span{1, 1}
        testing.expect(t, are_row_spans_mergable(r1, r2))
        testing.expect_value(t, merge_row_spans(r1, r2), r1)
    }
    {
        r1 := row_span{1, 10}
        r2 := row_span{2, 5}
        testing.expect(t, are_row_spans_mergable(r1, r2))
        testing.expect_value(t, merge_row_spans(r1, r2), r1)
    }
    {
        r1 := row_span{2, 5}
        r2 := row_span{1, 10}
        testing.expect(t, are_row_spans_mergable(r1, r2))
        testing.expect_value(t, merge_row_spans(r1, r2), r2)
    }
    {
        r1 := row_span{-2, 5}
        r2 := row_span{4, 10}
        testing.expect(t, are_row_spans_mergable(r1, r2))
        testing.expect_value(t, merge_row_spans(r1, r2), row_span{-2,10})
    }
    {
        r1 := row_span{-2, 5}
        r2 := row_span{5, 10}
        testing.expect(t, are_row_spans_mergable(r1, r2))
        testing.expect_value(t, merge_row_spans(r1, r2), row_span{-2,10})
    }
    {
        r1 := row_span{-2, 5}
        r2 := row_span{6, 10}
        testing.expect(t, are_row_spans_mergable(r1, r2))
        testing.expect_value(t, merge_row_spans(r1, r2), row_span{-2,10})
    }
    {
        r1 := row_span{-2, 5}
        r2 := row_span{7, 10}
        testing.expect(t, !are_row_spans_mergable(r1, r2))
    }
    {
        r1 := row_span{-200, -100}
        r2 := row_span{-90, -50}
        testing.expect(t, !are_row_spans_mergable(r1, r2))
    }
    {
        r1 := row_span{100, 200}
        r2 := row_span{201, 300}
        testing.expect(t, are_row_spans_mergable(r1, r2))
        testing.expect_value(t, merge_row_spans(r1, r2), row_span{100,300})
    }
    {
        r1 := row_span{201, 300}
        r2 := row_span{100, 200}
        testing.expect(t, are_row_spans_mergable(r1, r2))
        testing.expect_value(t, merge_row_spans(r1, r2), row_span{100,300})
    }
    {
        r1 := row_span{100, 200}
        r2 := row_span{202, 300}
        testing.expect(t, !are_row_spans_mergable(r1, r2))
    }
    {
        r1 := row_span{202, 300}
        r2 := row_span{100, 200}
        testing.expect(t, !are_row_spans_mergable(r1, r2))
    }
}
