#[macro_use]
extern crate lazy_static;
extern crate regex;

#[derive(Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop,
}

#[derive(Debug)]
struct Program {
    accumulator: isize,
    instruction_tracker: Vec<bool>,
    instructions: Vec<Instruction>,
}

impl Program {
    fn load(file_name: &str) -> Self {
        lazy_static! {
            // EXAMPLES:
            // light red bags contain 1 bright white bag, 2 muted yellow bags.
            // bright white bags contain 1 shiny gold bag.
            // faded blue bags contain no other bags.

            static ref INSTR_REGEX: regex::Regex =
                regex::Regex::new(r"(\w+) ([+-])(\d+)").unwrap();
        }

        let mut instructions = Vec::new();
        for line in input_helpers::read_lines(file_name) {
            let captures = INSTR_REGEX.captures(&line).unwrap();

            let multiplier: isize = match &captures[2] {
                "+" => 1,
                "-" => -1,
                _ => panic!("invalid multiplier"),
            };

            let argument = captures[3].parse::<isize>().unwrap() * multiplier;

            let instruction = match &captures[1] {
                "acc" => Instruction::Acc(argument),
                "jmp" => Instruction::Jmp(argument),
                "nop" => Instruction::Nop,
                _ => panic!("Invalid instruction"),
            };

            instructions.push(instruction);
        }

        let accumulator = 0;
        let instruction_tracker = instructions.iter().map(|_| false).collect();
        Program {
            accumulator,
            instruction_tracker,
            instructions,
        }
    }

    fn run_until_loop(&mut self) {
        let mut instr_index: usize = 0;
        while !self.instruction_tracker[instr_index] {
            let instruction = &self.instructions[instr_index];
            self.instruction_tracker[instr_index] = true;

            println!("Executing {:?}", instruction);
            let instr_inc = match instruction {
                Instruction::Acc(arg) => {
                    self.accumulator += arg;
                    1
                }
                Instruction::Jmp(arg) => *arg,
                Instruction::Nop => 1,
            };
            instr_index = ((instr_index as isize) + instr_inc) as usize;
        }
    }

    fn acc(&self) -> isize {
        self.accumulator
    }
}

fn main() {
    let mut program = Program::load("src/input.txt");
    program.run_until_loop();
    println!("Accumulator after loop: {}", program.acc());
}
