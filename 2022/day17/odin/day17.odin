package main

import "core:fmt"
import "core:strings"
import "core:strconv"

jet_t :: enum {
    left,
    right,
}

rock_t :: enum {
    flat_piece, // ####

    plus_piece, //  #
                // ###
                //  #

    elbow_piece, //   #
                 //   #
                 // ###

    vert_piece, // #
                // #
                // #
                // #

    box_piece, // ##
               // ##
}

main :: proc() {
    simple_input_file_contents := string(#load("day17_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day17_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day17_solve("simple", simple_input_lines[:len(simple_input_lines)-1])
    day17_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day17_solve :: proc(title: string, input_lines: []string) {
    jets := get_jet_pattern_from_input(input_lines)
    defer delete(jets)

    rock_pattern := []rock_t{ rock_t.flat_piece, rock_t.plus_piece, rock_t.elbow_piece, rock_t.vert_piece, rock_t.box_piece }
    tower_height := simulate_rocks_and_jets(rock_pattern[:], jets, 2022)
    // FIXME:
    // defer delete(board.cells)
    // tower_height := find_tower_height(board)
    fmt.printf("[{} pt1] Tower height={}\n", title, tower_height)
}

get_jet_pattern_from_input :: proc(input_lines: []string) -> []jet_t {
    assert(len(input_lines) == 1)
    jets := make([dynamic]jet_t)
    for c in input_lines[0] {
        switch c {
            case '>': append(&jets, jet_t.right)
            case '<': append(&jets, jet_t.left)
            case: assert(false) // bad input
        }
    }
    return jets[:]
}

simulate_rocks_and_jets :: proc(rock_pattern: []rock_t, jets: []jet_t, rock_count: int) -> int {
    cells := make([dynamic]bool)
    board_width := 7
    board_height := 0
    highest_rock_height := -1

    next_rock_index := 0
    next_jet_index := 0

    for i in 0 ..< rock_count {
        falling_rock := rock_pattern[next_rock_index]
        next_rock_index = (next_rock_index + 1) % len(rock_pattern)

        required_height := highest_rock_height + 4 + get_rock_height(falling_rock)

        // expand the board height if necessary
        if required_height > board_height {
            missing_height := (required_height - board_height)
            expand_cell_count := missing_height * board_width
            for i in 0 ..< expand_cell_count do append(&cells, false)
            board_height = required_height
        }

        // pos is set at the top-left corner of the piece
        falling_rock_pos := vec2 { 2, required_height - 1}

        // simulate next rock
        for simstep := 0; true; simstep += 1 {
            // { //FIXME: debug
            //     fmt.printf("Board @ rock=#{},simstep=#{}\n", i, simstep)

            //     falling_rock_pos_buffer, position_count := get_rock_cells(falling_rock_pos, falling_rock)
            //     falling_rock_positions := falling_rock_pos_buffer[:position_count]

            //     for r := board_height-1; r >= 0; r -= 1{
            //         fmt.print("|")
            //         for c in 0 ..< board_width {
            //             is_cell_falling_rock: bool
            //             for p in falling_rock_positions {
            //                 if p == (vec2{c, r}) {
            //                     is_cell_falling_rock = true
            //                     break
            //                 }
            //             }

            //             if is_cell_falling_rock {
            //                 fmt.print('@')
            //             } else if cells[r * board_width + c] {
            //                 fmt.print('#')
            //             } else {
            //                 fmt.print('.')
            //             }
            //         }
            //         fmt.print("|\n")
            //     }

            //     fmt.print("+")

            //     for c in 0 ..< board_width {
            //         fmt.print('-')
            //     }

            //     fmt.print("+\n")
            // }


            // simulate the next jet motion first
            {
                jet := jets[next_jet_index]
                next_jet_index = (next_jet_index + 1) % len(jets)

                jet_movement: vec2
                switch jet {
                    case jet_t.left: jet_movement = vec2{-1, 0}
                    case jet_t.right: jet_movement = vec2{1, 0}
                }

                simulated_pos := vec2_add(falling_rock_pos, jet_movement)
                if is_rock_allowed_in_board(simulated_pos, falling_rock, cells[:], board_width, board_height) {
                    falling_rock_pos = simulated_pos
                }
            }

            // simulate falling second
            has_stopped: bool = false
            {
                simulated_pos := vec2_add(falling_rock_pos, vec2{0,-1})
                if is_rock_allowed_in_board(simulated_pos, falling_rock, cells[:], board_width, board_height) {
                    falling_rock_pos = simulated_pos
                } else {
                    has_stopped = true
                    // FIXME(debug): fmt.printf("rock {} stopped @ {},{}\n", i, falling_rock_pos.x, falling_rock_pos.y)
                }
            }

            if has_stopped {
                // set the stopped rock
                positions_to_check_buffer, position_count := get_rock_cells(falling_rock_pos, falling_rock)
                for pos in positions_to_check_buffer[:position_count] {
                    pos_index := pos.y * board_width + pos.x
                    assert(cells[pos_index] == false)
                    cells[pos_index] = true
                }

                highest_rock_height = max(highest_rock_height, falling_rock_pos.y)
                break
            }
        }
    }

    // { //FIXME: debug
    //     fmt.printf("Board after simuation\n")

    //     for r := board_height-1; r >= 0; r -= 1{
    //         fmt.print("|")
    //         for c in 0 ..< board_width {
    //             if cells[r * board_width + c] {
    //                 fmt.print('#')
    //             } else {
    //                 fmt.print('.')
    //             }
    //         }
    //         fmt.print("|\n")
    //     }

    //     fmt.print("+")

    //     for c in 0 ..< board_width {
    //         fmt.print('-')
    //     }

    //     fmt.print("+\n")
    // }

    return highest_rock_height + 1
}

// find_tower_height :: proc(board: board_t) -> int {
//     for row := board.height - 1; row >= 0; row -= 1 {
//         for col in 0 ..< board.width {
//             if board.cells[row * board.width + col] {
//                 return (row + 1)
//             }
//         }
//     }
//
//     return 0
// }
//
get_rock_height :: proc(rock: rock_t) -> int {
    switch (rock) {
        case rock_t.flat_piece: return 1
        case rock_t.plus_piece: return 3
        case rock_t.elbow_piece: return 3
        case rock_t.vert_piece: return 4
        case rock_t.box_piece: return 2
        case: {
            assert(false)
            return 0
        }
    }
}

get_rock_width :: proc(rock: rock_t) -> int {
    switch (rock) {
        case rock_t.flat_piece: return 4
        case rock_t.plus_piece: return 3
        case rock_t.elbow_piece: return 3
        case rock_t.vert_piece: return 1
        case rock_t.box_piece: return 2
        case: {
            assert(false)
            return 0
        }
    }
}

is_rock_allowed_in_board :: proc(rock_pos: vec2, rock_type: rock_t, board_cells: []bool, board_width, board_height: int) -> bool {
    if rock_pos.x < 0 {
        return false
    }

    if rock_pos.x + get_rock_width(rock_type) - 1 >= board_width {
        return false
    }

    if rock_pos.y - get_rock_height(rock_type) + 1 < 0 {
        return false
    }

    positions_to_check_buffer, position_count := get_rock_cells(rock_pos, rock_type)
    for pos in positions_to_check_buffer[:position_count] {
        if board_cells[pos.y * board_width + pos.x] {
            return false
        }
    }

    return true
}

get_rock_cells :: proc(rock_pos: vec2, rock_type: rock_t) -> ([5]vec2, int) {
    positions_to_check_buffer: [5]vec2
    filled_positions: int
    switch rock_type {
        case rock_t.flat_piece: {
            positions_to_check_buffer[0] = vec2_add(rock_pos, vec2{0,0})
            positions_to_check_buffer[1] = vec2_add(rock_pos, vec2{1,0})
            positions_to_check_buffer[2] = vec2_add(rock_pos, vec2{2,0})
            positions_to_check_buffer[3] = vec2_add(rock_pos, vec2{3,0})
            filled_positions = 4
        }
        case rock_t.plus_piece: {
            positions_to_check_buffer[0] = vec2_add(rock_pos, vec2{1,0})
            positions_to_check_buffer[1] = vec2_add(rock_pos, vec2{0,-1})
            positions_to_check_buffer[2] = vec2_add(rock_pos, vec2{1,-1})
            positions_to_check_buffer[3] = vec2_add(rock_pos, vec2{2,-1})
            positions_to_check_buffer[4] = vec2_add(rock_pos, vec2{1,-2})
            filled_positions = 5
        }
        case rock_t.elbow_piece: {
            positions_to_check_buffer[0] = vec2_add(rock_pos, vec2{2,0})
            positions_to_check_buffer[1] = vec2_add(rock_pos, vec2{2,-1})
            positions_to_check_buffer[2] = vec2_add(rock_pos, vec2{0,-2})
            positions_to_check_buffer[3] = vec2_add(rock_pos, vec2{1,-2})
            positions_to_check_buffer[4] = vec2_add(rock_pos, vec2{2,-2})
            filled_positions = 5
        }
        case rock_t.vert_piece: {
            positions_to_check_buffer[0] = vec2_add(rock_pos, vec2{0,0})
            positions_to_check_buffer[1] = vec2_add(rock_pos, vec2{0,-1})
            positions_to_check_buffer[2] = vec2_add(rock_pos, vec2{0,-2})
            positions_to_check_buffer[3] = vec2_add(rock_pos, vec2{0,-3})
            filled_positions = 4
        }
        case rock_t.box_piece: {
            positions_to_check_buffer[0] = vec2_add(rock_pos, vec2{0,0})
            positions_to_check_buffer[1] = vec2_add(rock_pos, vec2{1,0})
            positions_to_check_buffer[2] = vec2_add(rock_pos, vec2{0,-1})
            positions_to_check_buffer[3] = vec2_add(rock_pos, vec2{1,-1})
            filled_positions = 4
        }
        case: {
            assert(false)
            filled_positions = 0
        }
    }

    return positions_to_check_buffer, filled_positions
}

// FIXME:
// print_board_state :: proc(
//     falling_rock: Maybe(rock_type),
//     falling_rock_pos: Maybe(vec2),
//     cells: []bool,
//     board_width, board_height: int
//     )
// {
// }
