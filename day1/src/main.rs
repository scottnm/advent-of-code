use std::io::BufRead;

fn read_lines<P>(file_name: P) -> std::io::Lines<std::io::BufReader<std::fs::File>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(file_name).unwrap();
    std::io::BufReader::new(file).lines()
}

fn get_report_entries_from_file(file_name: &str) -> Vec<u16> {
    let mut report_entries = Vec::<u16>::new();

    for read_line in read_lines(file_name) {
        match read_line {
            Err(line_err) => println!("Bad line! {}", line_err),
            Ok(line) => report_entries.push(line.parse().unwrap()),
        }
    }

    report_entries
}

const SUM_SOLUTION: u16 = 2020;

fn find_2020_sum_product_naive(entries: &[u16]) -> Option<(u16, u16)> {
    for i in 0..entries.len() {
        let a = entries[i];
        for b in &entries[i..] {
            if a + b == SUM_SOLUTION {
                return Some((a, *b));
            }
        }
    }

    None
}

fn find_2020_sum_product_real(entries: &[u16]) -> Option<(u16, u16)> {
    let tracker = {
        let mut mut_tracker = [false; SUM_SOLUTION as usize];
        for e in entries {
            if *e < SUM_SOLUTION {
                mut_tracker[*e as usize] = true;
            }
        }
        mut_tracker
    };

    for e in entries {
        if *e >= SUM_SOLUTION {
            continue; // the record can't be summed with anything else to make 2020
        }

        let matching_record = SUM_SOLUTION - *e;
        let matching_record_exists = tracker[matching_record as usize];
        if matching_record_exists {
            return Some((*e, matching_record));
        }
    }

    None
}

enum SolutionType {
    Naive,
    Real,
}

fn main() {
    let solution_type = {
        let mut solution_type = std::env::args().nth(1).unwrap_or(String::from("real"));
        solution_type.make_ascii_lowercase();
        match solution_type.as_str() {
            "real" => SolutionType::Real,
            "naive" => SolutionType::Naive,
            _ => panic!("Bad arg! must be real or naive"),
        }
    };

    let report_entries = get_report_entries_from_file("src\\input.txt");

    let time_start = std::time::Instant::now();

    let solution = match solution_type {
        SolutionType::Naive => find_2020_sum_product_naive(&report_entries),
        SolutionType::Real => find_2020_sum_product_real(&report_entries),
    };

    let runtime = time_start.elapsed();

    match solution {
        Some(entries) => println!(
            "Solved in {:?}! entries={:?}, product={}",
            runtime,
            entries,
            entries.0 as u64 * entries.1 as u64
        ),
        None => println!("no solution found :("),
    }
}
