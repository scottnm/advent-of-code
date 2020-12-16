type Joltage = usize;
fn get_joltages_from_input(file_name: &str) -> Vec<Joltage> {
    input_helpers::read_lines(file_name)
        .map(|line| line.parse::<Joltage>().unwrap())
        .collect()
}

fn calculate_joltage_differences(joltages: &[Joltage]) -> Vec<usize> {
    (0..joltages.len() - 1)
        .map(|i| (&joltages[i], &joltages[i + 1]))
        .map(|(a, b)| b - a)
        .collect()
}

fn brute_force_solution(joltages: &mut [Joltage], from: usize) -> usize {
    fn pop_n_squash(v: &mut [Joltage], i: usize) {
        v[i..].rotate_left(1);
    }

    fn push_n_restore(v: &mut [Joltage], i: usize) {
        v[i..].rotate_right(1);
    }

    let mut sum = 1;
    let num_joltages = joltages.len();
    for i in from..num_joltages - 1 {
        if joltages[i + 1] - joltages[i - 1] <= 3 {
            pop_n_squash(joltages, i);
            sum += brute_force_solution(&mut joltages[0..num_joltages - 1], i);
            push_n_restore(joltages, i);
        }
    }
    sum
}

fn main() {
    let input_file = match std::env::args().nth(1).as_ref().map(|s| s.as_str()) {
        Some("simple") => "src/simple_input.txt",
        Some("simple2") => "src/simple2_input.txt",
        Some("real") => "src/input.txt",
        _ => panic!("USAGE: ./day10 [simple|real|"),
    };

    let joltages = {
        let mut joltages = get_joltages_from_input(input_file);
        joltages.sort();
        joltages.insert(0, 0); // the first "adapter" in the chain is the wall outlet of joltage 0
        joltages.push(joltages.last().unwrap() + 3); // my personal adapter is always 3 higher than the highest adapter
        joltages
    };
    // dbg!(&joltages);

    let joltage_differences = calculate_joltage_differences(&joltages);
    // dbg!(&joltage_differences);

    let one_jolt_diffs = joltage_differences.iter().filter(|j| **j == 1usize).count();
    let three_jolt_diffs = joltage_differences.iter().filter(|j| **j == 3usize).count();
    println!(
        "1-J diffs ({}) x 3-J diffs ({}) = {}",
        one_jolt_diffs,
        three_jolt_diffs,
        one_jolt_diffs * three_jolt_diffs
    );

    let sol = brute_force_solution(&mut joltages.clone(), 1);
    println!("Sol: {}", sol);
}
