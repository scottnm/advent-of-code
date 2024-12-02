use input_helpers;
use std::process::ExitCode;

type InputPair = (isize, isize);

fn read_input_pairs_from_file(filename: &str) -> Result<Vec<InputPair>, String> {
    let lines = input_helpers::read_lines(filename);
    let mut pairs: Vec<InputPair> = Vec::new();
    for line in lines {
        let values: Vec<&str> = line.split_ascii_whitespace().collect();
        if values.len() != 2 {
            return Err(format!(
                "Invalid number of values on line. Expected 2 found {}. line={}",
                values.len(),
                line
            ));
        }

        fn parse_pair_value(v: &str, line: &str) -> Result<isize, String> {
            match v.parse() {
                Ok(value) => Ok(value),
                Err(e) => Err(format!("Invalid value: {}! '{}' line={}", e, v, line)),
            }
        }

        let v1 = parse_pair_value(values[0], &line)?;
        let v2 = parse_pair_value(values[1], &line)?;
        pairs.push((v1, v2));
    }

    Ok(pairs)
}

fn calculate_total_input_pair_distance(input_pairs: &[InputPair]) -> usize {
    let mut first_list: Vec<isize> = input_pairs.iter().map(|(v1, _)| *v1).collect();
    first_list.sort();
    let first_list = first_list;

    let mut second_list: Vec<isize> = input_pairs.iter().map(|(_, v2)| *v2).collect();
    second_list.sort();
    let second_list = second_list;

    let mut total_dist = 0;
    for (v1, v2) in first_list.iter().zip(second_list.iter()) {
        let dist = (v1 - v2).abs() as usize;
        total_dist += dist;
    }

    total_dist
}

fn calculate_similarity_score(input_pairs: &[InputPair]) -> usize {
    let mut second_list_counts = std::collections::HashMap::<isize, usize>::new();
    for (_, v2) in input_pairs {
        if let Some(v) = second_list_counts.get_mut(v2) {
            *v += 1;
        } else {
            second_list_counts.insert(*v2, 1);
        }
    }

    let mut total_similarity_score = 0;
    for (v1, _) in input_pairs.iter().cloned() {
        let similarity_score = (v1 as usize) * second_list_counts.get(&v1).unwrap_or(&0);
        total_similarity_score += similarity_score;
    }

    total_similarity_score
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_input_pairs_from_file(filename);
    let input_pairs = match parse_result {
        Ok(parsed_pairs) => parsed_pairs,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    let total_distance = calculate_total_input_pair_distance(&input_pairs);
    println!("Total distance: {}", total_distance);

    let similarity_score = calculate_similarity_score(&input_pairs);
    println!("Similarity score: {}", similarity_score);

    return ExitCode::SUCCESS;
}
