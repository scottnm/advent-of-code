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

#[derive(Debug)]
struct OperationData {
    wire_1: String,
    wire_2: String,
    result_wire: String,
}

#[derive(Debug)]
enum Operation {
    And(OperationData),
    Or(OperationData),
    Xor(OperationData),
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

        let wire_1 = operation_line_match
            .get(1)
            .unwrap()
            .as_str();

        let operation_type = operation_line_match  
            .get(2)
            .unwrap()
            .as_str();

        let wire_2 = operation_line_match
            .get(3)
            .unwrap()
            .as_str();

        let result_wire = operation_line_match
            .get(4)
            .unwrap()
            .as_str();

        let operation_data = OperationData { wire_1: wire_1.to_string(), wire_2: wire_2.to_string(), result_wire: result_wire.to_string() };
        let operation = match operation_type {
            "AND" => Operation::And(operation_data),
            "OR" => Operation::Or(operation_data),
            "XOR" => Operation::Xor(operation_data),
            _ => panic!("Unexpected operation type match {}", operation_type),
        };

        operations.push(operation);
    }

    Ok((initial_wire_values, operations))
}

fn run_wire_operations(operations: &[Operation], initial_wire_values: &WireValues) -> WireValues {
    let mut wire_values = initial_wire_values.clone();
    wire_values
}