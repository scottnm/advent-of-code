use input_helpers;
use std::process::ExitCode;

type StoneVal = usize;

fn dump_stones(dump_title: &str, stones: &[StoneVal]) {
    println!("{}: {:?}", dump_title, stones);
}

fn read_stone_arrangement(filename: &str) -> Result<Vec<StoneVal>, String> {
    let file_data = match input_helpers::read_file_to_string(filename) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to read input! {}", e)),
    };

    let mut stones: Vec<StoneVal> = vec![];
    for stone_val_str in file_data.split_ascii_whitespace() {
        let stone_val = match stone_val_str.parse() {
            Ok(stone_val) => stone_val,
            Err(e) => return Err(format!("Failed to parse stone value! {}", e)),
        };
        stones.push(stone_val);
    }

    Ok(stones)
}

fn count_digits(v: usize) -> usize {
    /* FIXME: it'd probably be faster to match than to loop with modolus which is a pretty slow operator
    // 18,446,744,073,709,551,616
    let digit_count = match v {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1_000..=9_999 => 4,
        10_000..=99_999 => 5,
        100_000..=999_999 => 6,
    }; */
    let mut digit_count = 1;
    let mut v = v;
    while v > 9 {
        v = v / 10;
        digit_count += 1;
    }

    digit_count
}

fn split_num(v: usize, digit_split: usize) -> (usize, usize) {
    let mut v = v;
    let mut low_digits = 0;
    let mut low_digits_multiplier = 1;

    for _ in 0..digit_split {
        let next_digit = v % 10;
        v /= 10;
        low_digits += low_digits_multiplier * next_digit;
        low_digits_multiplier *= 10;
    }

    (v, low_digits)
}

// FIXME: rather than taking a mutable vector and mutating in place,
// is there any more "functional" way to apply these updates that doesn't result
// in TONS of copies?
fn do_blink(stones: &mut Vec<StoneVal>) {
    let mut i = 0;
    while i < stones.len() {
        /*
        RULES:
        
        01. If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.

        02. If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The 
            left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on
            the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)

        03. If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 
            2024 is engraved on the new stone.
         */
        let prev_stone = stones[i];
        if prev_stone == 0 {
            stones[i] = 1;            
            i += 1;
        } else {
            let digit_count = count_digits(prev_stone);
            if digit_count % 2 == 0 {
                let (high_digits, low_digits) = split_num(prev_stone, digit_count / 2);
                stones[i] = high_digits;
                stones.insert(i + 1, low_digits);
                i += 2;
            } else {
                stones[i] = prev_stone * 2024; 
                i += 1;
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct StoneBlinkProgress {
    val: StoneVal,
    blinks_left: usize,
}

type BlinkResultCompressorMapKey = usize;

struct BlinkResultCompressorMap
{
    result_to_key_map: std::collections::HashMap<Vec<StoneVal>, BlinkResultCompressorMapKey>,
    key_to_result_map: Vec<Vec<StoneVal>>,
}

type BlinkResultsMap = std::collections::HashMap<StoneBlinkProgress, BlinkResultCompressorMapKey>;

impl BlinkResultCompressorMap {
    fn new() -> Self {
        Self {
            result_to_key_map: std::collections::HashMap::<Vec<StoneVal>, usize>::new(),
            key_to_result_map: vec![],
        }
    }

    fn insert(&mut self, result: &Vec<StoneVal>) -> BlinkResultCompressorMapKey {
        if let Some(key) = self.result_to_key_map.get(result) {
            return *key;
        }

        let key = self.key_to_result_map.len();
        self.key_to_result_map.push(result.clone());
        self.result_to_key_map.insert(result.clone(), key);
        key
    }
}

fn memoize_stone_blinks(
    stone_state: StoneBlinkProgress,
    memo: &mut BlinkResultsMap,
    compression_map: &mut BlinkResultCompressorMap) -> BlinkResultCompressorMapKey {

    if let Some(memod_result) = memo.get(&stone_state) {
        return *memod_result;
    }

    if stone_state.blinks_left == 0 {
        let result = vec![stone_state.val];
        let result_key = compression_map.insert(&result);
        memo.insert(stone_state, result_key);
        return result_key;
    }

    if stone_state.val == 0 {
        let result_key = memoize_stone_blinks(StoneBlinkProgress{val: 1, blinks_left: stone_state.blinks_left - 1}, memo, compression_map);
        memo.insert(stone_state, result_key);
        return result_key;
    } 

    let digit_count = count_digits(stone_state.val);
    if digit_count % 2 == 0 {
        let (high_digits, low_digits) = split_num(stone_state.val, digit_count / 2);
        let result_stones = {
            let high_digits_stone_results_key = 
                memoize_stone_blinks(StoneBlinkProgress{val: high_digits, blinks_left: stone_state.blinks_left - 1}, memo, compression_map);
            let low_digits_stone_results_key = 
                memoize_stone_blinks(StoneBlinkProgress{val: low_digits, blinks_left: stone_state.blinks_left - 1}, memo, compression_map);
            
            let mut results = compression_map.key_to_result_map[high_digits_stone_results_key].clone();
            results.append(&mut compression_map.key_to_result_map[low_digits_stone_results_key].clone());
            results
        };
        let result_key = compression_map.insert(&result_stones);
        memo.insert(stone_state, result_key);
        return result_key;
    } 

    let result_key = memoize_stone_blinks(StoneBlinkProgress{val: stone_state.val * 2024, blinks_left: stone_state.blinks_left - 1}, memo, compression_map);
    memo.insert(stone_state, result_key);
    return result_key;
}

fn do_blinks_memod(stones: &mut Vec<StoneVal>, blink_count: usize) {
    let mut result_compression_map = BlinkResultCompressorMap::new();
    let mut memoized_blink_results = BlinkResultsMap::new();
    let mut full_blink_results: Vec<StoneVal> = vec![];

    for stone in stones.iter().cloned() {
        let result_key = memoize_stone_blinks(
            StoneBlinkProgress{val: stone, blinks_left: blink_count}, 
            &mut memoized_blink_results,
            &mut result_compression_map);
        full_blink_results.append(&mut result_compression_map.key_to_result_map[result_key].clone());
    }

    *stones = full_blink_results;
}

fn get_nth_string_arg<'a>(args: &'a [String], n: usize) -> Result<&'a str, String> {
    if args.len() <= n {
        return Err(format!("Too few args! needed {}; had {}", n+1, args.len()));
    }

    Ok(&args[n])
}

fn get_nth_parsed_arg<T>(args: &[String], n: usize) -> Result<T, String> 
    where T: std::str::FromStr {
    if args.len() <= n {
        return Err(format!("Too few args! needed {}; had {}", n+1, args.len()));
    }

    match args[n].parse() {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Failed to parse arg! '{}'", &args[n])),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = get_nth_string_arg(args, 0)?;
    let blink_count: usize = get_nth_parsed_arg(args, 1)?;
    let use_memoization: bool = get_nth_parsed_arg(args, 2)?;

    let mut stones = read_stone_arrangement(filename)?;

    let original_stones = stones.clone();
    dump_stones("original", &original_stones);

    if use_memoization {
        do_blinks_memod(&mut stones, blink_count);
    } else {
        for i in 0..blink_count {
            println!("{:03}/{:03} blinks", i, blink_count);
            do_blink(&mut stones);
        }
    }

    if stones.len() < 50 {
        dump_stones("after blinks", &stones);
    }
    println!("result = {} stones", stones.len());

    /*
    println!("");

    {
        let trails = find_all_trails_pt2(&trail_map);
        let trailhead_ratings: Vec<usize> = trails
            .iter()
            .map(|(_trail_start, trail_ends)| trail_ends.len())
            .collect();
        let trailhead_rating_sum: usize = trailhead_ratings.iter().sum();
        println!("Pt 2: trailhead_rating_sum = {}", trailhead_rating_sum);
        if trails.len() < 20 {
            for trail in &trails {
                println!("- start={}; trail={:?}", trail.0, trail.1);
            }
        }
    } */

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Err: {}", e);
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(5), 1);
        assert_eq!(count_digits(9), 1);
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(1234), 4);
    }

    #[test]
    fn test_split_num() {
        assert_eq!(split_num(1000, 2), (10, 0));
        assert_eq!(split_num(12, 1), (1, 2));
        assert_eq!(split_num(12, 6), (0, 12));
        assert_eq!(split_num(120, 1), (12, 0));
        assert_eq!(split_num(120, 0), (120, 0));
    }
}