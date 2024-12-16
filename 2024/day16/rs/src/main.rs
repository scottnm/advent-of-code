use input_helpers;
use simple_grid::{Grid, GridPos};
use std::{env::home_dir, process::ExitCode};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Space {
    Empty,
    Wall,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Move {
    Forward,
    TurnCW,
    TurnCCW,
}

struct StartingState {
    maze: Grid<Space>,
    start_pos: GridPos,
    end_pos: GridPos,
    starting_dir: Direction,
}

type MazePath = Vec<Move>;

fn read_input(filename: &str) -> Result<StartingState, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.is_empty() {
        return Err(format!("Invalid input! no lines/maze"));
    }

    let width = lines[0].len();
    let height = lines.len();

    let mut start_pos = None;
    let mut end_pos = None;
    let mut maze_cells: Vec<Space> = vec![];
    for (line_idx, line) in lines.iter().enumerate() {
        if line.len() != width {
            return Err(format!(
                "Grid must have consistent line widths! Expected {} found {}",
                width,
                line.len()
            ));
        }

        for (chr_idx, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    let new_starting_pos = GridPos {
                        row: line_idx as isize,
                        col: chr_idx as isize,
                    };
                    if let Some(pos) = start_pos {
                        return Err(format!(
                            "Repeat starting position found {}! First found at {}",
                            new_starting_pos, pos
                        ));
                    }
                    maze_cells.push(Space::Empty);
                    start_pos = Some(new_starting_pos);
                }
                'E' => {
                    let new_end_pos = GridPos {
                        row: line_idx as isize,
                        col: chr_idx as isize,
                    };
                    if let Some(pos) = end_pos {
                        return Err(format!(
                            "Repeat end position found {}! First found at {}",
                            new_end_pos, pos
                        ));
                    }
                    maze_cells.push(Space::Empty);
                    end_pos = Some(new_end_pos);
                }
                '.' => maze_cells.push(Space::Empty),
                '#' => maze_cells.push(Space::Wall),
                _ => return Err(format!("Invalid maze char! {}", c)),
            }
        }
    }

    let maze_grid = Grid::<Space> {
        width: width,
        height: height,
        cells: maze_cells,
    };

    let result = StartingState {
        maze: maze_grid,
        start_pos: start_pos.ok_or(String::from("No start pos in grid"))?,
        end_pos: end_pos.ok_or(String::from("No end pos in grid"))?,
        starting_dir: Direction::East,
    };

    Ok(result)
}

/* FIXME: remove if unneeded
fn dump_warehouse(warehouse: &Warehouse, robot_pos: &GridPos) -> String {
    let mut buf = String::with_capacity((warehouse.width + 1) * warehouse.height);
    for r in 0..(warehouse.height as isize) {
        for c in 0..(warehouse.width as isize) {
            let cell_pos = GridPos { row: r, col: c };
            let cell_char = if cell_pos == *robot_pos {
                '@'
            } else {
                match warehouse.get_cell(r, c) {
                    Space::Empty => '.',
                    Space::Box => 'O',
                    Space::Wall => '#',
                }
            };
            buf.push(cell_char);
        }
        buf.push('\n');
    }
    buf
}

fn print_warehouse(title: Option<&str>, warehouse: &Warehouse, robot_pos: &GridPos) {
    if let Some(title) = title {
        println!("{}: ", title);
    }
    println!("{}", dump_warehouse(warehouse, robot_pos));
}
*/

fn find_all_maze_paths(maze: &Grid<Space>, start_pos: GridPos, start_dir: Direction, end_pos: GridPos) -> Vec<MazePath> {
    #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
    struct VisitSpace {
        space: Space,
        visited: bool,
    }

    fn find_all_maze_paths_reversed_helper(
        maze_tracker: &mut Grid<VisitSpace>, 
        curr_pos: GridPos, 
        curr_dir: Direction, 
        end_pos: GridPos) -> Option<Vec<MazePath>> {

        if curr_pos == end_pos {
            // FIXME: there's probably a better return type here that doens't require each new path end to instantiate a vector
            // and also require the caller to then copy that vector elsewhere. 
            return Some(vec![]);
        }

        let mut paths = vec![];

        // check forward move
        {
            let forward_move_offset = match curr_dir {
                Direction::North => GridPos{row: -1, col: 0},
                Direction::South => GridPos{row: 1, col: 0},
                Direction::East => GridPos{row: 0, col: 1},
                Direction::West => GridPos{row: 0, col: -1},
            };

            let forward_move_pos = GridPos {row: curr_pos.row + forward_move_offset.row, col: curr_pos.col + forward_move_offset.row};

            let forward_move_dir = curr_dir;

            if maze_tracker.is_pos_out_of_bounds(forward_move_pos.row, forward_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let forward_cell = maze_tracker.get_cell(forward_move_pos.row, forward_move_pos.col);
                if forward_cell.visited {
                    // noop; can't move to an already visited space
                } else if let Space::Wall = forward_cell.space {
                    // noop; can't move to a wall
                } else {
                    let found_maze_paths = find_all_maze_paths_reversed_helper(maze_tracker, forward_move_pos, forward_move_dir, end_pos);
                    if let Some(mut found_maze_paths) = found_maze_paths {
                        for path in found_maze_paths.iter_mut() {
                            path.push(Move::Forward);
                        }

                        paths.append(&mut found_maze_paths);
                    }
                }
            }
        }

        // check CW turn + move

        // check CCW turn + move

        // check 180 turn + move

        if paths.len() == 0 {
            None
        } else {
            Some(paths)
        }
    }

    let mut maze_tracker = Grid::<VisitSpace> {
        width: maze.width,
        height: maze.height,
        cells: maze.cells.iter().map(|space| VisitSpace {space: *space, visited: false }).collect(),
    };

    maze_tracker.get_cell_mut(start_pos.row, start_pos.col).visited = true;

    let found_reversed_paths = find_all_maze_paths_reversed_helper(&mut maze_tracker, start_pos, start_dir, end_pos)
        .unwrap_or(vec![]);

    // paths are found reversed for to avoid a bunch of vector shifts. reverse the lifts before returning. 
    found_reversed_paths
        .iter()
        .map(|path| {
            let mut reversed_path = path.clone();
            reversed_path.reverse();
            reversed_path
        })
        .collect()
}

fn calculate_maze_move_score(maze_move: Move) -> usize {
    match maze_move {
        Move::Forward => 1,
        Move::TurnCCW | Move::TurnCW => 1000,
    }
}
fn calculate_maze_path_score(maze_path_moves: &[Move]) -> usize {
    maze_path_moves.iter().cloned().map(calculate_maze_move_score).sum()
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();

    let StartingState {
        maze: maze,
        start_pos: start_pos,
        end_pos: end_pos,
        starting_dir: starting_dir,
    } = read_input(filename)?;

    dbg!(&maze.width);
    dbg!(&maze.height);
    dbg!(&maze.cells);
    dbg!(&start_pos);
    dbg!(&end_pos);
    dbg!(&starting_dir);

    {
        let maze_paths = find_all_maze_paths(&maze, start_pos, starting_dir, end_pos);
        let min_maze_path_score = maze_paths
            .iter()
            .map(|maze_path| calculate_maze_path_score(maze_path))
            .min();
        if let Some(min_maze_path_score) = min_maze_path_score {
            println!("pt 1: min score {}", min_maze_path_score);
        } else {
            println!("pt 1: no solutions to maze");
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Err: {}", e);
            ExitCode::FAILURE
        }
    }
}
