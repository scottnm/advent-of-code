use std::collections::HashMap;

fn play_memory_game(starting_numbers: &[usize], target_turn: usize) -> usize {
    let mut age_map = HashMap::new();
    for i in 0..starting_numbers.len() - 1 {
        age_map.insert(starting_numbers[i], i + 1);
    }

    let mut last_number_spoken = *starting_numbers.last().unwrap();
    for current_turn in starting_numbers.len() + 1..target_turn + 1 {
        let last_turn = current_turn - 1;
        let num_spoken = match age_map.get(&last_number_spoken) {
            Some(age) => last_turn - age,
            None => 0,
        };
        age_map.insert(last_number_spoken, last_turn);
        last_number_spoken = num_spoken;
    }

    last_number_spoken
}

fn main() {
    let input = [0, 3, 1, 6, 7, 5];
    println!("Input: {:?}", input);

    let pt1_output = play_memory_game(&input, 2020);
    println!("Pt1 => {}", pt1_output);

    // TODO: improve time to complete. this is SUPER slow, but it finishes
    let pt2_output = play_memory_game(&input, 30000000);
    println!("Pt2 => {}", pt2_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_game_pt1_samples() {
        assert_eq!(play_memory_game(&[0, 3, 6], 2020), 436);
        assert_eq!(play_memory_game(&[1, 3, 2], 2020), 1);
        assert_eq!(play_memory_game(&[2, 1, 3], 2020), 10);
        assert_eq!(play_memory_game(&[1, 2, 3], 2020), 27);
        assert_eq!(play_memory_game(&[2, 3, 1], 2020), 78);
        assert_eq!(play_memory_game(&[3, 2, 1], 2020), 438);
        assert_eq!(play_memory_game(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    fn memory_game_pt1() {
        assert_eq!(play_memory_game(&[0, 3, 1, 6, 7, 5], 2020), 852);
    }

    #[test]
    fn memory_game_pt2_samples() {
        assert_eq!(play_memory_game(&[0, 3, 6], 30000000), 175594);
        assert_eq!(play_memory_game(&[1, 3, 2], 30000000), 2578);
        assert_eq!(play_memory_game(&[2, 1, 3], 30000000), 3544142);
        assert_eq!(play_memory_game(&[1, 2, 3], 30000000), 261214);
        assert_eq!(play_memory_game(&[2, 3, 1], 30000000), 6895259);
        assert_eq!(play_memory_game(&[3, 2, 1], 30000000), 18);
        assert_eq!(play_memory_game(&[3, 1, 2], 30000000), 362);
    }

    #[test]
    fn memory_game_pt2() {
        assert_eq!(play_memory_game(&[0, 3, 1, 6, 7, 5], 30000000), 6007666);
    }
}
