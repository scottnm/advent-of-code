use input_helpers;
use std::process::ExitCode;

type MemoryLine = String;

#[derive(PartialEq, Eq, Clone, Copy)]
enum ProcessorState {
    OpsEnabled,
    OpsDisabled
}

type MulOp = (isize, isize, ProcessorState);

fn read_memory_line(filename: &str) -> Result<MemoryLine, String> {
    match input_helpers::read_file_to_string(filename) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to read file! {}", e)),
    }
}

fn extract_mul_ops(memory_line: &str) -> Vec<MulOp> {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results: Vec<MulOp> = vec![];
    
    fn extract_mul_ops_chunk(re: &regex::Regex, chunk: &str, processor_state: ProcessorState, ops_list: &mut Vec<MulOp>) {
        for (_, [op1_cap, op2_cap]) in re.captures_iter(chunk).map(|c| c.extract()) {
            let op1: isize = op1_cap.parse().unwrap();
            let op2: isize = op2_cap.parse().unwrap();
            ops_list.push((op1, op2, processor_state))
        }
    }

    let dont_splits: Vec<&str> = memory_line.split("don't").collect();
    if !dont_splits.is_empty() {
        extract_mul_ops_chunk(&re, dont_splits[0], ProcessorState::OpsEnabled, &mut results);
    }
    for dont_chunk in dont_splits.iter().skip(1) {
        if let Some(do_chunk_index) = dont_chunk.find("do") {
            extract_mul_ops_chunk(&re, &dont_chunk[..do_chunk_index], ProcessorState::OpsDisabled, &mut results);
            extract_mul_ops_chunk(&re, &dont_chunk[do_chunk_index..], ProcessorState::OpsEnabled, &mut results);
        } else {
            extract_mul_ops_chunk(&re, &dont_chunk, ProcessorState::OpsDisabled, &mut results);
        }
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

    let start_time = std::time::Instant::now();

    let mul_ops = extract_mul_ops(&memory_line);
    println!("EXTRACT TIME: ({:0.06}s)", start_time.elapsed().as_secs_f64());

    // for mul_op in mul_ops.iter() {
    //     println!("+ ({} * {})", mul_op.0, mul_op.1)
    // }
    let mul_sum = mul_ops.iter().map(|mul_op| mul_op.0 * mul_op.1).fold(0, |acc, v| acc + v);
    println!("= {} [unfiltered]", mul_sum);

    // for mul_op in mul_ops.iter() {
    //     if mul_op.2 == ProcessorState::OpsEnabled {
    //         println!("+ ({} * {})", mul_op.0, mul_op.1)
    //     }
    // }
    let filtered_mul_sum = mul_ops.iter().filter(|mul_op| mul_op.2 == ProcessorState::OpsEnabled).map(|mul_op| mul_op.0 * mul_op.1).fold(0, |acc, v| acc + v);
    println!("= {} [filtered]", filtered_mul_sum);

    println!("TIME: ({:0.06}s)", start_time.elapsed().as_secs_f64());
    return ExitCode::SUCCESS;
}