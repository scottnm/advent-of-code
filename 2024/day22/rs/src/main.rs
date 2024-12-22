use input_helpers;
use std::process::ExitCode;

fn read_input(filename: &str) -> Result<Vec<u64>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut initial_secrets: Vec<u64> = vec![];

    for line in lines {
        let next_secret: u64 = line.parse().map_err(|_| format!("Failed to parse '{}' as u64", line))?;
        initial_secrets.push(next_secret);
    }

    Ok(initial_secrets)
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();
    let do_pt2 = args
        .iter()
        .find(|a| a.as_str() == "-2" || a.as_str() == "--pt2")
        .is_some();

    let initial_secret_values = read_input(filename)?;

    dbg!(&initial_secret_values);

    {
        /*
        let mut design_test_memo = DesignTestMemoizer::new();
        let possible_designs: Vec<TargetDesign> = target_designs
            .iter()
            .filter(|design| {
                is_target_design_possible(design, &available_patterns, &mut design_test_memo)
            })
            .cloned()
            .collect();

        println!("Pt 1: {} designs possible", possible_designs.len());
        if verbose {
            println!("possible designs:");
            for design in &possible_designs {
                println!("  - {}", design);
            }
        }

        possible_designs */
    }

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
