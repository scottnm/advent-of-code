# solution to AoC 2015 Day 1
# doing as Day 0 of 2024 to get dev env setups


def main() -> None:
    instructions: str = input("Input floor instructions: ")
    floor: int = 0
    first_basement_instruction: int | None = None
    for instruction_index, instruction in enumerate(instructions):
        match instruction:
            case "(":
                floor += 1
            case ")":
                floor -= 1
            case _:
                raise ValueError(f"Invalid instruction '{instruction}'")
        if first_basement_instruction is None and floor < 0:
            first_basement_instruction = instruction_index + 1

    print(f"Result floor: {floor}")
    print(f"First basement instruction: {first_basement_instruction}")


if __name__ == "__main__":
    main()
