import dataclasses
import typing
import contextlib
import time
import logging

TCell = typing.TypeVar('TCell')
UCell = typing.TypeVar('UCell')

@dataclasses.dataclass
class Grid(typing.Generic[TCell]):
    width: int
    height: int
    cells: list[TCell]

    def at(self, row: int, col: int) -> TCell:
        assert self.in_bounds(row=row, col=col)
        return self.cells[self.width * row + col]

    def set(self, row: int, col: int, v: TCell):
        self.cells[self.width * row + col] = v

    def in_bounds(self, row: int, col: int) -> bool:
        return (0 <= row < self.height) and (0 <= col < self.width)

    def __str__(self):
        s = f"Grid: width={self.width};height={self.height}\n"
        for r in range(self.height):
            start_idx = self.width * r
            row_str = "".join(str(c) for c in self.cells[start_idx:start_idx+self.width])
            s += row_str + "\n"
        return s

    def __repr__(self):
        s = f"Grid: width={self.width};height={self.height}\n"
        for r in range(self.height):
            start_idx = self.width * r
            row_str = "".join(repr(c) for c in self.cells[start_idx:start_idx+self.width])
            s += row_str + "\n"
        return s


def read_grid_from_file(filepath: str) -> Grid[str]:
    with open(filepath, "r", encoding="utf8") as f:
        lines = f.readlines()
        return read_grid_from_lines(lines)

def read_grid_from_lines(lines: list[str]) -> Grid[str]:
    width = None
    cells: list[str] = []
    for line in lines:
        line = line.rstrip()
        line_width = len(line)
        if width is None:
            width = line_width
        if width != line_width:
            raise ValueError("lines must all be of equal width")
        cells += (c for c in line)
    width = width if width is not None else 0
    height = len(lines)
    return Grid(width=width, height=height, cells=cells)

def map_grid(grid: Grid[TCell], xform: typing.Callable[[TCell], UCell]) -> Grid[UCell]:
    new_cells: list[UCell] = [xform(c) for c in grid.cells]
    return Grid(width=grid.width, height=grid.height, cells=new_cells)

def get_normalized_file_lines(filepath: str) -> list[str]:
    with open(filepath, "r", encoding="utf8") as f:
        lines = f.readlines()
        return [ l.rstrip("\r\n") for l in lines ]


@contextlib.contextmanager
def time_section(section_title: str):
    time_start = time.perf_counter()
    try:
        yield  # Code before yield is __enter__, after yield is __exit__
    finally:
        time_end = time.perf_counter()
        logging.info("%s time %.6f seconds", 
            section_title, 
            time_end - time_start)