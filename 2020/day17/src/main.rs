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
struct Cpd {
    // Conway Pocket Dimension
    width: usize,  // addressable size of x-axis
    height: usize, // addressable size of y-axis
    depth: usize,  // addressable size of z-axis
    grid: Vec<CubeState>,
    grid_buffer: Vec<CubeState>,
}

type CellIterator = itertools::ConsTuples<
    itertools::Product<
        itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>>,
        std::ops::Range<usize>,
    >,
    ((usize, usize), usize),
>;

struct SeedGrid {
    width: usize,  // addressable size of x-axis
    height: usize, // addressable size of y-axis
    grid: Vec<CubeState>,
}

impl Cpd {
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

        let grid_buffer = vec![CubeState::Inactive; width * height * depth];
        let mut cpd = Self {
            width,
            height,
            depth,
            grid: grid_buffer.clone(),
            grid_buffer: grid_buffer,
        };

        let addr_offset = max_simulations + 1;
        for row in 0..seed_grid.height {
            for col in 0..seed_grid.width {
                cpd.set(
                    row + addr_offset,
                    col + addr_offset,
                    addr_offset,
                    seed_grid.get(row, col),
                );
            }
        }

        cpd
    }

    fn get_cell_index(&self, row: usize, col: usize, layer: usize) -> usize {
        let layer_offset = layer * self.width * self.height;
        let row_offset = row * self.width;
        layer_offset + row_offset + col
    }

    fn set(&mut self, row: usize, col: usize, layer: usize, v: CubeState) {
        // assert we aren't trying to set anything in the outer shell of the cube which only exists to avoid special neighbor checks
        assert!(row > 0 && row < self.height - 1);
        assert!(col > 0 && col < self.width - 1);
        assert!(layer > 0 && layer < self.depth - 1);

        let idx = self.get_cell_index(row, col, layer);
        self.grid[idx] = v;
    }

    fn get(&self, row: usize, col: usize, layer: usize) -> CubeState {
        let idx = self.get_cell_index(row, col, layer);
        self.grid[idx]
    }

    fn each_cell(&self) -> CellIterator {
        // we iterate starting at 1 and ending 1 before the width/height/depth because there's buffer shell
        // around the cube that should never be touched and is only intended for avoiding extra neighbor checks
        iproduct!(1..self.height - 1, 1..self.width - 1, 1..self.depth - 1)
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

    fn get(&self, row: usize, col: usize) -> CubeState {
        self.grid[row * self.width + col]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_setup_test() {
        let seed = SeedGrid::new(&[vec![CubeState::Active; 1]]);
        let cpd = Cpd::new(&seed, 1);

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
    }
}
