package main

import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:math/bits"

coord_t :: struct {
    row: int,
    col: int,
}

grid_t :: struct {
    cells: []i8,
    width: int,
    height: int,
    start: coord_t,
    end: coord_t,
}

main :: proc() {
    simple_input_file_contents := string(#load("day12_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day12_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day12_solve("simple", simple_input_lines[:len(simple_input_lines)-1])
    day12_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day12_solve :: proc(title: string, input_lines: []string) {
    grid := parse_grid_from_input(input_lines)
    // FIXME: rather than delete grid.cells, use a single allocation and just `free(grid)` or something
    defer delete(grid.cells)

    // fmt.println("GRID")
    // for row in 0 ..< grid.height {
    //     for col in 0 ..< grid.width {
    //         fmt.printf("%c",grid.cells[row*grid.width + col] + 'a')
    //     }
    //     fmt.print('\n')
    // }

    shortest_path_len, found := calculate_shortest_pathlen_to_end(grid)
    assert(found)
    fmt.printf("[%s] pt1 shortest path... %d\n", title, shortest_path_len)

    shortest_path_start := grid.start
    shortest_path_from_any_bottom_point := shortest_path_len
    for r in 0..< grid.height {
        for c in 0..< grid.width {
            // don't recalculate the path from the original start
            if (coord_t{r,c}) == grid.start do continue

            // we're only interested in browsing other starting points that start at elevation 0
            if grid.cells[r * grid.width + c] != 0 do continue

            grid_with_new_start := grid
            grid_with_new_start.start = coord_t{r,c}
            candidate_shortest_path_len, found := calculate_shortest_pathlen_to_end(grid_with_new_start)
            if found && candidate_shortest_path_len < shortest_path_from_any_bottom_point {
                shortest_path_from_any_bottom_point = candidate_shortest_path_len
                shortest_path_start = grid_with_new_start.start
            }
        }
    }

    fmt.printf("[{}] pt2 shortest path from any start... {} from {{row={},col={}}}\n",
        title, shortest_path_from_any_bottom_point, shortest_path_start.row, shortest_path_start.col)
}

parse_grid_from_input :: proc(input_lines: []string) -> grid_t {
    if len(input_lines) == 0 {
        return grid_t { cells=make([]i8, 0), width=0, height=0, start={}, end={} }
    }

    width := len(input_lines[0])
    height := len(input_lines)
    cells := make([]i8, width * height)
    start := coord_t { 0, 0 }
    end := coord_t { 0, 0 }

    for line,row in input_lines {
        for chr,col in line {
            elevation: i8
            switch chr {
                case 'S': {
                    start = coord_t { row, col }
                    elevation = 'a'
                }
                case 'E': {
                    end = coord_t { row, col }
                    elevation = 'z'
                }
                case: {
                    elevation = cast(i8)chr
                }
            }

            cells[row * width + col] = (elevation - 'a')
        }
    }

    return grid_t { cells , width, height, start, end }
}

calculate_shortest_pathlen_to_end :: proc(grid: grid_t) -> (uint, bool) {
    cell_count := grid.width * grid.height
    cell_visited := make([]b8, cell_count, context.temp_allocator)
    distances := make([]u32, cell_count, context.temp_allocator)
    defer free_all(context.temp_allocator)

    // We'll do dijkstra's here

    // 1. mark all nodes unvisited
    for i in 0 ..< cell_count {
        cell_visited[i] = false
    }

    // 2. set all distances to MAX (replacement for infinity) except the start which is 0
    for i in 0 ..< cell_count {
        distances[i] = 0xFFFFFFFF // FIXME: where's the u32_max
    }
    distances[grid.start.row * grid.width + grid.start.col] = 0

    shortest_path_len: u32 = 0xFFFFFFFF // FIXME: where's the u32 max
    shortest_path_found := false

    current_node := grid.start
    for {
        neighbor_candidates := [4]coord_t{
            coord_t { current_node.row - 1, current_node.col, },
            coord_t { current_node.row + 1, current_node.col, },
            coord_t { current_node.row,     current_node.col - 1, },
            coord_t { current_node.row,     current_node.col + 1, },
        }

        for candidate in neighbor_candidates {
            // ignore invalid neighbors
            if !is_valid_neighbor(current_node, candidate, grid) do continue

            // ignore already visited neighbors
            if cell_visited[candidate.row * grid.width + candidate.col] do continue

            neighbor := candidate
            current_node_dist := &distances[current_node.row * grid.width + current_node.col]
            neighbor_dist := &distances[neighbor.row * grid.width + neighbor.col]

            neighbor_dist^ = min(neighbor_dist^, current_node_dist^ + 1)
        }

        // mark current_node visited
        cell_visited[current_node.row * grid.width + current_node.col] = true

        if current_node == grid.end {
            shortest_path_len = distances[grid.end.row * grid.width + grid.end.col]
            shortest_path_found = true
            break
        }

        found_next_node := false
        next_node: coord_t

        for row in 0 ..< grid.height {
            for col in 0 ..< grid.width {
                if !cell_visited[row * grid.width + col] && distances[row * grid.width + col] < 0xFFFFFFFF {
                    if !found_next_node {
                        found_next_node = true
                        next_node = coord_t { row, col }
                    }
                    else if distances[next_node.row * grid.width + next_node.col] > distances[row * grid.width + col] {
                        next_node = coord_t { row, col }
                    }
                }
            }
        }

        if !found_next_node {
            shortest_path_found = false
            break
        }

        current_node = next_node
    }


    return cast(uint)shortest_path_len, shortest_path_found
}

is_valid_neighbor :: proc(node: coord_t, neighbor: coord_t, grid: grid_t) -> bool {
    node_index := node.row * grid.width + node.col
    neighbor_index := neighbor.row * grid.width + neighbor.col

    return neighbor.row >= 0 && neighbor.row < grid.height &&
        neighbor.col >= 0 && neighbor.col < grid.width &&
        grid.cells[neighbor_index] <= (grid.cells[node_index] + 1)

}

dump_grid_stats :: proc(grid: grid_t, distances: []u32, visited: []b8, current_node: coord_t) {
    fmt.println("GRID DISTANCES")
    for row in 0 ..< grid.height {
        for col in 0 ..< grid.width {
            c: i8
            switch {
                case row == grid.start.row && col == grid.start.col: c = 'S'
                case row == grid.end.row && col == grid.end.col: c = 'E'
                case: c = grid.cells[row*grid.width + col] + 'a'
            }

            visited_char: i8
            if (coord_t {row, col}) == current_node {
                visited_char = 'c'
            } else {
                visited_char = 'v' if visited[row*grid.width + col] else 'u'
            }

            d := distances[row*grid.width + col]
            switch d {
                case 0xFFFFFFFF: fmt.printf("[  %c/%c] ", visited_char, c)
                case : fmt.printf("[%02d%c/%c] ", d, visited_char, c)
            }
        }
        fmt.print('\n')
    }
}
