use input_helpers;
use simple_grid::{Grid, GridPos};
use std::{cell::Cell, process::ExitCode};

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

fn find_min_maze_path_score(maze: &Grid<Space>, start_pos: GridPos, start_dir: Direction, end_pos: GridPos) -> Option<usize> {
    #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
    struct VisitSpace {
        space: Space,
        visited: bool,
    }

    #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
    enum VisitDistance {
        Unreachable,
        MaxDist,
        Dist(usize),
    }

    #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
    struct CellVisitDistance {
        north: VisitDistance,
        south: VisitDistance,
        east: VisitDistance,
        west: VisitDistance,
    }

    fn get_cell_visit_dir_mut(visit_dist: &mut CellVisitDistance, dir: Direction) -> &mut VisitDistance {
        match dir {
            Direction::North => &mut visit_dist.north,
            Direction::South => &mut visit_dist.south,
            Direction::East => &mut visit_dist.east,
            Direction::West => &mut visit_dist.west,
        }
    }

    fn get_min_cell_visit_dist(visit_dist: &CellVisitDistance) -> Option<usize> {
        let mut min_val = None;
        if let VisitDistance::Dist(dist) = visit_dist.north {
            min_val = 
            if let Some(old_min_val) = min_val {
                Some(std::cmp::min(old_min_val, dist))
            } else {
                Some(dist)
            };
        }
        if let VisitDistance::Dist(dist) = visit_dist.south {
            min_val = 
            if let Some(old_min_val) = min_val {
                Some(std::cmp::min(old_min_val, dist))
            } else {
                Some(dist)
            };
        }
        if let VisitDistance::Dist(dist) = visit_dist.east {
            min_val = 
            if let Some(old_min_val) = min_val {
                Some(std::cmp::min(old_min_val, dist))
            } else {
                Some(dist)
            };
        }
        if let VisitDistance::Dist(dist) = visit_dist.west {
            min_val = 
            if let Some(old_min_val) = min_val {
                Some(std::cmp::min(old_min_val, dist))
            } else {
                Some(dist)
            };
        }

        min_val
    }

    let mut maze_tracker = Grid::<CellVisitDistance> {
        width: maze.width,
        height: maze.height,
        cells: maze.cells.iter().map(|space| {
            let dist = match space {
                Space::Empty => VisitDistance::MaxDist,
                Space::Wall => VisitDistance::Unreachable,
            };
            
            CellVisitDistance { north: dist, south: dist, east: dist, west: dist }
        }).collect(),
    };

    let starting_pos_dist = VisitDistance::Dist(0);
    *maze_tracker.get_cell_mut(start_pos.row, start_pos.col) = 
            CellVisitDistance { north: starting_pos_dist, south: starting_pos_dist, east: starting_pos_dist, west: starting_pos_dist };

    // initialize the unvisited set
    type UnvisitedSet = std::collections::HashSet<(GridPos, Direction)>;
    let mut unvisited_set = UnvisitedSet::new();
    for r in 0..maze_tracker.height as isize {
        for c in 0..maze_tracker.width as isize {
            let pos = GridPos {row: r, col: c};
            let cell = maze_tracker.get_cell(r, c);
            let candidates = [
                (Direction::North, cell.north),
                (Direction::South, cell.south),
                (Direction::East, cell.east),
                (Direction::West, cell.west),
                ];
            for unvisited_cand in candidates {
                if let VisitDistance::Unreachable = unvisited_cand.1 {
                    // ignore walls
                } else {
                    unvisited_set.insert((pos, unvisited_cand.0));
                }
            }
        }
    }

    fn find_next_current_node(maze_tracker: &Grid<CellVisitDistance>, unvisited_set: &UnvisitedSet) -> Option<(GridPos, Direction, usize)> {
        let mut min_dist_unvisited_node: Option<(GridPos, Direction, usize)> = None;
        for (candidate_pos, candidate_dir) in unvisited_set {
            let candidate_cell = maze_tracker.get_cell(candidate_pos.row, candidate_pos.col);
            let candidate_visit_dist = match candidate_dir {
                Direction::North => candidate_cell.north,
                Direction::South => candidate_cell.south,
                Direction::East => candidate_cell.east,
                Direction::West => candidate_cell.west,
            };

            if let VisitDistance::Dist(dist) = candidate_visit_dist {
                if let Some(min_dist_unvisited_node_data) = min_dist_unvisited_node  {
                    if dist < min_dist_unvisited_node_data.2 {
                        min_dist_unvisited_node = Some((*candidate_pos, *candidate_dir, dist));
                    }
                } else {
                    min_dist_unvisited_node = Some((*candidate_pos, *candidate_dir, dist));
                }
            }
        }

        min_dist_unvisited_node
    } 

    let mut curr_node_cand = (start_pos, start_dir, 0);
    loop {
        
        // check forward move
        {
            let forward_move_dir = curr_node_cand.1;
            let forward_move_pos = move_grid_position(curr_node_cand.0, forward_move_dir);

            if maze_tracker.is_pos_out_of_bounds(forward_move_pos.row, forward_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let forward_cell = maze_tracker.get_cell_mut(forward_move_pos.row, forward_move_pos.col);
                if !unvisited_set.contains(&(forward_move_pos, forward_move_dir)) {
                    // noop; can't move to an already visited space
                } else {
                    let cell_visit_dist = get_cell_visit_dir_mut(forward_cell, forward_move_dir);
                    let new_dist = curr_node_cand.2 + 1; // +1 forward
                    match cell_visit_dist.clone() {
                        VisitDistance::Dist(dist) => {
                            *cell_visit_dist = VisitDistance::Dist(std::cmp::min(dist, new_dist));
                        },
                        VisitDistance::MaxDist => {
                            *cell_visit_dist = VisitDistance::Dist(new_dist);
                        },
                        VisitDistance::Unreachable =>  {
                            // noop
                        }
                    }
                }
            }
        }

        // check CW turn + move
        {
            let cw_turn_dir = curr_node_cand.1.turn_cw();
            let cw_turn_move_pos = move_grid_position(curr_node_cand.0, cw_turn_dir);

            if maze_tracker.is_pos_out_of_bounds(cw_turn_move_pos.row, cw_turn_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let cw_turn_cell = maze_tracker.get_cell_mut(cw_turn_move_pos.row, cw_turn_move_pos.col);
                if !unvisited_set.contains(&(cw_turn_move_pos, cw_turn_dir)) {
                    // noop; can't move to an already visited space
                } else {
                    let cell_visit_dist = get_cell_visit_dir_mut(cw_turn_cell, cw_turn_dir);
                    let new_dist = curr_node_cand.2 + 1001; // +1000 turn; +1 fwd
                    match cell_visit_dist.clone() {
                        VisitDistance::Dist(dist) => {
                            *cell_visit_dist = VisitDistance::Dist(std::cmp::min(dist, new_dist));
                        },
                        VisitDistance::MaxDist => {
                            *cell_visit_dist = VisitDistance::Dist(new_dist);
                        },
                        VisitDistance::Unreachable =>  {
                            // noop
                        }
                    }
                }
            }
        }

        // check CCW turn + move
        {
            let ccw_turn_dir = curr_node_cand.1.turn_ccw();
            let ccw_turn_move_pos = move_grid_position(curr_node_cand.0, ccw_turn_dir);

            if maze_tracker.is_pos_out_of_bounds(ccw_turn_move_pos.row, ccw_turn_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let ccw_turn_cell = maze_tracker.get_cell_mut(ccw_turn_move_pos.row, ccw_turn_move_pos.col);
                if !unvisited_set.contains(&(ccw_turn_move_pos, ccw_turn_dir)) {
                    // noop; can't move to an already visited space
                } else {
                    let cell_visit_dist = get_cell_visit_dir_mut(ccw_turn_cell, ccw_turn_dir);
                    let new_dist = curr_node_cand.2 + 1001; // +1000 turn; +1 fwd
                    match cell_visit_dist.clone() {
                        VisitDistance::Dist(dist) => {
                            *cell_visit_dist = VisitDistance::Dist(std::cmp::min(dist, new_dist));
                        },
                        VisitDistance::MaxDist => {
                            *cell_visit_dist = VisitDistance::Dist(new_dist);
                        },
                        VisitDistance::Unreachable =>  {
                            // noop
                        }
                    }
                }
            }
        }

        // check 180 turn + move
        {
            let half_turn_dir = curr_node_cand.1.turn_180();
            let half_turn_move_pos = move_grid_position(curr_node_cand.0, half_turn_dir);

            if maze_tracker.is_pos_out_of_bounds(half_turn_move_pos.row, half_turn_move_pos.col) {
                // noop; can't move to oob position
            } else {
                let half_turn_cell = maze_tracker.get_cell_mut(half_turn_move_pos.row, half_turn_move_pos.col);
                if !unvisited_set.contains(&(half_turn_move_pos, half_turn_dir)) {
                    // noop; can't move to an already visited space
                } else {
                    let cell_visit_dist = get_cell_visit_dir_mut(half_turn_cell, half_turn_dir);
                    let new_dist = curr_node_cand.2 + 2001; // +2000 2x turn; +1 forward
                    match cell_visit_dist.clone() {
                        VisitDistance::Dist(dist) => {
                            *cell_visit_dist = VisitDistance::Dist(std::cmp::min(dist, new_dist));
                        },
                        VisitDistance::MaxDist => {
                            *cell_visit_dist = VisitDistance::Dist(new_dist);
                        },
                        VisitDistance::Unreachable =>  {
                            // noop
                        }
                    }
                }
            }
        }

        // end loop
        unvisited_set.remove(&(curr_node_cand.0, curr_node_cand.1));
        if let Some(next_curr_node) = find_next_current_node(&maze_tracker, &unvisited_set) {
            if next_curr_node.0 == end_pos {
                // FIXME: I'm not sure how correct this is...
                println!("Breaking due to finding end_pos min dist!");
                break;
            }
            curr_node_cand = next_curr_node;
        } else {
            break;
        }
    }

    get_min_cell_visit_dist(&maze_tracker.get_cell(end_pos.row, end_pos.col))
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
