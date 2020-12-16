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

    let joltages = get_joltages_from_input(input_file);
    dbg!(joltages);
}
