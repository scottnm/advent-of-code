package main

import "core:fmt"
import "core:strings"
import "core:strconv"

vec2 :: struct {
    x: int,
    y: int,
}

rock_path_t :: struct {
    path_points: []vec2,
}

cell_type_t :: enum u8 {
    sand = 'o',
    air = '.',
    rock = '#',
}

grid_t :: struct {
    cells: []cell_type_t,
    width: int,
    height: int,
    origin: vec2,
}

main :: proc() {
    simple_input_file_contents := string(#load("day14_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day14_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day14_solve("simple", simple_input_lines[:len(simple_input_lines)-1])
    day14_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day14_solve :: proc(title: string, input_lines: []string) {
    // set the main allocator to be the temp allocator and just free all memory at the end of this function
    context.allocator = context.temp_allocator
    defer free_all(context.temp_allocator)

    rock_paths := read_rock_paths_from_input(input_lines)
    sand_origin := vec2 {500, 0}
    {
        grid := create_grid_from_rock_paths_pt1(sand_origin, rock_paths)
        simulate_sand_flowing_through_grid_until_stable_pt1(sand_origin, grid)
        sand_count := count_grains_of_sand(grid)
        fmt.printf("[{}] pt1: stable sand unit count = {}\n", title, sand_count)
    }
    {
        grid := create_grid_from_rock_paths_pt2(sand_origin, rock_paths)
        simulate_sand_flowing_through_grid_until_stable_pt2(sand_origin, grid)
        sand_count := count_grains_of_sand(grid)
        fmt.printf("[{}] pt2: stable sand unit count = {}\n", title, sand_count)
    }
}

read_rock_paths_from_input :: proc(input_lines: []string) -> []rock_path_t {
    paths := make([]rock_path_t, len(input_lines))
    for line,i in input_lines {
        path_points := make([dynamic]vec2)

        l := line // have to get a copy because I can't use split_iterator on the foroloop var
        for path_point_str in strings.split_iterator(&l, " -> ") {
            path_point_str_parts := strings.split(path_point_str, ",")
            assert(len(path_point_str_parts) == 2)

            x := strconv.atoi(path_point_str_parts[0])
            y := strconv.atoi(path_point_str_parts[1])
            append(&path_points, vec2{x, y})
        }
        paths[i].path_points = path_points[:]
    }
    return paths
}

create_grid_from_rock_paths_pt1 :: proc(sand_origin: vec2, rock_paths: []rock_path_t) -> grid_t {
    min_xy := sand_origin
    max_xy := sand_origin

    // first calculate the corners of the grid so we can allocate it and know how to offset
    // the points in our rock path
    for rock_path in rock_paths {
        for path_point in rock_path.path_points {
            min_xy.x = min(path_point.x, min_xy.x)
            min_xy.y = min(path_point.y, min_xy.y)
            max_xy.x = max(path_point.x, max_xy.x)
            max_xy.y = max(path_point.y, max_xy.y)
        }
    }

    assert(max_xy.x >= min_xy.x)
    assert(max_xy.y >= min_xy.y)

    width := max_xy.x - min_xy.x + 1
    height := max_xy.y - min_xy.y + 1
    cell_count := (width * height)

    cells := make([]cell_type_t, cell_count)

    // initialize every cell to air first
    for i in 0 ..< cell_count {
        cells[i] = cell_type_t.air
    }

    // set the rock cells
    for rock_path in rock_paths {
        assert(len(rock_path.path_points) > 0)
        for i in 1..< len(rock_path.path_points) {
            start_point := rock_path.path_points[i-1]
            end_point   := rock_path.path_points[i]

            path_vec := vec2_sub(end_point, start_point)
            start_offset := vec2_sub(start_point, min_xy)

            assert((path_vec.x == 0) != (path_vec.y == 0)) // exactly one of the x or y translation should be zero
            if (path_vec.x == 0)
            {
                step := 1 if path_vec.y > 0 else -1
                for ydiff := 0; ydiff != path_vec.y; ydiff += step {
                    rock_path_point := vec2_add(start_offset, vec2{0, ydiff})
                    assert(vec2_in_bounds(rock_path_point, vec2{0,0}, vec2{width,height}))
                    cells[rock_path_point.y * width + rock_path_point.x] = cell_type_t.rock
                }
            } else if (path_vec.y == 0) {
                step := 1 if path_vec.x > 0 else -1
                for xdiff := 0; xdiff != path_vec.x; xdiff += step {
                    rock_path_point := vec2_add(start_offset, vec2{xdiff, 0})
                    assert(vec2_in_bounds(rock_path_point, vec2{0,0}, vec2{width,height}))
                    cells[rock_path_point.y * width + rock_path_point.x] = cell_type_t.rock
                }
            }
        }

        // fence post problem, still need to explicitly set the end path point
        end_path_point := rock_path.path_points[len(rock_path.path_points) - 1]
        offset_end_path_point := vec2_sub(end_path_point, min_xy)
        assert(vec2_in_bounds(offset_end_path_point, vec2{0,0}, vec2{width,height}))
        cells[offset_end_path_point.y * width + offset_end_path_point.x] = cell_type_t.rock
    }

    assert(cells[(sand_origin.y-min_xy.y) * width + (sand_origin.x-min_xy.x)] == cell_type_t.air)
    return grid_t{ cells, width, height, min_xy }
}

create_grid_from_rock_paths_pt2 :: proc(sand_origin: vec2, rock_paths: []rock_path_t) -> grid_t {
    min_xy := sand_origin
    max_xy := sand_origin

    // first calculate the corners of the grid so we can allocate it and know how to offset
    // the points in our rock path
    for rock_path in rock_paths {
        for path_point in rock_path.path_points {
            min_xy.x = min(path_point.x, min_xy.x)
            min_xy.y = min(path_point.y, min_xy.y)
            max_xy.x = max(path_point.x, max_xy.x)
            max_xy.y = max(path_point.y, max_xy.y)
        }
    }

    // the floor is 2 away from the highest rock coordinate we found
    max_xy.y += 2

    // HACK!!
    // expand the width 1000 on each side to allow for sand going off-grid
    // this size is a bit gratuitous but I don't feel like rewriting everything
    // to account for an infinite floor and an extra 2000 is enough to brute
    // force the problem :)
    max_xy.x += 1000
    min_xy.x -= 1000

    assert(max_xy.x >= min_xy.x)
    assert(max_xy.y >= min_xy.y)

    width := max_xy.x - min_xy.x + 1
    height := max_xy.y - min_xy.y + 1
    cell_count := (width * height)

    cells := make([]cell_type_t, cell_count)

    // initialize every cell to air first
    for i in 0 ..< cell_count {
        cells[i] = cell_type_t.air
    }

    // initialize the floor
    for i in 0 ..< width {
        cells[max_xy.y * width + i] = cell_type_t.rock
    }

    // set the rock cells
    for rock_path in rock_paths {
        assert(len(rock_path.path_points) > 0)
        for i in 1..< len(rock_path.path_points) {
            start_point := rock_path.path_points[i-1]
            end_point   := rock_path.path_points[i]

            path_vec := vec2_sub(end_point, start_point)
            start_offset := vec2_sub(start_point, min_xy)

            assert((path_vec.x == 0) != (path_vec.y == 0)) // exactly one of the x or y translation should be zero
            if (path_vec.x == 0)
            {
                step := 1 if path_vec.y > 0 else -1
                for ydiff := 0; ydiff != path_vec.y; ydiff += step {
                    rock_path_point := vec2_add(start_offset, vec2{0, ydiff})
                    assert(vec2_in_bounds(rock_path_point, vec2{0,0}, vec2{width,height}))
                    cells[rock_path_point.y * width + rock_path_point.x] = cell_type_t.rock
                }
            } else if (path_vec.y == 0) {
                step := 1 if path_vec.x > 0 else -1
                for xdiff := 0; xdiff != path_vec.x; xdiff += step {
                    rock_path_point := vec2_add(start_offset, vec2{xdiff, 0})
                    assert(vec2_in_bounds(rock_path_point, vec2{0,0}, vec2{width,height}))
                    cells[rock_path_point.y * width + rock_path_point.x] = cell_type_t.rock
                }
            }
        }

        // fence post problem, still need to explicitly set the end path point
        end_path_point := rock_path.path_points[len(rock_path.path_points) - 1]
        offset_end_path_point := vec2_sub(end_path_point, min_xy)
        assert(vec2_in_bounds(offset_end_path_point, vec2{0,0}, vec2{width,height}))
        cells[offset_end_path_point.y * width + offset_end_path_point.x] = cell_type_t.rock
    }

    assert(cells[(sand_origin.y-min_xy.y) * width + (sand_origin.x-min_xy.x)] == cell_type_t.air)
    return grid_t{ cells, width, height, min_xy }
}

simulate_sand_flowing_through_grid_until_stable_pt1 :: proc(sand_origin: vec2, grid: grid_t) {
    sand_offset_origin := vec2_sub(sand_origin, grid.origin)
    for {
        sand_landed := false
        falling_sand_pos := sand_offset_origin

        for {
            new_pos_down := vec2_add(falling_sand_pos, vec2{0,1})
            new_pos_downleft := vec2_add(falling_sand_pos, vec2{-1,1})
            new_pos_downright := vec2_add(falling_sand_pos, vec2{1,1})

            cell_down := get_grid_cell(grid, new_pos_down)
            cell_downleft := get_grid_cell(grid, new_pos_downleft)
            cell_downright := get_grid_cell(grid, new_pos_downright)

            // First check if we can move the sand down into a valid cell
            if cell_type,ok := cell_down.?; ok && cell_type == cell_type_t.air {
                falling_sand_pos = new_pos_down
                continue
            }

            if cell_type,ok := cell_downleft.?; ok && cell_type == cell_type_t.air {
                falling_sand_pos = new_pos_downleft
                continue
            }

            if cell_type,ok := cell_downright.?; ok && cell_type == cell_type_t.air {
                falling_sand_pos = new_pos_downright
                continue
            }

            // If we couldn't move the sand down into a valid cell, check if it's
            // because the next cell we'd move into is out-of-bounds and hence in
            // the void.
            if _,ok := cell_down.?; !ok {
                break
            }

            if _,ok := cell_downleft.?; !ok {
                break
            }

            if _,ok := cell_downright.?; !ok {
                break
            }

            // Else the sand landed. Set it in the grid.
            grid.cells[falling_sand_pos.y * grid.width + falling_sand_pos.x] = cell_type_t.sand
            sand_landed = true
            break
        }

        if !sand_landed {
            break
        }
    }
}

simulate_sand_flowing_through_grid_until_stable_pt2 :: proc(sand_origin: vec2, grid: grid_t) {
    sand_offset_origin := vec2_sub(sand_origin, grid.origin)
    sand_spawn_plugged := false
    for !sand_spawn_plugged {

        falling_sand_pos := sand_offset_origin
        for {
            new_pos_down := vec2_add(falling_sand_pos, vec2{0,1})
            new_pos_downleft := vec2_add(falling_sand_pos, vec2{-1,1})
            new_pos_downright := vec2_add(falling_sand_pos, vec2{1,1})

            cell_down := get_grid_cell(grid, new_pos_down)
            cell_downleft := get_grid_cell(grid, new_pos_downleft)
            cell_downright := get_grid_cell(grid, new_pos_downright)

            // HACK! for ease, let's see if we can just assert that no sand will ever go off-grid
            // There's a matching hack in create_grid_from_rock_paths_pt2 where I cheat and avoid worrying about
            // infinite floors and going off a fixed grid by just making the grid really wide.
            if _,ok := cell_down.?; !ok do assert(false, "down position is off-grid")
            if _,ok := cell_downleft.?; !ok do assert(false, "downleft position is off-grid")
            if _,ok := cell_downright.?; !ok do assert(false, "downright position is off-grid")

            // First check if we can move the sand down into a valid cell
            if cell_type,ok := cell_down.?; ok && cell_type == cell_type_t.air {
                falling_sand_pos = new_pos_down
                continue
            }

            if cell_type,ok := cell_downleft.?; ok && cell_type == cell_type_t.air {
                falling_sand_pos = new_pos_downleft
                continue
            }

            if cell_type,ok := cell_downright.?; ok && cell_type == cell_type_t.air {
                falling_sand_pos = new_pos_downright
                continue
            }

            // Else the sand landed. Set it in the grid.
            grid.cells[falling_sand_pos.y * grid.width + falling_sand_pos.x] = cell_type_t.sand

            // if we just placed sand where the sand spawns, no more sand will flow
            sand_spawn_plugged = (falling_sand_pos == sand_offset_origin)
            break
        }
    }
}

count_grains_of_sand :: proc(grid: grid_t) -> uint {
    sand_count: uint = 0
    for c in grid.cells {
        if (c == cell_type_t.sand) {
            sand_count += 1
        }
    }
    return sand_count
}

print_rock_paths :: proc(rock_paths: []rock_path_t) {
    for rock_path,i in rock_paths {
        fmt.printf("{}: ", i)
        for point,j in rock_path.path_points {
            if j != 0 do fmt.print(" -> ")
            fmt.printf("{},{}", point.x, point.y)
        }
        fmt.print("\n")
    }
}

print_grid :: proc(grid: grid_t) {
    fmt.printf("size = {}x{}\n", grid.width, grid.width)
    fmt.printf("origin = {},{}\n", grid.origin.x, grid.origin.y)
    for y in 0 ..< grid.height {
        for x in 0 ..< grid.width {
            fmt.printf("%c", cast(u8)grid.cells[y * grid.width + x])
        }
        fmt.print("\n")
    }
}

get_grid_cell :: proc(grid: grid_t, pos: vec2) -> Maybe(cell_type_t) {
    if !vec2_in_bounds(pos, vec2{0,0}, vec2{grid.width, grid.height}) {
        return nil
    }

    return grid.cells[pos.y * grid.width + pos.x]
}

vec2_add :: proc(lhs: vec2, rhs: vec2) -> vec2 {
    x := lhs.x + rhs.x
    y := lhs.y + rhs.y
    return vec2 {x,y}
}

vec2_sub :: proc(lhs: vec2, rhs: vec2) -> vec2 {
    x := lhs.x - rhs.x
    y := lhs.y - rhs.y
    return vec2 {x,y}
}

vec2_in_bounds :: proc(v: vec2, lower_bound: vec2, upper_bound: vec2) -> bool {
        return (v.x >= lower_bound.x) &&
               (v.x < upper_bound.x) &&
               (v.y >= lower_bound.y) &&
               (v.y < upper_bound.y)
}
