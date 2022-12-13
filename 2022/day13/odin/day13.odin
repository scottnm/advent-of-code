package main

import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:mem"
import "core:sort"

PacketIntElem :: struct {
    num: int,
}

PacketListElem :: struct {
    list: []PacketElem,
}

PacketElem :: union {
    PacketIntElem,
    PacketListElem,
}

PacketPair :: struct {
    left_packet: PacketListElem,
    right_packet: PacketListElem,
}

PacketPairOrder :: enum {
    // the packets are ordered correctly
    Ordered,
    // the packets are ordered incorrectly
    Misordered,
    // the packets are identical and so have no order
    Indeterminate,
}

main :: proc() {
    simple_input_file_contents := string(#load("day13_simple.txt"))
    simple_input_lines := strings.split_lines(simple_input_file_contents)
    defer delete(simple_input_lines)

    real_input_file_contents := string(#load("day13_real.txt"))
    real_input_lines := strings.split_lines(real_input_file_contents)
    defer delete(real_input_lines)

    day13_solve("simple", simple_input_lines[:len(simple_input_lines)-1])
    day13_solve("real", real_input_lines[:len(real_input_lines)-1])
}

day13_solve :: proc(title: string, input_lines: []string) {
    // FIXME: I should double check that with this call I'm actually not leaking any memory
    //        I could write a custom allocator to track this but maybe I can valgrind my way to success?
    //        step 1: write a program with a leak and verify valgrind catches it.
    //        step 2: run this program through valgrind and verify no leak
    defer free_all(context.temp_allocator)

    // FIXME: rather than passing this allocator all over the place is there, can I simplify by just scoping the these
    // calls with a new context where the general allocator is the same as the temp_allocator
    packet_pairs := parse_packet_pairs_from_input(input_lines, context.temp_allocator)
    // print_packet_pairs(packet_pairs)

    //
    // Pt1. Find the pairs of packets which are in the correct order and sum their indices
    //
    ordered_packet_pair_indexes := get_ordered_packet_pairs(packet_pairs, context.temp_allocator)
    ordered_packet_pair_index_sum := 0
    for packet_pair_index in ordered_packet_pair_indexes {
        // N.B. for some reason these packet indices are 1-based in the problem rather than 0-based.
        ordered_packet_pair_index_sum += (packet_pair_index + 1)
    }
    fmt.printf("[{}] pt1: {} (of {})\n", title, ordered_packet_pair_index_sum, len(packet_pairs))

    //
    // Pt2. Flatten the pairs into a single list, add two special sentinel packets, sort the list, and find the indices
    //      of the sentinel packets
    //
    packet_list := create_packet_list_with_dividers(packet_pairs, context.temp_allocator)
    sort.merge_sort_proc(packet_list, packet_order_cmp)
    divider_packet_indices := get_divider_packet_indices(packet_list)
    decoder_key := (divider_packet_indices[0]+1) * (divider_packet_indices[1]+1)
    fmt.printf("[{}] pt2: {} ({} * {})\n", title, decoder_key, divider_packet_indices[0]+1, divider_packet_indices[1]+1)
}

parse_packet_pairs_from_input :: proc(input_lines: []string, allocator: mem.Allocator) -> []PacketPair {
    pairs := make([dynamic]PacketPair, allocator)

    for i := 0; i < len(input_lines); i += 3 {
        left_packet := parse_packet_from_line(input_lines[i], allocator)
        right_packet := parse_packet_from_line(input_lines[i+1], allocator)
        append(&pairs, PacketPair{left_packet, right_packet})
    }

    return pairs[:]
}

parse_packet_from_line :: proc(line: string, allocator: mem.Allocator) -> PacketListElem {
    assert(len(line) >= 2) // must be at least '[]'
    assert(line[0] == '[')

    listElem, charsParsed := parse_packet_from_line_helper(line[1:], allocator)
    assert(charsParsed + 1 == len(line))

    return listElem
}

parse_packet_from_line_helper :: proc(line: string, allocator: mem.Allocator) -> (PacketListElem, int) {
    assert(len(line) >= 1) // must be at least ']'

    elemList := make([dynamic]PacketElem, allocator)

    lastParsedChar: u8 = 0
    nextCharIndex := 0
    currentNumStringSliceStartIndex := 0
    currentNumStringSliceLength := 0
    foundEndOfList := false

    for nextCharIndex < len(line) && !foundEndOfList {
        nextChar := line[nextCharIndex]
        switch nextChar {
            case '0'..='9': {
                // Found another digit in the number we are currently processing (or need to start processing)
                if (currentNumStringSliceLength == 0) {
                    currentNumStringSliceStartIndex = nextCharIndex
                }
                currentNumStringSliceLength += 1
                lastParsedChar = nextChar
                nextCharIndex += 1
            }
            case ',', ']': {
                // Found a delimiter telling us that our current element is done parsing and we're about to find
                // another one on the next char
                if (currentNumStringSliceLength > 0) {
                    // before finding this delimiter we were reading a number.
                    // parse and store it
                    numStart := currentNumStringSliceStartIndex
                    numEnd := currentNumStringSliceStartIndex + currentNumStringSliceLength
                    currentNumString := line[numStart:numEnd]
                    num := strconv.atoi(currentNumString)
                    append(&elemList, PacketIntElem { num })

                    currentNumStringSliceLength = 0
                } else if (nextChar == ',') {
                    // before finding this delimiter we weren't reading a number.
                    // which means we must have just finished reading a sublist
                    assert(lastParsedChar == ']')
                }
                lastParsedChar = nextChar
                nextCharIndex += 1
                // this was also the end of the list if the delimter we found was the close bracket
                foundEndOfList = (nextChar == ']')
            }
            case '[': {
                // Found a sublist while parsing the current list. Recurse!!!
                listElem , numCharsParsed := parse_packet_from_line_helper(line[nextCharIndex+1:], allocator)
                append(&elemList, listElem)
                lastParsedChar = ']'
                nextCharIndex += numCharsParsed + 1
            }
            case: {
                fmt.printf("Found unexpected character parsing packet line! {}\n", nextChar)
                assert(false, "Found unexpected character parsing packet line")
            }
        }
    }

    totalCharsParsed := nextCharIndex
    return PacketListElem { list=elemList[:] }, totalCharsParsed
}

get_ordered_packet_pairs :: proc(pairs: []PacketPair, allocator: mem.Allocator) -> []int {
    ordered_pairs := make([dynamic]int, allocator)
    for pair,i in pairs {
        switch get_packet_pair_order(pair.left_packet, pair.right_packet) {
            case PacketPairOrder.Ordered: append(&ordered_pairs, i)
            case PacketPairOrder.Misordered: { } // noop
            case PacketPairOrder.Indeterminate: assert(false, "Found packet pair with identical packets")
        }
    }
    return ordered_pairs[:]
}

packet_order_cmp :: proc(left_packet: PacketListElem, right_packet: PacketListElem) -> int {
    ord := get_packet_pair_order(left_packet, right_packet)
    cmp_value: int

    switch ord {
        case PacketPairOrder.Ordered: cmp_value = -1
        case PacketPairOrder.Indeterminate: cmp_value = 0
        case PacketPairOrder.Misordered: cmp_value = 1
    }

    return cmp_value
}

get_packet_pair_order :: proc(left_packet: PacketListElem, right_packet: PacketListElem) -> PacketPairOrder {
    maxIndexToCheck := min(len(left_packet.list), len(right_packet.list))
    for i := 0; i < maxIndexToCheck; i += 1 {
        left_elem := left_packet.list[i]
        right_elem := right_packet.list[i]

        switch left_value in left_elem {

            case PacketIntElem: {
                switch right_value in right_elem {
                    // both left_value and right_value are ints!
                    // Check for relative order
                    case PacketIntElem: {
                        if (left_value.num < right_value.num) {
                            return PacketPairOrder.Ordered
                        } else if (right_value.num < left_value.num) {
                            return PacketPairOrder.Misordered
                        }
                    }
                    // left value is an int but right is a list
                    // convert left to a list and retry
                    case PacketListElem: {
                        left_arr := [1]PacketElem { left_elem }
                        order := get_packet_pair_order(PacketListElem{list=left_arr[:]}, right_value)
                        if (order != PacketPairOrder.Indeterminate) {
                            return order
                        }
                    }
                }
            }

            case PacketListElem: {
                switch right_value in right_elem {
                    // both left_value and right_value are lists!
                    // Check for relative order
                    case PacketListElem: {
                        order := get_packet_pair_order(left_value, right_value)
                        if (order != PacketPairOrder.Indeterminate) {
                            return order
                        }
                    }
                    // right value is an int but left is a list
                    // convert right to a list and retry
                    case PacketIntElem: {
                        right_arr := [1]PacketElem { right_elem }
                        order := get_packet_pair_order(left_value, PacketListElem{list=right_arr[:]})
                        if (order != PacketPairOrder.Indeterminate) {
                            return order
                        }
                    }
                }
            }
        }
    }

    left_exhausted := maxIndexToCheck == len(left_packet.list)
    right_exhausted := maxIndexToCheck == len(right_packet.list)
    if (left_exhausted && right_exhausted) {
        return PacketPairOrder.Indeterminate
    } else if (left_exhausted) {
        return PacketPairOrder.Ordered
    } else {
        assert(right_exhausted)
        return PacketPairOrder.Misordered
    }
}

print_packet_pairs :: proc(packet_pairs: []PacketPair) {
    for pair,i in packet_pairs {
        fmt.printf("Packet %02d:\n", i)
        print_packet(pair.left_packet)
        print_packet(pair.right_packet)
        fmt.printf("\n")
    }
}

print_packet :: proc(packet: PacketListElem, print_trailing_newline: bool = true) {
    fmt.print("[")
    for packet_elem, i in packet.list {
        if i != 0 {
            fmt.print(",")
        }

        switch p in packet_elem {
            case PacketIntElem: fmt.printf("{}", p.num)
            case PacketListElem: print_packet(p, false)
        }
    }
    fmt.print("]")
    if print_trailing_newline {
        fmt.print("\n")
    }
}

create_packet_list_with_dividers :: proc(packet_pairs: []PacketPair, allocator: mem.Allocator) -> []PacketListElem {
    // 2 packets per pair + 2 more divider packets
    packet_list := make([]PacketListElem, len(packet_pairs) * 2 + 2, allocator)

    for packet_pair,i in packet_pairs {
        packet_list[i*2 + 0] = packet_pair.left_packet
        packet_list[i*2 + 1] = packet_pair.right_packet
    }
    packet_list[len(packet_pairs)*2 + 0] = parse_packet_from_line("[[2]]", allocator)
    packet_list[len(packet_pairs)*2 + 1] = parse_packet_from_line("[[6]]", allocator)

    return packet_list
}

get_divider_packet_indices :: proc(packet_list: []PacketListElem) -> [2]int {
    divider_indices := [2]int{-1, -1}

    for p, packet_index in packet_list {
        // check for p==[[2]] or [[6]]

        // p doesn't have exactly one element so it can't be one of our divider elements
        if (len(p.list) != 1) {
            continue
        }

        p_innerlist: PacketListElem
        switch first_elem in p.list[0] {
            case PacketIntElem: continue
            case PacketListElem: p_innerlist = first_elem
        }

        // the inner sublist doesn't have exactly one element so it can't be our divider element
        // e.g. p_innerlist==[2] or [6]
        if (len(p_innerlist.list) != 1) {
            continue
        }

        switch first_elem in p_innerlist.list[0] {
            case PacketIntElem: {
                switch first_elem.num {
                    case 2: {
                        assert(divider_indices[0] == -1) // check not yet set
                        divider_indices[0] = packet_index
                    }
                    case 6: {
                        assert(divider_indices[1] == -1) // check not yet set
                        divider_indices[1] = packet_index
                    }
                    case: {
                        // otherwise, an unrelated packet which just happens to look like one of our dividers
                        continue
                    }
                }
            }
            case PacketListElem: continue
        }
    }

    // assert all divider_indices found
    for d in divider_indices {
        assert(d != -1)
    }

    return divider_indices
}
