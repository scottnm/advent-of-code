import sys
import re
import dataclasses
import logging
import copy
import time

@dataclasses.dataclass
class BatteryBank:
    battery_joltages: list[int]

def process_input(lines: list[str]) -> list[BatteryBank]:
    banks = []
    for line in lines:
        joltages = []
        for joltage_value_char in line.rstrip():
            try:
                joltage_value = int(joltage_value_char)
                if not (1 <= joltage_value <= 9):
                    raise ValueError
                joltages.append(joltage_value)
            except ValueError:
                logging.fatal("Invalid joltage value: '%s' in line %s", joltage_value_char, line)
                sys.exit(1)
        banks.append(BatteryBank(battery_joltages=joltages))
    return banks

def find_largest_pt1_joltage_from_bank(bank: BatteryBank) -> int:
    if len(bank.battery_joltages) < 2:
        raise ValueError("bank must have at least 2 batteries")
    
    max_from_left = [0] * len(bank.battery_joltages)
    max_from_left[0] = bank.battery_joltages[0]
    max_from_right = [0] * len(bank.battery_joltages)
    max_from_right[-1] = bank.battery_joltages[-1]

    for i in range(1, len(bank.battery_joltages)):
        max_from_left[i] = max(max_from_left[i - 1], bank.battery_joltages[i])

    for i in range(len(bank.battery_joltages) - 2, -1, -1):
        max_from_right[i] = max(max_from_right[i + 1], bank.battery_joltages[i])

    largest_digits = (0, 0)
    for i in range(0, len(bank.battery_joltages) - 1):
        left_digit = max_from_left[i]
        right_digit = max_from_right[i+1]
        if left_digit > largest_digits[0] or (left_digit == largest_digits[0] and right_digit > largest_digits[1]):
            largest_digits = (left_digit, right_digit)

    return largest_digits[0] * 10 + largest_digits[1]

@dataclasses.dataclass
class BatteryBankJoltageMemoKey:
    list_start_idx: int
    sequence_length: int
    
    def __eq__(self, other):
        return isinstance(other, BatteryBankJoltageMemoKey) and \
            self.list_start_idx == other.list_start_idx and \
            self.sequence_length == other.sequence_length
    
    def __hash__(self):
        return hash((self.list_start_idx, self.sequence_length))

type BatteryBankJoltageMemo = dict[BatteryBankJoltageMemoKey, int]

def prepend_digit(digit: int, right_hand_digits: int) -> int:
    assert 1 <= digit <= 9
    assert right_hand_digits >= 0
    right_hand_digit_count = len(str(right_hand_digits))
    digit_shift_multiplier = 10 ** right_hand_digit_count
    return (digit * digit_shift_multiplier) + right_hand_digits

def find_largest_pt2_joltage_from_bank(
    bank: BatteryBank,
    memo: BatteryBankJoltageMemo | None = None,
    bank_start_idx =  0,
    required_sequence_size = 12
    ) -> int:

    assert bank_start_idx < len(bank.battery_joltages), "invalid bank start idx"
    if (len(bank.battery_joltages) - bank_start_idx) < required_sequence_size:
        raise ValueError(f"bank must have at least {required_sequence_size} batteries starting from idx {bank_start_idx}")

    if memo is None:
        memo = dict()

    battery_bank_joltage_memo_key = BatteryBankJoltageMemoKey(bank_start_idx, required_sequence_size)
    memod_value = memo.get(battery_bank_joltage_memo_key)
    if memod_value is not None:
        return memod_value
    
    max_value = 0 
    for i in range(bank_start_idx, len(bank.battery_joltages) - (required_sequence_size - 1)):
        new_candidate_digit = bank.battery_joltages[i]
        if required_sequence_size > 1:
            new_candidate_least_significant_digits = find_largest_pt2_joltage_from_bank(bank, memo, i+1, required_sequence_size-1)
            max_value_candidate = prepend_digit(new_candidate_digit, new_candidate_least_significant_digits)
        else:
            max_value_candidate = new_candidate_digit
        max_value = max(max_value, max_value_candidate)

    memo[battery_bank_joltage_memo_key] = max_value
    return max_value

def main() -> None:
    if len(sys.argv) < 2:
        print("need input file")
        sys.exit(-1)

    log_verbose = "--verbose" in sys.argv
    log_level = logging.DEBUG if log_verbose else logging.INFO
    logging.basicConfig(level=log_level)

    filename = sys.argv[1]
    with open(filename, "r", encoding="utf8") as f:
        input_lines = f.readlines()
        logging.debug("input_lines: %s", input_lines)
        banks = process_input(input_lines)

    logging.debug("banks: %s", banks)

    pt1_time_start = time.perf_counter()
    largest_joltages_pt1 = [find_largest_pt1_joltage_from_bank(bank) for bank in banks]
    pt1_time_end = time.perf_counter()
    for (largest_joltage, bank) in zip(largest_joltages_pt1, banks):
        logging.debug("bank %s :: largest_joltage=%d", bank, largest_joltage)
    logging.info("Pt1. total output joltage: %d (calc time = %.6f seconds)", 
        sum(largest_joltages_pt1), 
        pt1_time_end - pt1_time_start)

    pt2_time_start = time.perf_counter()
    largest_joltages_pt2 = [find_largest_pt2_joltage_from_bank(bank) for bank in banks]
    pt2_time_end = time.perf_counter()
    for (largest_joltage, bank) in zip(largest_joltages_pt2, banks):
        logging.debug("bank %s :: largest_joltage=%d", bank, largest_joltage)
    logging.info("Pt2. total output joltage: %d (calc time = %.6f seconds)", 
        sum(largest_joltages_pt2),
        pt2_time_end - pt2_time_start)

if __name__ == "__main__":
    main()
