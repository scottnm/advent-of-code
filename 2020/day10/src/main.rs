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

fn cnt_num_paths_to_joltage_adapter(joltages: &[Joltage], target_adapter_index: usize) -> usize {
    // Create a cache to store the cnts to each adapter as we solve for them.
    // This helps us significantly reduce recalculating this as we recursively check the various
    // path permutations
    let mut path_cnts = vec![Option::<usize>::None; joltages.len()];

    // the only path which ends at the first adapter is the path which only contain
    path_cnts[0] = Some(1);

    fn cnt_num_paths_to_joltage_adapter_int(
        path_cnts: &mut [Option<usize>],
        joltages: &[Joltage],
        target_adapter_index: usize,
    ) -> usize {
        if path_cnts[target_adapter_index].is_some() {
            return path_cnts[target_adapter_index].unwrap();
        } else {
            // we initialize the 0th adapter cnt to 1 so it should never be a none value.
            assert_ne!(target_adapter_index, 0);
        }

        let mut sum = 0;
        for cmp_adapter_index in (0..target_adapter_index).rev() {
            if joltages[target_adapter_index] - joltages[cmp_adapter_index] <= 3 {
                sum += cnt_num_paths_to_joltage_adapter_int(path_cnts, joltages, cmp_adapter_index);
            } else {
                break;
            }
        }

        path_cnts[target_adapter_index] = Some(sum);
        sum
    }

    cnt_num_paths_to_joltage_adapter_int(&mut path_cnts, joltages, target_adapter_index)
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

    let sol = cnt_num_paths_to_joltage_adapter(&joltages, joltages.len() - 1);
    println!("Sol: {}", sol);
}
