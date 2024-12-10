use input_helpers;
use itertools::Itertools;
use std::process::ExitCode;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct GridPos {
    row: isize,
    col: isize,
}

struct Grid<T>
where
    T: Clone + Copy,
{
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone + Copy,
{
    fn get_cell_idx(&self, row: isize, col: isize) -> usize {
        assert!(!self.is_pos_out_of_bounds(row, col));

        (row as usize * self.width) + (col as usize)
    }

    fn get_cell(&self, row: isize, col: isize) -> T {
        self.cells[self.get_cell_idx(row, col)]
    }

    fn get_cell_mut(&mut self, row: isize, col: isize) -> &mut T {
        let idx = self.get_cell_idx(row, col);
        &mut self.cells[idx]
    }

    fn cell_pos_from_idx(width: usize, height: usize, idx: usize) -> GridPos {
        assert!(idx < (width * height));
        let col = (idx % width) as isize;
        let row = (idx / width) as isize;
        GridPos { row, col }
    }

    fn is_pos_in_bounds(&self, row: isize, col: isize) -> bool {
        !self.is_pos_out_of_bounds(row, col)
    }

    fn is_pos_out_of_bounds(&self, row: isize, col: isize) -> bool {
        row < 0 || col < 0 || row as usize >= self.height || col as usize >= self.width
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct HeightIndex {
    val: u8,
}

impl HeightIndex {
    fn is_climbable(&self, other: &Self) -> bool {
        self.val + 1 == other.val
    }

    fn is_trailhead(&self) -> bool {
        self.val == 0
    }

    fn is_trailend(&self) -> bool {
        self.val == 9
    }
}

fn dump_trail_map(trail_map: &TopographicTrailMap) {
    for r in 0..(trail_map.height as isize) {
        for c in 0..(trail_map.width as isize) {
            let height_index_char = (trail_map.get_cell(r, c).val + ('0' as u8)) as char;
            print!("{}", height_index_char);
        }
        println!("");
    }
}

type TopographicTrailMap = Grid<HeightIndex>;

fn read_topographic_trail_map(filename: &str) -> Result<TopographicTrailMap, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() == 0 {
        return Ok(Grid{width: 0, height: 0, cells: vec![]});
    }

    let height = lines.len();
    let width = lines[0].len();

    let mut cells: Vec<HeightIndex> = vec![];
    for line in lines {
        if line.len() != width {
            return Err(format!(
                "Grid must have consistent line widths! Expected {} found {}",
                width,
                line.len()
            ));
        }

        for c in line.chars() {
            let cell = match c {
                '0'..='9' => {
                    let u8_height_val: u8 = (c as u8) - ('0' as u8);
                    HeightIndex{val: u8_height_val}
                }
                _ => return Err(format!("Invalid frequency tower grid char! {}", c)),
            };
            cells.push(cell);
        }
    }

    Ok(Grid {width, height, cells})
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_topographic_trail_map(filename);
    let trail_map = match parse_result {
        Ok(trail_map) => trail_map,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    dump_trail_map(&trail_map);

    /*
    {
        let antinode_positions_pt1 = calculate_all_antinode_positions_pt1(&tower_grid);
        println!("Pt 1: antinode position count = {}", antinode_positions_pt1.len());
        if antinode_positions_pt1.len() < 10 {
            for p in antinode_positions_pt1 {
                println!("- (r:{},c:{})", p.row, p.col);
            }
        }
    } */

    /*
    println!("");

    {
        let antinode_positions_pt2 = calculate_all_antinode_positions_pt2(&tower_grid);
        println!("Pt 2: antinode position count = {}", antinode_positions_pt2.len());
        if antinode_positions_pt2.len() < 10 {
            for p in antinode_positions_pt2 {
                println!("- (r:{},c:{})", p.row, p.col);
            }
        }
    } */

    return ExitCode::SUCCESS;
}
