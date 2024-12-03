use input_helpers;
use std::{io::Read, process::ExitCode};

type MemoryLine = String;

type MulOp = (isize, isize);

fn read_memory_line(filename: &str) -> Result<MemoryLine, String> {
    let mut memory_line = String::new();

    // let lines = input_helpers::read_lines(filename);
    // for line in lines {
    //     memory_line.push_str(&line);
    // }

    let mut file = std::fs::File::open(filename).unwrap();
    let res = file.read_to_string(&mut memory_line);
    match res {
        Ok(_) => Ok(memory_line),
        Err(e) => Err(format!("Failed to read file! {}", e)),
    }
}

fn extract_mul_ops(memory_line: &str) -> Vec<MulOp> {
    
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results: Vec<MulOp> = vec![];
    for (_, [op1, op2]) in re.captures_iter(memory_line).map(|c| c.extract()) {
        results.push((op1.parse().unwrap(), op2.parse().unwrap()))
    }
    results
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_memory_line(filename);
    let memory_line = match parse_result {
        Ok(parsed_memory_line) => parsed_memory_line,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    let mul_ops = extract_mul_ops(&memory_line);
    let mul_sum = mul_ops.iter().map(|mul_op| mul_op.0 * mul_op.1).fold(0, |acc, v| acc + v);
    for mul_op in mul_ops.iter() {
        println!("+ ({} * {})", mul_op.0, mul_op.1)
    }
    println!("= {}", mul_sum);

    return ExitCode::SUCCESS;
}