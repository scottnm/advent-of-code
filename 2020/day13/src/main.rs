type Timestamp = usize;
type BusId = usize;

fn get_input(file_name: &str) -> (Timestamp, Vec<BusId>) {
    let mut lines = input_helpers::read_lines(file_name);
    let timestamp_line = lines.next().unwrap();
    let bus_list_line = lines.next().unwrap();
    assert!(lines.next().is_none());

    let mut buses = Vec::new();
    for bus_str in bus_list_line.split(',') {
        if bus_str == "x" {
            continue;
        }

        buses.push(bus_str.parse::<BusId>().unwrap());
    }

    (timestamp_line.parse().unwrap(), buses)
}

fn main() {
    let (timestamp, buses) = get_input(&input_helpers::get_input_file_from_args(
        &mut std::env::args(),
    ));

    dbg!(timestamp, buses);
}
