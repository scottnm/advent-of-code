import dataclasses
import logging
import os
import re
import sys
import typing

current_dir = os.path.dirname(os.path.realpath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.append(project_root)
from shared import util

@dataclasses.dataclass
class Vec2:
    x: int
    y: int

def process_input(lines: list[str]) -> list[Vec2]:
    positions = []
    for line in lines:
        parts = line.split(",")
        if len(parts) != 2:
            util.fatal_error("invalid number of parts on line! expected 2; found %d! line=%s", len(parts), line)
        try:
            x,y = parts
            positions.append(Vec2(x=int(x), y=int(y)))
        except ValueError:
            util.fatal_error("invalid non-numeric parts on line! line=%s", line)
    return positions

def calc_rect_area(corner1: Vec2, corner2: Vec2) -> int:
    x_dist = abs(corner1.x - corner2.x) + 1
    y_dist = abs(corner1.y - corner2.y) + 1
    return x_dist * y_dist

def p1_calc_max_rect_area(positions: list[Vec2]) -> (Vec2, Vec2, int):
    rect_areas = ((p1, p2, calc_rect_area(p1, p2)) for i,p1 in enumerate(positions[:-1]) for p2 in positions[i:])
    return max(rect_areas, key=lambda ra: ra[2])

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = "--verbose" in sys.argv
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    input_lines = util.read_normalized_file_lines(filename)
    logging.debug("input_lines: %s", input_lines)
    positions = process_input(input_lines)
    logging.debug("positions: %s", positions)

    with util.time_section("pt1"):
        max_rect = p1_calc_max_rect_area(positions)
        logging.info("max rect area (%d): %s <-> %s", max_rect[2], max_rect[0], max_rect[1])

    with util.time_section("pt1"):
        logging.info("pt2. : NOTIMPL")

if __name__ == "__main__":
    main()
