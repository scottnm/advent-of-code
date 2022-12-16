package main

import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:c/libc"
import "core:testing"
import "core:sort"
import "core:runtime"

valve_data_t :: struct {
    flow_rate_ppm: int, // pressure per min
    dest_valves: []string,
}

main :: proc() {
    simple_input_file_contents := string(#load("day16_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day16_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day16_solve("simple", simple_input_lines[:len(simple_input_lines)-1])
    // day16_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day16_solve :: proc(title: string, input_lines: []string) {
    valve_map := read_valve_map_entries(input_lines)
    defer delete(valve_map)
    // print_valve_entries("debug", valve_map)

    max_pressure := find_max_possible_releasable_pressure(valve_map, 30)
    fmt.printf("[{} pt1] max pressure = {}\n", title, max_pressure)

    // fmt.printf("[{}] TODO: impl pt2\n", title)
}

read_valve_map_entries :: proc(lines: []string) -> map[string]valve_data_t {
    valve_map := make(map[string]valve_data_t)
    for line in lines {
        valve_name, valve_data := read_valve_map_entry(line)
        valve_map[valve_name] = valve_data
    }
    return valve_map
}

read_valve_map_entry :: proc(line: string) -> (string, valve_data_t) {
    split_index := strings.index(line, "tunnel")
    assert(split_index != -1)

    line_pt1 := line[:split_index]
    line_pt2 := line[split_index:]

    line_cstr_pt1 := strings.clone_to_cstring(line_pt1)
    defer delete(line_cstr_pt1)

    valve_name: string
    flow_rate: int
    {
        valve_name_cstr_buffer: [1024]u8
        valve_name_cstr := transmute(cstring)runtime.Raw_Cstring{data=&valve_name_cstr_buffer[0]}
        scan_res_pt1 := libc.sscanf(line_cstr_pt1, "Valve %s has flow rate=%d;", &valve_name_cstr_buffer[0], &flow_rate);
        assert(scan_res_pt1 == 2)
        valve_name = strings.clone_from_cstring(valve_name_cstr)
    }

    dest_valve_names: string
    if (strings.has_prefix(line_pt2, "tunnels")) {
        dest_valve_names = line_pt2[len("tunnels lead to valves "):]
    } else {
        assert(strings.has_prefix(line_pt2, "tunnel"))
        dest_valve_names = line_pt2[len("tunnel leads to valve "):]
    }

    dest_valves := make([dynamic]string) // arbitrarily start with capacity 5 since that it seems like the input tops out at
    for dest_valve_name in strings.split_iterator(&dest_valve_names, ", ") {
        append(&dest_valves, strings.clone(dest_valve_name))
    }

    return valve_name,valve_data_t { flow_rate_ppm=flow_rate, dest_valves=dest_valves[:] }
}

print_valve_entries :: proc(suffix: string, valve_map: map[string]valve_data_t) {
    fmt.printf("PRINT VALVE ENTRIES [{}]\n", suffix)
    for valve_name,valve_data in valve_map {
        fmt.printf("Valve {} has flow rate={}; tunnel(s) lead to valve(s) ", valve_name, valve_data.flow_rate_ppm)
        for dest,i in valve_data.dest_valves {
            if i != 0 do fmt.printf(", ")
            fmt.printf("{}", dest)
        }
        fmt.print("\n")
    }
}

find_max_possible_releasable_pressure :: proc(valve_map: map[string]valve_data_t, min_remaining: int) -> int {
    valves_open_map := make(map[string]bool)
    defer delete(valves_open_map)

    return find_max_possible_releasable_pressure_dfs_helper("AA", valve_map, valves_open_map, min_remaining)
}

find_max_possible_releasable_pressure_dfs_helper :: proc(
    current_valve_name: string,
    valve_map: map[string]valve_data_t,
    valves_open_map: map[string]bool,
    min_remaining: int
) -> int
{
    valves_open_map := valves_open_map // explicitly shadow the open_map to allow for mutation
    if min_remaining <= 0 {
        return 0
    }

    current_valve, ok := valve_map[current_valve_name]
    assert(ok)

    cumulative_pressures := make([dynamic]int, 0, 10)
    defer delete(cumulative_pressures)

    // Regardless of whether we open this valve or not, we still have to check what happens if we didn't open this
    // valve and just moved on to one of the next valves. So calculate that unconditionally first
    for dest_valve in current_valve.dest_valves {
        cumulative_pressure := find_max_possible_releasable_pressure_dfs_helper(dest_valve, valve_map, valves_open_map, min_remaining-1)
        append(&cumulative_pressures, cumulative_pressure)
    }

    if (current_valve.flow_rate_ppm > 0 && !valves_open_map[current_valve_name]) {
        // if the valve wasn't open and has a flow rate, we also want to check what happens if we do open it.
        valves_open_map[current_valve_name] = true

        // It takes a minute to open the curent valve
        current_valve_cumulative_pressure := (min_remaining-1)*current_valve.flow_rate_ppm
        for dest_valve in current_valve.dest_valves {
            cumulative_pressure := current_valve_cumulative_pressure +
                find_max_possible_releasable_pressure_dfs_helper(dest_valve, valve_map, valves_open_map, min_remaining-2)
            append(&cumulative_pressures, cumulative_pressure)
        }
        // reset the valve state afterwards
        valves_open_map[current_valve_name] = false
    }

    max_pressure := 0
    for p in cumulative_pressures {
        max_pressure = max(max_pressure, p)
    }

    return max_pressure
}
