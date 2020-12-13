fn get_report_entries_from_file(file_name: &str) -> Vec<u16> {
    let mut report_entries = Vec::<u16>::new();

    for read_line in input_helpers::read_lines(file_name) {
        match read_line {
            Err(line_err) => println!("Bad line! {}", line_err),
            Ok(line) => report_entries.push(line.parse().unwrap()),
        }
    }

    report_entries
}

const SUM_SOLUTION: u16 = 2020;

#[derive(Debug)]
enum Solution {
    Sum2(u16, u16),
    Sum3(u16, u16, u16),
}

impl Solution {
    fn product(&self) -> u64 {
        match self {
            Solution::Sum2(a, b) => *a as u64 * *b as u64,
            Solution::Sum3(a, b, c) => *a as u64 * *b as u64 * *c as u64,
        }
    }
}

fn find_2020_sum_product_naive(entries: &[u16]) -> Option<Solution> {
    for i in 0..entries.len() {
        let a = entries[i];
        for b in &entries[i..] {
            if a + b == SUM_SOLUTION {
                return Some(Solution::Sum2(a, *b));
            }
        }
    }

    None
}

fn find_2020_sum_product_real(entries: &[u16]) -> Option<Solution> {
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
            return Some(Solution::Sum2(*e, matching_record));
        }
    }

    None
}

fn find_2020_sum_product_naive3(entries: &[u16]) -> Option<Solution> {
    for i in 0..entries.len() - 2 {
        let a = entries[i];
        for j in i + 1..entries.len() - 1 {
            let b = entries[j];
            for c in &entries[j..] {
                if a + b + c == SUM_SOLUTION {
                    return Some(Solution::Sum3(a, b, *c));
                }
            }
        }
    }

    None
}

fn find_2020_sum_product_real3(entries: &[u16]) -> Option<Solution> {
    let tracker = {
        let mut mut_tracker = [false; SUM_SOLUTION as usize];
        for e in entries {
            if *e < SUM_SOLUTION {
                mut_tracker[*e as usize] = true;
            }
        }
        mut_tracker
    };

    for i in 0..entries.len() {
        let a = entries[i];
        for b in &entries[i..] {
            if a + *b >= SUM_SOLUTION {
                continue;
            }

            let matching_c = SUM_SOLUTION - (a + *b);
            let matching_c_exists = tracker[matching_c as usize];
            if matching_c_exists {
                return Some(Solution::Sum3(a, *b, matching_c));
            }
        }
    }

    None
}

enum SolutionType {
    Naive,
    Real,
    Naive3,
    Real3,
}

fn main() {
    let solution_type = {
        let mut solution_type = std::env::args().nth(1).unwrap_or(String::from("real"));
        solution_type.make_ascii_lowercase();
        match solution_type.as_str() {
            "real" => SolutionType::Real,
            "naive" => SolutionType::Naive,
            "real3" => SolutionType::Real3,
            "naive3" => SolutionType::Naive3,
            _ => panic!("Bad arg! must be real|naive|real3|naive3"),
        }
    };

    let report_entries = get_report_entries_from_file("src\\input.txt");

    let time_start = std::time::Instant::now();

    let maybe_solution = match solution_type {
        SolutionType::Naive => find_2020_sum_product_naive(&report_entries),
        SolutionType::Real => find_2020_sum_product_real(&report_entries),
        SolutionType::Naive3 => find_2020_sum_product_naive3(&report_entries),
        SolutionType::Real3 => find_2020_sum_product_real3(&report_entries),
    };

    let runtime = time_start.elapsed();

    match maybe_solution {
        Some(solution) => println!(
            "Solved in {:?}! solution={:?}, product={}",
            runtime,
            solution,
            solution.product(),
        ),
        None => println!("no solution found :("),
    }
}
