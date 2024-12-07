use input_helpers;
use std::{hash::{Hash, Hasher}, process::ExitCode};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Obstacle,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct GridPos {
    row: isize,
    col: isize,
}

impl Hash for GridPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.row, self.col).hash(state);
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn get_cell_idx(&self, row: isize, col: isize) -> usize {
        assert!(!self.is_pos_out_of_bounds(row, col));

        (row as usize * self.width) + (col as usize)
    }

    fn get_cell(&self, row: isize, col: isize) -> Cell {
        self.cells[self.get_cell_idx(row, col)]
    }

    fn get_cell_mut(&mut self, row: isize, col: isize) -> &mut Cell {
        let idx = self.get_cell_idx(row, col);
        &mut self.cells[idx]
    }

    fn cell_pos_from_idx(width: usize, height: usize, idx: usize) -> GridPos {
        assert!(idx < (width * height));
        let col = (idx % width) as isize;
        let row = (idx / width) as isize;
        GridPos{row, col}
    }

    fn is_pos_out_of_bounds(&self, row: isize, col: isize) -> bool {
        row < 0 ||
        col < 0 ||
        row as usize >= self.height ||
        col as usize >= self.width
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct PlayerState {
    pos: GridPos,
    dir: Direction,
}

impl Hash for PlayerState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.pos.row, self.pos.col, self.dir).hash(state);
    }
}

fn read_starting_board_state(filename: &str) -> Result<(Grid, PlayerState), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() == 0 {
        return Err(format!("Invalid empty board state"));
    }

    let height = lines.len();
    let width = lines[0].len();

    let mut cells: Vec<Cell> = vec![];
    let mut player_state: Option<PlayerState> = None;
    for line in lines {
        if line.len() != width {
            return Err(format!(
                "Grid must have consistent line widths! Expected {} found {}",
                width,
                line.len()
            ));
        }

        for c in line.chars() {
            match c {
                '.' => cells.push(Cell::Empty),
                '#' => cells.push(Cell::Obstacle),
                '^' | '>' | 'v' | '<' => {
                    if player_state.is_none() {
                        let player_pos = Grid::cell_pos_from_idx(width, height, cells.len());
                        let player_dir = match c {
                            '^' => Direction::Up,
                            '>' => Direction::Right,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            // FIXME: anyway to better specify this?
                            _ => unreachable!("Match arm protects against other values"),
                        };
                        player_state = Some(PlayerState{pos:player_pos, dir: player_dir});
                        cells.push(Cell::Empty);
                    } else {
                        return Err(format!("Duplicate player position"));
                    }
                },
                _ => return Err(format!("Invalid cell '{}'", c)),
            }
        }
    }

    if let Some(player_state) = player_state {
        Ok((Grid {width, height, cells}, player_state))
    } else {
        Err(format!("Did not find player position in input"))
    }
}

fn simulate_board_step(grid: &Grid, player_state: &PlayerState) -> PlayerState {
    let next_player_position = match player_state.dir {
        Direction::Up => GridPos {row: player_state.pos.row - 1, col: player_state.pos.col },
        Direction::Right => GridPos {row: player_state.pos.row, col: player_state.pos.col + 1 },
        Direction::Down => GridPos {row: player_state.pos.row + 1, col: player_state.pos.col },
        Direction::Left => GridPos {row: player_state.pos.row, col: player_state.pos.col - 1 },
    };

    if grid.is_pos_out_of_bounds(next_player_position.row, next_player_position.col) {
        PlayerState {pos: next_player_position, dir: player_state.dir }
    } 
    else {
        match grid.get_cell(next_player_position.row, next_player_position.col) {
            Cell::Empty => {
                PlayerState {pos: next_player_position, dir: player_state.dir }
            },
            Cell::Obstacle => {
                let next_player_dir = match player_state.dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
                PlayerState {pos: player_state.pos, dir: next_player_dir }
            },
        }
    }
}

#[allow(dead_code)]
fn print_board_state(grid: &Grid, player_state: &PlayerState) {
    for r in 0..(grid.height as isize) {
        for c in 0..(grid.width as isize) {
            if r == player_state.pos.row && c == player_state.pos.col {
                let player_char = match player_state.dir {
                    Direction::Up => '^',
                    Direction::Right => '>',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                };
                print!("{}", player_char);
            } else {
                let cell_char = match grid.get_cell(r, c) {
                    Cell::Empty => '.',
                    Cell::Obstacle => '#',
                };
                print!("{}", cell_char);
            }
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn print_move_history(grid: &Grid, move_history: &std::collections::HashSet<GridPos>) {
    for r in 0..(grid.height as isize) {
        for c in 0..(grid.width as isize) {
            if move_history.contains(&GridPos{row: r, col: c}) {
                print!("X");
            } else {
                let cell_char = match grid.get_cell(r, c) {
                    Cell::Empty => '.',
                    Cell::Obstacle => '#',
                };
                print!("{}", cell_char);
            }
        }
        println!();
    }
    println!();
}

fn test_for_simulation_loop(grid: &Grid, player_initial_state: &PlayerState) -> bool {

    let mut player_state = player_initial_state.clone();
    let mut player_state_history = std::collections::HashSet::new();

    //print_board_state(&grid, &player_state);

    while !grid.is_pos_out_of_bounds(player_state.pos.row, player_state.pos.col) {
        if player_state_history.contains(&player_state) {
            return true;
        }

        player_state_history.insert(player_state);
        player_state = simulate_board_step(&grid, &player_state);
        //print_board_state(&grid, &player_state);
    }

    false
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_starting_board_state(filename);
    let (grid, mut player_state) = match parse_result {
        Ok(board_state) => board_state,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    let player_initial_state = player_state.clone();

    let pt1_start_time = std::time::Instant::now();

    let mut player_space_history = std::collections::HashSet::new();

    //print_board_state(&grid, &player_state);

    loop {
        player_space_history.insert(player_state.pos);

        player_state = simulate_board_step(&grid, &player_state);
        //print_board_state(&grid, &player_state);

        if grid.is_pos_out_of_bounds(player_state.pos.row, player_state.pos.col) {
            break;
        }
    }

    let pt1_time = pt1_start_time.elapsed();

    println!(
        "{} unique player positions",
        player_space_history.len()
    );

    //print_move_history(&grid, &player_space_history);

    println!("TIME: ({:0.06}s)", pt1_time.as_secs_f64());

    println!("");

    let pt2_start_time = std::time::Instant::now();

    let mut looping_obstructions: Vec<GridPos> = Vec::new();
    
    let obstruction_candidates = {
        let mut obstruction_candidates = player_space_history.clone();
        obstruction_candidates.remove(&player_initial_state.pos);
        obstruction_candidates
    };

    for visited_player_space in obstruction_candidates {
        let mut obstructed_grid = Grid { width: grid.width, height: grid.height, cells: grid.cells.clone() };
        {
            let cell_ref = obstructed_grid.get_cell_mut(visited_player_space.row, visited_player_space.col);
            assert!(*cell_ref == Cell::Empty);
            *cell_ref = Cell::Obstacle;
        }

        let obstruction_loops_player = test_for_simulation_loop(&obstructed_grid, &player_initial_state);
        if obstruction_loops_player {
            looping_obstructions.push(visited_player_space);
        }

        // println!("Placing obstruction at (r:{},c:{}) looped player? {}",
        //     visited_player_space.row,
        //     visited_player_space.col,
        //     obstruction_loops_player);
    }

    let pt2_time = pt2_start_time.elapsed();
    println!("Found {} obstructions which cause loops", looping_obstructions.len());
    println!("TIME: ({:0.06}s)", pt2_time.as_secs_f64());

    return ExitCode::SUCCESS;
}
