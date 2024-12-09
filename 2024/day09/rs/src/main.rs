use input_helpers;
use std::{fmt::format, process::ExitCode};
use itertools::Itertools;

#[derive(Debug)]
enum DiskChunk {
    File{id: u32, block_count: u8}, // block_count is u8 since all each file's length is limited to a single digit
    FreeSpace{block_count: u8},
}

fn read_disk_layout(filename: &str) -> Result<Vec<DiskChunk>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() != 1 {
        return Err(format!("Expected exactly 1 line! Found {}", lines.len()));
    }

    let disk_line = &lines[0];

    let mut disk_chunks = vec![];

    let mut disk_chunk_toggle: bool = true;
    let mut next_file_id: u32 = 0;

    for digit_char in disk_line.chars() {
        let disk_chunk_len: u8 = match digit_char {
            '0'..='9' => (digit_char as u8) - ('0' as u8),
            _ => return Err(format!("Invalid non-digit char! '{}'", digit_char)),
        };

        let disk_chunk = if disk_chunk_toggle {
            let disk_chunk = DiskChunk::File { id: next_file_id, block_count: disk_chunk_len };
            next_file_id += 1;
            disk_chunk
        } else {
            DiskChunk::FreeSpace { block_count: disk_chunk_len }
        };

        disk_chunk_toggle = !disk_chunk_toggle;

        disk_chunks.push(disk_chunk);
    }

    Ok(disk_chunks)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_disk_layout(filename);
    let disk_chunks = match parse_result {
        Ok(disk_chunks) => disk_chunks,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    dbg!(disk_chunks);
    /*
    dump_tower_grid(&tower_grid);

    {
        let antinode_positions_pt1 = calculate_all_antinode_positions_pt1(&tower_grid);
        println!("Pt 1: antinode position count = {}", antinode_positions_pt1.len());
        if antinode_positions_pt1.len() < 10 {
            for p in antinode_positions_pt1 {
                println!("- (r:{},c:{})", p.row, p.col);
            }
        }
    }

    println!("");

    {
        let antinode_positions_pt2 = calculate_all_antinode_positions_pt2(&tower_grid);
        println!("Pt 2: antinode position count = {}", antinode_positions_pt2.len());
        if antinode_positions_pt2.len() < 10 {
            for p in antinode_positions_pt2 {
                println!("- (r:{},c:{})", p.row, p.col);
            }
        }
    }
    */

    return ExitCode::SUCCESS;
}
