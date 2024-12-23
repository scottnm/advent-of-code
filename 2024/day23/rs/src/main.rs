use input_helpers;
use std::process::{exit, ExitCode};

type CpuName = [char;2];

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
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();
    let do_pt2 = args
        .iter()
        .find(|a| a.as_str() == "-2" || a.as_str() == "--pt2")
        .is_some();

    let connections = read_input(filename)?;
    dbg!(&connections); 

    {
        let parties = find_parties(&connections);
        let chiefs_parties: Vec<(String, String, String)> = parties
            .iter()
            .filter(|p| party_has_chief(p))
            .cloned()
            .collect();

        let mut parties_with_chief = 0;
        for p in parties {
            if party_has_chief(&p) {
                parties_with_chief += 1;
                if verbose {
                    println!(" - {},{},{} (HAS CHIEF)", p.0, p.1, p.2);
                }
            } else {
                if verbose {
                    println!(" - {},{},{}", p.0, p.1, p.2);
                }
            }
        }
        println!("Pt1. # parties with chief: {}", parties_with_chief);
    }

    if do_pt2 {
        unimplemented!();
    }

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<(String,String)>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut connections: Vec<(String, String)> = vec![];

    for line in lines {
        let mut split_itr = line.split('-');
        let cpu1 = split_itr.next().ok_or(format!("Missing first cpu on line {}", line))?;
        let cpu2 = split_itr.next().ok_or(format!("Missing second cpu on line {}", line))?;
        if let Some(v) = split_itr.next() {
            return Err(format!("Unexpected values on line {}", line));
        }

        connections.push((cpu1.to_string(), cpu2.to_string()));
    }

    Ok(connections)
}

fn find_parties(connections: &[(String, String)]) -> Vec<(String, String, String)> {
    let mut per_pc_connections = std::collections::HashMap::<&str, std::collections::HashSet<&str>>::new();

    for (pc, other_pc) in connections.iter() {
        match per_pc_connections.entry(pc) {
            std::collections::hash_map::Entry::Occupied(existing_slot) => {
                existing_slot.into_mut().insert(&other_pc);
            },
            std::collections::hash_map::Entry::Vacant(vacant_slot) => {
                let mut pc_connection_set = std::collections::HashSet::<&str>::new();
                pc_connection_set.insert(&other_pc);
                vacant_slot.insert(pc_connection_set);
            },
        }

        match per_pc_connections.entry(other_pc) {
            std::collections::hash_map::Entry::Occupied(existing_slot) => {
                existing_slot.into_mut().insert(&pc);
            },
            std::collections::hash_map::Entry::Vacant(vacant_slot) => {
                let mut pc_connection_set = std::collections::HashSet::<&str>::new();
                pc_connection_set.insert(&pc);
                vacant_slot.insert(pc_connection_set);
            },
        }
    }

    // FIXME: remove
    if per_pc_connections.is_empty() {
        unimplemented!("need to fill per_pc_connections");
    }

    let mut parties = std::collections::HashSet::<(&str, &str, &str)>::new();

    for (pc_1, pc_1_connections) in per_pc_connections.iter() {
        let pc_1_connections_vec: Vec<&str> = pc_1_connections.iter().cloned().collect();
        for (pc_2_index, pc_2) in pc_1_connections_vec.iter().enumerate() {
            for pc_3 in &pc_1_connections_vec[pc_2_index..] {
                if per_pc_connections.get(pc_2).unwrap().contains(pc_3) {
                    let mut pcs = [pc_1, pc_2, pc_3];
                    pcs.sort();

                    parties.insert((pcs[0], pcs[1], pcs[2]));
                }
            }
        }
    }

    parties
        .iter()
        .map(|p| (p.0.to_string(), p.1.to_string(), p.2.to_string()))
        .collect()
}

fn party_has_chief(party: &(String, String, String)) -> bool {
    party.0.starts_with('t') ||
    party.1.starts_with('t') ||
    party.2.starts_with('t')
}