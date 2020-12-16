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

    fn get_update(&self, occupied_neighbor_count: usize) -> Self {
        match self {
            SeatCell::Floor => SeatCell::Floor,
            SeatCell::Free => {
                if occupied_neighbor_count == 0 {
                    SeatCell::Occupied
                } else {
                    SeatCell::Free
                }
            }
            SeatCell::Occupied => {
                if occupied_neighbor_count >= 4 {
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

    fn get_occupied_neighbor_count(&self, padded_cell_index: usize) -> usize {
        let padded_row_width = self.col_count + 2;
        let neighbor_indices = [
            padded_cell_index - padded_row_width - 1, // top-left
            padded_cell_index - padded_row_width,     // top-mid
            padded_cell_index - padded_row_width + 1, // top-right
            padded_cell_index - 1,                    // mid-left
            padded_cell_index + 1,                    // mid-right
            padded_cell_index + padded_row_width - 1, // bottom-left
            padded_cell_index + padded_row_width,     // bottom-mid
            padded_cell_index + padded_row_width + 1, // bottom-right
        ];

        neighbor_indices
            .iter()
            .filter(|i| self.grid[**i] == SeatCell::Occupied)
            .count()
    }

    fn simulate(&mut self) -> bool {
        debug_assert_eq!(self.grid, self.grid_buffer);

        let mut updated = false;
        for row_index in 0..self.row_count {
            let row_begin = self.get_padded_grid_index(row_index, 0);
            let row_end = self.get_padded_grid_index(row_index, self.col_count);
            for cell_index in row_begin..row_end {
                let occupied_neighbor_count = self.get_occupied_neighbor_count(cell_index);
                // Read the current cell from the frozen grid
                let current_cell = self.grid[cell_index];
                let updated_cell = current_cell.get_update(occupied_neighbor_count);

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
