use input_helpers;
use std::process::ExitCode;

type UpdateRuleSet = std::collections::HashMap<usize, std::collections::HashSet<usize>>;

type ManualUpdate = Vec<usize>;

struct ManualUpdateRequest
{
    rules: UpdateRuleSet,
    updates: Vec<ManualUpdate>,
}

fn read_manual_update_request(filename: &str) -> Result<ManualUpdateRequest, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let input_separator_line_idx = match lines.iter().position(|line| line == "") {
        Some(idx) => idx,
        None => return Err(format!("Missing required separator line")),
    };

    let rule_lines = &lines[..input_separator_line_idx];
    let update_lines = &lines[input_separator_line_idx+1..];

    let mut rules = UpdateRuleSet::new();
    for rule_line in rule_lines {
        let rule_line_parts: Vec<&str> = rule_line.split('|').collect();
        // FIXME: validate enough parts
        // FIXME: validate parts are parseable
        let before_page: usize = rule_line_parts[0].parse().unwrap();
        let follow_page: usize = rule_line_parts[1].parse().unwrap();

        if let Some(follow_pages) = rules.get_mut(&before_page) {
            follow_pages.insert(follow_page);
        } else {
            let mut follow_pages = std::collections::HashSet::new();
            follow_pages.insert(follow_page);
            rules.insert(before_page, follow_pages);
        }
    }

    let mut updates: Vec<ManualUpdate> = Vec::with_capacity(update_lines.len());
    for update_line in update_lines {
        // FIXME: validate parse correctly
        let update: ManualUpdate = update_line.split(',').map(|page_num_str| page_num_str.parse().unwrap()).collect();
        updates.push(update);
    }

    Ok(ManualUpdateRequest{rules, updates})
}

fn is_update_in_correct_order(rules: &UpdateRuleSet, update: &ManualUpdate) -> bool {
    for (i, page) in update.iter().enumerate() {
        // For each page in the update, check all pages before it and see if any rules
        // would be violated by those pages
        if let Some(follow_pages) = rules.get(&page) {
            for page_before in &update[..i] {
                if follow_pages.contains(page_before) {
                    return false;
                }
            }
        }
    }

    true
}

fn correct_update_ordering(rules: &UpdateRuleSet, update: &ManualUpdate) -> ManualUpdate {
    let mut new_update = Vec::with_capacity(update.len());

    for page in update {
        let queried_insert_position: Option<usize> = 
            if let Some(follow_pages) = rules.get(page) {
                new_update.iter().position(|new_update_page| follow_pages.contains(new_update_page))
            } else {
                None
            };

        match queried_insert_position {
            Some(insert_pos) => new_update.insert(insert_pos, *page),
            None => new_update.push(*page),
        }
    }

    new_update
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_manual_update_request(filename);
    let manual_update_request = match parse_result {
        Ok(manual_update_request) => manual_update_request,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    let pt1_start_time = std::time::Instant::now();

    let correctly_ordered_updates: Vec<(usize, ManualUpdate)> = manual_update_request.updates
        .iter()
        .enumerate()
        .filter(|(_i, update)| is_update_in_correct_order(&manual_update_request.rules, update))
        .map(|(i, update)| (i, update.clone()))
        .collect();

    let middle_page_sum: usize = correctly_ordered_updates.iter().map(|(_i, update)| update[update.len()/2]).sum();

    let pt1_time = pt1_start_time.elapsed();

    println!("{} updates correctly ordered", correctly_ordered_updates.len());

    // Only log more details about the updates when it won't bloat our output.
    // Useful for checking the sample_input.txt
    if correctly_ordered_updates.len() < 10 {
        for (update_idx,update) in &correctly_ordered_updates {
            println!(" - Update {:03} is correctly ordered! {:?}", update_idx, update);
        }
    }

    println!("middle page sum is: {}", middle_page_sum);

    let pt1_time_end = pt1_start_time.elapsed();

    println!("TIME: ({:0.06}s) / ({:0.06}s)", pt1_time.as_secs_f64(), pt1_time_end.as_secs_f64());

    println!("");

    let pt2_start_time = std::time::Instant::now();

    let incorrectly_ordered_updates: Vec<(usize, ManualUpdate)> = manual_update_request.updates
        .iter()
        .enumerate()
        .filter(|(i, _update)| !correctly_ordered_updates.iter().any(|(j, _correct_update)| i == j))
        .map(|(i, update)| (i, update.clone()))
        .collect();

    let corrected_updates: Vec<ManualUpdate> = incorrectly_ordered_updates.iter().map(|(_i, update)| correct_update_ordering(&manual_update_request.rules, update)).collect();
    let corrections_middle_page_sum: usize = corrected_updates.iter().map(|update| update[update.len()/2]).sum();

    let pt2_time = pt2_start_time.elapsed();

    println!("{} updates incorrectly ordered", incorrectly_ordered_updates.len());

    // Only log more details about the updates when it won't bloat our output.
    // Useful for checking the sample_input.txt
    if incorrectly_ordered_updates.len() < 10 {
        for ((update_idx, incorrect_update), corrected_update) in incorrectly_ordered_updates.iter().zip(corrected_updates.iter()) {

            println!(" - Update {:03} incorrect: {:?}", update_idx, incorrect_update);
            println!(" - Update {:03} corrected: {:?}", update_idx, corrected_update);
            println!("");
        }
    }
    println!("corrections middle page sum is: {}", corrections_middle_page_sum);

    let pt2_time_end = pt2_start_time.elapsed();

    println!("TIME: ({:0.06}s) / ({:0.06}s)", pt2_time.as_secs_f64(), pt2_time_end.as_secs_f64());

    return ExitCode::SUCCESS;
}
