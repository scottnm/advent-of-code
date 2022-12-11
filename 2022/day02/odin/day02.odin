package main

import "core:fmt"
import "core:strings"

RpsChoice :: enum {
    Rock,
    Paper,
    Scissors,
}

RoundData :: struct {
    opponent_choice: RpsChoice,
    player_choice: RpsChoice,
}

main :: proc() {
    simple_input_file_contents := string(#load("day02_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    // fmt.println(typeid_of(type_of(simple_input_file_contents)), typeid_of(type_of(simple_input_lines)))
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day02_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day02_solve("simple", simple_input_lines[:len(simple_input_lines)-1])
    day02_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day02_solve :: proc(title: string, input_lines: []string) {
    fmt.println("Pt1:", title)
    {
        rounds := get_rounds_from_lines_pt1(input_lines)
        defer delete(rounds)

        total_score := sum_round_score(rounds)
        fmt.println("    Total score...", total_score)
    }

    fmt.println("Pt2:", title)
    {
        rounds := get_rounds_from_lines_pt2(input_lines)
        defer delete(rounds)

        total_score := sum_round_score(rounds)
        fmt.println("    Total score...", total_score)
    }
}

get_rounds_from_lines_pt1 :: proc(input_lines: []string) -> []RoundData {
    rounds := make([]RoundData, len(input_lines))
    for line,i in input_lines {
        assert(len(line) == 3)
        assert(line[1] == ' ')

        switch line[0] {
            case 'A': rounds[i].opponent_choice = RpsChoice.Rock
            case 'B': rounds[i].opponent_choice = RpsChoice.Paper
            case 'C': rounds[i].opponent_choice = RpsChoice.Scissors
            case: {
                fmt.printf("Bad round data input! %c @ line %d: '%s'\n", line[0], i, line)
                assert(false);
            }
        }

        switch line[2] {
            case 'X': rounds[i].player_choice = RpsChoice.Rock
            case 'Y': rounds[i].player_choice = RpsChoice.Paper
            case 'Z': rounds[i].player_choice = RpsChoice.Scissors
            case: {
                fmt.printf("Bad round data input! %c @ line %d: '%s'\n", line[2], i, line)
                assert(false);
            }
        }
    }

    return rounds
}

get_rounds_from_lines_pt2 :: proc(input_lines: []string) -> []RoundData {
    rounds := make([]RoundData, len(input_lines))
    for line,i in input_lines {
        assert(len(line) == 3)
        assert(line[1] == ' ')

        opponent_choice: RpsChoice
        switch line[0] {
            case 'A': opponent_choice = RpsChoice.Rock
            case 'B': opponent_choice = RpsChoice.Paper
            case 'C': opponent_choice = RpsChoice.Scissors
            case: {
                fmt.printf("Bad round data input! %c @ line %d: '%s'\n", line[0], i, line)
                assert(false);
            }
        }

        player_choice: RpsChoice
        switch line[2] {
            // Generate player choice which loses
            case 'X': {
                switch opponent_choice {
                    case RpsChoice.Rock: player_choice = RpsChoice.Scissors
                    case RpsChoice.Paper: player_choice = RpsChoice.Rock
                    case RpsChoice.Scissors: player_choice = RpsChoice.Paper
                }
            }
            // Generate player choice which draws
            case 'Y': player_choice = opponent_choice
            // Generate player choice which wins
            case 'Z': {
                switch opponent_choice {
                    case RpsChoice.Rock: player_choice = RpsChoice.Paper
                    case RpsChoice.Paper: player_choice = RpsChoice.Scissors
                    case RpsChoice.Scissors: player_choice = RpsChoice.Rock
                }
            }
            case: {
                fmt.printf("Bad round data input! %c @ line %d: '%s'\n", line[2], i, line)
                assert(false);
            }
        }

        rounds[i] = { opponent_choice=opponent_choice, player_choice=player_choice }
    }

    return rounds
}

sum_round_score :: proc(rounds: []RoundData) -> uint {
    sum: uint = 0
    for round in rounds {
        sum += calculate_round_score(round)
    }
    return sum
}

calculate_round_score :: proc(round: RoundData) -> uint {
    round_result_score: uint
    if (round.opponent_choice == round.player_choice) {
        round_result_score = 3 // 'draw' is worth 3 points
    } else {
        won: bool
        switch round.player_choice {
            case RpsChoice.Rock: won = (round.opponent_choice == RpsChoice.Scissors)
            case RpsChoice.Paper: won = (round.opponent_choice == RpsChoice.Rock)
            case RpsChoice.Scissors: won = (round.opponent_choice == RpsChoice.Paper)
        }
        round_result_score = 6 if won else 0
    }

    choice_score: uint
    switch round.player_choice {
        case RpsChoice.Rock: choice_score = 1
        case RpsChoice.Paper: choice_score = 2
        case RpsChoice.Scissors: choice_score = 3
    }

    return round_result_score + choice_score
}
