use input_helpers;
use std::process::ExitCode;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

    /*
    let solved_equations_pt1: Vec<(Equation, Vec<Operation>)> = equations
        .iter()
        .map(|eq| (eq.clone(), solve_recursive_any_pt1(&eq)))
        .filter(|(_eq, solution)| solution.is_some())
        .map(|(eq, solution)| (eq, solution.unwrap()))
        .collect();

    // for (eq, _) in &solved_equations_pt1 {
    //     println!("Found sol for {:?}", eq);
    // }

    let sum_solvable_results_pt1: usize =
        solved_equations_pt1.iter().map(|(eq, _sol)| eq.result).sum();
    println!("Sum of solution results: {}", sum_solvable_results_pt1);

    println!("");

    println!("Pt 2:");

    let solved_equations_pt2: Vec<(Equation, Vec<Operation>)> = equations
        .iter()
        .map(|eq| (eq.clone(), solve_recursive_any_pt2(&eq)))
        .filter(|(_eq, solution)| solution.is_some())
        .map(|(eq, solution)| (eq, solution.unwrap()))
        .collect();

    // for (eq, _) in &solved_equations_pt2 {
    //     println!("Found sol for {:?}", eq);
    // }

    let sum_solvable_results_pt2: usize =
        solved_equations_pt2.iter().map(|(eq, _sol)| eq.result).sum();
    println!("Sum of solution results: {}", sum_solvable_results_pt2);
    */

    return ExitCode::SUCCESS;
}
