#[macro_use]
extern crate itertools;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CubeState {
    Inactive,
    Active,
}

// TODO: additionally, I wonder if it's faster to similar using a hashmap. I would imagine not, but I wonder if for
// sufficiently large but sparse spaces (i.e. the infinite void of the pocket dimension), if we can save some serious
// time only iterating as far as the outermost shell.
// TODO: I wonder if it's faster to make this more space compact and make this bitaddressible rather than byte addressable
// TODO: the space concerns become more interesting as pt2 requires my implementation from 3d to 4d
struct Cpd4d {
    // Conway Pocket Dimension
    width: usize,      // addressable size of x-axis
    height: usize,     // addressable size of y-axis
    depth: usize,      // addressable size of z-axis
    hyper_size: usize, // addressable size of w-axis
    grid: Vec<CubeState>,
    grid_buffer: Vec<CubeState>,
}

struct Cpd3d {
    cpd: Cpd4d, // we are effectively just providing a 3d interface to the 4d implementation
}

type CellIterator4d = itertools::ConsTuples<
    itertools::Product<
        itertools::ConsTuples<
            itertools::Product<
                itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>>,
                std::ops::Range<usize>,
            >,
            ((usize, usize), usize),
        >,
        std::ops::Range<usize>,
    >,
    ((usize, usize, usize), usize),
>;

type CellIterator3d =
    std::iter::Map<CellIterator4d, fn((usize, usize, usize, usize)) -> (usize, usize, usize)>;

#[derive(Debug, PartialEq, Eq)]
struct SeedGrid {
    width: usize,  // addressable size of x-axis
    height: usize, // addressable size of y-axis
    grid: Vec<CubeState>,
}

impl Cpd4d {
    fn new_3d(seed_grid: &SeedGrid, max_simulations: usize) -> Self {
        // each simulation could cause a new neighbor to pop into existence one layer outside
        // of our limit. Therefore we need to account an additional 2 units in each dimension
        // of our grid (2 units because a neighbor could pop up on either the positive or
        // negative side). We'll also add an additional layer so that we don't have to special
        // case any of the neighbor checks at the edge of the cube space.
        let max_grid_growth = (max_simulations + 1) * 2;
        let width = seed_grid.width + max_grid_growth;
        let height = seed_grid.height + max_grid_growth;
        let depth = 1 + max_grid_growth;
        let hyper_size = 3;

        let grid_buffer = vec![CubeState::Inactive; width * height * depth * hyper_size];
        let mut cpd = Self {
            width,
            height,
            depth,
            hyper_size,
            grid: grid_buffer.clone(),
            grid_buffer: grid_buffer,
        };

        let addr_offset = max_simulations + 1;
        for row in 0..seed_grid.height {
            for col in 0..seed_grid.width {
                cpd.queue_set(
                    row + addr_offset,
                    col + addr_offset,
                    addr_offset,
                    1,
                    seed_grid.get(row, col),
                );
            }
        }
        cpd.commit_sets();

        cpd
    }

    fn new(seed_grid: &SeedGrid, max_simulations: usize) -> Self {
        // each simulation could cause a new neighbor to pop into existence one layer outside
        // of our limit. Therefore we need to account an additional 2 units in each dimension
        // of our grid (2 units because a neighbor could pop up on either the positive or
        // negative side). We'll also add an additional layer so that we don't have to special
        // case any of the neighbor checks at the edge of the cube space.
        let max_grid_growth = (max_simulations + 1) * 2;
        let width = seed_grid.width + max_grid_growth;
        let height = seed_grid.height + max_grid_growth;
        let depth = 1 + max_grid_growth;
        let hyper_size = 1 + max_grid_growth;

        let grid_buffer = vec![CubeState::Inactive; width * height * depth * hyper_size];
        let mut cpd = Self {
            width,
            height,
            depth,
            hyper_size,
            grid: grid_buffer.clone(),
            grid_buffer: grid_buffer,
        };

        let addr_offset = max_simulations + 1;
        for row in 0..seed_grid.height {
            for col in 0..seed_grid.width {
                cpd.queue_set(
                    row + addr_offset,
                    col + addr_offset,
                    addr_offset,
                    addr_offset,
                    seed_grid.get(row, col),
                );
            }
        }
        cpd.commit_sets();

        cpd
    }

    fn get_cell_index(&self, row: usize, col: usize, layer: usize, hyper_layer: usize) -> usize {
        let hyper_layer_offset = hyper_layer * self.width * self.height * self.depth;
        let layer_offset = layer * self.width * self.height;
        let row_offset = row * self.width;
        hyper_layer_offset + layer_offset + row_offset + col
    }

    fn queue_set(
        &mut self,
        row: usize,
        col: usize,
        layer: usize,
        hyper_layer: usize,
        v: CubeState,
    ) {
        // assert we aren't trying to set anything in the outer shell of the cube which only exists to avoid special neighbor checks
        assert!(row > 0 && row < self.height - 1);
        assert!(col > 0 && col < self.width - 1);
        assert!(layer > 0 && layer < self.depth - 1);
        assert!(hyper_layer > 0 && hyper_layer < self.hyper_size - 1);

        let idx = self.get_cell_index(row, col, layer, hyper_layer);
        self.grid_buffer[idx] = v;
    }

    fn commit_sets(&mut self) {
        self.grid.copy_from_slice(&self.grid_buffer);
    }

    fn get(&self, row: usize, col: usize, layer: usize, hyper_layer: usize) -> CubeState {
        let idx = self.get_cell_index(row, col, layer, hyper_layer);
        self.grid[idx]
    }

    fn each_cell(&self) -> CellIterator4d {
        // we iterate starting at 1 and ending 1 before the width/height/depth because there's buffer shell
        // around the cube that should never be touched and is only intended for avoiding extra neighbor checks
        iproduct!(
            1..self.height - 1,
            1..self.width - 1,
            1..self.depth - 1,
            1..self.hyper_size - 1
        )
    }

    fn get_active_neighbor_count(
        &self,
        row: usize,
        col: usize,
        layer: usize,
        hyper_layer: usize,
    ) -> usize {
        let row_diffs = [row - 1, row, row + 1];
        let col_diffs = [col - 1, col, col + 1];
        let layer_diffs = [layer - 1, layer, layer + 1];
        let hyper_layer_diffs = [hyper_layer - 1, hyper_layer, hyper_layer + 1];
        let neighbors = iproduct!(&row_diffs, &col_diffs, &layer_diffs, &hyper_layer_diffs)
            .filter(|coord| *coord != (&row, &col, &layer, &hyper_layer));

        neighbors
            .filter(|(r, c, l, h)| self.get(**r, **c, **l, **h) == CubeState::Active)
            .count()
    }

    fn get_active_cell_count(&self) -> usize {
        self.each_cell()
            .filter(|(r, c, l, h)| self.get(*r, *c, *l, *h) == CubeState::Active)
            .count()
    }

    fn simulate(&mut self) {
        for cell in self.each_cell() {
            let cell_state = self.get(cell.0, cell.1, cell.2, cell.3);
            let cell_active_neighbor_count =
                self.get_active_neighbor_count(cell.0, cell.1, cell.2, cell.3);
            let new_state = match (cell_state, cell_active_neighbor_count) {
                (CubeState::Active, 2) => CubeState::Active,
                (CubeState::Active, 3) => CubeState::Active,
                (CubeState::Active, _) => CubeState::Inactive,
                (CubeState::Inactive, 3) => CubeState::Active,
                (CubeState::Inactive, _) => CubeState::Inactive,
            };
            self.queue_set(cell.0, cell.1, cell.2, cell.3, new_state);
        }

        self.commit_sets();
    }
}

impl Cpd3d {
    fn new(seed_grid: &SeedGrid, max_simulations: usize) -> Self {
        Self {
            cpd: Cpd4d::new_3d(seed_grid, max_simulations),
        }
    }

    fn get(&self, row: usize, col: usize, layer: usize) -> CubeState {
        self.cpd.get(row, col, layer, 1)
    }

    fn each_cell(&self) -> CellIterator3d {
        fn drop_4d((r, c, l, _): (usize, usize, usize, usize)) -> (usize, usize, usize) {
            (r, c, l)
        }

        self.cpd.each_cell().map(drop_4d)
    }

    fn get_active_neighbor_count(&self, row: usize, col: usize, layer: usize) -> usize {
        let row_diffs = [row - 1, row, row + 1];
        let col_diffs = [col - 1, col, col + 1];
        let layer_diffs = [layer - 1, layer, layer + 1];
        let neighbors = iproduct!(&row_diffs, &col_diffs, &layer_diffs)
            .filter(|coord| *coord != (&row, &col, &layer));

        neighbors
            .filter(|(r, c, l)| self.get(**r, **c, **l) == CubeState::Active)
            .count()
    }

    fn get_active_cell_count(&self) -> usize {
        self.cpd.get_active_cell_count()
    }

    fn simulate(&mut self) {
        self.cpd.simulate();
    }
}

impl SeedGrid {
    fn new(grid: &[Vec<CubeState>]) -> Self {
        let mut flat_grid = Vec::new();
        flat_grid.extend(grid.iter().flatten());
        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid: flat_grid,
        }
    }

    fn from_file(file_name: &str) -> Self {
        let mut grid_rows = Vec::new();
        for line in input_helpers::read_lines(file_name) {
            let grid_cols = line
                .chars()
                .map(|c| match c {
                    '.' => CubeState::Inactive,
                    '#' => CubeState::Active,
                    _ => panic!("Invalid char! {}", c),
                })
                .collect();
            grid_rows.push(grid_cols);
        }

        SeedGrid::new(&grid_rows)
    }

    fn get(&self, row: usize, col: usize) -> CubeState {
        self.grid[row * self.width + col]
    }
}

fn main() {
    let file_name = input_helpers::get_input_file_from_args(&mut std::env::args());
    let seed = SeedGrid::from_file(&file_name);

    let simulation_count = 6;
    let mut cpd = Cpd4d::new(&seed, simulation_count);
    for _ in 0..simulation_count {
        cpd.simulate();
    }

    println!(
        "{} active cells after {} simulations.",
        cpd.get_active_cell_count(),
        simulation_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_3d_setup_test() {
        let seed = SeedGrid::new(&[vec![CubeState::Active; 1]]);
        let cpd = Cpd3d::new(&seed, 1);

        let seed_cell = (2, 2, 2);
        assert_eq!(
            cpd.get(seed_cell.0, seed_cell.1, seed_cell.2),
            CubeState::Active
        );

        for (row, col, layer) in cpd.each_cell() {
            let expected_state = match (row, col, layer) {
                (2, 2, 2) => CubeState::Active,
                _ => CubeState::Inactive,
            };

            assert_eq!(cpd.get(row, col, layer), expected_state);
        }

        assert_eq!(cpd.get_active_cell_count(), 1);
    }

    #[test]
    fn basic_3d_simulation_test() {
        let seed = SeedGrid::new(&[
            vec![CubeState::Inactive, CubeState::Active, CubeState::Inactive],
            vec![CubeState::Inactive, CubeState::Inactive, CubeState::Active],
            vec![CubeState::Active, CubeState::Active, CubeState::Active],
        ]);

        let simulation_count = 6;
        let mut cpd = Cpd3d::new(&seed, simulation_count);
        assert_eq!(cpd.get_active_cell_count(), 5);

        for _ in 0..simulation_count {
            cpd.simulate();
        }

        assert_eq!(cpd.get_active_cell_count(), 112);
    }

    #[test]
    fn build_seed_from_file_test() {
        let seed = SeedGrid::from_file("src/simple_input.txt");
        let expected_seed = SeedGrid::new(&[
            vec![CubeState::Inactive, CubeState::Active, CubeState::Inactive],
            vec![CubeState::Inactive, CubeState::Inactive, CubeState::Active],
            vec![CubeState::Active, CubeState::Active, CubeState::Active],
        ]);
        assert_eq!(seed, expected_seed);
    }
}
