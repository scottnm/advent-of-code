extern crate input_helpers;
use std::iter::FromIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn get_update(&self, occupied_seat_count: usize) -> Self {
        match self {
            SeatCell::Floor => SeatCell::Floor,
            SeatCell::Free => {
                if occupied_seat_count == 0 {
                    SeatCell::Occupied
                } else {
                    SeatCell::Free
                }
            }
            SeatCell::Occupied => {
                if occupied_seat_count >= 5 {
                    SeatCell::Free
                } else {
                    SeatCell::Occupied
                }
            }
        }
    }
}

struct SeatGrid {
    grid: Vec<SeatCell>,
    grid_buffer: Vec<SeatCell>,
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
            grid: grid.clone(),
            grid_buffer: grid,
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
        for row_index in 0..self.row_count {
            let row_begin = self.get_padded_grid_index(row_index, 0);
            let row_end = self.get_padded_grid_index(row_index, self.col_count);
            let row_slice = &self.grid[row_begin..row_end];
            for cell in row_slice {
                char_vec.push(cell.to_ascii_char());
            }
            char_vec.push('\n');
        }
        String::from_iter(char_vec.iter())
    }

    fn search_for_seat<I1, I2>(&self, row_iter: I1, col_iter: I2) -> SeatCell
    where
        I1: Iterator<Item = usize>,
        I2: Iterator<Item = usize>,
    {
        for (r, c) in row_iter.zip(col_iter) {
            match self.grid[self.get_padded_grid_index(r, c)] {
                SeatCell::Floor => (),
                cell @ SeatCell::Occupied | cell @ SeatCell::Free => return cell,
            }
        }
        SeatCell::Floor
    }

    fn get_visible_occupied_seat_count(&self, row: usize, col: usize) -> usize {
        let row_down_iter = row + 1..self.row_count;
        let row_same_iter = std::iter::repeat(row);
        let row_up_iter = (0..row).rev();
        let col_left_iter = (0..col).rev();
        let col_same_iter = std::iter::repeat(col);
        let col_right_iter = col + 1..self.col_count;

        let neighbor_searches = [
            self.search_for_seat(row_down_iter.clone(), col_same_iter.clone()), // down
            self.search_for_seat(row_down_iter.clone(), col_left_iter.clone()), // down-left
            self.search_for_seat(row_same_iter.clone(), col_left_iter.clone()), // left
            self.search_for_seat(row_up_iter.clone(), col_left_iter.clone()),   // up-left
            self.search_for_seat(row_up_iter.clone(), col_same_iter.clone()),   // up
            self.search_for_seat(row_up_iter.clone(), col_right_iter.clone()),  // up-right
            self.search_for_seat(row_same_iter.clone(), col_right_iter.clone()), // right
            self.search_for_seat(row_down_iter.clone(), col_right_iter.clone()), // down-right
        ];

        neighbor_searches
            .iter()
            .filter(|neighbor| **neighbor == SeatCell::Occupied)
            .count()
    }

    fn simulate(&mut self) -> bool {
        debug_assert_eq!(self.grid, self.grid_buffer);

        let mut updated = false;
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                let cell_index = self.get_padded_grid_index(row, col);
                let occupied_seat_count = self.get_visible_occupied_seat_count(row, col);
                // Read the current cell from the frozen grid
                let current_cell = self.grid[cell_index];
                let updated_cell = current_cell.get_update(occupied_seat_count);

                // Write the updated cell to the grid buffer
                self.grid_buffer[cell_index] = updated_cell;
                updated = updated || (updated_cell != current_cell);
            }
        }

        // Refresh the frozen grid with the complete state from the grid buffer
        self.grid.copy_from_slice(&self.grid_buffer);
        !updated
    }

    fn get_occupied_seat_count(&self) -> usize {
        debug_assert_eq!(self.grid, self.grid_buffer);
        self.grid
            .iter()
            .filter(|seat| **seat == SeatCell::Occupied)
            .count()
    }
}

fn main() {
    let input_file = input_helpers::get_input_file_from_args(&mut std::env::args());
    let mut seat_grid = SeatGrid::from_file(&input_file);
    loop {
        println!("{}", &seat_grid.format_grid_as_str());
        let done = seat_grid.simulate();

        if done {
            break;
        }
    }

    let occupied_seat_count = seat_grid.get_occupied_seat_count();
    println!("Occupied: {}", occupied_seat_count);
}
