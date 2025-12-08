import dataclasses
import logging
import os
import sys
import typing
import functools

current_dir = os.path.dirname(os.path.realpath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.append(project_root)
from shared import util

@dataclasses.dataclass
class Vec3:
    x: int
    y: int
    z: int

    def __eq__(self, v2):
        return self.x == v2.x and \
               self.y == v2.y and \
               self.z == v2.z

    def __hash__(self) -> int:
        return hash((self.x,self.y,self.z))

@dataclasses.dataclass
class JunctionBoxDist:
    j1: Vec3
    j2: Vec3
    sqdist: int

class Circuit:
    def __init__(self):
        self.boxes = set()

def calculate_square_dist(v1: Vec3, v2: Vec3) -> int:
    xd = v2.x - v1.x
    yd = v2.y - v1.y
    zd = v2.z - v1.z
    return (xd*xd) + (yd*yd) + (zd*zd)

def calculate_junction_box_distance(box_positions: list[Vec3]) -> list[JunctionBoxDist]:
    positions: list[JunctionBoxDist] = []
    for i in range(len(box_positions)-1):
        for j in range(i+1, len(box_positions)):
            box_pos_i = box_positions[i]
            box_pos_j = box_positions[j]
            square_dist = calculate_square_dist(box_pos_i, box_pos_j)
            positions.append(JunctionBoxDist(j1=box_pos_i, j2=box_pos_j, sqdist=square_dist))
    positions.sort(key=lambda p: p.sqdist)
    return positions

def build_circuits(box_positions: list[Vec3], box_limit: int) -> list[Circuit]:
    sorted_box_distances = calculate_junction_box_distance(box_positions)
    box_limit = min(len(sorted_box_distances), box_limit)
    sorted_box_distances = sorted_box_distances[:box_limit]
    logging.debug("box distances: %s", sorted_box_distances)

    circuits: set[Circuit] = set()
    circuit_dict: dict[Vec3, Circuit] = dict()

    for next_shortest_junction_pair in sorted_box_distances:
        next_circuit = None
        j1_circuit = circuit_dict.get(next_shortest_junction_pair.j1, None)
        j2_circuit = circuit_dict.get(next_shortest_junction_pair.j2, None)
        if j1_circuit is None and j2_circuit is None:
            next_circuit = Circuit()
        elif j1_circuit is not None and j2_circuit is not None:
            next_circuit = j1_circuit
            if j1_circuit != j2_circuit:
                j1_circuit.boxes.update(j2_circuit.boxes)
                for box, box_circuit in circuit_dict.items():
                    if box_circuit == j2_circuit:
                        circuit_dict[box] = j1_circuit
                circuits.remove(j2_circuit)
                j2_circuit = None
        elif j1_circuit is not None:
            next_circuit = j1_circuit
        else:
            assert j2_circuit is not None
            next_circuit = j2_circuit

        next_circuit.boxes.add(next_shortest_junction_pair.j1)
        next_circuit.boxes.add(next_shortest_junction_pair.j2)
        logging.debug("connecting %s to %s in circuit %s (new_circ len=%d)",
            next_shortest_junction_pair.j1, next_shortest_junction_pair.j2, next_circuit, len(next_circuit.boxes))

        circuit_dict[next_shortest_junction_pair.j1] = next_circuit
        circuit_dict[next_shortest_junction_pair.j2] = next_circuit
        circuits.add(next_circuit)

    logging.debug("%d unconnected boxes! (circuit len = 1)",
        sum(1 for b in box_positions if b not in circuit_dict))

    return list(circuits)

def calculate_min_extension_cable_len(box_positions: list[Vec3]) -> typing.Tuple[Vec3, Vec3] | None:
    sorted_box_distances = calculate_junction_box_distance(box_positions)
    logging.debug("box distances: %s", sorted_box_distances)

    circuits: set[Circuit] = set()
    circuit_dict: dict[Vec3, Circuit] = dict()

    for next_shortest_junction_pair in sorted_box_distances:
        next_circuit = None
        j1_circuit = circuit_dict.get(next_shortest_junction_pair.j1, None)
        j2_circuit = circuit_dict.get(next_shortest_junction_pair.j2, None)
        if j1_circuit is None and j2_circuit is None:
            next_circuit = Circuit()
        elif j1_circuit is not None and j2_circuit is not None:
            next_circuit = j1_circuit
            if j1_circuit != j2_circuit:
                j1_circuit.boxes.update(j2_circuit.boxes)
                for box, box_circuit in circuit_dict.items():
                    if box_circuit == j2_circuit:
                        circuit_dict[box] = j1_circuit
                circuits.remove(j2_circuit)
                j2_circuit = None
        elif j1_circuit is not None:
            next_circuit = j1_circuit
        else:
            assert j2_circuit is not None
            next_circuit = j2_circuit

        next_circuit.boxes.add(next_shortest_junction_pair.j1)
        next_circuit.boxes.add(next_shortest_junction_pair.j2)
        logging.debug("connecting %s to %s in circuit %s (new_circ len=%d)",
            next_shortest_junction_pair.j1, next_shortest_junction_pair.j2, next_circuit, len(next_circuit.boxes))

        circuit_dict[next_shortest_junction_pair.j1] = next_circuit
        circuit_dict[next_shortest_junction_pair.j2] = next_circuit
        circuits.add(next_circuit)

        if len(circuits) == 1 and len(circuit_dict) == len(box_positions):
            return (next_shortest_junction_pair.j1, next_shortest_junction_pair.j2)

    return None


def process_input(lines: list[str]) -> list[Vec3]:
    junction_box_positions = []
    for line in lines:
        pos_parts = line.split(",")
        if len(pos_parts) != 3:
            util.fatal_error("Invalid input line! expected 3 parts. saw %d: %s", len(pos_parts), line)

        try:
            x, y, z = pos_parts
            new_pos = Vec3(x=int(x), y=int(y), z=int(z))
            junction_box_positions.append(new_pos)
        except ValueError:
            util.fatal_error("line contained non-numeric part: %s", line)
    return junction_box_positions

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = "--verbose" in sys.argv
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    input_lines = util.read_normalized_file_lines(filename)
    logging.debug("input_lines: %s", input_lines)
    junction_boxes = process_input(input_lines)
    logging.debug("junction box positions: %s", junction_boxes)

    with util.time_section("pt1"):
        is_sample_input = "sample" in filename
        circuits = build_circuits(junction_boxes, 10 if is_sample_input else 1000)
        for c in circuits:
            logging.debug("%s: %s", c, list(c.boxes))
        circuit_sizes = [len(c.boxes) for c in circuits]
        logging.debug("circuit_sizes: %s", circuit_sizes)
        top_3_circuit_sizes = sorted(circuit_sizes, reverse=True)[:3]
        logging.debug("top3: %s", top_3_circuit_sizes)
        result = functools.reduce(lambda a,b: a*b, top_3_circuit_sizes, 1)
        logging.info("pt1. : %d", result)

    with util.time_section("pt1"):
        logging.info("pt2. : NOTIMPL")
        is_sample_input = "sample" in filename
        max_dist_boxes = calculate_min_extension_cable_len(junction_boxes)
        if max_dist_boxes is None:
            util.fatal_error("boxes could not all form one circuit!")

        logging.info("max_dist_boxes: %s", max_dist_boxes)
        result = max_dist_boxes[0].x * max_dist_boxes[1].x
        logging.info("pt2. : %d", result)

if __name__ == "__main__":
    main()
