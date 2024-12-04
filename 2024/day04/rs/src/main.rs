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

#[derive(Clone, Copy)]
struct GridPos {
    row: usize,
    col: usize,
}

struct WordSearchSolution {
    #[allow(dead_code)] // FIXME: not using these fields yet
    start_pos: GridPos,
    #[allow(dead_code)] // FIXME: not using these fields yet
    dir: Direction,
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

fn find_word_search_solutions(grid: &Grid) -> Vec<WordSearchSolution> {
    let mut solutions: Vec<WordSearchSolution> = vec![];

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
                solutions.push(WordSearchSolution{start_pos: start_pos, dir: Direction::N});
            }
            if has_north_east_solution(&grid, start_pos) {
                solutions.push(WordSearchSolution{start_pos: start_pos, dir: Direction::NE});
            }
            if has_east_solution(&grid, start_pos) {
                solutions.push(WordSearchSolution{start_pos: start_pos, dir: Direction::E});
            }
            if has_south_east_solution(&grid, start_pos) {
                solutions.push(WordSearchSolution{start_pos: start_pos, dir: Direction::SE});
            }
            if has_south_solution(&grid, start_pos) {
                solutions.push(WordSearchSolution{start_pos: start_pos, dir: Direction::S});
            }
            if has_south_west_solution(&grid, start_pos) {
                solutions.push(WordSearchSolution{start_pos: start_pos, dir: Direction::SW});
            }
            if has_west_solution(&grid, start_pos) {
                solutions.push(WordSearchSolution{start_pos: start_pos, dir: Direction::W});
            }
            if has_north_west_solution(&grid, start_pos) {
                solutions.push(WordSearchSolution{start_pos: start_pos, dir: Direction::NW});
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

    let start_time = std::time::Instant::now();
    let solutions = find_word_search_solutions(&grid);
    println!("Found {} solutions", solutions.len());
    println!(
        "TIME: ({:0.06}s)",
        start_time.elapsed().as_secs_f64()
    );
    return ExitCode::SUCCESS;
}
