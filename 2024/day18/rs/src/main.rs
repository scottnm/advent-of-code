use input_helpers;
use simple_grid::{Grid, GridPos};
use std::process::ExitCode;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Space {
    Safe,
    Corrupted,
}

type ComputerSafetyGrid = Grid<Space>;

fn read_input(filename: &str) -> Result<(Grid<Space>, Vec<GridPos>), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() < 2 {
        return Err(format!(
            "Invalid input! Require at least 2 lines. Had {}",
            lines.len()
        ));
    }

    let (width, height) = {
        let dimension_line = &lines[0];
        let mut dimension_line_split = dimension_line.split_ascii_whitespace();
        let width_str = dimension_line_split
            .next()
            .ok_or(String::from("Missing width value on first line"))?;
        let height_str = dimension_line_split
            .next()
            .ok_or(String::from("Missing height value on first line"))?;
        if let Some(v) = dimension_line_split.next() {
            return Err(format!(
                "Unexpected values on first line after height: {}! '{}'",
                v, dimension_line
            ));
        }

        let width: isize = width_str
            .parse()
            .map_err(|_| format!("couldn't parse width value! '{}'", width_str))?;
        let height: isize = height_str
            .parse()
            .map_err(|_| format!("couldn't parse height value! '{}'", height_str))?;
        (width, height)
    };

    let cells = vec![Space::Safe; (width * height) as usize];

    if lines[1] != "" {
        return Err(format!(
            "line 1 should be an empty separator! found {}",
            lines[1]
        ));
    }

    let corrupted_position_lines = &lines[2..];
    let mut corrupted_positions = vec![];

    for (i, pos_line) in corrupted_position_lines.iter().enumerate() {
        let line_num = i + 2;
        let mut pos_line_split = pos_line.split(',');
        let x_str = pos_line_split.next().ok_or(format!(
            "Missing x value on line {}! {}",
            line_num, pos_line
        ))?;
        let y_str = pos_line_split.next().ok_or(format!(
            "Missing y value on line {}! {}",
            line_num, pos_line
        ))?;
        if let Some(v) = pos_line_split.next() {
            return Err(format!(
                "Unexpected values on line {} after y value: {}! '{}'",
                line_num, v, pos_line
            ));
        }

        let x: isize = x_str
            .parse()
            .map_err(|_| format!("couldn't parse x value! '{}'", x_str))?;
        let y: isize = y_str
            .parse()
            .map_err(|_| format!("couldn't parse y value! '{}'", y_str))?;
        let corrupted_pos = GridPos { row: y, col: x };
        corrupted_positions.push(corrupted_pos);
    }

    let memory_safety_grid = Grid::<Space> {
        width: width as usize,
        height: height as usize,
        cells,
    };

    Ok((memory_safety_grid, corrupted_positions))
}

fn dump_memory_safety_grid(memory_grid: &Grid<Space>) -> String {
    let mut buf = String::with_capacity((memory_grid.width + 1) * memory_grid.height);
    for r in 0..(memory_grid.height as isize) {
        for c in 0..(memory_grid.width as isize) {
            let cell_char = match memory_grid.get_cell(r, c) {
                Space::Safe => '.',
                Space::Corrupted => '#',
            };
            buf.push(cell_char);
        }
        buf.push('\n');
    }
    buf
}

fn print_memory_safety_grid(title: Option<&str>, memory_grid: &Grid<Space>) {
    if let Some(title) = title {
        println!("{}: ", title);
    }
    println!("{}", dump_memory_safety_grid(memory_grid));
}

fn corrupt_bytes(memory_grid: &mut Grid<Space>, bytes: &[GridPos]) {
    unimplemented!();
}

fn find_min_safe_path_length(
    memory_grid: &Grid<Space>,
    start_pos: GridPos,
    end_pos: GridPos,
) -> Option<usize> {
    unimplemented!();
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let num_bytes_to_simulate: usize = input_helpers::get_nth_parsed_arg(args, 1)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();

    let (initial_memory_safety_grid, corrupted_bytes) = read_input(filename)?;
    
    // FIXME: remove
    dbg!(&initial_memory_safety_grid);
    dbg!(&corrupted_bytes);

    {
        let mut corrupted_memory_grid = initial_memory_safety_grid.clone();
        corrupt_bytes(
            &mut corrupted_memory_grid,
            &corrupted_bytes[..num_bytes_to_simulate],
        );

        let start_pos = GridPos { row: 0, col: 0 };
        let end_pos = GridPos {
            row: (corrupted_memory_grid.height - 1) as isize,
            col: (corrupted_memory_grid.width - 1) as isize,
        };
        let min_safe_path_length =
            find_min_safe_path_length(&corrupted_memory_grid, start_pos, end_pos);
        if let Some(min_safe_path_length) = min_safe_path_length {
            println!("Pt 1: min path len = {}", min_safe_path_length);
        } else {
            println!("Pt 1: min path len = NO SOLUTION");
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
