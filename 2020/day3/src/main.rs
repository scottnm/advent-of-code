use std::convert::TryFrom;

#[derive(PartialEq)]
enum MapCell {
    Free,
    Tree,
}

impl std::convert::TryFrom<char> for MapCell {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(MapCell::Free),
            '#' => Ok(MapCell::Tree),
            _ => Err(std::format!("Invalid map character! {}", c)),
        }
    }
}

struct TobagganMap {
    map: Vec<Vec<MapCell>>,
}

fn get_map_from_input(file_name: &str) -> TobagganMap {
    let mut rows = Vec::new();

    let parse_map_row_from_line = |line: &String| {
        let mut row = Vec::new();
        for c in line.chars() {
            assert!(c.is_ascii());
            row.push(MapCell::try_from(c).unwrap());
        }
        row
    };

    for next_line in input_helpers::read_lines(file_name) {
        match next_line {
            Ok(line) => rows.push(parse_map_row_from_line(&line)),
            Err(line_err) => println!("Bad line! {}", line_err),
        }
    }

    TobagganMap { map: rows }
}

impl TobagganMap {
    fn calculate_tree_hits_from_slope(&self, x_slope: usize) -> Vec<(usize, usize)> {
        let num_rows = self.map.len();
        let num_cols = self.map[0].len();

        let calculate_coord_from_slope_and_row = |row| {
            let col = (row * x_slope) % num_cols;
            (row, col)
        };

        // for every row, calculate each (row, col) where the tobaggan will travel
        let possible_hits = (0..num_rows).map(calculate_coord_from_slope_and_row);

        // iterate over each cell traveled and return a hit if there was a tree there
        let hits = possible_hits.filter(|(row, col)| self.map[*row][*col] == MapCell::Tree);
        hits.collect()
    }
}

fn main() {
    let map = get_map_from_input("src/input.txt");
    let tree_hits = map.calculate_tree_hits_from_slope(3);
    println!("Hit {} trees!", tree_hits.len());
}
