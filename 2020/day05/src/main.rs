// TODO: there's implicit coupling between this and expecting 3 SeatBits as input.
// Wonder if there's a way to remove that coupling
const MAX_SEATS_IN_ROW: usize = 8;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RowBit {
    F,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SeatBit {
    L,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SeatInfo {
    row: usize,
    seat: usize,
}

impl SeatInfo {
    fn new(row: usize, seat: usize) -> Self {
        SeatInfo { row, seat }
    }

    fn next_seat(&self) -> Self {
        if self.seat == MAX_SEATS_IN_ROW - 1 {
            SeatInfo {
                row: self.row + 1,
                seat: 0,
            }
        } else {
            SeatInfo {
                row: self.row,
                seat: self.seat + 1,
            }
        }
    }

    fn seat_id(&self) -> usize {
        self.row * 8 + self.seat
    }

    fn cmp(a: &Self, b: &Self) -> std::cmp::Ordering {
        let row_cmp = a.row.partial_cmp(&b.row).unwrap();
        match row_cmp {
            std::cmp::Ordering::Equal => a.seat.partial_cmp(&b.seat).unwrap(),
            _ => row_cmp,
        }
    }
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

impl EncodedSeatBsp {
    fn calculate_seat_data(&self) -> SeatInfo {
        fn as_binary<T>(arr: &[T], get_bit_func: fn(T) -> usize) -> usize
        where
            T: Copy,
        {
            let mut binary_val: usize = 0;
            for (i, e) in arr.iter().rev().enumerate() {
                let bit = get_bit_func(*e);
                binary_val |= bit << i;
            }
            binary_val
        }

        let row = as_binary(&self.row_code, |rb: RowBit| match rb {
            RowBit::F => 0,
            RowBit::B => 1,
        });

        let seat = as_binary(&self.seat_code, |sb: SeatBit| match sb {
            SeatBit::L => 0,
            SeatBit::R => 1,
        });

        SeatInfo::new(row, seat)
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
    let seatings = get_seatings_from_input("src/input.txt");

    let seat_data = {
        let mut seat_data: Vec<SeatInfo> =
            seatings.iter().map(|s| s.calculate_seat_data()).collect();
        seat_data.sort_by(SeatInfo::cmp);
        seat_data
    };

    for seat in &seat_data {
        println!("{:?} - {}", seat, seat.seat_id());
    }

    let largest_seat_id = seat_data.iter().map(|s| s.seat_id()).max().unwrap();
    println!("largest seat id: {}", largest_seat_id);

    let first_seat = seat_data.first().unwrap();

    let mut my_seat = None;
    let mut current_seat = *first_seat;
    for seat in seat_data {
        if seat != current_seat {
            my_seat = Some(current_seat);
            break;
        }
        current_seat = current_seat.next_seat();
    }

    let my_seat = my_seat.unwrap();
    println!("My seat: {:?} - {}", my_seat, my_seat.seat_id());
}
