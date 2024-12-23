use input_helpers;
use std::process::ExitCode;

type CpuName = [char;2];

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
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();
    let do_pt2 = args
        .iter()
        .find(|a| a.as_str() == "-2" || a.as_str() == "--pt2")
        .is_some();

    let connections = read_input(filename)?;
    dbg!(&connections); 

    /*
    {
        let final_secret_values: Vec<u64> = initial_secret_values
            .iter()
            .map(|secret| gen_nth_secret(*secret, secret_gen_count))
            .collect();

        let final_secret_values_sum: u64 = final_secret_values.iter().sum();

        if verbose || initial_secret_values.len() < 20 {
            println!("after {} secret gen steps...", secret_gen_count);
            for (initial_secret, final_secret) in
                initial_secret_values.iter().zip(final_secret_values.iter())
            {
                println!("{}: {}", initial_secret, final_secret);
            }
        }

        println!(
            "pt 1: {}th secret sums = {}",
            secret_gen_count, final_secret_values_sum
        );
    } */

    if do_pt2 {
        unimplemented!();
    }

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<(String,String)>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut connections: Vec<(String, String)> = vec![];

    for line in lines {
        let mut split_itr = line.split('-');
        let cpu1 = split_itr.next().ok_or(format!("Missing first cpu on line {}", line))?;
        let cpu2 = split_itr.next().ok_or(format!("Missing second cpu on line {}", line))?;
        if let Some(v) = split_itr.next() {
            return Err(format!("Unexpected values on line {}", line));
        }

        connections.push((cpu1.to_string(), cpu2.to_string()));
    }

    Ok(connections)
}