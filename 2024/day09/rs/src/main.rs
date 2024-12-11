use input_helpers;
use std::{fmt::format, fs::File, ops::Index, process::ExitCode};
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

fn compact_disk_pt1(disk_chunks: &[DiskChunk]) -> Vec<DiskChunk> {
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
            let remaining_file_block_count = tail_file_chunk.block_count - free_chunk.block_count;
            compacted_disk_chunks[next_free_chunk_idx] = DiskChunk::File(FileChunk{id: tail_file_chunk.id, block_count: free_chunk.block_count});
            *compacted_disk_chunks.last_mut().unwrap() = DiskChunk::File(FileChunk{id: tail_file_chunk.id, block_count: remaining_file_block_count});
        } else if free_chunk.block_count == tail_file_chunk.block_count {
            compacted_disk_chunks[next_free_chunk_idx] = DiskChunk::File(tail_file_chunk);
            *compacted_disk_chunks.last_mut().unwrap() = DiskChunk::FreeSpace(FreeSpaceChunk { block_count: tail_file_chunk.block_count });
        } else {
            let remaining_free_space = free_chunk.block_count - tail_file_chunk.block_count;
            compacted_disk_chunks[next_free_chunk_idx] = DiskChunk::File(tail_file_chunk);
            *compacted_disk_chunks.last_mut().unwrap() = DiskChunk::FreeSpace(FreeSpaceChunk { block_count: tail_file_chunk.block_count });
            compacted_disk_chunks.insert(next_free_chunk_idx + 1, DiskChunk::FreeSpace(FreeSpaceChunk{block_count: remaining_free_space}));
        }

        next_free_chunk_search_offset = next_free_chunk_idx;
    }

    compacted_disk_chunks
}

/* FIXME: bug in pt 2 

Bugs in pt2:
1. incorrect shift from front...
    > Pt 2: checksum = 132
    > Original layout:  0..111....22222
    > Compacted layout: .0.111....22222

2. compacting sample_input.txt just generally looks wrong
    - inserting extra free space?
    > Pt 2: checksum = 3427
    > Original layout:  00...111...2...333.44.5555.6666.777.888899
    > Compacted layout: 00992...111....333.44.5555.6666.777.8888

asdfl;kjasfljadsl;f
*/

fn compact_disk_pt2(disk_chunks: &[DiskChunk]) -> Vec<DiskChunk> {
    let mut compacted_disk_chunks = disk_chunks.to_vec();

    fn is_free_space(chunk: &DiskChunk) -> bool {
        match chunk {
            DiskChunk::File(_) => false,
            DiskChunk::FreeSpace(_) => true,
        }
    }

    // FIXME: I could use enumerate instead of position(..) + an add op
    fn find_free_space(disk_chunks: &[DiskChunk], offset: usize) -> Option<usize> {
        disk_chunks.iter().skip(offset).position(is_free_space).map(|idx_with_skip| idx_with_skip + offset)
    }

    // FIXME: maybe add a rev-offset to prevent re-checking files which weren't moved
    fn find_next_file_chunk(disk_chunks: &[DiskChunk], max_file_id: u32) -> Option<usize> {
        fn is_earlier_file_chunk(chunk: &DiskChunk, max_file_id: u32) -> bool {
            if let DiskChunk::File(FileChunk { id, block_count: _block_count }) = chunk {
                max_file_id >= *id
            } else {
                false
            }
        }

        disk_chunks
            .iter()
            .enumerate()
            .rev()
            .find(|(_idx, chunk)| is_earlier_file_chunk(chunk, max_file_id))
            .map(|(idx, _chunk)| idx)
    }

    let mut next_free_chunk_search_offset = 0;
    let mut next_file_chunk_id = {
        let mut highest_file_chunk_id = None;
        for chunk in &compacted_disk_chunks {
            if let DiskChunk::File(FileChunk { id, block_count: _}) = chunk {
                if let Some(curr_highest_file_chunk_id) = highest_file_chunk_id {
                    if curr_highest_file_chunk_id < *id {
                        highest_file_chunk_id = Some(*id);
                    }
                } else {
                    highest_file_chunk_id = Some(*id);
                }
            }
        }
        highest_file_chunk_id
    };

    println!("compacted_disk_chunk start:  {}", stringify_disk_layout(&compacted_disk_chunks));
    loop {
        let max_file_chunk_id_to_compact = if let Some(next_file_chunk_id) = next_file_chunk_id {
            next_file_chunk_id
        } else {
            break;
        };


        //println!("attempting compact of ids <= {}", max_file_chunk_id_to_compact);

        let next_file_chunk_idx_search = find_next_file_chunk(&compacted_disk_chunks, max_file_chunk_id_to_compact);
        let next_file_chunk_idx = if let Some(next_file_chunk_idx) = next_file_chunk_idx_search {
            next_file_chunk_idx
        } else {
            break;
        };

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

        let next_file_chunk_to_compact = match &compacted_disk_chunks[next_file_chunk_idx] {
            DiskChunk::File(file_chunk) => file_chunk.clone(),
            // FIXME: there's got to be a more elegant way to do this. Maybe it means that
            // what I'm doing getting indexes isn't the right way to do this or using enums isn't the best tool here
            _ => {
                dbg!(compacted_disk_chunks);
                panic!("Unexpected chunk type at idx {}! Expected 'file'", next_file_chunk_idx);
            },
        };

        // set the file chunk id we'll search for on the next loop iteration.
        // must be lower than the current one we're attempting to compact.
        next_file_chunk_id = if next_file_chunk_to_compact.id > 0 {
            // FIXME: better names for next_file_chunk_id (the highest file chunk we should attempt to compact) vs next_file_chunk_to_compact (the file chunk we are currently attempting to compact)
            Some(next_file_chunk_to_compact.id - 1)
        } else {
            None
        };


        if free_chunk.block_count == next_file_chunk_to_compact.block_count {
            //println!("compacting {} from {} to {}", next_file_chunk_to_compact.id, next_file_chunk_idx, next_free_chunk_idx);
            compacted_disk_chunks[next_free_chunk_idx] = DiskChunk::File(next_file_chunk_to_compact);
            compacted_disk_chunks[next_file_chunk_idx] = DiskChunk::FreeSpace(FreeSpaceChunk{block_count: next_file_chunk_to_compact.block_count});
        } else if free_chunk.block_count > next_file_chunk_to_compact.block_count {
            //println!("compacting {} from {} to {}", next_file_chunk_to_compact.id, next_file_chunk_idx, next_free_chunk_idx);
            //println!("inserting remaning free space at {}", next_free_chunk_idx + 1);
            let remaining_free_space = free_chunk.block_count - next_file_chunk_to_compact.block_count;
            compacted_disk_chunks[next_free_chunk_idx] = DiskChunk::File(next_file_chunk_to_compact);
            compacted_disk_chunks[next_file_chunk_idx] = DiskChunk::FreeSpace(FreeSpaceChunk{block_count: next_file_chunk_to_compact.block_count});
            compacted_disk_chunks.insert(next_free_chunk_idx + 1, DiskChunk::FreeSpace(FreeSpaceChunk{block_count: remaining_free_space}));
        } else {
            //println!("not compacting {} from {}", next_file_chunk_to_compact.id, next_file_chunk_idx);
        }

        next_free_chunk_search_offset = next_free_chunk_idx;

        println!("compacted_disk_chunk update: {}", stringify_disk_layout(&compacted_disk_chunks));
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
                block_idx += *block_count as usize;
            },
        }
    }
    
    checksum
}

fn stringify_disk_layout(disk_chunks: &[DiskChunk]) -> String {
    let mut disk_layout_string = String::new();
    for chunk in disk_chunks {
        let (print_char, block_count) = match chunk {
            DiskChunk::File(FileChunk { id, block_count }) => {
                const ID_CHARS: [char; 16] = [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F' ];
                let id_char = ID_CHARS[*id as usize % ID_CHARS.len()];
                (id_char, *block_count)
            },
            DiskChunk::FreeSpace(FreeSpaceChunk { block_count }) => {
                ('.', *block_count)
            },
        };

        for _ in 0..block_count {
            disk_layout_string.push(print_char);
        }
    }
    
    disk_layout_string
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
        let compacted_disk_chunks = compact_disk_pt1(&disk_chunks);
        let checksum = calculate_checksum(&compacted_disk_chunks);
        println!("Pt 1: checksum = {}", checksum);
        if compacted_disk_chunks.len() < 20 {
            println!("Original layout:  {}", stringify_disk_layout(&disk_chunks));
            println!("Compacted layout: {}", stringify_disk_layout(&compacted_disk_chunks));
            //dbg!(compacted_disk_chunks);
        }
    }

    println!("");

    {
        let compacted_disk_chunks = compact_disk_pt2(&disk_chunks);
        let checksum = calculate_checksum(&compacted_disk_chunks);
        println!("Pt 2: checksum = {}", checksum);
        if compacted_disk_chunks.len() < 20 {
            println!("Original layout:  {}", stringify_disk_layout(&disk_chunks));
            println!("Compacted layout: {}", stringify_disk_layout(&compacted_disk_chunks));
            //dbg!(compacted_disk_chunks);
        }
    }

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

    #[test]
    fn test_rev_enumerator() {
        let nums = [0, 1, 2, 3, 4, 5, 6];
        let mut nums_rev_iter = nums.iter().enumerate().rev();
        assert_eq!(nums_rev_iter.next(), Some((6 as usize, &6)));
        assert_eq!(nums_rev_iter.next(), Some((5 as usize, &5)));
        assert_eq!(nums_rev_iter.next(), Some((4 as usize, &4)));
        assert_eq!(nums_rev_iter.next(), Some((3 as usize, &3)));
        assert_eq!(nums_rev_iter.next(), Some((2 as usize, &2)));
        assert_eq!(nums_rev_iter.next(), Some((1 as usize, &1)));
        assert_eq!(nums_rev_iter.next(), Some((0 as usize, &0)));
    }

    fn find_next_file_chunk(disk_chunks: &[DiskChunk], max_file_id: u32) -> Option<usize> {
        fn is_earlier_file_chunk(chunk: &DiskChunk, max_file_id: u32) -> bool {
            if let DiskChunk::File(FileChunk { id, block_count: _block_count }) = chunk {
                max_file_id >= *id
            } else {
                false
            }
        }

        disk_chunks
            .iter()
            .enumerate()
            .rev()
            .find(|(_idx, chunk)| is_earlier_file_chunk(chunk, max_file_id))
            .map(|(idx, _chunk)| idx)
    }

    #[test]
    fn test_find_next_file_chunk() {
        let chunks = [
            /* 0 */ DiskChunk::File(FileChunk { id: 0, block_count: 1 }),
            /* 1 */ DiskChunk::FreeSpace(FreeSpaceChunk { block_count: 1 }),
            /* 2 */ DiskChunk::File(FileChunk { id: 1, block_count: 1 }),
            /* 3 */ DiskChunk::FreeSpace(FreeSpaceChunk { block_count: 1 }),
            /* 4 */ DiskChunk::File(FileChunk { id: 2, block_count: 1 }),
            /* 5 */ DiskChunk::FreeSpace(FreeSpaceChunk { block_count: 1 }),
            /* 6 */ DiskChunk::File(FileChunk { id: 3, block_count: 1 }),
            /* 7 */ DiskChunk::FreeSpace(FreeSpaceChunk { block_count: 1 }),
            /* 8 */ DiskChunk::File(FileChunk { id: 4, block_count: 1 }),
        ];

        assert_eq!(find_next_file_chunk(&chunks, 4), Some(8));
        assert_eq!(find_next_file_chunk(&chunks, 3), Some(6));
        assert_eq!(find_next_file_chunk(&chunks, 2), Some(4));
        assert_eq!(find_next_file_chunk(&chunks, 1), Some(2));
        assert_eq!(find_next_file_chunk(&chunks, 0), Some(0));
    }
}