type Joltage = usize;
fn get_joltages_from_input(file_name: &str) -> Vec<Joltage> {
    input_helpers::read_lines(file_name)
        .map(|line| line.parse::<Joltage>().unwrap())
        .collect()
}

fn main() {
    let input_file = match std::env::args().nth(1).as_ref().map(|s| s.as_str()) {
        Some("simple") => "src/simple_input.txt",
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

    dbg!(&joltages);

    let joltage_differences: Vec<usize> = (0..joltages.len() - 1)
        .map(|i| (&joltages[i], &joltages[i + 1]))
        .map(|(a, b)| b - a)
        .collect();

    dbg!(&joltage_differences);

    let one_jolt_diffs = joltage_differences.iter().filter(|j| **j == 1usize).count();
    let three_jolt_diffs = joltage_differences.iter().filter(|j| **j == 3usize).count();
    println!(
        "1J diffs ({}) x 3J diffs ({}) = {}",
        one_jolt_diffs,
        three_jolt_diffs,
        one_jolt_diffs * three_jolt_diffs
    );
}
