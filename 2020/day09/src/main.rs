use std::convert::From;

fn get_sequence_from_input(file_name: &str) -> Vec<usize> {
    input_helpers::read_lines(file_name)
        .map(|line| line.parse().unwrap())
        .collect()
}

fn find_rule_breaker(sequence: &[usize], preamble_len: usize) -> Option<usize> {
    fn is_num_sum_of_two_entries(sequence: &[usize], num: usize) -> bool {
        for i in 0..sequence.len() - 1 {
            for j in i + 1..sequence.len() {
                if num == (sequence[i] + sequence[j]) {
                    return true;
                }
            }
        }
        false
    }

    let mut oldest_sequence_number = 0;
    let mut sequence_buffer = Vec::from(&sequence[0..preamble_len]);
    for i in preamble_len..sequence.len() {
        let next_sequence_number = sequence[i];
        if !is_num_sum_of_two_entries(&sequence_buffer, next_sequence_number) {
            return Some(i);
        }
        sequence_buffer[oldest_sequence_number] = next_sequence_number;
        oldest_sequence_number = (oldest_sequence_number + 1) % sequence_buffer.len();
    }

    None
}

fn find_range_with_matching_sum(sequence: &[usize], sum: usize) -> Option<(usize, usize)> {
    for i in 0..sequence.len() - 2 {
        for j in i + 1..sequence.len() {
            if sequence[i..j].iter().sum::<usize>() == sum {
                return Some((i, j));
            }
        }
    }

    None
}

fn collect_contiguous_ranges_helper(sequence: &[usize], max_value: usize) -> Vec<(usize, usize)> {
    let mut next_range = (0, 0);
    let mut growing_range = true;
    let mut ranges = Vec::new();

    let is_less_than_max_value = |n: usize| n < max_value;

    for i in 0..sequence.len() {
        if growing_range && !is_less_than_max_value(sequence[i]) {
            next_range.1 = i;
            ranges.push(next_range);
            growing_range = false;
        } else if !growing_range && is_less_than_max_value(sequence[i]) {
            next_range.0 = i;
            growing_range = true;
        }
    }

    if growing_range {
        next_range.1 = sequence.len();
        ranges.push(next_range);
    }

    ranges
}

fn main() {
    let (input_file, preamble_len) = match std::env::args().nth(1).as_ref().map(|s| s.as_str()) {
        Some("simple") => ("src/simple_input.txt", 5),
        Some("real") => ("src/input.txt", 25),
        _ => panic!("USAGE: ./day9 [simple|real|"),
    };

    let xmas_sequence = get_sequence_from_input(input_file);
    let rule_breaker = find_rule_breaker(&xmas_sequence, preamble_len).unwrap();

    println!(
        "sequence[{}] = {}",
        rule_breaker, xmas_sequence[rule_breaker]
    );

    let ranges_to_test =
        collect_contiguous_ranges_helper(&xmas_sequence, xmas_sequence[rule_breaker]);

    let mut range = None;
    for range_to_test in ranges_to_test {
        range = find_range_with_matching_sum(
            &xmas_sequence[range_to_test.0..range_to_test.1],
            xmas_sequence[rule_breaker],
        );
        if range.is_some() {
            break;
        }
    }
    if range.is_none() {
        panic!("Valid range not found!");
    }

    let range = range.unwrap();
    println!("sequence[{}..{}]", range.0, range.1);

    let range = &xmas_sequence[range.0..range.1];
    let min_in_range = range.iter().min().unwrap();
    let max_in_range = range.iter().max().unwrap();
    println!(
        "min: {}, max: {}, sum: {}",
        min_in_range,
        max_in_range,
        min_in_range + max_in_range
    );
}
