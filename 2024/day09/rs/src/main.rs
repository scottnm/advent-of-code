use input_helpers;
use std::{fmt::format, ops::Index, process::ExitCode};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct FileChunk {
    id: u32,
    block_count: u8,
}

#[derive(Debug, Clone, Copy)]
struct FreeSpaceChunk {
    block_count: u8,
}

#[derive(Debug, Clone, Copy)]
enum DiskChunk {
    File(FileChunk), // block_count is u8 since all each file's length is limited to a single digit
    FreeSpace(FreeSpaceChunk),
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
            let disk_chunk = DiskChunk::File(FileChunk{id: next_file_id, block_count: disk_chunk_len});
            next_file_id += 1;
            disk_chunk
        } else {
            DiskChunk::FreeSpace(FreeSpaceChunk { block_count: disk_chunk_len })
        };

        disk_chunk_toggle = !disk_chunk_toggle;

        disk_chunks.push(disk_chunk);
    }

    Ok(disk_chunks)
}

fn compact_disk(disk_chunks: &[DiskChunk]) -> Vec<DiskChunk> {
    let mut compacted_disk_chunks = disk_chunks.to_vec();

    fn is_free_space(chunk: &DiskChunk) -> bool {
        match chunk {
            DiskChunk::File(_) => false,
            DiskChunk::FreeSpace(_) => true,
        }
    }

    fn find_free_space(disk_chunks: &[DiskChunk], offset: usize) -> Option<usize> {
        disk_chunks.iter().skip(offset).position(is_free_space).map(|idx_with_skip| idx_with_skip + offset)
    }

    let mut next_free_chunk_search_offset = 0;
    loop {
        while !compacted_disk_chunks.is_empty() && is_free_space(compacted_disk_chunks.last().unwrap()) {
            compacted_disk_chunks.pop();
        }

        let next_free_chunk_idx_search = find_free_space(&compacted_disk_chunks, next_free_chunk_search_offset);

        let next_free_chunk_idx = if let Some(next_free_chunk_idx) = next_free_chunk_idx_search {
            next_free_chunk_idx
        } else {
            break;
        };

        // There has to be at least one free chunk from the beginning of the search offset and one non-free chunk at the end.
        assert!((compacted_disk_chunks.len() - next_free_chunk_search_offset) >= 2);

        let free_chunk = match &compacted_disk_chunks[next_free_chunk_idx] {
            DiskChunk::FreeSpace(free_chunk) => free_chunk.clone(),
            // FIXME: there's got to be a more elegant way to do this. Maybe it means that
            // what I'm doing getting indexes isn't the right way to do this or using enums isn't the best tool here
            _ => panic!("Unexpected chunk type! Expected 'free'"),
        };

        let tail_file_chunk = match compacted_disk_chunks.last().unwrap() {
            DiskChunk::File(file_chunk) => file_chunk.clone(),
            // FIXME: there's got to be a more elegant way to do this. Maybe it means that
            // what I'm doing getting indexes isn't the right way to do this or using enums isn't the best tool here
            _ => panic!("Unexpected chunk type! Expected 'file'"),
        };

        if free_chunk.block_count < tail_file_chunk.block_count {
            compacted_disk_chunks[next_free_chunk_idx] = DiskChunk::File(FileChunk{id: tail_file_chunk.id, block_count: free_chunk.block_count});
            *compacted_disk_chunks.last_mut().unwrap() = DiskChunk::File(FileChunk{id: tail_file_chunk.id, block_count: tail_file_chunk.block_count - free_chunk.block_count});
        } else if free_chunk.block_count == tail_file_chunk.block_count {
            compacted_disk_chunks[next_free_chunk_idx] = DiskChunk::File(FileChunk{id: tail_file_chunk.id, block_count: free_chunk.block_count});
            compacted_disk_chunks.pop();
        } else {
            let remaining_free_space = free_chunk.block_count - tail_file_chunk.block_count;
            compacted_disk_chunks[next_free_chunk_idx] = DiskChunk::File(tail_file_chunk);
            compacted_disk_chunks.pop();
            compacted_disk_chunks.insert(next_free_chunk_idx + 1, DiskChunk::FreeSpace(FreeSpaceChunk{block_count: remaining_free_space}));
        }

        next_free_chunk_search_offset = next_free_chunk_idx;
    }

    compacted_disk_chunks
}

fn calculate_checksum(disk_chunks: &[DiskChunk]) -> usize {
    // FIXME: I should be able to short-circuit this if I know it's compacted.
    // anyway to make the caller have to guarantee that? or to safely and quickly check it myself?
    let mut checksum: usize = 0;

    let mut block_idx: usize = 0;
    for chunk in disk_chunks {
        match chunk {
            DiskChunk::File(FileChunk { id, block_count }) => {
                let block_count = *block_count as usize;
                let block_idx_sum: usize = (block_idx..(block_idx+block_count)).sum();
                let id = *id as usize;
                checksum += block_idx_sum * id;
                block_idx += block_count;
            },
            DiskChunk::FreeSpace(FreeSpaceChunk { block_count }) => {
                block_idx += (*block_count as usize);
            },
        }
    }
    
    checksum
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

    {
        let compacted_disk_chunks = compact_disk(&disk_chunks);
        let checksum = calculate_checksum(&compacted_disk_chunks);
        println!("Pt 1: checksum = {}", checksum);
        if compacted_disk_chunks.len() < 10 {
            dbg!(compacted_disk_chunks);
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }

    #[test]
    fn test_skip_position() {
        let nums = [0, 1, 2, 3, 4, 5, 6];
        assert_eq!(nums.iter().position(|n| *n == 2), Some(2));
        //fails: assert_eq!(nums.iter().skip(2).position(|n| *n == 2), Some(2));
        assert_eq!(nums.iter().skip(3).position(|n| *n == 2), None);
        //fails: assert_eq!(nums.iter().skip(3).position(|n| *n == 6), Some(6));
    }
}