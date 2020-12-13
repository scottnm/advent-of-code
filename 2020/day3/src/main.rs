#[derive(PartialEq)]
enum MapCell {
    Free,
    Tree,
}

struct TobagganMap {
    map: Vec<Vec<MapCell>>,
}

fn get_map_from_input(_file_name: &str) -> TobagganMap {
    // input_helpers::read_lines("src/simple_input.txt");
    let mut rows = Vec::new();
    rows.push(vec![
        MapCell::Free,
        MapCell::Free,
        MapCell::Free,
        MapCell::Free,
    ]);
    rows.push(vec![
        MapCell::Free,
        MapCell::Tree,
        MapCell::Free,
        MapCell::Free,
    ]);
    rows.push(vec![
        MapCell::Free,
        MapCell::Free,
        MapCell::Free,
        MapCell::Free,
    ]);
    rows.push(vec![
        MapCell::Free,
        MapCell::Free,
        MapCell::Free,
        MapCell::Tree,
    ]);

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
    let map = get_map_from_input("src/simple_input.txt");
    let tree_hits = map.calculate_tree_hits_from_slope(1);
    println!("Hit {} trees!", tree_hits.len());
}
