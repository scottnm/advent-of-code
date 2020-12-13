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
    fn calculate_tree_hits_from_slope(
        &self,
        x_slope: usize,
        y_slope: usize,
    ) -> Vec<(usize, usize)> {
        let num_rows = self.map.len();
        let num_cols = self.map[0].len();

        let calculate_coord_from_slope_and_row = |row| {
            let col = (row * x_slope / y_slope) % num_cols;
            (row, col)
        };

        // for every row, calculate each (row, col) where the tobaggan will travel
        let row_steps = (0..num_rows).step_by(y_slope);
        let possible_hits = row_steps.map(calculate_coord_from_slope_and_row);

        // iterate over each cell traveled and return a hit if there was a tree there
        let hits = possible_hits.filter(|(row, col)| self.map[*row][*col] == MapCell::Tree);
        hits.collect()
    }
}

fn main() {
    let treemap = get_map_from_input("src/input.txt");

    let slopes_to_test = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product_of_tree_hits = {
        let mut product = 1;
        for (x_slope, y_slope) in slopes_to_test.iter() {
            let tree_hits = treemap.calculate_tree_hits_from_slope(*x_slope, *y_slope);
            product *= tree_hits.len();
        }
        product
    };
    println!("Product = {}", product_of_tree_hits);
}
