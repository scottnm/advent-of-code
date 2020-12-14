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
    seat_code: [SeatBit; 3],
}

impl std::str::FromStr for EncodedSeatBsp {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(String::from("Invalid length of input!"));
        }

        let parse_row_input = |c: u8| -> Result<RowBit, Self::Err> {
            match c as char {
                'F' => Ok(RowBit::F),
                'B' => Ok(RowBit::B),
                _ => Err(String::from("Invalid row input!")),
            }
        };

        let parse_seat_input = |c: u8| -> Result<SeatBit, Self::Err> {
            match c as char {
                'L' => Ok(SeatBit::L),
                'R' => Ok(SeatBit::R),
                _ => Err(String::from("Invalid seat input!")),
            }
        };

        let row_input = &s.as_bytes()[0..7];
        let seat_input = &s.as_bytes()[7..];

        let row_code = [
            parse_row_input(row_input[0])?,
            parse_row_input(row_input[1])?,
            parse_row_input(row_input[2])?,
            parse_row_input(row_input[3])?,
            parse_row_input(row_input[4])?,
            parse_row_input(row_input[5])?,
            parse_row_input(row_input[6])?,
        ];

        let seat_code = [
            parse_seat_input(seat_input[0])?,
            parse_seat_input(seat_input[1])?,
            parse_seat_input(seat_input[2])?,
        ];

        Ok(EncodedSeatBsp {
            row_code,
            seat_code,
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
