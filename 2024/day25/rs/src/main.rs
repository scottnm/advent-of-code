use input_helpers;
use regex;
use std::process::ExitCode;

#[derive(Clone, Copy, Debug)]
struct Lock {
    pin_heights: (u8, u8, u8, u8, u8),
}

#[derive(Clone, Copy, Debug)]
struct Key {
    notch_heights: (u8, u8, u8, u8, u8),
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

    let (locks, keys) = read_input(filename)?;

    dbg!(&locks);
    dbg!(&keys);

    {
        /*
        let result_wire_values = run_wire_operations(&operations, &initial_wire_states);
        let z_value = sum_wire_bits_as_binary_value('z', &result_wire_values);
        println!("Pt1. z value: {} ({:#b})", z_value, z_value); */
    }

    if do_pt2 {
        unimplemented!();
    }

    Ok(())
}

fn read_input(filename: &str) -> Result<(Vec<Lock>, Vec<Key>), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut locks = vec![];
    let mut keys = vec![];

    let mut line_idx = 0;
    while line_idx <= lines.len() - 7 {
        let item_lines = &lines[line_idx..line_idx+7];
        line_idx += 7;

        if line_idx < lines.len() {
            if lines[line_idx] != "" {
                return Err(format!("Unexpected line at {}! Expected empty separator. Found {}", 
                    line_idx, 
                    lines[line_idx]));
            }
            line_idx += 1;
        }

        if item_lines.first().unwrap() == "#####" {
            let lock = read_lock_lines(&item_lines[1..])?;
            locks.push(lock);
        } else if item_lines.last().unwrap() == "#####" {
            let key = read_key_lines(&item_lines[..item_lines.len()-1])?;
            keys.push(key);
        } else {
            return Err(format!("Invalid lines! Either first or last line in group should be all #'s! lines={:?}", 
                item_lines));
        }
    }

    Ok((locks, keys))
}

fn read_lock_lines(lines: &[String]) -> Result<Lock, String> {
    unimplemented!();
}

fn read_key_lines(lines: &[String]) -> Result<Key, String> {
    unimplemented!();
}