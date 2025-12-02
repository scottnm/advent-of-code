import sys
import re
import dataclasses
import logging
import copy

@dataclasses.dataclass
class Range:
    lower: int
    upper: int

def in_range(range: Range, n: int) -> bool:
    return range.lower <= n <= range.upper
LINE_RGX = re.compile(r"(\d+)-(\d+)")
def process_input(lines: list[str]) -> list[Range]:
    if len(lines) != 1:
        print("Warning: expected only one line of input")
    input_line = "".join(lines)
    range_strings = input_line.split(",")
    ranges = []
    for range_string in range_strings:
        range_match = LINE_RGX.match(range_string)
        if not range_match:
            logging.fatal("Unexpected invalid line: %s", range_string)
            sys.exit(1)

        try:
            range_lower_bound = int(range_match.group(1))
            range_upper_bound = int(range_match.group(2))
        except ValueError:
            logging.fatal("Unexpected invalid line: %s :: bad numeric value", range_string)
            sys.exit(1)

        new_range = Range(lower=range_lower_bound, upper=range_upper_bound)
        ranges.append(new_range)
    return ranges

def calculate_shift_multiplier(n: int) -> int:
    magnitude_multiplier = 10
    while n >= magnitude_multiplier:
        magnitude_multiplier *= 10

    return magnitude_multiplier

def repeat_number(n: int) -> int:
    magnitude_multiplier = calculate_shift_multiplier(n)
    return (n * magnitude_multiplier) + n

def find_invalid_ids_pt1(id_ranges: list[Range]) -> list[int]:
    min_id_range = min(r.lower for r in id_ranges)
    max_id_range = max(r.upper for r in id_ranges)
    assert min_id_range <= max_id_range, "invalid id ranges provided"

    number_pattern = 1
    invalid_ids = []
    while True:
        invalid_id_candidate = repeat_number(number_pattern)
        number_pattern += 1

        # don't bother searching through the id range list if we know its smaller than all id ranges
        if invalid_id_candidate < min_id_range:
            continue

        # if we're past the upper limit of all id ranges there's no reason to continue checking
        if invalid_id_candidate > max_id_range:
            break

        # TODO: we could probably speed this up by finding some clever way to eliminate id_ranges as we go
        if any(in_range(r, invalid_id_candidate) for r in id_ranges):
            invalid_ids.append(invalid_id_candidate)

    return invalid_ids

def find_invalid_ids_pt2(id_ranges: list[Range]) -> list[int]:
    min_id_range = min(r.lower for r in id_ranges)
    max_id_range = max(r.upper for r in id_ranges)
    assert min_id_range <= max_id_range, "invalid id ranges provided"

    base_number_pattern = 1
    invalid_ids: set[int] = set()
    while True: # outer loop: iterate over the possible base number sequences e.g. 1, 12, 123, etc etc
        curr_number_pattern = base_number_pattern
        base_number_pattern += 1
        number_pattern_shift_multiplier = calculate_shift_multiplier(curr_number_pattern)
        invalid_id_candidate = curr_number_pattern * number_pattern_shift_multiplier + curr_number_pattern

        # if the first invalid id candidate (e.g. 11 or 1212 or 123123) is greater than our max id value, there are no additional solutions
        if invalid_id_candidate > max_id_range:
            break

        while True: # inner loop: repeat the base number sequence and check for valid values: e.g. 1212, 121212, etc
            if invalid_id_candidate > max_id_range:
                break

            if invalid_id_candidate >= min_id_range: 
                if any(in_range(r, invalid_id_candidate) for r in id_ranges):
                    invalid_ids.add(invalid_id_candidate)

            # update to check the next invalid_id_candidate
            invalid_id_candidate = invalid_id_candidate * number_pattern_shift_multiplier + curr_number_pattern

    return list(invalid_ids)

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = "--verbose" in sys.argv
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    with open(filename, "r", encoding="utf8") as f:
        input_lines = f.readlines()
        logging.debug("input_lines: %s", input_lines)
        id_ranges = process_input(input_lines)

    logging.debug("id ranges: %s", id_ranges)

    invalid_ids_pt1 = find_invalid_ids_pt1(id_ranges)
    logging.debug("invalid ids pt1: %s", invalid_ids_pt1)
    logging.info("Pt1. sum invalid ids(%d): %d", len(invalid_ids_pt1), sum(invalid_ids_pt1))

    invalid_ids_pt2 = find_invalid_ids_pt2(id_ranges)
    logging.debug("invalid ids pt2: %s", invalid_ids_pt2)
    logging.info("Pt2. sum invalid ids(%d): %d", len(invalid_ids_pt2), sum(invalid_ids_pt2))


if __name__ == "__main__":
    main()
