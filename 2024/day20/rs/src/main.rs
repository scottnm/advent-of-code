use core::fmt;
use input_helpers;
use simple_grid::{Grid, GridPos};
use std::process::ExitCode;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Space {
    Empty,
    Wall,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum DijDist {
    Dist(usize),
    Inf,
}
struct StartingState {
    maze: Grid<Space>,
    start_pos: GridPos,
    end_pos: GridPos,
}

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
    };

    Ok(result)
}

// FIXME:
// fn dump_memory_safety_grid(memory_grid: &Grid<Space>) -> String {
//     let mut buf = String::with_capacity((memory_grid.width + 1) * memory_grid.height);
//     for r in 0..(memory_grid.height as isize) {
//         for c in 0..(memory_grid.width as isize) {
//             let cell_char = match memory_grid.get_cell(r, c) {
//                 Space::Safe => '.',
//                 Space::Corrupted => '#',
//             };
//             buf.push(cell_char);
//         }
//         buf.push('\n');
//     }
//     buf
// }

// FIXME:
// fn print_memory_safety_grid(title: Option<&str>, memory_grid: &Grid<Space>) {
//     if let Some(title) = title {
//         println!("{}: ", title);
//     }
//     println!("{}", dump_memory_safety_grid(memory_grid));
// }

// FIXME:
// fn dump_dist_grid(memory_grid: &Grid<Space>, dist_tracker: &Grid<DijDist>) -> String {
//     fn count_digits(n: usize) -> usize {
//         let mut n = n;
//         let mut digit_count = 1;
//         while n > 9 {
//             n /= 10;
//             digit_count += 1;
//         }
//         digit_count
//     }
// 
//     let max_digit_count = dist_tracker
//         .cells
//         .iter()
//         .map(|dist| {
//             match dist {
//                 DijDist::Dist(d) => count_digits(*d),
//                 DijDist::Inf => 3, // "Inf"
//             }
//         })
//         .max()
//         .unwrap_or(0);
// 
//     let corrupted_cell_str = {
//         let mut buf = String::new();
//         buf.push('[');
//         for _ in 0..max_digit_count {
//             buf.push('#');
//         }
//         buf.push(']');
//         buf
//     };
// 
//     let inf_cell_str = {
//         assert!(max_digit_count >= "INF".len());
//         let mut buf = String::new();
//         buf.push('[');
//         for _ in 0..(max_digit_count / 2 - 1) {
//             buf.push(' ');
//         }
//         buf.push('I');
//         buf.push('N');
//         buf.push('F');
//         let remaining_digit_spaces = max_digit_count - (max_digit_count / 2 + 3 - 1);
//         for _ in 0..remaining_digit_spaces {
//             buf.push(' ');
//         }
//         buf.push(']');
//         buf
//     };
// 
//     fn fmt_num_cell(n: usize, max_digit_count: usize) -> String {
//         let digit_count = count_digits(n);
//         assert!(max_digit_count >= digit_count);
//         let mut buf = String::new();
//         buf.push('[');
//         for _ in 0..(max_digit_count / 2 - digit_count / 2) {
//             buf.push(' ');
//         }
//         buf.push_str(&n.to_string());
//         let remaining_digit_spaces =
//             max_digit_count - (max_digit_count / 2 - digit_count / 2) - (digit_count);
//         for _ in 0..remaining_digit_spaces {
//             buf.push(' ');
//         }
//         buf.push(']');
//         buf
//     }
// 
//     let mut buf = String::with_capacity((memory_grid.width + 1) * memory_grid.height);
//     for r in 0..(memory_grid.height as isize) {
//         for c in 0..(memory_grid.width as isize) {
//             let cell_str = match memory_grid.get_cell(r, c) {
//                 Space::Safe => match dist_tracker.get_cell(r, c) {
//                     DijDist::Dist(dist) => fmt_num_cell(dist, max_digit_count),
//                     DijDist::Inf => inf_cell_str.clone(),
//                 },
//                 Space::Corrupted => corrupted_cell_str.clone(),
//             };
//             buf.push_str(&cell_str);
//         }
//         buf.push('\n');
//     }
//     buf
// }

// FIXME:
// fn find_min_safe_path(
//     memory_grid: &Grid<Space>,
//     start_pos: GridPos,
//     end_pos: GridPos,
//     verbose: bool,
// ) -> Option<Vec<GridPos>> {
//     let mut dist_tracker = Grid::<DijDist> {
//         width: memory_grid.width,
//         height: memory_grid.height,
//         cells: vec![DijDist::Inf; memory_grid.width * memory_grid.height],
//     };
// 
//     let mut dist_path_tracker = Grid::<Option<GridPos>> {
//         width: memory_grid.width,
//         height: memory_grid.height,
//         cells: vec![None; memory_grid.width * memory_grid.height],
//     };
// 
//     let mut unvisited_cells = std::collections::HashSet::<GridPos>::new();
//     for r in 0..(memory_grid.height as isize) {
//         for c in 0..(memory_grid.width as isize) {
//             if let Space::Safe = memory_grid.get_cell(r, c) {
//                 let reachable_cell_pos = GridPos { row: r, col: c };
//                 unvisited_cells.insert(reachable_cell_pos);
//             }
//         }
//     }
// 
//     *dist_tracker.get_cell_mut(start_pos.row, start_pos.col) = DijDist::Dist(0);
// 
//     fn get_unvisited_node_with_min_dist(
//         unvisited_cells: &std::collections::HashSet<GridPos>,
//         dist_tracker: &Grid<DijDist>,
//     ) -> Option<GridPos> {
//         let mut min_dist_cell: Option<(GridPos, usize)> = None;
//         for unvisited_cell_pos in unvisited_cells {
//             let unvisited_cell_dist =
//                 match dist_tracker.get_cell(unvisited_cell_pos.row, unvisited_cell_pos.col) {
//                     DijDist::Dist(dist) => dist,
//                     DijDist::Inf => continue,
//                 };
// 
//             if let Some((_, min_dist_cell_dist)) = min_dist_cell {
//                 if unvisited_cell_dist < min_dist_cell_dist {
//                     min_dist_cell = Some((*unvisited_cell_pos, unvisited_cell_dist));
//                 }
//             } else {
//                 min_dist_cell = Some((*unvisited_cell_pos, unvisited_cell_dist));
//             }
//         }
// 
//         min_dist_cell.map(|(pos, _)| pos)
//     }
// 
//     fn get_unvisited_neighbor(
//         pos: GridPos,
//         offset_row: isize,
//         offset_col: isize,
//         unvisited_cells: &std::collections::HashSet<GridPos>,
//         memory_grid: &Grid<Space>,
//     ) -> Option<GridPos> {
//         let offset_pos = GridPos {
//             row: pos.row + offset_row,
//             col: pos.col + offset_col,
//         };
//         if memory_grid.is_pos_out_of_bounds(offset_pos.row, offset_pos.col) {
//             return None;
//         }
// 
//         if !unvisited_cells.contains(&offset_pos) {
//             return None;
//         }
// 
//         // assert the cell is safe
//         if let Space::Wall = memory_grid.get_cell(pos.row, pos.col) {
//             panic!("Unexpected found a corrupted cell in the unvisited cell list");
//         }
// 
//         Some(offset_pos)
//     }
// 
//     let mut current_node_pos: GridPos = start_pos;
//     loop {
//         let mut unvisited_neighbors_buffer = [GridPos { row: 0, col: 0 }; 4];
//         let mut unvisited_neighbor_count = 0;
//         let current_dist = match dist_tracker.get_cell(current_node_pos.row, current_node_pos.col) {
//             DijDist::Dist(dist) => dist,
//             DijDist::Inf => panic!("Nodes with distance 'Inf' should never be selected as current"),
//         };
// 
//         // see if we have an 'up' neighbor to check
//         if let Some(neighbor_pos) =
//             get_unvisited_neighbor(current_node_pos, -1, 0, &unvisited_cells, memory_grid)
//         {
//             unvisited_neighbors_buffer[unvisited_neighbor_count] = neighbor_pos;
//             unvisited_neighbor_count += 1;
//         }
// 
//         // see if we have an 'down' neighbor to check
//         if let Some(neighbor_pos) =
//             get_unvisited_neighbor(current_node_pos, 1, 0, &unvisited_cells, memory_grid)
//         {
//             unvisited_neighbors_buffer[unvisited_neighbor_count] = neighbor_pos;
//             unvisited_neighbor_count += 1;
//         }
// 
//         // see if we have an 'left' neighbor to check
//         if let Some(neighbor_pos) =
//             get_unvisited_neighbor(current_node_pos, 0, -1, &unvisited_cells, memory_grid)
//         {
//             unvisited_neighbors_buffer[unvisited_neighbor_count] = neighbor_pos;
//             unvisited_neighbor_count += 1;
//         }
// 
//         // see if we have an 'right' neighbor to check
//         if let Some(neighbor_pos) =
//             get_unvisited_neighbor(current_node_pos, 0, 1, &unvisited_cells, memory_grid)
//         {
//             unvisited_neighbors_buffer[unvisited_neighbor_count] = neighbor_pos;
//             unvisited_neighbor_count += 1;
//         }
// 
//         let unvisited_neighbors = &unvisited_neighbors_buffer[0..unvisited_neighbor_count];
//         for unvisited_neighbor_pos in unvisited_neighbors {
//             let neighbor_dist_cell =
//                 dist_tracker.get_cell_mut(unvisited_neighbor_pos.row, unvisited_neighbor_pos.col);
//             let neighbor_dist_path_cell = dist_path_tracker
//                 .get_cell_mut(unvisited_neighbor_pos.row, unvisited_neighbor_pos.col);
// 
//             let path_to_neighbor_dist = current_dist + 1;
// 
//             let update_path = match neighbor_dist_cell.clone() {
//                 DijDist::Dist(dist) => path_to_neighbor_dist < dist,
//                 DijDist::Inf => true,
//             };
// 
//             if update_path {
//                 *neighbor_dist_cell = DijDist::Dist(path_to_neighbor_dist);
//                 *neighbor_dist_path_cell = Some(current_node_pos);
//             }
//         }
// 
//         unvisited_cells.remove(&current_node_pos);
// 
//         if let Some(next_node_pos) =
//             get_unvisited_node_with_min_dist(&unvisited_cells, &dist_tracker)
//         {
//             if next_node_pos == end_pos {
//                 break;
//             } else {
//                 current_node_pos = next_node_pos;
//             }
//         } else {
//             break;
//         }
//     }
// 
//     if verbose {
//         let dist_grid_str = dump_dist_grid(memory_grid, &dist_tracker);
//         println!("result grid distances:");
//         print!("{}", dist_grid_str);
//     }
// 
//     match dist_tracker.get_cell(end_pos.row, end_pos.col) {
//         DijDist::Dist(end_pos_dist_from_start) => {
//             let path = {
//                 let mut reverse_path = Vec::with_capacity(end_pos_dist_from_start);
// 
//                 let mut curr_pos = end_pos;
//                 loop {
//                     reverse_path.push(curr_pos);
//                     if let Some(prev_pos) = dist_path_tracker.get_cell(curr_pos.row, curr_pos.col) {
//                         curr_pos = prev_pos;
//                     } else {
//                         assert!(curr_pos == start_pos);
//                         break;
//                     }
//                 }
// 
//                 assert!(reverse_path.len() == (end_pos_dist_from_start + 1));
//                 reverse_path.reverse();
//                 reverse_path
//             };
//             Some(path)
//         }
//         DijDist::Inf => None,
//     }
// }

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let min_picosecond_savings: usize = input_helpers::get_nth_parsed_arg(args, 1)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();
    let do_pt2 = args
        .iter()
        .find(|a| a.as_str() == "-2" || a.as_str() == "--pt2")
        .is_some();

    let StartingState {maze, start_pos, end_pos } = read_input(filename)?;

    dbg!(&maze);
    dbg!(&start_pos);
    dbg!(&end_pos);
    /*
    {
        let mut corrupted_memory_grid = initial_memory_safety_grid.clone();
        corrupt_bytes(
            &mut corrupted_memory_grid,
            &corrupted_bytes[..num_bytes_to_simulate_in_pt1],
        );

        if verbose {
            print_memory_safety_grid(Some("memory after corruption"), &corrupted_memory_grid);
        }

        let min_safe_path = find_min_safe_path(&corrupted_memory_grid, start_pos, end_pos, verbose);
        if let Some(min_safe_path) = min_safe_path {
            println!("Pt 1: min path len = {}", min_safe_path.len() - 1);
        } else {
            println!("Pt 1: min path len = NO SOLUTION");
        }
    } */

    if do_pt2 {
        unimplemented!();
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
