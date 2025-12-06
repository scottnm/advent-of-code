import copy
import dataclasses
import logging
import os
import sys
import enum
import functools

current_dir = os.path.dirname(os.path.realpath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.append(project_root)
from shared import util

class Operator(enum.Enum):
    ADD = enum.auto()
    MUL = enum.auto()

@dataclasses.dataclass
class Problem:
    operands: list[int]
    operator: Operator

    def solve(self) -> int:
        match self.operator:
            case Operator.MUL:
                return functools.reduce(lambda a, b: a * b, self.operands, 1)
            case Operator.ADD:
                return functools.reduce(lambda a, b: a + b, self.operands, 0)
            case _:
                raise RuntimeError(f"Invalid operator: {self.operator}")

def process_input_pt1(lines: list[str]) -> list[Problem]:
    if len(lines) < 3:
        logging.error("bad input: need at least 3 lines")
        sys.exit(1)

    operator_line = lines[-1]
    problems: list[Problem] = []
    for operator_str in operator_line.split():
        match operator_str:
            case "*":
                problems.append(Problem(operands=[], operator=Operator.MUL))
            case "+":
                problems.append(Problem(operands=[], operator=Operator.ADD))
            case _:
                logging.error("bad input: invalid operator %s", operator_str)
                sys.exit(1)

    for line in lines[:-1]:
        next_operand_set = line.split()
        if len(next_operand_set) != len(problems):
            logging.error("bad input: every line expected to have %d operands. had %d operands",
                len(problems),
                len(next_operand_set))
            sys.exit(1)

        for i, operand_str in enumerate(next_operand_set):
            try:
                problems[i].operands.append(int(operand_str))
            except ValueError:
                logging.error("Invalid operand in line %d: '%s'", i, operand_str)
                sys.exit(1)
    return problems

def process_input_pt2(lines: list[str]) -> list[Problem]:
    if len(lines) < 3:
        logging.error("bad input: need at least 3 lines")
        sys.exit(1)

    operator_line = lines[-1]
    problem_operand_widths = []
    working_column_width = None
    for i, c in enumerate(operator_line):
        if c == " ":
            if working_column_width is None:
                logging.error("Invalid operator line! should start with operator: %s", operator_line)
                sys.exit(1)
            else:
                working_column_width += 1

            if i == (len(operator_line) - 1):
                working_column_width += 1 # the final column doesn't have a separator. add one to account for that
                problem_operand_widths.append(working_column_width)
                working_column_width = 0
        elif c == "*" or c == "+":
            if working_column_width is not None:
                problem_operand_widths.append(working_column_width)
            working_column_width = 0
    logging.debug("operator line: %s", operator_line)
    logging.debug("problem widths: %s", problem_operand_widths)
    logging.debug("final working column width: %d", working_column_width)

    operator_line = lines[-1]
    problems: list[Problem] = []
    line_offset = 0
    for problem_operand_width in problem_operand_widths:
        column_width = problem_operand_width + 1
        operator_str = operator_line[line_offset:line_offset+column_width].rstrip()
        match operator_str:
            case "*":
                operator = Operator.MUL
            case "+":
                operator = Operator.ADD
            case _:
                logging.error("bad input: invalid operator %s", operator_str)
                sys.exit(1)

        operand_row_strings = [ line[line_offset:line_offset+problem_operand_width] for line in lines[:-1] ]
        ceph_operands = read_ceph_operands(operand_row_strings)
        problems.append(Problem(operands=ceph_operands, operator=operator))

        line_offset += column_width

    return problems

def read_ceph_operands(digit_rows: list[str]) -> list[int]:
    if len(digit_rows) == 0:
        return []

    row_width = len(digit_rows[0])
    assert all(len(digit_row) == row_width for digit_row in digit_rows)

    operands: list[int] = []
    for i in range(row_width - 1, -1, -1):
        digit_cols = [digit_row[i] for digit_row in digit_rows]
        digit_col_padded_str = functools.reduce(lambda a,b: a+b, digit_cols, "")
        try:
            operand = int(digit_col_padded_str.strip())
        except ValueError:
            logging.error("Invalid digit cols could not form ceph operand: %s", digit_cols)
            sys.exit(1)
        operands.append(operand)
    return operands

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = "--verbose" in sys.argv
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    input_lines = util.get_normalized_file_lines(filename)
    logging.debug("input_lines: %s", input_lines)

    with util.time_section("pt1"):
        problems = process_input_pt1(input_lines)
        logging.debug("pt1. problems: %s", problems)
        solutions = [ p.solve() for p in problems ]
        for i, sol in enumerate(solutions):
            logging.debug("solution #%d: %d", i, sol)
        solution_sum_str = "+".join(str(s) for s in solutions)
        logging.debug("pt1 solution operands: %s", solution_sum_str)
        logging.info("Pt1. sum=%d", sum(solutions))

    with util.time_section("pt2"):
        problems = process_input_pt2(input_lines)
        logging.debug("pt2. problems: %s", problems)
        solutions = [ p.solve() for p in problems ]
        for i, sol in enumerate(solutions):
            logging.debug("solution #%d: %d", i, sol)
        solution_sum_str = "+".join(str(s) for s in solutions)
        logging.debug("pt2 solution operands: %s", solution_sum_str)
        logging.info("Pt2. sum=%d", sum(solutions))

if __name__ == "__main__":
    main()
