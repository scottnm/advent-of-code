import sys
import enum
import re
import dataclasses

class Direction(enum.Enum):
    Left = enum.auto()
    Right = enum.auto()

InputPair = tuple[Direction, int]

LINE_RGX = re.compile(r"(L|R)(\d+)")
def process_input(lines: list[str]) -> list[InputPair]:
    pairs: list[InputPair] = []
    for line in lines:
        lineMatch = LINE_RGX.match(line)
        if not lineMatch:
            print(f"Unexpected invalid line: {line}")
            sys.exit(1)

        direction: Direction = Direction.Left if lineMatch.group(1) == "L" else Direction.Right
        try:
            value = int(lineMatch.group(2))
        except ValueError:
            print(f"Unexpected invalid line: {line} :: bad numeric value")
            sys.exit(1)

        pair: InputPair = (direction, value)
        pairs.append(pair)
    return pairs

def simulate_dial_positions(dial_start: int, dial_min: int, dial_max: int, steps: list[InputPair]) -> list[(int, int)]:
    assert dial_max > dial_min, "invalid arg: dial_min >= dial_max"
    assert dial_start >= dial_min, "invalid arg: dial_start < dial_min"
    assert dial_start <= dial_max, "invalid arg: dial_start > dial_max"

    dial_range = dial_max - dial_min + 1
    dial_start_norm = dial_start - dial_min

    dial_pos = dial_start_norm
    positions = [(dial_pos, 0)]
    for (direction, offset) in steps:
        zero_crosses = offset // dial_range
        simple_offset = offset % dial_range
        match direction:
            case Direction.Left:
                zero_crosses += (1 if (dial_pos != 0 and simple_offset >= dial_pos) else 0)
                dial_pos += (dial_range - simple_offset) 
                dial_pos %= dial_range
            case Direction.Right:
                zero_crosses += (1 if (dial_pos != 0 and ((simple_offset + dial_pos) >= dial_range)) else 0)
                dial_pos += offset
                dial_pos %= dial_range
            case _:
                print(f"Invalid direction {direction}")
                sys.exit()
        positions.append((dial_pos, zero_crosses))

    return positions

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    filename = sys.argv[1]
    with open(filename, "r", encoding="utf8") as f:
        input_lines = f.readlines()
        input_pairs = process_input(input_lines)

    # print(f"inputs: {input_pairs}")
    dial_positions = simulate_dial_positions(50, 0, 99, input_pairs)
    print(f"dial_positions: {dial_positions}")
    zero_count = sum(1 for (p, xc) in dial_positions if p == 0)
    print(f"number of zeroes: {zero_count}")
    zero_crosses = sum(xc for (p, xc) in dial_positions)
    print(f"number of zero crosses: {zero_crosses}")

if __name__ == "__main__":
    main()
