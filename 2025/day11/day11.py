import dataclasses
import logging
import os
import re
import sys
import typing
import pprint

current_dir = os.path.dirname(os.path.realpath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.append(project_root)
from shared import util

ConnectionGraph = typing.Dict[str, list[str]]

RGX = re.compile(r"^(\w\w\w)$")
def process_input(lines: list[str]) -> ConnectionGraph:
    graph: ConnectionGraph = dict()
    for line in lines:
        line_parts = line.split(": ")
        if len(line_parts) != 2:
            util.fatal_error("Line does not match expected pattern. Need at least one input and one output device! '%s'", line)
        input_device, output_device_list_part = line_parts
        output_device_parts = output_device_list_part.split(" ")
        if not RGX.match(input_device):
            util.fatal_error("Line does not match expected pattern. Bad input device '%s'", input_device)

        for output_device in output_device_parts:
            if not RGX.match(output_device):
                util.fatal_error("Line does not match expected pattern. Bad output device '%s'", input_device)

        if input_device in graph:
            util.fatal_error("Unexpected duplicate input device line!\n%s %s\nlast line %s %s",
                input_device,
                output_device_parts,
                input_device,
                graph[input_device])

        graph[input_device] = output_device_parts

    return graph

def find_all_paths(
    graph: ConnectionGraph,
    start_node: str,
    end_node: str
    ) -> list[list[str]]:

    paths = find_all_paths_helper(graph, start_node, end_node, set())
    for p in paths:
        p.reverse()
    return paths

def find_all_paths_helper(
    graph: ConnectionGraph,
    start_node: str,
    end_node: str,
    visited_nodes: typing.Set[str]
    ) -> list[list[str]]:

    if start_node == end_node:
        return [[end_node]]

    visited_nodes.add(start_node)
    paths = []
    for child in graph.get(start_node, []):
        if child in visited_nodes:
            continue
        child_paths = find_all_paths_helper(graph, start_node=child, end_node=end_node, visited_nodes=visited_nodes)
        paths += child_paths
    
    for p in paths:
        p.append(start_node)
    
    visited_nodes.remove(start_node)
    return paths

def find_all_paths_pt2(
    graph: ConnectionGraph,
    start_node: str,
    req_intermediate_nodes: typing.Set[str],
    end_node: str
    ) -> list[list[str]]:

    int_paths: list[list[str]] = []
    for int_node in req_intermediate_nodes:
        int_paths += find_all_paths_helper(graph, start_node=start_node, end_node=int_node, visited_nodes=set())


    int_paths = [ p for p in int_paths if all(rin in p for rin in req_intermediate_nodes) ]
    final_paths: list[list[str]] = []
    for int_path in int_paths:
        int_path_end_node = int_path[0]
        final_path_parts = find_all_paths_helper(
            graph, 
            start_node=int_path_end_node, 
            end_node=end_node, 
            visited_nodes=set(int_path))
        for fp in final_path_parts:
            final_paths.append(fp + int_path)

    for p in final_paths:
        p.reverse()

    return final_paths 

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = any(v_arg in sys.argv for v_arg in ("-v", "--verbose"))
    util.setup_aoc_logger(log_verbose=log_verbose)

    filename = sys.argv[1]
    input_lines = util.read_normalized_file_lines(filename)
    logging.debug("input_lines: %s", input_lines)
    device_flow_graph = process_input(input_lines)
    logging.debug("device_flow_graph: %s", pprint.pformat(device_flow_graph))

    # only do the pt1 'you'->'out' path finding if I've passed an appropriate input file
    if "you" in device_flow_graph:
        with util.time_section("pt1"):
            paths = find_all_paths(device_flow_graph, "you", "out")
            logging.info("paths: %s", pprint.pformat(paths))
            logging.info("pt1. path count: %d", len(paths))
    else:
        logging.warning("skipping pt 1 for input file '%s'", filename)

    # only do the pt2 'svr'->'out' path finding if I've passed an appropriate input file
    if "svr" in device_flow_graph:
        with util.time_section("pt2"):
            paths = find_all_paths_pt2(device_flow_graph, "svr", {"dac", "fft"}, "out")
            logging.info("paths: %s", pprint.pformat(paths))
            logging.info("pt2. path count: %d", len(paths))
    else:
        logging.warning("skipping pt 2 for input file '%s'", filename)

if __name__ == "__main__":
    main()
