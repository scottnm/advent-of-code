use input_helpers;
use std::process::ExitCode;

type TowelPattern = String;
type TargetDesign = String;

fn find_invalid_stripe(stripe_seq: &str) -> Option<char> {
    for stripe_char in stripe_seq.chars() {
        match stripe_char {
            'w' | 'u' | 'b' | 'r' | 'g' => continue, // valid towel patterns
            _ => return Some(stripe_char),
        }
    }

    None
}

fn is_valid_stripe_sequence(stripe_seq: &str) -> bool {
    find_invalid_stripe(stripe_seq).is_none()
}

fn read_input(filename: &str) -> Result<(Vec<TowelPattern>, Vec<TargetDesign>), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() < 3 {
        return Err(format!(
            "Invalid input! Require at least 3 lines. Had {}",
            lines.len()
        ));
    }

    if lines[1] != "" {
        return Err(format!("line 2 must be an empty separator"));
    }

    let towel_patterns: Vec<String> = lines[0].split(", ").map(|s| s.to_string()).collect();
    for towel_pattern in &towel_patterns {
        if let Some(invalid_stripe_char) = find_invalid_stripe(&towel_pattern) {
            return Err(format!("Invalid towel pattern char '{}'", invalid_stripe_char));
        }
    }

    let target_designs: Vec<String> = lines[2..].iter().cloned().collect();

    Ok((towel_patterns, target_designs))
}

fn is_target_design_possible(target_design: &TargetDesign, available_patterns: &[TowelPattern]) -> bool {
    // FIXME: impl
    false
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();
    let skip_pt1 = args
        .iter()
        .find(|a| a.as_str() == "-n1" || a.as_str() == "--skip-pt1")
        .is_some();
    let do_pt2 = args
        .iter()
        .find(|a| a.as_str() == "-2" || a.as_str() == "--pt2")
        .is_some();

    let (available_patterns, target_designs) = read_input(filename)?;

    dbg!(&available_patterns);
    dbg!(&target_designs);

    if !skip_pt1 {
        let possible_designs: Vec<TargetDesign> = target_designs
            .iter()
            .filter(|design| is_target_design_possible(design, &available_patterns))
            .cloned()
            .collect();

        println!("Pt 1: {} designs possible", possible_designs.len());
        if verbose {
            println!("possible designs:");
            for design in possible_designs {
                println!("  - {}", design);
            }
        }
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
