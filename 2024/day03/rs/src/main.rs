use input_helpers;
use std::process::ExitCode;

type MemoryLine = String;

type MulOp = (isize, isize);

fn read_memory_line(filename: &str) -> Result<MemoryLine, String> {
    match input_helpers::read_file_to_string(filename) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to read file! {}", e)),
    }
}

fn filter_out_disabled_ops(memory_line: &str) -> String {
    let mut filtered_memory_line = String::new();
    let dont_splits: Vec<&str> = memory_line.split("don't").collect();
    if !dont_splits.is_empty() {
        filtered_memory_line.push_str(dont_splits[0]);
    }
    for dont_chunk in dont_splits.iter().skip(1) {
        if let Some(do_chunk_index) = dont_chunk.find("do") {
            filtered_memory_line.push_str(&dont_chunk[do_chunk_index..]);
        }
    }
    filtered_memory_line
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
    // for mul_op in mul_ops.iter() {
    //     println!("+ ({} * {})", mul_op.0, mul_op.1)
    // }
    println!("= {} [unfiltered]", mul_sum);

    let filtered_memory_line = filter_out_disabled_ops(&memory_line);
    let filtered_mul_ops = extract_mul_ops(&filtered_memory_line);
    let filtered_mul_sum = filtered_mul_ops.iter().map(|mul_op| mul_op.0 * mul_op.1).fold(0, |acc, v| acc + v);
    // for mul_op in filtered_mul_ops.iter() {
    //     println!("+ ({} * {})", mul_op.0, mul_op.1)
    // }
    println!("= {} [filtered]", filtered_mul_sum);

    return ExitCode::SUCCESS;
}