#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum TriBit {
    O,
    I,
    X,
}

type Mask36 = [TriBit; 36];

fn set_bit(index: usize, value: usize) -> usize {
    let mask = 1usize << index;
    value | mask
}

fn clear_bit(index: usize, value: usize) -> usize {
    let mask = !(1usize << index);
    value & mask
}

fn apply_mask(mask: &Mask36, value: usize) -> usize {
    let mut masked_value = value;
    for (i, mask_bit) in mask.iter().enumerate() {
        masked_value = match mask_bit {
            TriBit::O => clear_bit(i, masked_value),
            TriBit::I => set_bit(i, masked_value),
            TriBit::X => masked_value,
        };
    }
    masked_value
}

fn get_addresses_from_mask(mask: &Mask36, addr: usize) -> Vec<usize> {
    // sum each of the single-bit masks into one mask that we can use to set all bits at once
    let one_bits_from_mask: usize = mask
        .iter()
        .enumerate()
        .filter(|(_, bit)| **bit == TriBit::I)
        .map(|(i, _)| 1usize << i)
        .fold(0, |a, b| a | b);

    let floating_bit_indices = mask
        .iter()
        .enumerate()
        .filter(|(_, bit)| **bit == TriBit::X)
        .map(|(i, _)| i);

    // 0 out every floating bit on the base address. That way we can be sure what the beginning state of each bit is
    // when we go to flip each of them
    let x_bits_from_mask: usize = floating_bit_indices
        .clone()
        .map(|i| 1usize << i)
        .fold(0, |a, b| a | b);

    // construct the starting address by 0'ing out every 'x' bit and setting every '1' bit
    let base_address = (addr | one_bits_from_mask) & !x_bits_from_mask;

    let mut addresses = Vec::new();
    addresses.push(base_address);

    for floating_bit_index in floating_bit_indices.clone() {
        // the addresses currently in the vector represent the 0 setting of the current flip bit.
        // duplciate every address currently in the vector with the flip bit set to 1.

        // iterate over indices so we don't we don't invalidate the iterate when we push
        let current_address_count = addresses.len();
        for i in 0..current_address_count {
            addresses.push(set_bit(floating_bit_index, addresses[i]));
        }
    }

    addresses
}

#[derive(Debug)]
enum Instr {
    SetMask(Mask36),
    MemSet(usize, usize),
}

#[derive(Debug)]
struct Memory {
    set_addresses: HashMap<usize, usize>,
}

impl Memory {
    fn new() -> Self {
        Memory {
            set_addresses: HashMap::new(),
        }
    }

    fn set(&mut self, addr: usize, value: usize) {
        self.set_addresses.insert(addr, value);
    }

    fn sum_memory(&self) -> usize {
        self.set_addresses.iter().map(|(_, v)| v).sum()
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instr>,
}

impl Program {
    fn from_file(file_name: &str) -> Self {
        use regex::Regex;
        lazy_static! {
            // 0 = whole thing
            // 1 = mask = XXXXXXXXXXX
            // 2 =        ^
            // 3 = mem[8] = 11
            // 4 =     ^
            // 5 =          ^
            static ref R: &'static str = r"(mask = (.+))|(mem\[(\d+)\] = (.+))";
            static ref INSTR_REGEX: Regex = Regex::new(&R).unwrap();
        }

        fn mask_from_str(mask_str: &str) -> Mask36 {
            let mut mask = [TriBit::X; 36];
            for (i, c) in mask_str.chars().rev().enumerate() {
                mask[i] = match c {
                    '0' => TriBit::O,
                    '1' => TriBit::I,
                    'X' => TriBit::X,
                    _ => panic!("Invalid character in mask: {}", c),
                };
            }
            mask
        }

        let mut instructions = Vec::new();
        for line in input_helpers::read_lines(file_name) {
            let captures = INSTR_REGEX.captures(&line).unwrap();
            let instruction = match (captures.get(1), captures.get(3)) {
                (Some(_mask_match), None) => Instr::SetMask(mask_from_str(&captures[2])),
                (None, Some(_mem_match)) => {
                    Instr::MemSet(captures[4].parse().unwrap(), captures[5].parse().unwrap())
                }
                _ => unreachable!(),
            };
            instructions.push(instruction);
        }

        Program { instructions }
    }

    fn execute_v1(&self) -> Memory {
        let mut mem = Memory::new();
        let mut mask: Mask36 = [TriBit::X; 36];

        for instruction in &self.instructions {
            match instruction {
                Instr::SetMask(new_mask) => mask = *new_mask,
                Instr::MemSet(addr, value) => mem.set(*addr, apply_mask(&mask, *value)),
            }
        }

        mem
    }
    fn execute_v2(&self) -> Memory {
        let mut mem = Memory::new();
        let mut mask: Mask36 = [TriBit::O; 36];

        for instruction in &self.instructions {
            match instruction {
                Instr::SetMask(new_mask) => mask = *new_mask,
                Instr::MemSet(base_address, value) => {
                    let addresses = get_addresses_from_mask(&mask, *base_address);
                    for address in addresses {
                        mem.set(address, *value);
                    }
                }
            }
        }

        mem
    }
}

fn main() {
    let program = Program::from_file(&input_helpers::get_input_file_from_args(
        &mut std::env::args(),
    ));

    let initialized_memory_v1 = program.execute_v1();
    println!("MemV1: {:?}", initialized_memory_v1);
    println!("MemV1 sum: {}", initialized_memory_v1.sum_memory());

    let initialized_memory_v2 = program.execute_v2();
    println!("MemV2: {:?}", initialized_memory_v2);
    println!("MemV2 sum: {}", initialized_memory_v2.sum_memory());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_addresses_from_mask_1() {
        // address: 000000000000000000000000000000101010  (decimal 42)
        // mask:    000000000000000000000000000000X1001X
        // result:  000000000000000000000000000000X1101X
        let addr = 42;
        let mut mask = [TriBit::O; 36];
        mask[0] = TriBit::X;
        mask[1] = TriBit::I;
        mask[2] = TriBit::O;
        mask[3] = TriBit::O;
        mask[4] = TriBit::I;
        mask[5] = TriBit::X;

        let mut addresses = get_addresses_from_mask(&mask, addr);
        addresses.sort();

        let expected_addresses = [26, 27, 58, 59];
        for (addr, expected_addr) in addresses.iter().zip(expected_addresses.iter()) {
            assert_eq!(addr, expected_addr);
        }
    }

    #[test]
    fn test_get_addresses_from_mask_2() {
        // address: 000000000000000000000000000000011010  (decimal 26)
        // mask:    00000000000000000000000000000000X0XX
        // result:  00000000000000000000000000000001X0XX
        let addr = 26;
        let mut mask = [TriBit::O; 36];
        mask[0] = TriBit::X;
        mask[1] = TriBit::X;
        mask[2] = TriBit::O;
        mask[3] = TriBit::X;

        let mut addresses = get_addresses_from_mask(&mask, addr);
        addresses.sort();

        let expected_addresses = [16, 17, 18, 19, 24, 25, 26, 27];
        for (addr, expected_addr) in addresses.iter().zip(expected_addresses.iter()) {
            assert_eq!(addr, expected_addr);
        }
    }
}
