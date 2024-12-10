use input_helpers;
use std::process::ExitCode;
use itertools::Itertools;

fn read_(filename: &str) -> Result<(), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_(filename);
    let result = match parse_result {
        Ok(result) => result,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    dbg!(&result);

    /*
    {
        let antinode_positions_pt1 = calculate_all_antinode_positions_pt1(&tower_grid);
        println!("Pt 1: antinode position count = {}", antinode_positions_pt1.len());
        if antinode_positions_pt1.len() < 10 {
            for p in antinode_positions_pt1 {
                println!("- (r:{},c:{})", p.row, p.col);
            }
        }
    } */

    /*
    println!("");

    {
        let antinode_positions_pt2 = calculate_all_antinode_positions_pt2(&tower_grid);
        println!("Pt 2: antinode position count = {}", antinode_positions_pt2.len());
        if antinode_positions_pt2.len() < 10 {
            for p in antinode_positions_pt2 {
                println!("- (r:{},c:{})", p.row, p.col);
            }
        }
    } */

    return ExitCode::SUCCESS;
}
