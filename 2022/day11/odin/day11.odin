package main

import "core:fmt"
import "core:strings"
import "core:strconv"

MulOp :: struct {
    operand: uint,
}

AddOp :: struct {
    operand: uint,
}

SquareSelfOp :: struct {
}

InspectOp :: union {
    MulOp,
    AddOp,
    SquareSelfOp,
}

ThrowTest :: struct {
    divisor: uint,
    true_target: uint,
    false_target: uint,
}

MonkeyState :: struct {
    id: uint,
    items: [dynamic]uint,
    inspect_op: InspectOp,
    throw_test: ThrowTest,
    inspect_count: uint,
}

ReductionType :: enum {
    Part1,
    Part2,
}

main :: proc() {
    simple_input_file_contents := string(#load("day11_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    // fmt.println(typeid_of(type_of(simple_input_file_contents)), typeid_of(type_of(simple_input_lines)))
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day11_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day11_solve_pt1("simple pt1", simple_input_lines[:len(simple_input_lines)-1])
    day11_solve_pt1("real pt1", real_input_lines[:len(real_input_lines)-1])

    day11_solve_pt2("simple pt2", simple_input_lines[:len(simple_input_lines)-1])
    day11_solve_pt2("real pt2", real_input_lines[:len(real_input_lines)-1])
}

day11_solve_pt1 :: proc(title: string, input_lines: []string) {
    day11_solve(title, input_lines, ReductionType.Part1, 20)
}

day11_solve_pt2 :: proc(title: string, input_lines: []string) {
    day11_solve(title, input_lines, ReductionType.Part2, 10_000)
}

day11_solve :: proc(title: string, input_lines: []string, reduction_type: ReductionType, round_count: uint) {
    monkey_states := parse_initial_monkey_states(input_lines)
    defer delete(monkey_states)

    simulate_keep_away(monkey_states[:], round_count, reduction_type)

    monkey_business_lvl := calculate_monkey_business(monkey_states[:])
    fmt.printf("[%s] monkey business = %d\n", title, monkey_business_lvl)
}

parse_initial_monkey_states :: proc(input_lines: []string) -> []MonkeyState {
    monkey_state_count := (len(input_lines)/7)+1 // each monkey state takes 6 lines and there's a line of whitespace between each entry
    monkey_states := make([]MonkeyState, monkey_state_count)

    for i := 0; i < monkey_state_count; i += 1 {
        monkey_state_input_lines := input_lines[(i*7):]
        monkey_id_line := strings.trim_left_space(monkey_state_input_lines[0])
        starting_items_line := strings.trim_left_space(monkey_state_input_lines[1])
        operation_line := strings.trim_left_space(monkey_state_input_lines[2])
        test_divisor_line := strings.trim_left_space(monkey_state_input_lines[3])
        test_true_path_line := strings.trim_left_space(monkey_state_input_lines[4])
        test_false_path_line := strings.trim_left_space(monkey_state_input_lines[5])

        monkey_id_string := monkey_id_line[len("Monkey "):len(monkey_id_line)-1]
        monkey_id, ok := strconv.parse_int(monkey_id_string);
        assert(ok, "Invalid monkey_id")
        assert(monkey_id == i)

        starting_items_string := starting_items_line[len("Starting items:"):]
        for item_str in strings.split_iterator(&starting_items_string, ",") {
            item, ok := strconv.parse_uint(item_str[1:])
            assert(ok, "Invalid item str")
            append(&monkey_states[i].items, item)
        }

        operation_string := operation_line[len("Operation: new = "):]
        switch {
            case operation_string == "old * old": {
                monkey_states[i].inspect_op = SquareSelfOp{}
            }
            case strings.has_prefix(operation_string, "old * "): {
                operand_str := operation_string[len("old * "):]
                operand, ok := strconv.parse_uint(operand_str)
                assert(ok, "bad mul operation line")
                monkey_states[i].inspect_op = MulOp { operand=operand }
            }
            case strings.has_prefix(operation_string, "old + "): {
                operand_str := operation_string[len("old + "):]
                operand, ok := strconv.parse_uint(operand_str)
                assert(ok, "bad addition operation line")
                monkey_states[i].inspect_op = AddOp { operand=operand }
            }
            case: {
                assert(false, "Invalid operation string")
            }
        }

        // FIXME: ok_1,2,3 is horrible naming below but I can't seem to find a good odin idiom for "just assert that
        // something succeeded and move on with my life"

        test_divisor_string := test_divisor_line[len("Test: divisible by "):]
        test_divisor, ok_1 := strconv.parse_uint(test_divisor_string)
        assert(ok_1, "Bad divisible test")

        test_true_target_string := test_true_path_line[len("If true: throw to monkey "):]
        test_true_target, ok_2 := strconv.parse_uint(test_true_target_string)
        assert(ok_2, "Bad 'If true' line")

        test_false_target_string := test_false_path_line[len("If false: throw to monkey "):]
        test_false_target, ok_3 := strconv.parse_uint(test_false_target_string)
        assert(ok_3, "Bad 'If false' line")

        monkey_states[i].throw_test = ThrowTest {
            divisor = test_divisor,
            true_target = test_true_target,
            false_target = test_false_target,
        }

        monkey_states[i].inspect_count = 0
    }

    return monkey_states
}

dbg_print_monkey_states :: proc(monkey_states: []MonkeyState, verbose: bool) {
    for monkey_state, i in monkey_states {
        fmt.printf("Monkey %d: (inspect=%10d)\n", i, monkey_state.inspect_count)
        if verbose {
            fmt.print("\tItems: [")
            for item,i in monkey_state.items {
                if i != 0 do fmt.printf(", ")
                fmt.print(item)
            }
            fmt.print("]\n")
            fmt.print("\tOption: new = old ")
            switch op in monkey_state.inspect_op {
                case MulOp: fmt.printf("* %d\n", op.operand)
                case AddOp: fmt.printf("+ %d\n", op.operand)
                case SquareSelfOp: fmt.print("* old\n")
            }

            fmt.printf("\tTest: divisible by %d\n", monkey_state.throw_test.divisor)
            fmt.printf("\t\tIf true => monkey %d\n", monkey_state.throw_test.true_target)
            fmt.printf("\t\tIf false => monkey %d\n", monkey_state.throw_test.false_target)
        }
    }
}

simulate_keep_away :: proc(monkey_states: []MonkeyState, round_count: uint, reduction_type: ReductionType) {
    // for pt2: keep track of the divisor product
    divisor_product: uint = 1
    for m in monkey_states {
        divisor_product *= m.throw_test.divisor
    }

    for round in 0 ..< round_count {
        for _,m in monkey_states {

            // monkey inspects the item and our worry level changes
            for _,i in monkey_states[m].items {
                switch op in monkey_states[m].inspect_op {
                    case MulOp: monkey_states[m].items[i] *= op.operand
                    case AddOp: monkey_states[m].items[i] += op.operand
                    case SquareSelfOp: monkey_states[m].items[i] *= monkey_states[m].items[i]
                }
            }

            switch reduction_type {
                case ReductionType.Part1: {
                    for _,i in monkey_states[m].items {
                        monkey_states[m].items[i] /= 3
                    }
                }
                case ReductionType.Part2: {
                    for _,i in monkey_states[m].items {
                        monkey_states[m].items[i] %= divisor_product
                    }
                }
            }

            monkey_states[m].inspect_count += len(monkey_states[m].items)

            for len(monkey_states[m].items) > 0 {
                item := pop_front(&monkey_states[m].items)
                throw_test := monkey_states[m].throw_test
                target_monkey := throw_test.true_target if item % throw_test.divisor == 0 else throw_test.false_target
                append(&monkey_states[target_monkey].items, item)
            }
        }
    }
}

calculate_monkey_business :: proc(monkey_states: []MonkeyState) -> uint {
    top_monkey_inspect_counts: [2]uint

    for _,i in monkey_states {
        inspect_count := monkey_states[i].inspect_count

        slot: ^uint
        for _,j in top_monkey_inspect_counts {
            if inspect_count > top_monkey_inspect_counts[j] {
                if slot == nil {
                    slot = &top_monkey_inspect_counts[j]
                } else if top_monkey_inspect_counts[j] < slot^ {
                    slot = &top_monkey_inspect_counts[j]
                }
            }
        }

        if slot != nil {
            slot^ = inspect_count
        }
    }

    monkey_business: uint = 1
    for top_inspect_count in top_monkey_inspect_counts {
        monkey_business *= top_inspect_count
    }
    return monkey_business
}
