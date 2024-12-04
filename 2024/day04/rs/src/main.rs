use input_helpers;
use std::process::ExitCode;

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

enum Rotation {
    // M . S
    // . A .
    // M . S
    Zero,
    // M . M
    // . A .
    // S . S
    Quarter,
    // S . M
    // . A .
    // S . M
    Half,
    // S . S
    // . A .
    // M . M
    ThreeQuarter,
}

#[derive(Clone, Copy)]
struct GridPos {
    row: usize,
    col: usize,
}

struct Pt1WordSearchSolution {
    #[allow(dead_code)] // FIXME: not using these fields yet
    start_pos: GridPos,
    #[allow(dead_code)] // FIXME: not using these fields yet
    dir: Direction,
}

struct Pt2WordSearchSolution {
    #[allow(dead_code)] // FIXME: not using these fields yet
    start_pos: GridPos,
    #[allow(dead_code)] // FIXME: not using these fields yet
    rot: Rotation,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<char>,
}

impl Grid {
    fn get_cell(&self, row: usize, col: usize) -> char {
        assert!(row < self.height);
        assert!(col < self.width);
        self.cells[(row * self.width) + col]
    }
}

fn read_grid(filename: &str) -> Result<Grid, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() == 0 {
        return Ok(Grid{width: 0, height: 0, cells: vec![]});
    }
    
    let height = lines.len();
    let width = lines[0].len();

    let mut cells: Vec<char> = vec![];
    for line in lines {
        if line.len() != width {
            return Err(format!("Grid must have consistent line widths! Expected {} found {}", width, line.len()));
        }

        for c in line.chars() {
            cells.push(c);
        }
    }

    Ok(Grid{width: width, height: height, cells: cells})
}

fn find_pt1_word_search_solutions(grid: &Grid) -> Vec<Pt1WordSearchSolution> {
    let mut solutions: Vec<Pt1WordSearchSolution> = vec![];

    fn has_north_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.height >= 4 &&
        start_pos.row >= 3 &&
        grid.get_cell(start_pos.row - 0, start_pos.col) == 'X' &&
        grid.get_cell(start_pos.row - 1, start_pos.col) == 'M' &&
        grid.get_cell(start_pos.row - 2, start_pos.col) == 'A' &&
        grid.get_cell(start_pos.row - 3, start_pos.col) == 'S'
    }

    fn has_north_east_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.height >= 4 &&
        grid.width >= 4 &&
        start_pos.row >= 3 &&
        start_pos.col <= (grid.width - 4) &&
        grid.get_cell(start_pos.row - 0, start_pos.col + 0) == 'X' &&
        grid.get_cell(start_pos.row - 1, start_pos.col + 1) == 'M' &&
        grid.get_cell(start_pos.row - 2, start_pos.col + 2) == 'A' &&
        grid.get_cell(start_pos.row - 3, start_pos.col + 3) == 'S'
    }

    fn has_east_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.width >= 4 &&
        start_pos.col <= (grid.width - 4) &&
        grid.get_cell(start_pos.row, start_pos.col + 0) == 'X' &&
        grid.get_cell(start_pos.row, start_pos.col + 1) == 'M' &&
        grid.get_cell(start_pos.row, start_pos.col + 2) == 'A' &&
        grid.get_cell(start_pos.row, start_pos.col + 3) == 'S'
    }

    fn has_south_east_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.height >= 4 &&
        grid.width >= 4 &&
        start_pos.row <= (grid.height - 4) &&
        start_pos.col <= (grid.width - 4) &&
        grid.get_cell(start_pos.row + 0, start_pos.col + 0) == 'X' &&
        grid.get_cell(start_pos.row + 1, start_pos.col + 1) == 'M' &&
        grid.get_cell(start_pos.row + 2, start_pos.col + 2) == 'A' &&
        grid.get_cell(start_pos.row + 3, start_pos.col + 3) == 'S'
    }

    fn has_south_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.height >= 4 &&
        start_pos.row <= (grid.height - 4) &&
        grid.get_cell(start_pos.row + 0, start_pos.col) == 'X' &&
        grid.get_cell(start_pos.row + 1, start_pos.col) == 'M' &&
        grid.get_cell(start_pos.row + 2, start_pos.col) == 'A' &&
        grid.get_cell(start_pos.row + 3, start_pos.col) == 'S'
    }

    fn has_south_west_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.height >= 4 &&
        grid.width >= 4 &&
        start_pos.row <= (grid.height - 4) &&
        start_pos.col >= 3 &&
        grid.get_cell(start_pos.row + 0, start_pos.col - 0) == 'X' &&
        grid.get_cell(start_pos.row + 1, start_pos.col - 1) == 'M' &&
        grid.get_cell(start_pos.row + 2, start_pos.col - 2) == 'A' &&
        grid.get_cell(start_pos.row + 3, start_pos.col - 3) == 'S'
    }

    fn has_west_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.width >= 4 &&
        start_pos.col >= 3 &&
        grid.get_cell(start_pos.row, start_pos.col - 0) == 'X' &&
        grid.get_cell(start_pos.row, start_pos.col - 1) == 'M' &&
        grid.get_cell(start_pos.row, start_pos.col - 2) == 'A' &&
        grid.get_cell(start_pos.row, start_pos.col - 3) == 'S'
    }

    fn has_north_west_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.height >= 4 &&
        grid.width >= 4 &&
        start_pos.row >= 3 &&
        start_pos.col >= 3 &&
        grid.get_cell(start_pos.row - 0, start_pos.col - 0) == 'X' &&
        grid.get_cell(start_pos.row - 1, start_pos.col - 1) == 'M' &&
        grid.get_cell(start_pos.row - 2, start_pos.col - 2) == 'A' &&
        grid.get_cell(start_pos.row - 3, start_pos.col - 3) == 'S'
    }

    for r in 0..grid.height {
        for c in 0..grid.width {
            let start_pos = GridPos {row: r, col: c};
            if has_north_solution(&grid, start_pos) {
                solutions.push(Pt1WordSearchSolution{start_pos: start_pos, dir: Direction::N});
            }
            if has_north_east_solution(&grid, start_pos) {
                solutions.push(Pt1WordSearchSolution{start_pos: start_pos, dir: Direction::NE});
            }
            if has_east_solution(&grid, start_pos) {
                solutions.push(Pt1WordSearchSolution{start_pos: start_pos, dir: Direction::E});
            }
            if has_south_east_solution(&grid, start_pos) {
                solutions.push(Pt1WordSearchSolution{start_pos: start_pos, dir: Direction::SE});
            }
            if has_south_solution(&grid, start_pos) {
                solutions.push(Pt1WordSearchSolution{start_pos: start_pos, dir: Direction::S});
            }
            if has_south_west_solution(&grid, start_pos) {
                solutions.push(Pt1WordSearchSolution{start_pos: start_pos, dir: Direction::SW});
            }
            if has_west_solution(&grid, start_pos) {
                solutions.push(Pt1WordSearchSolution{start_pos: start_pos, dir: Direction::W});
            }
            if has_north_west_solution(&grid, start_pos) {
                solutions.push(Pt1WordSearchSolution{start_pos: start_pos, dir: Direction::NW});
            }
        }
    }

    solutions
}

fn find_pt2_word_search_solutions(grid: &Grid) -> Vec<Pt2WordSearchSolution> {
    let mut solutions: Vec<Pt2WordSearchSolution> = vec![];

    if grid.width < 3 || grid.height < 3 {
        // This grid is too small for any pt2 solutions
        //
        // M . S
        // . A .
        // M . S
        return solutions;
    }

    // M . S
    // . A .
    // M . S
    fn has_zero_rot_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.get_cell(start_pos.row - 1, start_pos.col - 1) == 'M' &&
        grid.get_cell(start_pos.row - 1, start_pos.col + 1) == 'S' &&
        grid.get_cell(start_pos.row + 0, start_pos.col + 0) == 'A' &&
        grid.get_cell(start_pos.row + 1, start_pos.col - 1) == 'M' &&
        grid.get_cell(start_pos.row + 1, start_pos.col + 1) == 'S' 
    }

    // M . M
    // . A .
    // S . S
    fn has_quarter_rot_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.get_cell(start_pos.row - 1, start_pos.col - 1) == 'M' &&
        grid.get_cell(start_pos.row - 1, start_pos.col + 1) == 'M' &&
        grid.get_cell(start_pos.row + 0, start_pos.col + 0) == 'A' &&
        grid.get_cell(start_pos.row + 1, start_pos.col - 1) == 'S' &&
        grid.get_cell(start_pos.row + 1, start_pos.col + 1) == 'S' 
    }

    // S . M
    // . A .
    // S . M
    fn has_half_rot_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.get_cell(start_pos.row - 1, start_pos.col - 1) == 'S' &&
        grid.get_cell(start_pos.row - 1, start_pos.col + 1) == 'M' &&
        grid.get_cell(start_pos.row + 0, start_pos.col + 0) == 'A' &&
        grid.get_cell(start_pos.row + 1, start_pos.col - 1) == 'S' &&
        grid.get_cell(start_pos.row + 1, start_pos.col + 1) == 'M' 
    }

    // S . S
    // . A .
    // M . M
    fn has_three_quarter_rot_solution(grid: &Grid, start_pos: GridPos) -> bool {
        grid.get_cell(start_pos.row - 1, start_pos.col - 1) == 'S' &&
        grid.get_cell(start_pos.row - 1, start_pos.col + 1) == 'S' &&
        grid.get_cell(start_pos.row + 0, start_pos.col + 0) == 'A' &&
        grid.get_cell(start_pos.row + 1, start_pos.col - 1) == 'M' &&
        grid.get_cell(start_pos.row + 1, start_pos.col + 1) == 'M' 
    }

    for r in 1..(grid.height-1) {
        for c in 1..(grid.width-1) {
            let start_pos = GridPos {row: r, col: c};
            if has_zero_rot_solution(&grid, start_pos) {
                solutions.push(Pt2WordSearchSolution{start_pos: start_pos, rot: Rotation::Zero});
            }
            if has_quarter_rot_solution(&grid, start_pos) {
                solutions.push(Pt2WordSearchSolution{start_pos: start_pos, rot: Rotation::Quarter});
            }
            if has_half_rot_solution(&grid, start_pos) {
                solutions.push(Pt2WordSearchSolution{start_pos: start_pos, rot: Rotation::Half});
            }
            if has_three_quarter_rot_solution(&grid, start_pos) {
                solutions.push(Pt2WordSearchSolution{start_pos: start_pos, rot: Rotation::ThreeQuarter});
            }
        }
    }

    solutions
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_grid(filename);
    let grid = match parse_result {
        Ok(grid) => grid,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    let pt1_start_time = std::time::Instant::now();
    let pt1_solutions = find_pt1_word_search_solutions(&grid);
    let pt1_time = pt1_start_time.elapsed();
    println!("Pt1. Found {} solutions", pt1_solutions.len());
    println!(
        "TIME: ({:0.06}s)",
        pt1_time.as_secs_f64()
    );
    let pt2_start_time = std::time::Instant::now();
    let pt2_solutions = find_pt2_word_search_solutions(&grid);
    let pt2_time = pt2_start_time.elapsed();
    println!("Pt2. Found {} solutions", pt2_solutions.len());
    println!(
        "TIME: ({:0.06}s)",
        pt2_time.as_secs_f64()
    );
    return ExitCode::SUCCESS;
}
