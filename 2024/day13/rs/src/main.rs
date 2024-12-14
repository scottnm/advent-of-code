use input_helpers;
use std::process::ExitCode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vec2 {
    x: usize,
    y: usize,
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

        let x: usize = line_match
            .get(1)
            .ok_or(format!("Missing 'x' on {} line", line_desc))?
            .as_str()
            .parse()
            .map_err(|_| format!("Failed to parse {}'s x value as int", line_desc))?;

        let y: usize = line_match
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

fn calculate_claw_position(claw_machine: &ClawMachine, a_press_cnt: usize, b_press_cnt: usize) -> Vec2 {
    Vec2 {
        x: (claw_machine.button_a_move.x * a_press_cnt) + (claw_machine.button_b_move.x * b_press_cnt),
        y: (claw_machine.button_a_move.y * a_press_cnt) + (claw_machine.button_b_move.y * b_press_cnt),
    }
}

fn find_all_solutions(claw_machine: &ClawMachine) -> Vec<ClawMachineSolution> {
    let mut solutions = vec![];

    let max_a_presses_to_x = claw_machine.prize_pos.x / claw_machine.button_a_move.x;
    let max_a_presses_to_y = claw_machine.prize_pos.y / claw_machine.button_a_move.y;
    let max_a_presses = std::cmp::min(max_a_presses_to_x, max_a_presses_to_y);

    let max_b_presses_to_x = claw_machine.prize_pos.x / claw_machine.button_b_move.x;
    let max_b_presses_to_y = claw_machine.prize_pos.y / claw_machine.button_b_move.y;
    let max_b_presses = std::cmp::min(max_b_presses_to_x, max_b_presses_to_y);

    for a_press_cnt in 0..max_a_presses+1 {
        for b_press_cnt in 0..max_b_presses+1 {
            let claw_position = calculate_claw_position(claw_machine, a_press_cnt, b_press_cnt);

            // Found solution
            if claw_position == claw_machine.prize_pos {
                solutions.push(ClawMachineSolution{a_press_count: a_press_cnt, b_press_count: b_press_cnt});
                break;
            }

            // No solution
            if claw_position.x > claw_machine.prize_pos.x || claw_position.y > claw_machine.prize_pos.y {
                break;
            }
        }
    }

    solutions
}

fn count_tokens_for_solution(solution: &ClawMachineSolution) -> usize {
    (solution.a_press_count * 3) + solution.b_press_count
}

fn get_nth_string_arg<'a>(args: &'a [String], n: usize) -> Result<&'a str, String> {
    if args.len() <= n {
        return Err(format!(
            "Too few args! needed {}; had {}",
            n + 1,
            args.len()
        ));
    }

    Ok(&args[n])
}

fn get_nth_parsed_arg<T>(args: &[String], n: usize) -> Result<T, String>
where
    T: std::str::FromStr,
{
    if args.len() <= n {
        return Err(format!(
            "Too few args! needed {}; had {}",
            n + 1,
            args.len()
        ));
    }

    match args[n].parse() {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Failed to parse arg! '{}'", &args[n])),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = get_nth_string_arg(args, 0)?;
    let claw_machine_offset: usize = get_nth_parsed_arg(args, 1)?;

    let claw_machines = read_claw_machine_summaries(filename)?;

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

    println!("");

    /*
    {
        let print_machines = claw_machines.len() < 10;
        let mut total_min_tokens: Option<usize> = None;
        for claw_machine in &claw_machines {
            let solutions = find_all_solutions_with_offset(claw_machine, claw_machine_offset);
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
            println!("Pt 2: min token count = {}", total_min_tokens);
        } else {
            println!("Pt 2: min token count = NO SOLUTIONS");
        }
    } */

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