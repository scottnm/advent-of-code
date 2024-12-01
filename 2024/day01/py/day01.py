import sys

InputPair = tuple[int, int]


def process_input(lines: list[str]) -> list[InputPair]:
    pairs = []
    for line in lines:
        line_values = [int(v) for v in line.split()]
        pairs.append((line_values[0], line_values[1]))
    return pairs


def calculate_total_input_pair_distance(input_pairs: list[InputPair]) -> int:
    first_list = sorted(v for (v, _) in input_pairs)
    second_list = sorted(v for (_, v) in input_pairs)

    total_dist = 0
    for v1, v2 in zip(first_list, second_list):
        dist = abs(v1 - v2)
        total_dist += dist

    return total_dist


def calculate_similarity_score(input_pairs: list[InputPair]) -> int:
    second_list_counts: dict[int, int] = {}
    for _, v in input_pairs:
        if v in second_list_counts:
            second_list_counts[v] += 1
        else:
            second_list_counts[v] = 1

    total_similarity_score = 0
    for v, _ in input_pairs:
        similarity_score = second_list_counts.get(v, 0) * v
        total_similarity_score += similarity_score

    return total_similarity_score


def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    filename = sys.argv[1]
    with open(filename, "r", encoding="utf8") as f:
        input_lines = f.readlines()
        input_pairs = process_input(input_lines)

    total_distance = calculate_total_input_pair_distance(input_pairs)
    print(f"Total distance: {total_distance}")
    total_similarity_score = calculate_similarity_score(input_pairs)
    print(f"Similarity score: {total_similarity_score}")


if __name__ == "__main__":
    main()
