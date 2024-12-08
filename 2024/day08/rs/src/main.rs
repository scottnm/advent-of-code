use input_helpers;
use std::process::ExitCode;

#[derive(Debug, Clone)]
struct Equation {
    result: usize,
    operands: Vec<usize>,
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Concat,
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
    let result_sep_idx = line
        .find(": ")
        .ok_or(format!("Failed to find result separator in equation line"))?;

    if line.len() <= result_sep_idx + 2 {
        return Err(format!("Missing operands after separator"));
    }

    let result_str = &line[..result_sep_idx];
    let result: usize = result_str
        .parse()
        .map_err(|err| format!("Failed to parse result value '{}'! {}", result_str, err))?;
    let operands_str = &line[(result_sep_idx + 2)..];
    let mut operands: Vec<usize> = vec![];
    for operand_str in operands_str.split_ascii_whitespace() {
        let operand = operand_str
            .parse()
            .map_err(|err| format!("Failed to parse operand value '{}'! {}", operand_str, err))?;
        operands.push(operand);
    }

    Ok(Equation { result, operands })
}

fn solve_recursive_any_pt1(equation: &Equation) -> Option<Vec<Operation>> {
    fn solve_recursive_any_helper(
        exp_result: usize,
        curr_value: usize,
        operands_left: &[usize],
    ) -> Option<Vec<Operation>> {
        if operands_left.len() == 0 {
            return if exp_result == curr_value {
                Some(vec![])
            } else {
                None
            };
        }

        let next_rhs_op: usize = operands_left[0];
        let next_add_candidate = curr_value + next_rhs_op;
        if let Some(mut solved_operation_list) =
            solve_recursive_any_helper(exp_result, next_add_candidate, &operands_left[1..])
        {
            solved_operation_list.push(Operation::Add);
            Some(solved_operation_list)
        } else {
            let next_mul_candidate = curr_value * next_rhs_op;
            if let Some(mut solved_operation_list) =
                solve_recursive_any_helper(exp_result, next_mul_candidate, &operands_left[1..])
            {
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

    if let Some(mut solution) = solve_recursive_any_helper(
        equation.result,
        equation.operands[0],
        &equation.operands[1..],
    ) {
        // the operands from the recursive any helper are returned in reverse order for efficiency (vector pushback is faster than vector pushfront)
        solution.reverse();
        Some(solution)
    } else {
        None
    }
}

fn concat_values(lhs: usize, rhs: usize) -> usize {
    // max u64 value = 9,223,372,036,854,775,807
    let mut rhs_digit_counter = rhs;
    let mut lhs_concat_scale = 10;
    while rhs_digit_counter >= 10 {
        rhs_digit_counter /= 10;
        lhs_concat_scale *= 10;
    }

    (lhs * lhs_concat_scale) + rhs
}

fn solve_recursive_any_pt2(equation: &Equation) -> Option<Vec<Operation>> {
    fn solve_recursive_any_helper(
        exp_result: usize,
        curr_value: usize,
        operands_left: &[usize],
    ) -> Option<Vec<Operation>> {
        if operands_left.len() == 0 {
            return if exp_result == curr_value {
                Some(vec![])
            } else {
                None
            };
        }

        let next_rhs_op: usize = operands_left[0];
        let next_add_candidate = curr_value + next_rhs_op;
        if let Some(mut solved_operation_list) =
            solve_recursive_any_helper(exp_result, next_add_candidate, &operands_left[1..])
        {
            solved_operation_list.push(Operation::Add);
            Some(solved_operation_list)
        } else {
            let next_mul_candidate = curr_value * next_rhs_op;
            if let Some(mut solved_operation_list) =
                solve_recursive_any_helper(exp_result, next_mul_candidate, &operands_left[1..])
            {
                solved_operation_list.push(Operation::Mul);
                Some(solved_operation_list)
            } else {
                let next_concat_candidate = concat_values(curr_value, next_rhs_op);
                if let Some(mut solved_operation_list) = solve_recursive_any_helper(
                    exp_result,
                    next_concat_candidate,
                    &operands_left[1..],
                ) {
                    solved_operation_list.push(Operation::Concat);
                    Some(solved_operation_list)
                } else {
                    None
                }
            }
        }
    }

    if equation.operands.len() == 0 {
        return solve_recursive_any_helper(equation.result, 0, &[]);
    }

    if let Some(mut solution) = solve_recursive_any_helper(
        equation.result,
        equation.operands[0],
        &equation.operands[1..],
    ) {
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

    println!("Pt 1:");

    let solved_equations_pt1: Vec<(Equation, Vec<Operation>)> = equations
        .iter()
        .map(|eq| (eq.clone(), solve_recursive_any_pt1(&eq)))
        .filter(|(_eq, solution)| solution.is_some())
        .map(|(eq, solution)| (eq, solution.unwrap()))
        .collect();

    // for (eq, _) in &solved_equations_pt1 {
    //     println!("Found sol for {:?}", eq);
    // }

    let sum_solvable_results_pt1: usize =
        solved_equations_pt1.iter().map(|(eq, _sol)| eq.result).sum();
    println!("Sum of solution results: {}", sum_solvable_results_pt1);

    println!("");

    println!("Pt 2:");

    let solved_equations_pt2: Vec<(Equation, Vec<Operation>)> = equations
        .iter()
        .map(|eq| (eq.clone(), solve_recursive_any_pt2(&eq)))
        .filter(|(_eq, solution)| solution.is_some())
        .map(|(eq, solution)| (eq, solution.unwrap()))
        .collect();

    // for (eq, _) in &solved_equations_pt2 {
    //     println!("Found sol for {:?}", eq);
    // }

    let sum_solvable_results_pt2: usize =
        solved_equations_pt2.iter().map(|(eq, _sol)| eq.result).sum();
    println!("Sum of solution results: {}", sum_solvable_results_pt2);

    return ExitCode::SUCCESS;
}
