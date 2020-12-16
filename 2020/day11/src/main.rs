extern crate input_helpers;
use std::iter::FromIterator;

enum SeatCell {
    Floor,
    Free,
    Occupied,
}

impl SeatCell {
    fn from_char(c: char) -> SeatCell {
        match c {
            '.' => SeatCell::Floor,
            'L' => SeatCell::Free,
            '#' => SeatCell::Occupied,
            _ => panic!("Invalid character ({})", c),
        }
    }

    fn to_ascii_char(&self) -> char {
        match self {
            SeatCell::Floor => '.',
            SeatCell::Free => 'L',
            SeatCell::Occupied => '#',
        }
    }
}

struct SeatGrid {
    grid: Vec<SeatCell>,
    row_count: usize,
    col_count: usize,
}

impl SeatGrid {
    fn from_file(file_name: &str) -> Self {
        let mut lines = input_helpers::read_lines(file_name).peekable();
        let unpadded_grid_width = {
            let first_line = lines.peek().unwrap();
            first_line.len()
        };

        let padded_grid_width = unpadded_grid_width + 2;

        let mut grid = Vec::new();

        // add a padding row above the grid
        for _ in 0..padded_grid_width {
            grid.push(SeatCell::Floor);
        }

        // add each grid row with a padding cell on each side
        let mut line_count = 0;
        for line in lines {
            line_count += 1;
            grid.push(SeatCell::Floor);
            for c in line.chars() {
                grid.push(SeatCell::from_char(c));
            }
            grid.push(SeatCell::Floor);
        }

        // add a padding row below the grid
        for _ in 0..padded_grid_width {
            grid.push(SeatCell::Floor);
        }

        SeatGrid {
            grid,
            row_count: line_count,
            col_count: unpadded_grid_width,
        }
    }

    fn get_padded_grid_index(&self, row: usize, col: usize) -> usize {
        let (padded_row, padded_col) = (row + 1, col + 1);
        padded_row * (self.col_count + 2) + padded_col
    }

    fn format_grid_as_str(&self) -> String {
        let mut char_vec = Vec::new();
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                let padded_grid_index = self.get_padded_grid_index(row, col);
                char_vec.push(self.grid[padded_grid_index].to_ascii_char());
            }
            char_vec.push('\n');
        }
        String::from_iter(char_vec.iter())
    }
}

fn main() {
    let input_file = input_helpers::get_input_file_from_args(&mut std::env::args());
    let seat_grid = SeatGrid::from_file(&input_file);
    println!("{}", &seat_grid.format_grid_as_str());
}
