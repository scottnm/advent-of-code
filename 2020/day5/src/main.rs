use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum RowBit {
    F,
    B,
}

#[derive(Debug, Clone, Copy)]
enum SeatBit {
    L,
    R,
}

#[derive(Debug)]
struct EncodedSeatBsp {
    row_code: [RowBit; 7],
    seat_code: [SeatBit; 7],
}

impl std::str::FromStr for EncodedSeatBsp {
    type Err = String;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(EncodedSeatBsp {
            row_code: [RowBit::F; 7],
            seat_code: [SeatBit::L; 7],
        })
    }
}

fn get_seatings_from_input(file_name: &str) -> Vec<EncodedSeatBsp> {
    let mut seatings = Vec::new();

    for line in input_helpers::read_lines(file_name) {
        seatings.push(EncodedSeatBsp::from_str(&line).unwrap());
    }

    seatings
}

fn main() {
    for seating in get_seatings_from_input("src/simple_input.txt") {
        println!("{:?}", seating);
    }
}
