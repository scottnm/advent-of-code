import logging
import os
import sys
import enum
import typing

class GridCell(enum.Enum):
    Empty = enum.auto()
    Tachyon = enum.auto()

    @staticmethod
    def from_str(s: str) -> "GridCell":
        match s:
            case "." | "S":
                return GridCell.Empty
            case "^":
                return GridCell.Tachyon
            case _:
                raise RuntimeError(f"Unexpected GridCell str: {s}")

current_dir = os.path.dirname(os.path.realpath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.append(project_root)
from shared import util

def simulate_beam_split(grid: util.Grid[GridCell], start_pos: util.GridCellPos) -> int:
    beam_cols: typing.Set[int] = { start_pos.col }
    tachyon_hit: typing.Set[util.GridCellPos] = set()
    for beam_row in range(start_pos.row, grid.height):
        logging.debug("beam row: %d/%d (beam_cols len=%d)", beam_row, grid.height, len(beam_cols))

        new_beam_cols = []
        for beam_col in beam_cols:
            if grid.at(row=beam_row, col=beam_col) == GridCell.Tachyon:
                tachyon_hit.add(util.GridCellPos(row=beam_row, col=beam_col))
                new_beam_cols.append(beam_col - 1)
                new_beam_cols.append(beam_col + 1)
            else:
                new_beam_cols.append(beam_col)
        beam_cols = set(new_beam_cols)
    return len(tachyon_hit)

def simulate_many_worlds(
    grid: util.Grid[GridCell],
    start_pos: util.GridCellPos,
    memo: dict | None = None) -> int:

    if memo is None:
        memo = dict()

    if start_pos.row >= grid.height:
        memo[start_pos] = 1
        return 1

    for beam_row in range(start_pos.row, grid.height):
        # logging.debug("beam row: %d/%d (beam_cols len=%d)", beam_row, grid.height, len(beam_cols))

        if grid.at(row=beam_row, col=start_pos.col) == GridCell.Tachyon:
            left_beam_pos = util.GridCellPos(row=beam_row, col=start_pos.col - 1)
            left_world_count = memo.get(left_beam_pos, None)
            if left_world_count is None:
                left_world_count = simulate_many_worlds(grid, left_beam_pos, memo)
                memo[left_beam_pos] = left_world_count

            right_beam_pos = util.GridCellPos(row=beam_row, col=start_pos.col + 1)
            right_world_count = memo.get(right_beam_pos, None)
            if right_world_count is None:
                right_world_count = simulate_many_worlds(grid, right_beam_pos, memo)
                memo[right_beam_pos] = right_world_count

            total_world_count = left_world_count + right_world_count
            memo[start_pos] = total_world_count
            return total_world_count

    memo[start_pos] = 1
    return 1

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = "--verbose" in sys.argv
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    input_grid = util.read_grid_from_file(filename)
    logging.debug("input_grid: %s", input_grid)
    grid: util.Grid[GridCell] = util.map_grid(input_grid, xform=GridCell.from_str)
    start_pos = input_grid.find_pos(lambda _,c: c == "S")
    if start_pos is None:
        logging.error("input grid missing start pos")
        sys.exit(1)

    with util.time_section("pt1"):
        tach_hits = simulate_beam_split(grid, start_pos)
        logging.info("pt1. hits: %d", tach_hits)

    with util.time_section("pt2"):
        timeline_cnt = simulate_many_worlds(grid, start_pos)
        logging.info("pt2. timelines: %d", timeline_cnt)

if __name__ == "__main__":
    main()
