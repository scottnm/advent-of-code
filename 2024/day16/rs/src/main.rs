use input_helpers;
use simple_grid::{Grid, GridPos};
use std::process::ExitCode;

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

impl Direction {
    fn turn_cw(&self) -> Self {
        match *self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn turn_ccw(&self) -> Self {
        match *self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn turn_180(&self) -> Self {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn move_grid_position(pos: GridPos, dir: Direction) -> GridPos {
    match dir {
        Direction::North => GridPos{row: pos.row - 1, col: pos.col},
        Direction::South => GridPos{row: pos.row + 1, col: pos.col},
        Direction::East => GridPos{row: pos.row, col: pos.col + 1},
        Direction::West => GridPos{row: pos.row, col: pos.col - 1},
    }
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

fn find_min_maze_path_score(maze: &Grid<Space>, start_pos: GridPos, start_dir: Direction, end_pos: GridPos) -> Option<usize> {
    #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
    struct VisitSpace {
        space: Space,
        visited: bool,
    }

    /*FIXME: remove
    fn dump_visit_state(
        title: &str,
        maze_tracker: &Grid<VisitSpace>, 
        curr_pos: GridPos, 
        curr_dir: Direction, 
        end_pos: GridPos) {
        let mut buf = String::with_capacity((maze_tracker.width + 1) * maze_tracker.height);
        for r in 0..(maze_tracker.height as isize) {
            for c in 0..(maze_tracker.width as isize) {
                let cell_pos = GridPos { row: r, col: c };
                let cell_char = if cell_pos == curr_pos {
                    match curr_dir {
                        Direction::North => '^',
                        Direction::South => 'v',
                        Direction::East => '<',
                        Direction::West => '>',
                    }
                } else {
                    let cell = maze_tracker.get_cell(r, c);
                    match cell {
                        VisitSpace { space: Space::Wall, visited: _ } => '#',
                        VisitSpace { space: Space::Empty, visited: visited } => {
                            if visited {
                                'x'
                            } else {
                                '.'
                            }
                        },
                    }
                };
                buf.push(cell_char);
            }
            buf.push('\n');
        }
        println!("{}:", title);
        print!("{}", buf);
    }*/

    fn find_min_maze_path_score_helper(
        maze_tracker: &mut Grid<VisitSpace>, 
        curr_pos: GridPos, 
        curr_dir: Direction, 
        end_pos: GridPos) -> Option<usize> {

        if curr_pos == end_pos {
            // FIXME: there's probably a better return type here that doens't require each new path end to instantiate a vector
            // and also require the caller to then copy that vector elsewhere. 
            //println!("Found solution!");
            return Some(0);
        }

        // mark the current cell as visited for the duration of this recursive call stack frame
        {
            let curr_cell = maze_tracker.get_cell_mut(curr_pos.row, curr_pos.col);
            assert!(!curr_cell.visited);
            curr_cell.visited = true;
        }

        // check forward move
        let mut min_fwd_score: Option<usize> = None;
        {
            let forward_move_dir = curr_dir;
            let forward_move_pos = move_grid_position(curr_pos, forward_move_dir);

            if maze_tracker.is_pos_out_of_bounds(forward_move_pos.row, forward_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let forward_cell = maze_tracker.get_cell(forward_move_pos.row, forward_move_pos.col);
                if forward_cell.visited {
                    // noop; can't move to an already visited space
                } else if let Space::Wall = forward_cell.space {
                    // noop; can't move to a wall
                } else {
                    let subpath_min_score = find_min_maze_path_score_helper(maze_tracker, forward_move_pos, forward_move_dir, end_pos);
                    if let Some(subpath_min_score) = subpath_min_score {
                        min_fwd_score = Some(subpath_min_score + 1); // +1 for move
                        //FIXME:println!("Found FWD subpath solution(s) with score {}.", min_fwd_score.unwrap());
                    }
                }
            }
        }

        // check CW turn + move
        let mut min_cw_score: Option<usize> = None;
        {
            let cw_turn_dir = curr_dir.turn_cw();
            let cw_turn_move_pos = move_grid_position(curr_pos, cw_turn_dir);

            if maze_tracker.is_pos_out_of_bounds(cw_turn_move_pos.row, cw_turn_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let cw_turn_move_cell = maze_tracker.get_cell(cw_turn_move_pos.row, cw_turn_move_pos.col);
                if cw_turn_move_cell.visited {
                    // noop; can't move to an already visited space
                } else if let Space::Wall = cw_turn_move_cell.space {
                    // noop; can't move to a wall
                } else {
                    let subpath_min_score = find_min_maze_path_score_helper(maze_tracker, cw_turn_move_pos, cw_turn_dir, end_pos);
                    if let Some(subpath_min_score) = subpath_min_score {
                        min_cw_score = Some(subpath_min_score + 1001); // +1 for move; +1000 for CW turn
                        //FIXME:println!("Found CW+MV subpath solution(s) with min {}.", min_cw_score.unwrap());
                    }
                }
            }
        }

        // check CCW turn + move
        let mut min_ccw_score: Option<usize> = None;
        {
            let ccw_turn_dir = curr_dir.turn_ccw();
            let ccw_turn_move_pos = move_grid_position(curr_pos, ccw_turn_dir);

            if maze_tracker.is_pos_out_of_bounds(ccw_turn_move_pos.row, ccw_turn_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let ccw_turn_move_cell = maze_tracker.get_cell(ccw_turn_move_pos.row, ccw_turn_move_pos.col);
                if ccw_turn_move_cell.visited {
                    // noop; can't move to an already visited space
                } else if let Space::Wall = ccw_turn_move_cell.space {
                    // noop; can't move to a wall
                } else {
                    let subpath_min_score = find_min_maze_path_score_helper(maze_tracker, ccw_turn_move_pos, ccw_turn_dir, end_pos);
                    if let Some(subpath_min_score) = subpath_min_score {
                        min_ccw_score = Some(subpath_min_score + 1001); // +1 for move; +1000 for CCW turn
                        //FIXME:println!("Found CCW+MV subpath solution(s) with min {}.", min_ccw_score.unwrap());
                    }
                }
            }
        }

        // check 180 turn + move
        let mut min_180_score: Option<usize> = None;
        {
            let half_turn_dir = curr_dir.turn_180();
            let half_turn_move_pos = move_grid_position(curr_pos, half_turn_dir);

            if maze_tracker.is_pos_out_of_bounds(half_turn_move_pos.row, half_turn_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let half_turn_move_cell = maze_tracker.get_cell(half_turn_move_pos.row, half_turn_move_pos.col);
                if half_turn_move_cell.visited {
                    // noop; can't move to an already visited space
                } else if let Space::Wall = half_turn_move_cell.space {
                    // noop; can't move to a wall
                } else {
                    let subpath_min_score = find_min_maze_path_score_helper(maze_tracker, half_turn_move_pos, half_turn_dir, end_pos);
                    if let Some(subpath_min_score) = subpath_min_score {
                        min_180_score = Some(subpath_min_score + 2001); // +1 for move; +2000 for 2 CW turns
                        //FIXME:println!("Found 180+MV subpath solution(s) with min {}.", min_180_score.unwrap());
                    }
                }
            }
        }

        maze_tracker.get_cell_mut(curr_pos.row, curr_pos.col).visited = false;

        let min_score = [("fwd", min_fwd_score), ("cw", min_cw_score), ("ccw", min_ccw_score), ("180", min_180_score)]
            .iter()
            .map(|(name, maybe_score)| {
                if let Some(score) = maybe_score {
                    Some((*name, *score))
                } else {
                    None
                }
            })
            .filter_map(|f| f)
            .min_by_key(|(_name, score)| *score);

        if let Some((name, min_score)) = min_score {
            //FIXME:println!("Picked {} solution with score {}", name, min_score);
            Some(min_score)
        } else {
            None
        }
    }

    let mut maze_tracker = Grid::<VisitSpace> {
        width: maze.width,
        height: maze.height,
        cells: maze.cells.iter().map(|space| VisitSpace {space: *space, visited: false }).collect(),
    };

    find_min_maze_path_score_helper(&mut maze_tracker, start_pos, start_dir, end_pos)
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
        maze,
        start_pos,
        end_pos,
        starting_dir,
    } = read_input(filename)?;

    // FIXME: remove
    // dbg!(&maze.width);
    // dbg!(&maze.height);
    // dbg!(&maze.cells);
    // dbg!(&start_pos);
    // dbg!(&end_pos);
    // dbg!(&starting_dir);

    {
        println!("Searching...");
        let min_maze_path_score = find_min_maze_path_score(&maze, start_pos, starting_dir, end_pos);
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
