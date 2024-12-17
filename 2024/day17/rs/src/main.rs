use input_helpers;
use simple_grid::{Grid, GridPos};
use std::process::ExitCode;

struct CpuState {
    instruction_pointer: usize,
    reg_a: isize,
    reg_b: isize,
    reg_c: isize,
}

enum Operand {
    Lit1,
    Lit2,
    Lit3,
    Lit4,
    Lit5,
    Lit6,
    Lit7,
    ComboLit0,
    ComboLit1,
    ComboLit2,
    ComboLit3,
    ComboRegA,
    ComboRegB,
    ComboRegC,
}

impl Operand {
    fn parse_literal_op(c: char) -> Result<Self, String> {
        let op = match c {
            '1' => Operand::Lit1,
            '2' => Operand::Lit2,
            '3' => Operand::Lit3,
            '4' => Operand::Lit4,
            '5' => Operand::Lit5,
            '6' => Operand::Lit6,
            '7' => Operand::Lit7,
            _ => return Err(format!("Invalid literal op '{}'", c)),
        };

        Ok(op)
    }

    fn parse_combo_op(c: char) -> Result<Self, String> {
        let op = match c {
            '0' => Operand::ComboLit0,
            '1' => Operand::ComboLit1,
            '2' => Operand::ComboLit2,
            '3' => Operand::ComboLit3,
            '4' => Operand::ComboRegA,
            '5' => Operand::ComboRegB,
            '6' => Operand::ComboRegC,
            '7' => return Err(String::from("Combo operator 7 is reserved and invalid")),
            _ => return Err(format!("Invalid literal op '{}'", c)),
        };

        Ok(op)
    }
}

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
        return Err(format!(
            "Invalid input! Require at least 4 lines. Had {}",
            lines.len()
        ));
    }

    fn is_full_wall_line(line: &str) -> bool {
        !line.is_empty() && line.chars().all(|c| c == '#')
    }

    let first_grid_line = &lines[0];
    if !is_full_wall_line(first_grid_line) {
        return Err(format!(
            "Invalid input! First line must be all '#'! Found '{}'",
            first_grid_line
        ));
    }

    let last_grid_line_idx = lines[1..]
        .iter()
        .enumerate()
        .rev()
        .find(|(_idx, line)| is_full_wall_line(line))
        .map(|(idx, _line)| idx + 1) // +1 since we start iterating from 1
        .ok_or(String::from("Missing trailing wall line"))?;

    let grid_lines = &lines[0..last_grid_line_idx + 1];

    let separator_line_idx = last_grid_line_idx + 1;
    if separator_line_idx >= lines.len() || lines[separator_line_idx] != "" {
        return Err(format!(
            "Expected empty line on line {}! Found '{}'",
            separator_line_idx, lines[separator_line_idx]
        ));
    }

    let move_lines = &lines[separator_line_idx + 1..];
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
                let new_robot_pos = GridPos {
                    row: line_idx as isize,
                    col: chr_idx as isize,
                };
                if let Some(robot_pos) = robot_pos {
                    return Err(format!(
                        "Repeat robot_pos found {}! First found at {}",
                        new_robot_pos, robot_pos
                    ));
                }

                warehouse_cells.push(Space::Empty);
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

    let warehouse_grid = Grid::<Space> {
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

fn do_move(warehouse: &mut Warehouse, robot_pos: &mut GridPos, move_instr: Move) {
    let (row_offset, col_offset) = match move_instr {
        Move::Up => (-1, 0),
        Move::Left => (0, -1),
        Move::Down => (1, 0),
        Move::Right => (0, 1),
    };

    let next_cell_pos = GridPos {
        row: robot_pos.row + row_offset,
        col: robot_pos.col + col_offset,
    };
    if warehouse.is_pos_out_of_bounds(next_cell_pos.row, next_cell_pos.col) {
        return;
    }

    fn recursive_move_boxes(
        row_offset: isize,
        col_offset: isize,
        warehouse: &mut Warehouse,
        box_pos: &GridPos,
    ) -> bool {
        let next_cell_pos = GridPos {
            row: box_pos.row + row_offset,
            col: box_pos.col + col_offset,
        };
        if warehouse.is_pos_out_of_bounds(next_cell_pos.row, next_cell_pos.col) {
            return false;
        }

        match warehouse.get_cell(next_cell_pos.row, next_cell_pos.col) {
            Space::Wall => false,
            Space::Box => {
                if recursive_move_boxes(row_offset, col_offset, warehouse, &next_cell_pos) {
                    *warehouse.get_cell_mut(next_cell_pos.row, next_cell_pos.col) = Space::Box;
                    *warehouse.get_cell_mut(box_pos.row, box_pos.col) = Space::Empty;
                    true
                } else {
                    false
                }
            }
            Space::Empty => {
                *warehouse.get_cell_mut(next_cell_pos.row, next_cell_pos.col) = Space::Box;
                *warehouse.get_cell_mut(box_pos.row, box_pos.col) = Space::Empty;
                true
            }
        }
    }

    match warehouse.get_cell(next_cell_pos.row, next_cell_pos.col) {
        Space::Wall => (),                          // no move for wall
        Space::Empty => *robot_pos = next_cell_pos, // move into the empty space
        Space::Box => {
            if recursive_move_boxes(row_offset, col_offset, warehouse, &next_cell_pos) {
                *robot_pos = next_cell_pos;
            }
        }
    }
}

fn calc_box_gps(box_pos: &GridPos) -> usize {
    ((100 * box_pos.row) + box_pos.col) as usize
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();

    let (mut warehouse, mut robot_pos, moves) = read_input(filename)?;

    {
        if verbose {
            print_warehouse(Some("warehouse start"), &warehouse, &robot_pos);
        }

        for (i, move_instr) in moves.iter().enumerate() {
            do_move(&mut warehouse, &mut robot_pos, *move_instr);
            if verbose {
                if i < moves.len() {
                    print_warehouse(
                        Some(&format!("after move {:03}", i)),
                        &warehouse,
                        &robot_pos,
                    );
                } else {
                    print_warehouse(Some("warehouse end"), &warehouse, &robot_pos);
                }
            }
        }

        let mut sum_gps_coords = 0;
        for r in 0..warehouse.height as isize {
            for c in 0..warehouse.width as isize {
                if let Space::Box = warehouse.get_cell(r, c) {
                    let box_pos = GridPos { row: r, col: c };
                    let box_gps = calc_box_gps(&box_pos);
                    sum_gps_coords += box_gps;
                }
            }
        }

        println!("Pt 1: sum gps = {}", sum_gps_coords);
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
