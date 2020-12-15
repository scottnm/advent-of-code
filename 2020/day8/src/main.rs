#[macro_use]
extern crate lazy_static;
extern crate regex;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug, PartialEq)]
enum RunResult {
    Corrupted,
    Succeeded,
}

#[derive(Debug, Clone)]
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
                "nop" => Instruction::Nop(argument),
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

    fn run(&mut self) -> RunResult {
        let mut instr_index: usize = 0;
        while instr_index < self.instruction_tracker.len() && !self.instruction_tracker[instr_index]
        {
            let instruction = &self.instructions[instr_index];
            self.instruction_tracker[instr_index] = true;

            let instr_inc = match instruction {
                Instruction::Acc(arg) => {
                    self.accumulator += arg;
                    1
                }
                Instruction::Jmp(arg) => *arg,
                Instruction::Nop(_) => 1,
            };
            instr_index = ((instr_index as isize) + instr_inc) as usize;
        }

        if instr_index < self.instructions.len() {
            RunResult::Corrupted // we looped
        } else {
            assert_eq!(instr_index, self.instructions.len());
            RunResult::Succeeded // we completed the program
        }
    }

    fn find_corrupted_instruction(&self) -> Option<(Instruction, usize)> {
        for i in 0..self.instructions.len() {
            let mut scratch_program = self.clone();
            let new_instruction = match scratch_program.instructions[i] {
                Instruction::Acc(_) => continue,
                Instruction::Jmp(arg) => Instruction::Nop(arg),
                Instruction::Nop(arg) => Instruction::Jmp(arg),
            };

            scratch_program.instructions[i] = new_instruction;
            match scratch_program.run() {
                RunResult::Corrupted => continue,
                RunResult::Succeeded => return Some((new_instruction, i)),
            }
        }

        None
    }

    fn fix_instruction(&mut self, fixed_instruction: Instruction, instruction_index: usize) {
        self.instructions[instruction_index] = fixed_instruction;
    }

    fn acc(&self) -> isize {
        self.accumulator
    }
}

fn main() {
    let mut program = Program::load("src/input.txt");

    let (fixed_instruction, instruction_index) = program.find_corrupted_instruction().unwrap();
    println!(
        "Instruction fix: {:?},{}",
        fixed_instruction, instruction_index
    );

    program.fix_instruction(fixed_instruction, instruction_index);
    let run_result = program.run();
    assert_eq!(run_result, RunResult::Succeeded);
    println!("Accumulator after loop: {}", program.acc());
}
