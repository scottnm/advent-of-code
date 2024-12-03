use input_helpers;
use std::process::ExitCode;

type MemoryLine = String;

type MulOp = (isize, isize);

fn read_memory_lines(filename: &str) -> Result<Vec<MemoryLine>, String> {
    let lines = input_helpers::read_lines(filename);
    let mut memory_lines: Vec<MemoryLine> = Vec::new();
    for line in lines {
        memory_lines.push(line);
    }
    Ok(memory_lines)
}

fn extract_mul_ops(report_data: &[MemoryLine]) -> Vec<MulOp> {
    unimplemented!();
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_memory_lines(filename);
    let memory_lines = match parse_result {
        Ok(parsed_memory_lines) => parsed_memory_lines,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    let mul_ops = extract_mul_ops(&memory_lines);
    let mul_sum = mul_ops.iter().map(|mul_op| mul_op.0 * mul_op.1).fold(0, |acc, v| acc + v);
    println!("RESULT: {}", mul_sum);
    for (i, mul_op) in mul_ops.iter().enumerate() {
        if i == 0 {
            print!("    = ");
        } else {
            print!(" + ");
        }
        print!("({} * {})", mul_op.0, mul_op.1)
    }

    return ExitCode::SUCCESS;
}