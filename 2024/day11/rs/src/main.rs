use input_helpers;
use itertools::Itertools;
use std::process::ExitCode;

type StoneVal = usize;

fn dump_stones(stones: &[StoneVal]) {
    println!("{:?}", stones);
}

fn read_stone_arrangement(filename: &str) -> Result<Vec<StoneVal>, String> {
    let file_data = match input_helpers::read_file_to_string(filename) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to read input! {}", e)),
    };

    let mut stones: Vec<StoneVal> = vec![];
    for stone_val_str in file_data.split_ascii_whitespace() {
        let stone_val = match stone_val_str.parse() {
            Ok(stone_val) => stone_val,
            Err(e) => return Err(format!("Failed to parse stone value! {}", e)),
        };
        stones.push(stone_val);
    }

    Ok(stones)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_stone_arrangement(filename);
    let stones = match parse_result {
        Ok(stones) => stones,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    dump_stones(&stones);

    /*
    {
        let trails = find_all_trails_pt1(&trail_map);
        let trailhead_scores: Vec<usize> = trails
            .iter()
            .map(|(_trail_start, trail_ends)| trail_ends.len())
            .collect();
        let trailhead_score_sum: usize = trailhead_scores.iter().sum();
        println!("Pt 1: trailhead_score_sum = {}", trailhead_score_sum);
        if trails.len() < 20 {
            for trail in &trails {
                println!("- start={}; trail={:?}", trail.0, trail.1);
            }
        }
    } */

    /*
    println!("");

    {
        let trails = find_all_trails_pt2(&trail_map);
        let trailhead_ratings: Vec<usize> = trails
            .iter()
            .map(|(_trail_start, trail_ends)| trail_ends.len())
            .collect();
        let trailhead_rating_sum: usize = trailhead_ratings.iter().sum();
        println!("Pt 2: trailhead_rating_sum = {}", trailhead_rating_sum);
        if trails.len() < 20 {
            for trail in &trails {
                println!("- start={}; trail={:?}", trail.0, trail.1);
            }
        }
    } */

    return ExitCode::SUCCESS;
}
