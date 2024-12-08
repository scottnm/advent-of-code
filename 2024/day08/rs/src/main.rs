use input_helpers;
use std::process::ExitCode;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct GridPos {
    row: isize,
    col: isize,
}

struct Grid<T> 
    where T: Clone + Copy
    {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> where T: Clone + Copy {
    fn get_cell_idx(&self, row: isize, col: isize) -> usize {
        assert!(!self.is_pos_out_of_bounds(row, col));

        (row as usize * self.width) + (col as usize)
    }

    fn get_cell(&self, row: isize, col: isize) -> T {
        self.cells[self.get_cell_idx(row, col)]
    }

    fn get_cell_mut(&mut self, row: isize, col: isize) -> &mut T {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tower {
    freq: char
}

type TowerGrid = Grid<Option<Tower>>;

fn read_tower_grid(filename: &str) -> Result<TowerGrid, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() == 0 {
        return Ok(Grid{width: 0, height: 0, cells: vec![]});
    }

    let height = lines.len();
    let width = lines[0].len();

    let mut cells: Vec<Option<Tower>> = vec![];
    for line in lines {
        if line.len() != width {
            return Err(format!(
                "Grid must have consistent line widths! Expected {} found {}",
                width,
                line.len()
            ));
        }

        for c in line.chars() {
            let cell = match c {
                '.' => None,
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Some(Tower{freq: c}),
                _ => return Err(format!("Invalid frequency tower grid char! {}", c)),
            };
            cells.push(cell);
        }
    }

    Ok(Grid {width, height, cells})
}

fn dump_tower_grid(tower_grid: &TowerGrid) {
    for r in 0..(tower_grid.height as isize) {
        for c in 0..(tower_grid.width as isize) {
            let print_char = match tower_grid.get_cell(r, c) {
                Some(tower) => tower.freq,
                None => '.',
            };
            print!("{}", print_char);
        }
        println!("");
    }
}

fn calculate_all_antinode_positions(tower_grid: &TowerGrid) -> std::collections::HashSet<GridPos> {
    // FIXME: rather than preprocessing the tower grid here, the input should probably just be read in this format
    let tower_positions = {
        let mut tower_positions = std::collections::HashMap::<char, Vec<GridPos>>::new();
        for r in 0..(tower_grid.height as isize) {
            for c in 0..(tower_grid.width as isize) {
                if let Some(tower) = tower_grid.get_cell(r, c) {
                    if let Some(single_freq_tower_positions) = tower_positions.get_mut(&tower.freq) {
                        single_freq_tower_positions.push(GridPos{row: r, col: c});
                    } else {
                        let mut single_freq_tower_positions = vec![];
                        single_freq_tower_positions.push(GridPos{row: r, col: c});
                        tower_positions.insert(tower.freq, single_freq_tower_positions);
                    }
                }
            }
        }
        tower_positions
    };

    let mut antinode_positions = std::collections::HashSet::<GridPos>::new();

    for (_freq, tower_positions) in &tower_positions {
        // FIXME: drop all of the extra printlns in here
        println!("freq({}): pos={:?}", _freq, tower_positions);
        for (tower_a, tower_b) in tower_positions.iter().tuple_combinations() {
            let antinode_pos_1 = GridPos {
                row: tower_a.row + (tower_a.row - tower_b.row),
                col: tower_a.col + (tower_a.col - tower_b.col),
            };

            print!("    testing pos: ({},{})...", antinode_pos_1.row, antinode_pos_1.col);

            if !tower_grid.is_pos_out_of_bounds(antinode_pos_1.row, antinode_pos_1.col) {
                antinode_positions.insert(antinode_pos_1);
                println!("inserted");
            } else {
                println!("OOB !!");
            }

            let antinode_pos_2 = GridPos {
                row: tower_b.row + (tower_b.row - tower_a.row),
                col: tower_b.col + (tower_b.col - tower_a.col),
            };

            print!("    testing pos: ({},{})...", antinode_pos_1.row, antinode_pos_1.col);

            if !tower_grid.is_pos_out_of_bounds(antinode_pos_2.row, antinode_pos_2.col) {
                antinode_positions.insert(antinode_pos_2);
                println!("inserted");
            } else {
                println!("OOB !!");
            }
        }
    }

    antinode_positions
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_tower_grid(filename);
    let tower_grid = match parse_result {
        Ok(tower_grid) => tower_grid,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    println!("Pt 1:");
    dump_tower_grid(&tower_grid);
    let antinode_positions = calculate_all_antinode_positions(&tower_grid);
    println!("antinode position count: {}", antinode_positions.len());
    if antinode_positions.len() < 10 {
        for p in antinode_positions {
            println!("- (r:{},c:{})", p.row, p.col);
        }
    }

    println!("");

    println!("Pt 2:");

    // FIXME: do pt 2

    return ExitCode::SUCCESS;
}
