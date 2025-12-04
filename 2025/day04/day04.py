import sys
import os
import logging
import time
import enum
import typing
import copy

current_dir = os.path.dirname(os.path.realpath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.append(project_root)

from shared import util

class CellState(enum.Enum):
    Empty = enum.auto()
    Inaccessible = enum.auto()
    Accessible = enum.auto()

    def is_filled(self) -> bool:
        return self == CellState.Inaccessible or self == CellState.Accessible

    @staticmethod
    def from_str(s: str) -> typing.Self:
        match s:
            case "@":
                return CellState.Inaccessible
            case "x":
                return CellState.Accessible
            case ".":
                return CellState.Empty
            case _:
                raise RuntimeError(f"Invalid cell: '{s}'")

    def __str__(self):
        match self:
            case CellState.Inaccessible:
                return "@"
            case CellState.Accessible:
                return "x"
            case CellState.Empty:
                return "."

def map_accessible_cells(grid: util.Grid[CellState]) -> util.Grid[CellState]:
    mapped_grid = copy.deepcopy(grid)

    for r in range(grid.height):
        below_top_row = r > 0
        above_bottom_row = r < (grid.height - 1)
        for c in range(grid.width):
            if grid.at(r, c) == CellState.Empty:
                continue

            after_first_col = c > 0 
            before_last_col = c < (grid.width - 1)
            has_top_left = below_top_row and after_first_col and grid.at(row=r-1, col=c-1).is_filled()
            has_top_mid = below_top_row and grid.at(row=r-1, col=c).is_filled()
            has_top_right = below_top_row and before_last_col and grid.at(row=r-1, col=c+1).is_filled()
            has_mid_left = after_first_col and grid.at(row=r, col=c-1).is_filled()
            has_mid_right = before_last_col and grid.at(row=r, col=c+1).is_filled()
            has_bottom_left = above_bottom_row and after_first_col and grid.at(row=r+1, col=c-1).is_filled()
            has_bottom_mid = above_bottom_row and grid.at(row=r+1, col=c).is_filled()
            has_bottom_right = above_bottom_row and before_last_col and grid.at(row=r+1, col=c+1).is_filled()

            filled_neighbor_count = \
                (1 if has_top_left else 0) + \
                (1 if has_top_mid else 0) + \
                (1 if has_top_right else 0) + \
                (1 if has_mid_left else 0) + \
                (1 if has_mid_right else 0) + \
                (1 if has_bottom_left else 0) + \
                (1 if has_bottom_mid else 0) + \
                (1 if has_bottom_right else 0)
            

            cell_state = CellState.Accessible if filled_neighbor_count < 4 else CellState.Inaccessible
            mapped_grid.set(r, c, cell_state)
    return mapped_grid

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = "--verbose" in sys.argv
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    raw_input_grid = util.read_grid_from_file(filename)
    logging.debug("raw input: %s", raw_input_grid)
    input_grid: util.Grid[CellState] = util.map_grid(raw_input_grid, xform=CellState.from_str)
    logging.debug("processed input grid: %s", input_grid)
    pt1_time_start = time.perf_counter()
    accessible_grid = map_accessible_cells(input_grid)
    pt1_time_end = time.perf_counter()
    logging.debug("accessible grid: %s", accessible_grid)
    accessible_count = sum(1 for c in accessible_grid.cells if c == CellState.Accessible)
    logging.info("Pt1. accessible count: %d (calc time = %.6f seconds)", 
        accessible_count,
        pt1_time_end - pt1_time_start)

    pt2_time_start = time.perf_counter()
    current_grid = copy.deepcopy(input_grid)
    original_fill_count = sum(1 for c in current_grid.cells if c.is_filled())
    while True:
        current_grid = map_accessible_cells(current_grid)
        logging.debug("accessible grid: %s", current_grid)
        if not any(c for c in current_grid.cells if c == CellState.Accessible):
            break
        current_grid.cells = [ CellState.Empty if c == CellState.Accessible else c for c in current_grid.cells ]
        logging.debug("swept grid: %s", current_grid)
    final_fill_count = sum(1 for c in current_grid.cells if c.is_filled())
    pt2_time_end = time.perf_counter()
    logging.info("Pt2. accessible count: %d (calc time = %.6f seconds)", 
        original_fill_count - final_fill_count,
        pt2_time_end - pt2_time_start)

if __name__ == "__main__":
    main()
