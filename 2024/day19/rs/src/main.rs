use input_helpers;
use std::{env::var, process::ExitCode};

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
            return Err(format!(
                "Invalid towel pattern char '{}'",
                invalid_stripe_char
            ));
        }
    }

    let target_designs: Vec<String> = lines[2..].iter().cloned().collect();

    Ok((towel_patterns, target_designs))
}

fn is_target_design_possible(target_design: &str, available_patterns: &[TowelPattern]) -> bool {
    if !is_valid_stripe_sequence(target_design) {
        return false;
    }

    fn is_target_design_possible_helper(target_design: &str, available_patterns: &[TowelPattern]) -> bool {
        if target_design == "" {
            return true;
        }

        for available_pattern in available_patterns {
            if target_design.starts_with(available_pattern) {
                let design_possible = is_target_design_possible(
                    &target_design[available_pattern.len()..],
                    available_patterns,
                );
                if design_possible {
                    return true;
                }
            }
        }

        false
    }

    is_target_design_possible_helper(target_design, available_patterns)
}

type DesignVariantMemoizer = std::collections::HashMap<String, usize>;

fn count_and_memo_possible_target_design_variants(
    target_design: &str, 
    available_patterns: &[TowelPattern],
    variant_memo: &mut DesignVariantMemoizer) -> usize {
    if !is_valid_stripe_sequence(target_design) {
        return 0;
    }

    fn count_possible_target_design_variants_helper(
        target_design: &str, 
        available_patterns: &[TowelPattern],
        variant_memo: &mut DesignVariantMemoizer) -> usize {

        if target_design == "" {
            return 1;
        }

        if let Some(memod_count) = variant_memo.get(target_design) {
            return *memod_count;
        }

        let mut possible_design_count = 0;
        for available_pattern in available_patterns {
            if target_design.starts_with(available_pattern) {
                possible_design_count += count_possible_target_design_variants_helper(
                    &target_design[available_pattern.len()..],
                    available_patterns,
                    variant_memo,
                );
            }
        }

        variant_memo.insert(target_design.to_string(), possible_design_count);
        possible_design_count
    }

    count_possible_target_design_variants_helper(target_design, available_patterns, variant_memo)
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

    let (available_patterns, target_designs) = read_input(filename)?;

    dbg!(&available_patterns);
    dbg!(&target_designs);

    let possible_designs = {
        let possible_designs: Vec<TargetDesign> = target_designs
            .iter()
            .filter(|design| is_target_design_possible(design, &available_patterns))
            .cloned()
            .collect();

        println!("Pt 1: {} designs possible", possible_designs.len());
        if verbose {
            println!("possible designs:");
            for design in &possible_designs {
                println!("  - {}", design);
            }
        }

        possible_designs
    };

    if do_pt2 {
        let mut variant_count_memo = DesignVariantMemoizer::new();
        let possible_design_variant_counts: Vec<usize> = possible_designs
            .iter()
            .map(|design| count_and_memo_possible_target_design_variants(&design, &available_patterns, &mut variant_count_memo))
            .collect();
        let sum_total_design_variant_counts: usize = possible_design_variant_counts.iter().sum();
        println!("Pt 2: {} sum total design variants", sum_total_design_variant_counts);
        if verbose {
            println!("variant counts:");
            for (design, variant_count) in possible_designs.iter().zip(possible_design_variant_counts.iter()) {
                println!("  - ({}) {}", variant_count, design);
            }
        }
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
