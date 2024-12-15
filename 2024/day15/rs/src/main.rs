use input_helpers;
use std::process::ExitCode;
use simple_grid::{Grid, GridPos};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Space {
    Empty,
    Box,
    Wall,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

type Warehouse = Grid<Space>;

fn read_input(filename: &str) -> Result<(Grid<Space>, GridPos, Vec<Move>), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() < 4 {
        return Err(format!("Invalid input! Require at least 4 lines. Had {}", lines.len()));
    }

    fn is_full_wall_line(line: &str) -> bool {
        !line.is_empty() && line.chars().all(|c| c == '#')
    }

    let first_grid_line = &lines[0];
    if !is_full_wall_line(first_grid_line) {
        return Err(format!("Invalid input! First line must be all '#'! Found '{}'", first_grid_line));
    }

    let last_grid_line_idx = lines[1..]
        .iter()
        .enumerate()
        .rev()
        .find(|(_idx, line)| is_full_wall_line(line))
        .map(|(idx, _line)| idx+1) // +1 since we start iterating from 1
        .ok_or(String::from("Missing trailing wall line"))?;

    let grid_lines = &lines[0..last_grid_line_idx+1];

    let separator_line_idx = last_grid_line_idx + 1;
    if separator_line_idx >= lines.len() || lines[separator_line_idx] != "" {
        return Err(format!("Expected empty line on line {}! Found '{}'", separator_line_idx, lines[separator_line_idx]));
    }

    let move_lines = &lines[separator_line_idx+1..];
    if move_lines.is_empty() {
        return Err(String::from("Missing move line(s)"));
    }

    let width = first_grid_line.len();
    let height = grid_lines.len();

    let mut robot_pos = None;
    let mut warehouse_cells: Vec<Space> = vec![];
    for (line_idx, line) in grid_lines.iter().enumerate() {
        if line.len() != width {
            return Err(format!(
                "Grid must have consistent line widths! Expected {} found {}",
                width,
                line.len()
            ));
        }

        for (chr_idx, c) in line.chars().enumerate() {
            if c == '@' {
                let new_robot_pos = GridPos {row: line_idx as isize, col: chr_idx as isize };
                if let Some(robot_pos) = robot_pos {
                    return Err(format!("Repeat robot_pos found {}! First found at {}", new_robot_pos, robot_pos));
                }

                robot_pos = Some(new_robot_pos);
            } else {
                let cell = match c {
                    '.' => Space::Empty,
                    'O' => Space::Box,
                    '#' => Space::Wall,
                    _ => return Err(format!("Invalid warehouse char! {}", c)),
                };
                warehouse_cells.push(cell);
            }
        }
    }

    let mut moves = vec![];
    for line in move_lines {
        for c in line.chars() {
            let move_instr = match c {
                '<' => Move::Left,
                '^' => Move::Up,
                '>' => Move::Right,
                'v' => Move::Down,
                _ => return Err(format!("Invalid move char! {}", c)),
            };

            moves.push(move_instr);
        }
    }

    let warehouse_grid  = Grid::<Space> {
        width: width,
        height: height,
        cells: warehouse_cells,
    };
    
    let result = (
        warehouse_grid,
        robot_pos.ok_or(String::from("Missing robot pos in input"))?,
        moves,
    );

    Ok(result)
}

fn dump_warehouse(warehouse: &Warehouse, robot_pos: &GridPos) -> String {
    let mut buf = String::with_capacity((warehouse.width+1) * warehouse.height);
    for r in 0..(warehouse.height as isize) {
        for c in 0..(warehouse.width as isize) {
            let cell_pos = GridPos {row: r, col: c};
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

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;

    let (warehouse, robot_pos, moves) = read_input(filename)?;

    dbg!(&warehouse.width);
    dbg!(&warehouse.height);
    dbg!(&warehouse.cells);
    dbg!(&robot_pos);
    dbg!(&moves);
    /*
    {
        let print_machines = claw_machines.len() < 10;
        let mut total_min_tokens: Option<usize> = None;
        for claw_machine in &claw_machines {
            let solutions = find_all_solutions(claw_machine);
            let min_cost_solution = solutions
                .iter()
                .enumerate()
                .map(|(i, solution)| (i, count_tokens_for_solution(solution)))
                .min_by_key(|(_i, solution_token_count)| solution_token_count.clone());
            if let Some((solution_idx, min_cost_solution_token_count)) = min_cost_solution {
                if let Some(token_count) = total_min_tokens {
                    total_min_tokens = Some(token_count + min_cost_solution_token_count);
                } else {
                    total_min_tokens = Some(min_cost_solution_token_count);
                }

                if print_machines {
                    println!("{}", claw_machine);
                    println!("{}", solutions[solution_idx]);
                    println!("");
                }
            } else {
                if print_machines {
                    println!("{}", claw_machine);
                    println!("NO SOLUTION");
                    println!("");
                }
            }
        }

        if let Some(total_min_tokens) = total_min_tokens {
            println!("Pt 1: min token count = {}", total_min_tokens);
        } else {
            println!("Pt 1: min token count = NO SOLUTIONS");
        }
    } */

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