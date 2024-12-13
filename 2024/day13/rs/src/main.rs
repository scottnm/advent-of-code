use input_helpers;
use std::process::ExitCode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ClawMachine {
    button_a_move: Vec2,
    button_b_move: Vec2,
    prize_pos: Vec2,    
}

impl std::fmt::Display for ClawMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClawMachine{{A(+{},+{}); B(+{},{}); Prize({},{})}}",
            self.button_a_move.x, 
            self.button_a_move.y, 
            self.button_b_move.x, 
            self.button_b_move.y, 
            self.prize_pos.x, 
            self.prize_pos.y)
    }
}

impl std::fmt::Display for ClawMachineSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClawMachineSolution{{A: {}, B: {}}}",
            self.a_press_count,
            self.b_press_count)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ClawMachineSolution {
    a_press_count: usize,
    b_press_count: usize,
}

fn read_claw_machine_summaries(filename: &str) -> Result<Vec<ClawMachine>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.is_empty() {
        return Ok(vec![]);
    }

    if (lines.len() + 1) % 4 != 0 {
        return Err(format!("Invalid number of lines! Expecting 3 lines per claw machine with 1 line in between each machine description. Found {}",
            lines.len()));
    }

    let button_a_line_re = regex::Regex::new(r"Button\s+A:\s+X\+(\d+),\s+Y\+(\d+)").unwrap();
    let button_b_line_re = regex::Regex::new(r"Button\s+B:\s+X\+(\d+),\s+Y\+(\d+)").unwrap();
    let prize_line_re = regex::Regex::new(r"Prize:\s+X=(\d+),\s+Y=(\d+)").unwrap();
    
    fn parse_and_extract_vec2_from_int_captures(captures: Option<regex::Captures>, line_desc: &str) -> Result<Vec2, String> {
        let line_match = captures.ok_or(format!("Missing '{}'", line_desc))?;

        let x: isize = line_match
            .get(1)
            .ok_or(format!("Missing 'x' on {} line", line_desc))?
            .as_str()
            .parse()
            .map_err(|_| format!("Failed to parse {}'s x value as int", line_desc))?;

        let y: isize = line_match
            .get(2)
            .ok_or(format!("Missing 'y' on {} line", line_desc))?
            .as_str()
            .parse()
            .map_err(|_| format!("Failed to parse {}'s y value as int", line_desc))?;
        
        Ok(Vec2 {x, y})
    }

    let read_claw_machine_summary = |button_a_line: &str, button_b_line: &str, prize_line: &str| -> Result<ClawMachine, String> {
        let button_a_vec = parse_and_extract_vec2_from_int_captures(button_a_line_re.captures(button_a_line), "Button A")?;
        let button_b_vec = parse_and_extract_vec2_from_int_captures(button_b_line_re.captures(button_b_line), "Button B")?;
        let prize_vec = parse_and_extract_vec2_from_int_captures(prize_line_re.captures(prize_line), "Prize")?;

        Ok(ClawMachine{button_a_move: button_a_vec, button_b_move: button_b_vec, prize_pos: prize_vec})
    };

    let mut claw_machines = vec![];

    let mut line_idx = 0;

    assert!(lines.len() >= 3); // verified by the modulo check above
    while line_idx <= (lines.len() - 3) {
        let button_a_line = &lines[line_idx + 0];
        let button_b_line = &lines[line_idx + 1];
        let prize_line = &lines[line_idx + 2];
        line_idx += 3;

        if line_idx < lines.len() {
            if lines[line_idx] != "" {
                return Err(format!("Expected empty line at idx {}! Found '{}'", line_idx, &lines[line_idx]));
            }
            line_idx += 1;
        }

        let next_machine = read_claw_machine_summary(button_a_line, button_b_line, prize_line)?;
        claw_machines.push(next_machine);
    }

    Ok(claw_machines)
}

fn find_all_solutions(claw_machine: &ClawMachine) -> Vec<ClawMachineSolution> {
    // TODO: real solution
    vec![ClawMachineSolution{a_press_count: 0, b_press_count: 0}]
}

fn count_tokens_for_solution(solution: &ClawMachineSolution) -> usize {
    (solution.a_press_count * 3) + solution.b_press_count
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_claw_machine_summaries(filename);
    let claw_machines = match parse_result {
        Ok(claw_machines) => claw_machines,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    {
        let print_machines = claw_machines.len() < 10;
        let mut total_min_tokens: Option<usize> = None;
        for claw_machine in &claw_machines {
            let solutions = find_all_solutions(claw_machine);
            let min_cost_solution = solutions
                .iter()
                .enumerate()
                .map(|(i, solution)| (i, count_tokens_for_solution(solution)))
                .min_by_key(|(_i, solution_token_count)| solution_token_count.clone());
            if let Some((solution_idx, min_cost_solution_token_count)) = min_cost_solution {
                if let Some(token_count) = total_min_tokens {
                    total_min_tokens = Some(token_count + min_cost_solution_token_count);
                } else {
                    total_min_tokens = Some(min_cost_solution_token_count);
                }

                if print_machines {
                    println!("{}", claw_machine);
                    println!("{}", solutions[solution_idx]);
                    println!("");
                }
            } else {
                if print_machines {
                    println!("{}", claw_machine);
                    println!("NO SOLUTION");
                    println!("");
                }
            }
        }

        if let Some(total_min_tokens) = total_min_tokens {
            println!("Pt 1: min token count = {}", total_min_tokens);
        } else {
            println!("Pt 1: min token count = NO SOLUTIONS");
        }
    }

    // println!("");

    /*
    {
        let compacted_disk_chunks = compact_disk_pt2(&disk_chunks);
        let checksum = calculate_checksum(&compacted_disk_chunks);
        println!("Pt 2: checksum = {}", checksum);
        if compacted_disk_chunks.len() < 20 {
            println!("Original layout:  {}", stringify_disk_layout(&disk_chunks));
            println!(
                "Compacted layout: {}",
                stringify_disk_layout(&compacted_disk_chunks)
            );
            //dbg!(compacted_disk_chunks);
        }
    } */

    return ExitCode::SUCCESS;
}