use input_helpers;
use simple_grid::{Grid, GridPos};
use std::process::ExitCode;

#[derive(Debug)]
struct CpuState {
    instruction_pointer: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
}

impl std::fmt::Display for CpuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(IP:{}, A:{}, B:{}, C:{})", 
            self.instruction_pointer,
            self.reg_a,
            self.reg_b,
            self.reg_c)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LiteralOperand {
    Lit0,
    Lit1,
    Lit2,
    Lit3,
    Lit4,
    Lit5,
    Lit6,
    Lit7,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ComboOperand {
    Lit0,
    Lit1,
    Lit2,
    Lit3,
    RegA,
    RegB,
    RegC,
}

impl LiteralOperand {
    fn from_char(c: char) -> Result<Self, String> {
        let op = match c {
            '0' => LiteralOperand::Lit0,
            '1' => LiteralOperand::Lit1,
            '2' => LiteralOperand::Lit2,
            '3' => LiteralOperand::Lit3,
            '4' => LiteralOperand::Lit4,
            '5' => LiteralOperand::Lit5,
            '6' => LiteralOperand::Lit6,
            '7' => LiteralOperand::Lit7,
            _ => return Err(format!("Invalid literal op '{}'", c)),
        };

        Ok(op)
    }

    fn to_char(&self) -> char {
        match *self {
            LiteralOperand::Lit0 => '0',
            LiteralOperand::Lit1 => '1',
            LiteralOperand::Lit2 => '2',
            LiteralOperand::Lit3 => '3',
            LiteralOperand::Lit4 => '4',
            LiteralOperand::Lit5 => '5',
            LiteralOperand::Lit6 => '6',
            LiteralOperand::Lit7 => '7',
        }
    }
}

impl ComboOperand {
    fn from_char(c: char) -> Result<Self, String> {
        let op = match c {
            '0' => ComboOperand::Lit0,
            '1' => ComboOperand::Lit1,
            '2' => ComboOperand::Lit2,
            '3' => ComboOperand::Lit3,
            '4' => ComboOperand::RegA,
            '5' => ComboOperand::RegB,
            '6' => ComboOperand::RegC,
            '7' => return Err(String::from("Combo operator 7 is reserved and invalid")),
            _ => return Err(format!("Invalid literal op '{}'", c)),
        };

        Ok(op)
    }

    fn to_char(&self) -> char {
        match *self {
            ComboOperand::Lit0 => '0',
            ComboOperand::Lit1 => '1',
            ComboOperand::Lit2 => '2',
            ComboOperand::Lit3 => '3',
            ComboOperand::RegA => '4',
            ComboOperand::RegB => '5',
            ComboOperand::RegC => '6',
        }
    }
}

fn ignored_op_from_char(c: char) -> Result<(), String> {
    match c {
        '0'..='7' => Ok(()),
        _ => Err(format!("Invalid op (ignored) '{}'", c)),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct AdvInstr {
    op: ComboOperand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct BxlInstr {
    op: LiteralOperand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct BstInstr {
    op: ComboOperand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct JnzInstr {
    op: LiteralOperand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct OutInstr {
    op: ComboOperand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct BdvInstr {
    op: ComboOperand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct CdvInstr {
    op: ComboOperand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instr {
    Adv(AdvInstr), // Op(0): RegA = RegA / 2^(op)
    Bxl(BxlInstr), // Op(1): RegB = RegB ^ (lit)
    Bst(BstInstr), // Op(2): RegB = (op) % 8
    Jnz(JnzInstr), // Op(3): if (RegA!=0) *ip = (lit)
    Bxc,           // Op(4): RegB = RegB ^ RegC    
    Out(OutInstr), // Op(5): [output] (op) % 8 
    Bdv(BdvInstr), // Op(6): RegB = RegA / 2^(op)
    Cdv(CdvInstr), // Op(7): RegC = RegA / 2^(op)
}

fn parse_register_line(line: &str, reg_prefix: &str) -> Result<usize, String> {
    if !line.starts_with(reg_prefix) {
        return Err(format!("register line missing prefix '{}'! '{}'", reg_prefix, line));
    }

    let reg_val_str = &line[reg_prefix.len()..];
    reg_val_str.parse().map_err(|_| format!("Failed to parse register value '{}'", reg_val_str))
}

fn read_initial_cpu_state(filename: &str) -> Result<(CpuState, Vec<Instr>), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() != 5 {
        return Err(format!(
            "Invalid input! Expected 5 lines. Had {}",
            lines.len()
        ));
    }

    let reg_a_line = &lines[0];
    let reg_b_line = &lines[1];
    let reg_c_line = &lines[2];
    let separator_line = &lines[3];
    let program_line = &lines[4];

    if separator_line != "" {
        return Err(format!("invalid separator line! '{}'", separator_line));
    }

    let cpu_state = {
        let reg_a = parse_register_line(&reg_a_line, "Register A: ")?;
        let reg_b = parse_register_line(&reg_b_line, "Register B: ")?;
        let reg_c = parse_register_line(&reg_c_line, "Register C: ")?;
        CpuState { instruction_pointer: 0, reg_a, reg_b, reg_c }
    };

    const PROGRAM_LINE_PREFIX: &str = "Program: ";
    if !program_line.starts_with(PROGRAM_LINE_PREFIX) {
        return Err(format!("Invalid program line! '{}'", program_line))
    }

    let program_data = &program_line[PROGRAM_LINE_PREFIX.len()..];

    let mut instructions = vec![];

    fn verify_one_char(s: &str) -> Result<char, String> {
        let mut s_chars = s.chars();
        let c = if let Some(c) = s_chars.next() {
            c
        } else {
            return Err(format!("'{}' is not a single char", s));
        };

        if let Some(_) = s_chars.next() {
            return Err(format!("'{}' is not a single char", s));
        }

        Ok(c)
    }

    let mut split_iter = program_data.split(',');
    while let Some(next_operation) = split_iter.next() {
        let next_operator_char = verify_one_char(next_operation)?;

        let next_operand = split_iter.next().ok_or(format!("Missing operand for operator {}", next_operator_char))?;
        let next_operand_char = verify_one_char(next_operand)?;

        let instr = match next_operator_char {
            '0' => { 
                let op = ComboOperand::from_char(next_operand_char)?;
                Instr::Adv(AdvInstr{op}) 
            },
            '1' => { 
                let op = LiteralOperand::from_char(next_operand_char)?;
                Instr::Bxl(BxlInstr{op}) 
            },
            '2' => { 
                let op = ComboOperand::from_char(next_operand_char)?;
                Instr::Bst(BstInstr{op}) 
            },
            '3' => { 
                let op = LiteralOperand::from_char(next_operand_char)?;
                Instr::Jnz(JnzInstr{op}) 
            },
            '4' => { 
                ignored_op_from_char(next_operand_char)?;
                Instr::Bxc
            },
            '5' => { 
                let op = ComboOperand::from_char(next_operand_char)?;
                Instr::Out(OutInstr{op}) 
            },
            '6' => { 
                let op = ComboOperand::from_char(next_operand_char)?;
                Instr::Bdv(BdvInstr{op}) 
            },
            '7' => { 
                let op = ComboOperand::from_char(next_operand_char)?;
                Instr::Cdv(CdvInstr{op}) 
            },
            _ => return Err(format!("Invalid operator! {}", next_operator_char)),
        };

        instructions.push(instr);
    }

    Ok((cpu_state, instructions))
}

fn do_next_instruction(cpu: &mut CpuState, instructions: &[Instr]) -> Result<Option<usize>, String> {
    if cpu.instruction_pointer % 2 != 0 {
        return Err(format!("instruction pointer @ {} not pointing to valid instruction", cpu.instruction_pointer));
    }
    unimplemented!();
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();

    let (mut cpu_state, instructions) = read_initial_cpu_state(filename)?;

    dbg!(&cpu_state);
    dbg!(&instructions);

    {
        let mut output = vec![];
        while cpu_state.instruction_pointer < (instructions.len() * 2) {
            let instr_output = do_next_instruction(&mut cpu_state, &instructions)?;
            if let Some(instr_output) = instr_output {
                output.push(instr_output);
            }
        }

        let output_str = output
            .iter()
            .map(|output_val| output_val.to_string())
            .collect::<Vec<String>>()
            .join(",");
            
        println!("Pt 1: output = {}", output_str);
    }

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
