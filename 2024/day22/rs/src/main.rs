use input_helpers;
use std::process::ExitCode;

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

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let secret_gen_count: usize = input_helpers::get_nth_parsed_arg(args, 1)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();
    let do_pt2 = args
        .iter()
        .find(|a| a.as_str() == "-2" || a.as_str() == "--pt2")
        .is_some();

    let initial_secret_values = read_input(filename)?;

    {
        let final_secret_values: Vec<u64> = initial_secret_values
            .iter()
            .map(|secret| gen_nth_secret(*secret, secret_gen_count))
            .collect();

        let final_secret_values_sum: u64 = final_secret_values.iter().sum();

        if verbose || initial_secret_values.len() <  20 {
            println!("after {} secret gen steps...", secret_gen_count);
            for (initial_secret, final_secret) in initial_secret_values.iter().zip(final_secret_values.iter()) {
                println!("{}: {}", initial_secret, final_secret);
            }
        }

        println!("pt 1: {}th secret sums = {}", secret_gen_count, final_secret_values_sum);
    }

    if do_pt2 {
        let buyer_secret_sequences: Vec<Vec<u64>> = initial_secret_values
            .iter()
            .map(|initial_secret| generate_secret_sequence(*initial_secret, secret_gen_count))
            .collect();

        let buyer_secret_sequence_price_maps = {
            let mut buyer_secret_sequence_price_maps = std::collections::HashMap::<BuyerId, std::collections::HashMap<SellSequence, i8>>::new();

            for buyer_secret_sequence in &buyer_secret_sequences {
                let (buyer_id, sell_sequence_price_map) = 
                    calculate_sell_sequence_values_for_buyer(&buyer_secret_sequence);
                
                buyer_secret_sequence_price_maps.insert(buyer_id, sell_sequence_price_map);
            }
            
            buyer_secret_sequence_price_maps
        };

        if verbose {
            println!("sell prices for each buyer's known sequence values: ({})", buyer_secret_sequence_price_maps.len());
            for (buyer_id, sell_sequence_price_map) in buyer_secret_sequence_price_maps.iter() {
                println!("\t[{}]", buyer_id);
                for (sell_sequence, price) in sell_sequence_price_map.iter() {
                    println!("\t\t{},{},{},{} = ${}", sell_sequence.0, sell_sequence.1, sell_sequence.2, sell_sequence.3, price);
                }
            }
        }

        let sell_sequence_prices = { 
            let mut sell_sequence_prices = std::collections::HashMap::<SellSequence, std::collections::HashMap<BuyerId, i8>>::new();
            for (buyer_id, sell_sequence_price_map) in buyer_secret_sequence_price_maps.iter() {
                for (sell_sequence, price) in sell_sequence_price_map.iter() {
                    if let Some(buyer_price_map_for_seq) = sell_sequence_prices.get_mut(&sell_sequence) {
                        let old_value = buyer_price_map_for_seq.insert(*buyer_id, *price);
                        if let Some(old_value) = old_value {
                            panic!("Found multiple prices from buyer {} for sell_seq {},{},{},{}! First {}, then {}",
                                buyer_id,
                                sell_sequence.0,
                                sell_sequence.1,
                                sell_sequence.2,
                                sell_sequence.3,
                                old_value,
                                price);
                        }
                    } else {
                        let mut new_buyer_price_map_for_seq = std::collections::HashMap::<BuyerId, i8>::new();
                        new_buyer_price_map_for_seq.insert(*buyer_id, *price);
                        sell_sequence_prices.insert(*sell_sequence, new_buyer_price_map_for_seq);
                    }
                }
            }

            sell_sequence_prices
        };

        let sell_sequence_totals = { 
            let mut sell_sequence_totals = std::collections::HashMap::<SellSequence, u64>::new(); 

            for (sell_sequence, buyer_price_map) in sell_sequence_prices.iter() {
                let total_value = buyer_price_map
                    .iter()
                    .map(|(_buyer, price)| (*price as u64))
                    .sum::<u64>();

                assert!(!sell_sequence_totals.contains_key(&sell_sequence));
                sell_sequence_totals.insert(*sell_sequence, total_value);
            }

            sell_sequence_totals
        };

        let max_sell_sequence = sell_sequence_totals
            .iter()
            .max_by_key(|(_sell_sequence, total_sell_value)| *total_sell_value)
            .map(|(sell_seq_ref, sell_value_ref)| (*sell_seq_ref, *sell_value_ref));
        if let Some((sell_sequence, total)) = max_sell_sequence {
            println!("Pt 2:\t\ntotal={}\t\nseq={},{},{},{}",
                total,
                sell_sequence.0,
                sell_sequence.1,
                sell_sequence.2,
                sell_sequence.3);
        } else {
            println!("Pt 2: NO SOLUTION????");
        }
    }

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<u64>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut initial_secrets: Vec<u64> = vec![];

    for line in lines {
        let next_secret: u64 = line.parse().map_err(|_| format!("Failed to parse '{}' as u64", line))?;
        initial_secrets.push(next_secret);
    }

    Ok(initial_secrets)
}

fn do_secret_gen(initial_secret_value: u64) -> u64 {
    let mut curr_secret_value = initial_secret_value;

    // secret gen phase 1
    curr_secret_value = mix_secret(curr_secret_value, curr_secret_value * 64);
    curr_secret_value = prune_secret(curr_secret_value);

    // secret gen phase 2
    curr_secret_value = mix_secret(curr_secret_value, curr_secret_value / 32);
    curr_secret_value = prune_secret(curr_secret_value);

    // secret gen phase 3
    curr_secret_value = mix_secret(curr_secret_value, curr_secret_value * 2048);
    curr_secret_value = prune_secret(curr_secret_value);
    
    curr_secret_value
}

fn mix_secret(secret: u64, mix_value: u64) -> u64 {
    secret ^ mix_value
}

fn prune_secret(secret: u64) -> u64 {
    secret % 16777216
}

fn gen_nth_secret(initial_secret_value: u64, secret_gen_count: usize) -> u64 {
    let mut curr_secret_value = initial_secret_value;
    for _ in 0..secret_gen_count {
        curr_secret_value = do_secret_gen(curr_secret_value);
    }

    curr_secret_value
}

type SellSequence = (i8, i8, i8, i8);
type BuyerId = u64;

fn calculate_sell_sequence_values_for_buyer(buyer_secret_seq: &[u64]) -> (BuyerId, std::collections::HashMap<SellSequence, i8>) {
    if buyer_secret_seq.is_empty() {
        panic!("Invalid secret sequence! Must at least have initial secret value but secret list is empty");
    }

    let buyer_id = buyer_secret_seq[0];

    // if we don't have at least 5 secret values we can't construct a sell sequence so the returned map should be empty
    if buyer_secret_seq.len() < 5 {
        return (buyer_id, std::collections::HashMap::<SellSequence, i8>::new());
    }

    let mut sell_sequence_prices = std::collections::HashMap::<SellSequence, i8>::new();

    let prices: Vec<i8> = buyer_secret_seq.iter().map(|secret| get_price_from_secret_value(*secret)).collect();
    let price_changes = {
        let mut price_changes = Vec::with_capacity(prices.len() - 1);
        for i in 1..prices.len() {
            let price_change = prices[i] - prices[i - 1];
            price_changes.push(price_change);
        }
        price_changes
    };

    for i in 4..prices.len() {
        let sell_sequence = (price_changes[i-4], price_changes[i-3], price_changes[i-2], price_changes[i-1]);
        if let std::collections::hash_map::Entry::Vacant(slot) = sell_sequence_prices.entry(sell_sequence) {
            slot.insert(prices[i]);
        }
        assert!(sell_sequence_prices.contains_key(&sell_sequence));
    }

    (buyer_id, sell_sequence_prices)
}

fn generate_secret_sequence(initial_secret_value: u64, secret_gen_count: usize) -> Vec<u64> {
    let mut curr_secret_value = initial_secret_value;
    let mut seq = vec![ initial_secret_value ];
    for _ in 0..secret_gen_count {
        curr_secret_value = do_secret_gen(curr_secret_value);
        seq.push(curr_secret_value)
    }
    seq
}

fn get_price_from_secret_value(secret_value: u64) -> i8 {
    (secret_value % 10) as i8
}