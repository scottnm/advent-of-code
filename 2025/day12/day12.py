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

class ShapeSpec:
    # FIXME: all inputs are 3x3 for this problem.
    # let's start with that constraint for simplicity
    def __init__(self, sid: int, alt_id: str, width = 3, height = 3, cells = None):
        self.id = sid
        self.alt_id = alt_id
        self.width = width
        self.height = height
        self.cells = cells
        if self.cells is None:
            self.cells = [False] * (self.width * self.height)

    def __str__(self):
        s = f"{self.id}:"
        for i in range(self.height):
            row_start = self.width * i
            row = "".join([ "#" if c else "." for c in self.cells[row_start:row_start+self.width] ])
            s += "\n" + row
        return s

    @staticmethod
    def from_lines(sid: int, lines: list[str]) -> 'ShapeSpec':
        assert sid<26 # FIXME: hack to get a quick id
        s = ShapeSpec(sid, chr(65+sid))
        cells = []
        for line in lines:
            line = line.strip()
            line_cells = [False] * s.width
            if len(line) != s.width:
                raise ValueError(f"Invalid line data! shape line should be {s.width} width! line='{line}'")
            for i,c in enumerate(line):
                match c:
                    case "#":
                        line_cells[i] = True
                    case ".":
                        line_cells[i] = False
                    case _:
                        raise ValueError(f"Invalid line data! line had invalid char {c}! line='{line}'")
            cells += line_cells
        assert(len(cells) == len(s.cells))
        s.cells = cells
        return s

    @staticmethod
    def __index(width: int, row: int, col: int) -> int:
        return width * row + col

    def y_axis_flip_cells(self) -> 'ShapeSpec':
        y_axis_flip_cells = [False] * (self.width * self.height)
        for r in range(self.height):
            for c in range(self.width):
                flip_col = self.width - 1 - c
                y_axis_flip_cells[ShapeSpec.__index(width=self.width, row=r,col=flip_col)] = \
                    self.cells[ShapeSpec.__index(width=self.width, row=r,col=c)]
        return ShapeSpec(sid=self.id, alt_id=self.alt_id, width=self.width, height=self.height, cells=y_axis_flip_cells)
        
    def x_axis_flip_cells(self) -> 'ShapeSpec':
        x_axis_flip_cells = [False] * (self.width * self.height)
        for r in range(self.height):
            flip_row = self.height - 1 - r
            for c in range(self.width):
                x_axis_flip_cells[ShapeSpec.__index(width=self.width, row=flip_row,col=c)] = \
                    self.cells[ShapeSpec.__index(width=self.width, row=r,col=c)]
        return ShapeSpec(sid=self.id, alt_id=self.alt_id, width=self.width, height=self.height, cells=x_axis_flip_cells)
        
    def rot_clockwise(self) -> 'ShapeSpec':
        rot_cw_cells = [False] * (self.width * self.height)
        for r in range(self.height):
            rot_clockwise_col = self.height - r - 1
            for c in range(self.width):
                rot_clockwise_row = c
                rot_cw_cells[ShapeSpec.__index(width=self.height, row=rot_clockwise_row,col=rot_clockwise_col)] = \
                    self.cells[ShapeSpec.__index(width=self.width, row=r, col=c)]
        return ShapeSpec(sid=self.id, alt_id=self.alt_id, width=self.height, height=self.width, cells=rot_cw_cells)

    def calc_variants(self) -> list['ShapeSpec']:
        no_flip_rot_0 = self
        y_flip_rot_0 = no_flip_rot_0.y_axis_flip_cells()
        x_flip_rot_0 = no_flip_rot_0.x_axis_flip_cells()

        no_flip_rot_1 = self.rot_clockwise()
        y_flip_rot_1 = no_flip_rot_1.y_axis_flip_cells()
        x_flip_rot_1 = no_flip_rot_1.x_axis_flip_cells()

        no_flip_rot_2 = no_flip_rot_1.rot_clockwise()
        y_flip_rot_2 = no_flip_rot_2.y_axis_flip_cells()
        x_flip_rot_2 = no_flip_rot_2.x_axis_flip_cells()

        no_flip_rot_3 = no_flip_rot_2.rot_clockwise()
        y_flip_rot_3 = no_flip_rot_3.y_axis_flip_cells()
        x_flip_rot_3 = no_flip_rot_3.x_axis_flip_cells()

        variant_candidates = [
            no_flip_rot_0,
            y_flip_rot_0,
            x_flip_rot_0,

            no_flip_rot_1,
            y_flip_rot_1,
            x_flip_rot_1,

            no_flip_rot_2,
            y_flip_rot_2,
            x_flip_rot_2,

            no_flip_rot_3,
            y_flip_rot_3,
            x_flip_rot_3,
        ]

        variants: list[ShapeSpec] = []
        for vc in variant_candidates:
            if all(v.cells != vc.cells for v in variants):
                variants.append(vc)

        return variants

@dataclasses.dataclass
class RegionSpec:
    width: int
    height: int
    shapes_to_fit: list[typing.Tuple[ShapeSpec, int]]

    def __str__(self):
        shape_counts_str = ' '.join(str((s[0].id, s[1])) for s in self.shapes_to_fit)
        return f"{self.width}x{self.height}: {shape_counts_str}"

def process_input(filedata: str) -> typing.Tuple[list[ShapeSpec], list[RegionSpec]]:
    chunk_blobs = filedata.split("\n\n") # each blob of data in the input is separate by two newlines
    chunk_line_sets: list[list[str]]= [ c.strip().splitlines() for c in chunk_blobs ]

    shape_chunks = chunk_line_sets[:-1]
    region_lines = chunk_line_sets[-1]

    INDEX_LINE_RGX = re.compile(r"(\d+):$")
    shapes: list[ShapeSpec] = []
    for shape_chunk_lines in shape_chunks:
        if len(shape_chunk_lines) < 2:
            raise RuntimeError(f"Not enough lines in shape chunk! {len(shape_chunk_lines)}")

        index_line = shape_chunk_lines[0].strip()
        index_match = INDEX_LINE_RGX.match(index_line)
        if index_match is None:
            raise RuntimeError(f"Bad shape chunk index line: {index_line}")
        try:
            shape_index = int(index_match.group(1))
            if shape_index != len(shapes):
                raise RuntimeError(f"Bad shape index! expected {len(shapes)}; got {shape_index}")
        except ValueError as exc:
            raise RuntimeError(f"Bad index line: {index_line}") from exc

        shapes.append(ShapeSpec.from_lines(shape_index, shape_chunk_lines[1:]))

    regions: list[RegionSpec] = []
    REGION_DIM_RGX = re.compile(r"(\d+)x(\d+)")
    for region_line in region_lines:
        region_parts = region_line.split(": ")
        if len(region_parts) != 2:
            raise RuntimeError(f"Bad region line formatting! {region_line}")

        region_dimensions_str, shape_counts_str = region_parts
        region_dimensions_match = REGION_DIM_RGX.match(region_dimensions_str)
        if not region_dimensions_match:
            raise RuntimeError(f"Bad region dimensions! {region_dimensions_str}")
        try:
            region_width = int(region_dimensions_match.group(1))
            region_height = int(region_dimensions_match.group(2))
        except ValueError as exc:
            raise RuntimeError(f"Bad region dimensions: {region_dimensions_str}") from exc

        try:
            shape_counts: list[int] = [ int(shape_count) for shape_count in shape_counts_str.split(" ") ]
        except ValueError as exc:
            raise RuntimeError(f"Bad shape indices! {shape_counts_str}") from exc

        if len(shape_counts) != len(shapes):
            raise RuntimeError(f"Bad shapes line! had counts for {len(shape_counts)} shapes but expected {len(shapes)}")

        shape_count_pairs = [ (shapes[i], shape_count) for i,shape_count in enumerate(shape_counts) if shape_count != 0 ]
        regions.append(RegionSpec(width=region_width, height=region_height, shapes_to_fit=shape_count_pairs))

    return (shapes, regions)

def solve_pt1(shapes: list[ShapeSpec], region: RegionSpec) -> bool:
    shape_variants: typing.Dict[int, list[ShapeSpec]] = dict()
    for shape in shapes:
        shape_variants[shape.id] = shape.calc_variants()

    region_buffer: list[bool] = [False] * (region.width * region.height)
    shapes_to_fit_flattened: list[int] = []
    for (shape,count) in region.shapes_to_fit:
        shapes_to_fit_flattened += [shape.id] * count
    return solve_pt1_helper(shape_variants, shapes_to_fit_flattened, region, region_buffer)

def solve_pt1_helper(
    shapes: typing.Dict[int, list[ShapeSpec]],
    shape_ids_left_to_fit: list[int],
    region: RegionSpec,
    region_buffer: list[bool]) -> bool:

    if len(shape_ids_left_to_fit) == 0:
        return True

    shape_to_fit = shape_ids_left_to_fit[0]
    for shape_variant in shapes[shape_to_fit]:
        for r in range(region.height - shape_variant.height):
            for c in range(region.width - shape_variant.width):
                if try_place_shape_in_region(shape_variant, r, c, region, region_buffer):
                    if solve_pt1_helper(shapes, shape_ids_left_to_fit[1:], region, region_buffer):
                        return True
                    clear_shape_in_region(shape_variant, r, c, region, region_buffer)

    return False

def try_place_shape_in_region(shape: ShapeSpec, tl_row: int, tl_col: int, region: RegionSpec, region_buffer: list[bool]) -> bool:
    if tl_row + shape.height > region.height:
        return False
    if tl_col + shape.width > region.width:
        return False
    
    for r in range(shape.height):
        region_row = tl_row + r
        for c in range(shape.width):
            region_col = tl_col + c
            if region_buffer[region_row * region.width + region_col]:
                return False

    for r in range(shape.height):
        region_row = tl_row + r
        for c in range(shape.width):
            region_col = tl_col + c
            region_buffer[region_row * region.width + region_col] = True

    return True

def clear_shape_in_region(shape: ShapeSpec, tl_row: int, tl_col: int, region: RegionSpec, region_buffer: list[bool]):
    assert tl_row + shape.height <= region.height
    assert tl_col + shape.width <= region.width
    
    for r in range(shape.height):
        region_row = tl_row + r
        for c in range(shape.width):
            region_col = tl_col + c
            region_buffer[region_row * region.width + region_col] = False


def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = any(v_arg in sys.argv for v_arg in ("-v", "--verbose"))
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    with open(filename, "r", encoding="utf8") as f:
        input_data = f.read()
        logging.debug("input: %s", input_data)
        shapes, regions = process_input(input_data)
    logging.debug("Shapes:")
    for shape in shapes:
        logging.debug("%s", str(shape))
    logging.debug("Regions:")
    for region in regions:
        logging.debug("%s", region)

    with util.time_section("pt1"):
        fittable_regions = [ r for r in regions if solve_pt1(shapes, r) ]
        logging.info("pt1. fittable region count : %d", len(fittable_regions))

    with util.time_section("pt1"):
        logging.info("pt2. : NOTIMPL")

if __name__ == "__main__":
    main()
