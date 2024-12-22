use input_helpers;
use std::process::ExitCode;

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

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let secret_gen_count: usize = input_helpers::get_nth_parsed_arg(args, 1)?;
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
        let final_secret_values: Vec<u64> = initial_secret_values
            .iter()
            .map(|secret| do_secret_gen(*secret, secret_gen_count))
            .collect();

        let final_secret_values_sum: u64 = final_secret_values.iter().sum();

        if initial_secret_values.len() <  20 {
            println!("after {} secret gen steps...", secret_gen_count);
            for (initial_secret, final_secret) in initial_secret_values.iter().zip(final_secret_values.iter()) {
                println!("{}: {}", initial_secret, final_secret);
            }
        }

        println!("pt 1: {}th secret sums = {}", secret_gen_count, final_secret_values_sum);
    }

    if do_pt2 {
        unimplemented!();
    }

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<u64>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut initial_secrets: Vec<u64> = vec![];

    for line in lines {
        let next_secret: u64 = line.parse().map_err(|_| format!("Failed to parse '{}' as u64", line))?;
        initial_secrets.push(next_secret);
    }

    Ok(initial_secrets)
}

fn do_secret_gen(initial_secret_value: u64, secret_gen_count: usize) -> u64 {
    unimplemented!();
}