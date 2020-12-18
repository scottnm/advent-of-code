#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

type Mask36 = [Option<bool>; 36];

fn apply_mask(mask: &Mask36, value: usize) -> usize {
    fn set_bit(index: usize, value: usize) -> usize {
        let mask = 1usize << index;
        value | mask
    }

    fn clear_bit(index: usize, value: usize) -> usize {
        let mask = !(1usize << index);
        value & mask
    }

    let mut masked_value = value;
    for (i, mask_bit) in mask.iter().enumerate() {
        masked_value = match mask_bit {
            Some(false) => clear_bit(i, masked_value),
            Some(true) => set_bit(i, masked_value),
            None => masked_value,
        };
    }
    masked_value
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
            let mut mask = [None; 36];
            for (i, c) in mask_str.chars().rev().enumerate() {
                mask[i] = match c {
                    '0' => Some(false),
                    '1' => Some(true),
                    'X' => None,
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

    fn execute(&self) -> Memory {
        let mut mem = Memory::new();
        let mut mask: Mask36 = [None; 36];

        for instruction in &self.instructions {
            match instruction {
                Instr::SetMask(new_mask) => mask = *new_mask,
                Instr::MemSet(addr, value) => mem.set(*addr, apply_mask(&mask, *value)),
            }
        }

        mem
    }
}

fn main() {
    let program = Program::from_file(&input_helpers::get_input_file_from_args(
        &mut std::env::args(),
    ));

    let initialized_memory = program.execute();
    println!("Mem: {:?}", initialized_memory);
    println!("Mem sum: {}", initialized_memory.sum_memory());
}
