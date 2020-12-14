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

#[derive(Debug, Clone, Copy)]
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
    let seatings = [
        EncodedSeatBsp::from_str("BFFFBBFRRR").unwrap(),
        EncodedSeatBsp::from_str("FFFBBBFRRR").unwrap(),
        EncodedSeatBsp::from_str("BBFFBBFRLL").unwrap(),
    ];

    seatings.to_vec()
}

fn main() {
    for seating in get_seatings_from_input("src/simple_input.txt") {
        println!("{:?}", seating);
    }
}
