import dataclasses
import logging
import os
import re
import sys
import typing
import functools

current_dir = os.path.dirname(os.path.realpath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.append(project_root)
from shared import util

@dataclasses.dataclass
class ButtonSchematic:
    toggle_mask: int

    def __eq__(self, other):
        return self.toggle_mask == other.toggle_mask

    def __hash__(self) -> int:
        return hash(self.toggle_mask)

    def __repr__(self) -> str:
        buttons_str = ",".join(self.__buttons_str())
        return f"ButtonSchematic<0x{self.toggle_mask:x}(" + ",".join(buttons_str) + ")>"

    def __str__(self) -> str:
        buttons_str = ",".join(self.__buttons_str())
        return "(" + buttons_str + ")"

    def __buttons_str(self) -> list[str]:
        i = 0
        button_strs = []
        while (1<<i) <= self.toggle_mask:
            if (1<<i) & self.toggle_mask != 0:
                button_strs.append(str(i))
            i += 1
        return button_strs

@dataclasses.dataclass
class TargetLightState:
    target_light_bits: int
    light_count: int

@dataclasses.dataclass
class MachineSpec:
    target_light_state: int
    light_count: int
    toggle_buttons: list[ButtonSchematic]

    def __repr__(self) -> str:
        light_str = self.__lights_str()
        button_str = " ".join((repr(b) for b in self.toggle_buttons))
        return f"MachineSpec<lights=0x{self.target_light_state:x}[{light_str}] buttons={button_str}>"

    def __str__(self) -> str:
        light_str = self.__lights_str()
        button_str = " ".join((str(b) for b in self.toggle_buttons))
        return f"[{light_str}] buttons={button_str}"

    def __lights_str(self) -> str:
        lights_str = ""
        for i in range(self.light_count):
            light_char = "#" if ((1<<i) & self.target_light_state) != 0 else "."
            lights_str += light_char
        return lights_str


def make_light_state(lights: list[bool]) -> TargetLightState:
    v = 0
    for i,light_on in enumerate(lights):
        if light_on:
            v |= (1 << i)
    return TargetLightState(target_light_bits=v, light_count=len(lights))

LIGHT_STATE_RGX = re.compile(r"^\[([#.]+)\]$")
def process_input(lines: list[str]) -> list[MachineSpec]:
    machines: list[MachineSpec] = []
    for line in lines:
        machine_spec_parts = line.split(" ")
        if len(machine_spec_parts) < 3:
            util.fatal_error("Invalid line! expected >=3 parts. found %d. line='%s'", 
                len(machine_spec_parts), 
                line)

        light_state_part = machine_spec_parts[0] 
        joltage_part = machine_spec_parts[-1]
        button_parts = machine_spec_parts[1:-1]
        assert len(button_parts) > 0

        light_state_match = LIGHT_STATE_RGX.match(light_state_part)
        if not light_state_match:
            util.fatal_error("Invalid line! light state improperly formatted. lightstate='%s'", light_state_part)
        light_state = make_light_state([c == "#" for c in light_state_match.group(1)])

        toggle_buttons: list[ButtonSchematic] = []
        for button_part in button_parts:
            if not (button_part.startswith("(") and button_part.endswith(")")):
                util.fatal_error("Invalid line! bad button part. expected () wrapping. part='%s'", button_part)
            button_part = button_part[1:-1]
            button_strs = button_part.split(",")
            if len(button_strs) == 0:
                util.fatal_error("Invalid line! bad button part. empty button list. part='%s'", button_part)
            
            try:
                buttons = (int(s) for s in button_strs)
                mask_values = [1 << b for b in buttons]
                mask = functools.reduce(lambda x,y: x|y, mask_values, 0)
                toggle_buttons.append(ButtonSchematic(toggle_mask=mask))
            except ValueError:
                util.fatal_error("Invalid line! bad button part. non-numeric button value. part='%s'", button_part)
        
        new_machine = MachineSpec(
            target_light_state=light_state.target_light_bits,
            light_count=light_state.light_count,
            toggle_buttons=toggle_buttons)
        machines.append(new_machine)
    return machines

def calc_light_state(buttons: typing.Iterable[ButtonSchematic]) -> int:
    result = 0
    for b in buttons:
        result ^= b.toggle_mask
    return result

def shortest_button_press_sequence(
    machine: MachineSpec
    ) -> list[ButtonSchematic] | None:

    shortest_button_combos: dict[int, typing.Set[ButtonSchematic]] = dict()
    shortest_button_combos[0] = set() 

    candidates: list[typing.Set[ButtonSchematic]] = [ set() ]
    for seq_size in range(len(machine.toggle_buttons)):
        new_candidates = []
        for candidate in candidates:
            base_light_state = calc_light_state(candidate)
            for button in machine.toggle_buttons:
                # don't repeat buttons in a toggle sequence
                if button in candidate:
                    continue

                new_light_state = base_light_state ^ button.toggle_mask
                # if we've already calculated a shortest length to achieve this button state
                # we don't need to keep searching
                if new_light_state in shortest_button_combos:
                    continue

                new_candidate = candidate | {button}
                if new_light_state == machine.target_light_state:
                    return list(new_candidate)
                else:
                    shortest_button_combos[new_light_state] = new_candidate
                    new_candidates.append(new_candidate)

        candidates = new_candidates
    return None

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
    machines = process_input(input_lines)
    logging.debug("machines:")
    for i,m in enumerate(machines):
        logging.debug("  %03d: %r", i, m)

    with util.time_section("pt1"):
        shortest_sequences = []
        for m in machines:
            seq = shortest_button_press_sequence(m)
            if seq is None:
                util.fatal_error("Machine %s could not be solved", m)
            shortest_sequences.append(seq)
        for m, shortest_sequence in zip(machines, shortest_sequences):
            logging.debug("shortest sequence for %s = %s", m, " ".join(str(b) for b in shortest_sequence))
        logging.info("pt1. : %d", sum(len(seq) for seq in shortest_sequences))

    with util.time_section("pt1"):
        logging.info("pt2. : NOTIMPL")

if __name__ == "__main__":
    main()
