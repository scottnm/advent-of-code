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

fn part_2(current_time: Timestamp, buses: &[Option<BusId>]) {
    println!("pt2:");
}

fn main() {
    let (current_time, buses) = get_input(&input_helpers::get_input_file_from_args(
        &mut std::env::args(),
    ));

    part_1(current_time, &buses);
    part_2(current_time, &buses);
}
