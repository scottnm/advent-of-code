import sys
import re
import dataclasses
import logging
import copy
import time
import typing

@dataclasses.dataclass
class IdRange:
    min: int
    max: int

def process_input(lines: list[str]) -> typing.Tuple[list[IdRange], list[int]]:
    ranges = []
    ids = []

    scanning_ranges = True
    lines = [line.rstrip() for line in lines]
    try:
        input_split_idx = lines.index("")
    except ValueError:
        logging.error("Invalid input: missing input separator")
        sys.exit(1)
    range_lines = lines[:input_split_idx]
    id_lines = lines[input_split_idx+1:]

    for line in range_lines:
        try:
            range_parts = line.split("-")
            if len(range_parts) != 2:
                logging.error("invalid range line: %s", line)
                sys.exit(1)
            new_range = IdRange(min=int(range_parts[0]), max=int(range_parts[1]))
            ranges.append(new_range)
        except ValueError:
            logging.error("invalid range line: %s", line)
            sys.exit(1)

    for line in id_lines:
        try:
            ids.append(int(line))
        except ValueError:
            logging.error("invalid id line: %s", line)
            sys.exit(1)
    return (ranges, ids)

def simplify_ranges(ranges: list[IdRange]) -> list[IdRange]:
    simplified_ranges: list[IdRange] = []
    while len(ranges) != len(simplified_ranges):
        for id_range in ranges:
            id_range_used = False
            for i, simplified_range in enumerate(simplified_ranges):
                if simplified_range.min <= id_range.min <= simplified_range.max:
                    simplified_range.max = max(id_range.max, simplified_range.max)
                    id_range_used = True
                    break
                if simplified_range.min <= id_range.max <= simplified_range.max:
                    simplified_range.min = min(id_range.min, simplified_range.min)
                    id_range_used = True
                    break
                if id_range.min <= simplified_range.min <= id_range.max:
                    id_range.max = max(id_range.max, simplified_range.max)
                    simplified_ranges[i] = id_range
                    id_range_used = True
                    break
                if id_range.min <= simplified_range.max <= id_range.max:
                    id_range.min = min(id_range.min, simplified_range.min)
                    simplified_ranges[i] = id_range
                    id_range_used = True
                    break
            if not id_range_used:
                simplified_ranges.append(id_range)

        if len(ranges) != len(simplified_ranges):
            ranges = simplified_ranges
            simplified_ranges = []

    return simplified_ranges

def pt1_is_id_fresh(ranges: list[IdRange], id_v: int):
    return any(id_range.min <= id_v <= id_range.max for id_range in ranges)

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
        ranges, ids = process_input(input_lines)

    logging.debug("ranges: %s", ranges)
    logging.debug("ids: %s", ids)

    simplify_time_start = time.perf_counter()
    ranges = simplify_ranges(ranges)
    simplify_time_end = time.perf_counter()
    logging.info("simplified_ranges count: %d (calc time = %.6f seconds)",
        len(ranges),
        simplify_time_end - simplify_time_start)
    logging.debug("simplified_ranges: %s", ranges)

    pt1_time_start = time.perf_counter()
    fresh_ids = [id for id in ids if pt1_is_id_fresh(ranges, id)]
    pt1_time_end = time.perf_counter()
    logging.info("Pt1. %d fresh ids (calc time = %.6f seconds)",
        len(fresh_ids),
        pt1_time_end - pt1_time_start)
    logging.debug("pt1. fresh ids: %s", fresh_ids)

    pt2_time_start = time.perf_counter()
    total_fresh_id_count = sum(r.max + 1 - r.min for r in ranges)
    pt2_time_end = time.perf_counter()
    logging.info("Pt2. %d fresh ids in all ranges (calc time = %.6f seconds)",
        total_fresh_id_count,
        pt2_time_end - pt2_time_start)

if __name__ == "__main__":
    main()
