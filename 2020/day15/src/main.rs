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
    let pt1_input = ([0, 3, 1, 6, 7, 5], 2020);
    let pt1_output = play_memory_game(&pt1_input.0, pt1_input.1);
    println!("Pt1({:?}) => {}", pt1_input, pt1_output);
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

    /*
    Given 0,3,6, the 30000000th number spoken is 175594.
    Given 1,3,2, the 30000000th number spoken is 2578.
    Given 2,1,3, the 30000000th number spoken is 3544142.
    Given 1,2,3, the 30000000th number spoken is 261214.
    Given 2,3,1, the 30000000th number spoken is 6895259.
    Given 3,2,1, the 30000000th number spoken is 18.
    Given 3,1,2, the 30000000th number spoken is 362.
    */
}
