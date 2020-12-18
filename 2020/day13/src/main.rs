type Timestamp = usize;
type BusId = usize;

fn get_input(file_name: &str) -> (Timestamp, Vec<Option<BusId>>) {
    let mut lines = input_helpers::read_lines(file_name);
    let timestamp_line = lines.next().unwrap();
    let bus_list_line = lines.next().unwrap();
    assert!(lines.next().is_none());

    let mut buses = Vec::new();
    for bus_str in bus_list_line.split(',') {
        let opt_bus_id = match bus_str {
            "x" => None,
            _ => Some(bus_str.parse::<BusId>().unwrap()),
        };
        buses.push(opt_bus_id);
    }

    (timestamp_line.parse().unwrap(), buses)
}

fn get_bus_period(bus_id: BusId) -> Timestamp {
    // the bus arrives at a periodic rate equal to its id. i.e. Bus 5 arrives every 5 minutes.
    bus_id as Timestamp
}

fn get_next_bus_arrival_time(current_time: Timestamp, bus_id: BusId) -> Timestamp {
    let bus_period = get_bus_period(bus_id);
    let prev_bus_arrival_time = (current_time / bus_period) * bus_period;
    if prev_bus_arrival_time == current_time {
        current_time
    } else {
        prev_bus_arrival_time + bus_period
    }
}

fn part_1(current_time: Timestamp, buses: &[Option<BusId>]) {
    let next_arrival_times: Vec<(BusId, Timestamp)> = buses
        .iter()
        .filter(|opt_b| opt_b.is_some())
        .map(|opt_b| opt_b.unwrap())
        .map(|b| (b, get_next_bus_arrival_time(current_time, b)))
        .collect();

    let (earliest_bus, earliest_arrival) = next_arrival_times
        .iter()
        .min_by(|(_, time_1), (_, time_2)| time_1.cmp(time_2))
        .unwrap();

    let time_to_wait = earliest_arrival - current_time;
    let solution = time_to_wait * earliest_bus;
    println!(
        "pt1: Earliest bus={}, arrival time={}, time to wait={}, solution={}",
        earliest_bus, earliest_arrival, time_to_wait, solution
    );
}

fn does_bus_arrive_at_time(t: Timestamp, bus: BusId) -> bool {
    t % get_bus_period(bus) == 0
}

fn find_earliest_timestamp_with_matching_pattern(buses: &[Option<BusId>]) -> Option<Timestamp> {
    // the description for part 2, requires that the first bus on the schedule not be ignored.
    assert!(buses[0].is_some());
    let first_bus_period = get_bus_period(buses[0].unwrap());

    let required_bus_times: Vec<(Timestamp, BusId)> = buses
        .iter()
        .enumerate()
        .map(|(i, opt_bus)| (i, opt_bus))
        .filter(|(_, opt_bus)| opt_bus.is_some())
        .map(|(t_offset, opt_bus)| (t_offset, opt_bus.unwrap()))
        .collect();

    for t in (0..).step_by(first_bus_period) {
        let bus_pattern_satisfied = required_bus_times
            .iter()
            .all(|(offset, bus)| does_bus_arrive_at_time(t + offset, *bus));

        if bus_pattern_satisfied {
            return Some(t);
        }
    }

    None
}

fn part_2(buses: &[Option<BusId>]) {
    let t = find_earliest_timestamp_with_matching_pattern(buses).unwrap();
    println!("pt2: earliest timestamp={}", t);
}

fn main() {
    let (current_time, buses) = get_input(&input_helpers::get_input_file_from_args(
        &mut std::env::args(),
    ));

    part_1(current_time, &buses);
    part_2(&buses);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_test_1() {
        // 17,x,13,19 first occurs at timestamp 3417.
        assert_eq!(
            find_earliest_timestamp_with_matching_pattern(&[Some(17), None, Some(13), Some(19)]),
            Some(3417),
        );
    }

    #[test]
    fn part_2_test_2() {
        // 67,7,59,61 first occurs at timestamp 754018.
        assert_eq!(
            find_earliest_timestamp_with_matching_pattern(&[Some(67), Some(7), Some(59), Some(61)]),
            Some(754018),
        );
    }

    #[test]
    fn part_2_test_3() {
        // 67,x,7,59,61 first occurs at timestamp 779210.
        assert_eq!(
            find_earliest_timestamp_with_matching_pattern(&[
                Some(67),
                None,
                Some(7),
                Some(59),
                Some(61)
            ]),
            Some(779210),
        );
    }

    #[test]
    fn part_2_test_4() {
        // 67,7,x,59,61 first occurs at timestamp 1261476.
        assert_eq!(
            find_earliest_timestamp_with_matching_pattern(&[
                Some(67),
                Some(7),
                None,
                Some(59),
                Some(61)
            ]),
            Some(1261476),
        );
    }

    #[test]
    fn part_2_test_5() {
        // 1789,37,47,1889 first occurs at timestamp 1202161486
        assert_eq!(
            find_earliest_timestamp_with_matching_pattern(&[
                Some(1789),
                Some(37),
                Some(47),
                Some(1889)
            ]),
            Some(1202161486),
        );
    }
}
