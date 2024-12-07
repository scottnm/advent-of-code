use input_helpers;
use std::process::ExitCode;

#[derive(Debug, Clone)]
struct Equation {
    result: isize,
    operands: Vec<isize>,
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

fn read_equations(filename: &str) -> Result<Vec<Equation>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut equations = vec![];
    for line in lines {
        let eq = parse_equation_from_line(&line)?;
        equations.push(eq);
    }

    Ok(equations)
}

fn parse_equation_from_line(line: &str) -> Result<Equation, String> {
    let result_sep_idx = line.find(": ")
        .ok_or(format!("Failed to find result separator in equation line"))?;

    if line.len() <= result_sep_idx + 2 {
        return Err(format!("Missing operands after separator"));
    }

    let result_str = &line[..result_sep_idx];
    let result: isize = result_str.parse().map_err(|err| format!("Failed to parse result value '{}'! {}", result_str, err))?;
    let operands_str = &line[(result_sep_idx+2)..];
    let mut operands: Vec<isize> = vec![];
    for operand_str in operands_str.split_ascii_whitespace() {
        let operand = operand_str.parse().map_err(|err| format!("Failed to parse operand value '{}'! {}", operand_str, err))?;
        operands.push(operand);
    }

    Ok(Equation{result, operands})
}

fn solve_recursive_any(equation: &Equation) -> Option<Vec<Operation>> {
    fn solve_recursive_any_helper(exp_result: isize, curr_value: isize, operands_left: &[isize]) -> Option<Vec<Operation>> {
        if operands_left.len() == 0 {
            return if exp_result == curr_value {
                Some(vec![])
            } else {
                None
            }
        }

        let next_rhs_op: isize = operands_left[0];
        let next_add_candidate = curr_value + next_rhs_op;
        if let Some(mut solved_operation_list) = solve_recursive_any_helper(exp_result, next_add_candidate, &operands_left[1..]) {
            solved_operation_list.push(Operation::Add);
            Some(solved_operation_list)
        } else {
            let next_mul_candidate = curr_value * next_rhs_op;
            if let Some(mut solved_operation_list) = solve_recursive_any_helper(exp_result, next_mul_candidate, &operands_left[1..]) {
                solved_operation_list.push(Operation::Mul);
                Some(solved_operation_list)
            } else {
                None
            }
        }
    }

    if equation.operands.len() == 0 {
        return solve_recursive_any_helper(equation.result, 0, &[]);
    }

    if let Some(mut solution) = solve_recursive_any_helper(equation.result, equation.operands[0], &equation.operands[1..]) {
        // the operands from the recursive any helper are returned in reverse order for efficiency (vector pushback is faster than vector pushfront)
        solution.reverse();
        Some(solution)
    } else {
        None
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_equations(filename);
    let equations = match parse_result {
        Ok(equations) => equations,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    let solved_equations: Vec<(Equation, Vec<Operation>)> = 
        equations
        .iter()
        .map(|eq| (eq.clone(), solve_recursive_any(&eq)))
        .filter(|(eq, solution)| solution.is_some())
        .map(|(eq, solution)| (eq, solution.unwrap()))
        .collect();

    for (eq, _) in &solved_equations {
        println!("Found sol for {:?}", eq);
    }

    let sum_solvable_results: isize = solved_equations.iter().map(|(eq, sol)| eq.result).sum();
    println!("Sum of solution results: {}", sum_solvable_results);

    /*
    let player_initial_state = player_state.clone();

    let pt1_start_time = std::time::Instant::now();

    let mut player_space_history = std::collections::HashSet::new();

    //print_board_state(&grid, &player_state);

    loop {
        player_space_history.insert(player_state.pos);

        player_state = simulate_board_step(&grid, &player_state);
        //print_board_state(&grid, &player_state);

        if grid.is_pos_out_of_bounds(player_state.pos.row, player_state.pos.col) {
            break;
        }
    }

    let pt1_time = pt1_start_time.elapsed();

    println!(
        "{} unique player positions",
        player_space_history.len()
    );

    //print_move_history(&grid, &player_space_history);

    println!("TIME: ({:0.06}s)", pt1_time.as_secs_f64());

    println!("");

    let pt2_start_time = std::time::Instant::now();

    let mut looping_obstructions: Vec<GridPos> = Vec::new();
    
    let obstruction_candidates = {
        let mut obstruction_candidates = player_space_history.clone();
        obstruction_candidates.remove(&player_initial_state.pos);
        obstruction_candidates
    };

    for visited_player_space in obstruction_candidates {
        let mut obstructed_grid = Grid { width: grid.width, height: grid.height, cells: grid.cells.clone() };
        {
            let cell_ref = obstructed_grid.get_cell_mut(visited_player_space.row, visited_player_space.col);
            assert!(*cell_ref == Cell::Empty);
            *cell_ref = Cell::Obstacle;
        }

        let obstruction_loops_player = test_for_simulation_loop(&obstructed_grid, &player_initial_state);
        if obstruction_loops_player {
            looping_obstructions.push(visited_player_space);
        }

        // println!("Placing obstruction at (r:{},c:{}) looped player? {}",
        //     visited_player_space.row,
        //     visited_player_space.col,
        //     obstruction_loops_player);
    }

    let pt2_time = pt2_start_time.elapsed();
    println!("Found {} obstructions which cause loops", looping_obstructions.len());
    println!("TIME: ({:0.06}s)", pt2_time.as_secs_f64());
    */

    return ExitCode::SUCCESS;
}
