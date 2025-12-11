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

def process_input(lines: list[str]) -> list[typing.Any]:
    return []

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = any(v_arg in sys.argv for v_arg in ("-v", "--verbose"))
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    input_lines = util.read_normalized_file_lines(filename)
    logging.debug("input_lines: %s", input_lines)
    input_data = process_input(input_lines)
    logging.debug("input: %s", input_data)

    with util.time_section("pt1"):
        logging.info("pt1. : NOTIMPL")

    with util.time_section("pt1"):
        logging.info("pt2. : NOTIMPL")

if __name__ == "__main__":
    main()
