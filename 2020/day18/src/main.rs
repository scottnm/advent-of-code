#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub aem_calculator); // calculator with "Add Eq Mul" precendence
lalrpop_mod!(pub abm_calculator); // calculator with "Add Before Mul" precendence

fn main() {
    let input_file = input_helpers::get_input_file_from_args();
    let expressions: Vec<String> = input_helpers::read_lines(&input_file).collect();

    let aem_parser = aem_calculator::ExprParser::new();
    let aem_results: Vec<isize> = expressions
        .iter()
        .map(|e| aem_parser.parse(e).unwrap())
        .collect();

    println!("Sum of AEM results: {}", aem_results.iter().sum::<isize>());

    let abm_parser = abm_calculator::ExprParser::new();
    let abm_results: Vec<isize> = expressions
        .iter()
        .map(|e| abm_parser.parse(e).unwrap())
        .collect();

    println!("Sum of AEM results: {}", abm_results.iter().sum::<isize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculator_test() {
        let abm_parser = abm_calculator::ExprParser::new();
        let exprs = [
            ("(22)", 22),
            ("(22) + (5 * 4)", 42),
            ("(2 * 11) + (5 * 4)", 42),
            ("(1 + 1 * 11) + (5 * 4)", 42),
            ("(1 + (1 * 11)) + (5 * 4)", 32),
        ];

        for (test_expr, expected_value) in &exprs {
            println!("Testing...{}", test_expr);
            let parse_result = abm_parser.parse(test_expr);
            assert_eq!(parse_result.unwrap(), *expected_value);
        }
    }

    #[test]
    fn pt1_sample_test() {
        let aem_parser = aem_calculator::ExprParser::new();
        let exprs = [
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];

        for (test_expr, expected_value) in &exprs {
            println!("Testing...{}", test_expr);
            let parse_result = aem_parser.parse(test_expr);
            assert_eq!(parse_result.unwrap(), *expected_value);
        }
    }
}
