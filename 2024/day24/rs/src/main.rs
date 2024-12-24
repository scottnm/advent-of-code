use input_helpers;
use std::process::ExitCode;
use regex;

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

    let (initial_wire_states, operations) = read_input(filename)?;

    dbg!(&initial_wire_states);
    dbg!(&operations);

    {
        let result_wire_values = run_wire_operations(&operations, &initial_wire_states);
        
        let mut next_z_wire_position = 0;
        let mut z_value = 0;
        loop {
            let next_z_wire_name = format!("z{:02}", next_z_wire_position);
            if let Some(next_z_wire_set) = result_wire_values.get(&next_z_wire_name) {
                if *next_z_wire_set {
                    let bit_value = (2 as usize).pow(next_z_wire_position);
                    z_value += bit_value;
                    if verbose {
                        println!("z-bit {} set! New z-value {} ({})", next_z_wire_name, z_value, z_value);
                    }
                } else {
                    if verbose {
                        println!("z-bit {} not set. Z-value still {} ({})", next_z_wire_name, z_value, z_value);
                    }
                }
            } else {
                if verbose {
                    println!("Z-bit {} not found! Assuming end of z-bits", next_z_wire_name);
                }
                break;
            }
            next_z_wire_position += 1;
        }

        println!("Pt1. z value: {} ({:#b})", z_value, z_value);
        /*
        let parties = find_3p_parties(&connections);
        let mut parties_with_chief = 0;
        for p in parties {
            if party_has_chief(&p) {
                parties_with_chief += 1;
                if verbose {
                    println!(" - {},{},{} (HAS CHIEF)", p.0, p.1, p.2);
                }
            } else {
                if verbose {
                    println!(" - {},{},{}", p.0, p.1, p.2);
                }
            }
        }
        println!("Pt1. # parties with chief: {}", parties_with_chief);
        */
    }

    if do_pt2 {
        unimplemented!();
    }

    Ok(())
}

type WireValues = std::collections::HashMap<String, bool>;

#[derive(Debug, Clone)]
struct Operation {
    op: OperationType,
    wire_a: String,
    wire_b: String,
    result_wire: String,
}

#[derive(Debug, Clone)]
enum OperationType {
    And,
    Or,
    Xor,
}

fn read_input(filename: &str) -> Result<(WireValues, Vec<Operation>), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut initial_wire_values = WireValues::new();
    let mut operations = vec![];

    let separator_line_idx = if let Some(separator_line_idx) = lines.iter().position(|l| l == "") {
        separator_line_idx
    } else {
        return Err(String::from("Missing separator line"));
    };

    let wire_value_line_re = regex::Regex::new(r"(\w{3}): (1|0)").unwrap();
    let operation_line_re = regex::Regex::new(r"(\w{3}) (AND|OR|XOR) (\w{3}) -> (\w{3})").unwrap();

    let init_wire_value_lines = &lines[0..separator_line_idx];
    for line in init_wire_value_lines {
        let wire_line_match = wire_value_line_re
            .captures(line)
            .ok_or(format!("Invalid wire line {}", line))?;
        let wire = wire_line_match
            .get(1)
            .unwrap()
            .as_str();
        let is_wire_set_value = wire_line_match
            .get(2)
            .unwrap()
            .as_str();
        let is_wire_set = match is_wire_set_value {
            "1" => true,
            "0" => false,
            _ => panic!("Unexpected wire set value {}", is_wire_set_value),
        };

        let old_wire_value = initial_wire_values.insert(wire.to_string(), is_wire_set);
        if let Some(old_wire_value) = old_wire_value {
            return Err(format!("Wire value {} initialized twice! (first {}, then {})", 
                wire, 
                old_wire_value, 
                is_wire_set));
        }
    }

    let operation_lines = &lines[separator_line_idx+1..];

    for line in operation_lines {
        let operation_line_match = operation_line_re 
            .captures(line)
            .ok_or(format!("Invalid operation line {}", line))?;

        let wire_a = operation_line_match
            .get(1)
            .unwrap()
            .as_str();

        let operation_type_value = operation_line_match  
            .get(2)
            .unwrap()
            .as_str();

        let wire_b = operation_line_match
            .get(3)
            .unwrap()
            .as_str();

        let result_wire = operation_line_match
            .get(4)
            .unwrap()
            .as_str();

        let operation_type = match operation_type_value {
            "AND" => OperationType::And,
            "OR" => OperationType::Or,
            "XOR" => OperationType::Xor,
            _ => panic!("Unexpected operation type match {}", operation_type_value),
        };

        let operation = Operation { op: operation_type, wire_a: wire_a.to_string(), wire_b: wire_b.to_string(), result_wire: result_wire.to_string() };
        operations.push(operation);
    }

    Ok((initial_wire_values, operations))
}

fn run_wire_operations(operations: &[Operation], initial_wire_values: &WireValues) -> WireValues {
    let mut wire_values = initial_wire_values.clone();

    let mut operations_remaining = operations.to_vec();

    // FIXME: rather than looping and relooping, there are smarter ways to process this to run faster.
    // For every new result, check which operations are affected by that result and do those first
    while !operations_remaining.is_empty() {
        let prev_operations_remaining_count = operations_remaining.len();

        let mut next_operation_idx = 0;
        while next_operation_idx < operations_remaining.len() {
            let next_operation = &operations_remaining[next_operation_idx];

            // any remaining operations should not write to an already calculated wire value
            assert!(!wire_values.contains_key(&next_operation.result_wire));

            let mut operation_completed = false;
            if let Some(wire_a_set) = wire_values.get(&next_operation.wire_a) {
                if let Some(wire_b_set) = wire_values.get(&next_operation.wire_b) {
                    let result = match next_operation.op {
                        OperationType::And => *wire_a_set && *wire_b_set,
                        OperationType::Or => *wire_a_set || *wire_b_set,
                        OperationType::Xor => *wire_a_set != *wire_b_set,
                    };

                    wire_values.insert(next_operation.result_wire.clone(), result);
                    operation_completed = true;
                }
            }

            if operation_completed {
                operations_remaining.remove(next_operation_idx);
            } else {
                next_operation_idx += 1;
            }
        }

        // This has to change across loop iterations or we've gotten stuck and aren't making progress.
        assert!(operations_remaining.len() != prev_operations_remaining_count);
    }

    wire_values
}