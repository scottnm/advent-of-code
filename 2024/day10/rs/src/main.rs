use input_helpers;
use itertools::Itertools;
use std::process::ExitCode;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct GridPos {
    row: isize,
    col: isize,
}

impl std::fmt::Display for GridPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r:{}, c:{})", self.row, self.col)
    }
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
    fn can_climb_to(&self, other: &Self) -> bool {
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

fn find_trails_pt1(trail_map: &TopographicTrailMap, start_pos: &GridPos) -> Vec<GridPos> {
    let mut trailends = std::collections::HashSet::<GridPos>::new();

    fn get_neighbor_at_offset(
        trail_map: &TopographicTrailMap, 
        pos: &GridPos,
        row_offset: isize, 
        col_offset: isize) -> Option<(GridPos, HeightIndex)> {

        let neighbor_pos = GridPos { row: pos.row + row_offset, col: pos.col + col_offset };
        if trail_map.is_pos_out_of_bounds(neighbor_pos.row, neighbor_pos.col) {
            None
        } else {
            // FIXME: this could probably be baked directly into the grid helper as some get_cell variant
            Some((neighbor_pos, trail_map.get_cell(neighbor_pos.row, neighbor_pos.col)))
        }
    }

    fn find_trails_rec_helper(
        trail_map: &TopographicTrailMap, 
        curr_pos: &GridPos, 
        trailends: &mut std::collections::HashSet<GridPos>) {
        
        let curr_trail_cell = trail_map.get_cell(curr_pos.row, curr_pos.col);
        if curr_trail_cell.is_trailend() {
            trailends.insert(curr_pos.clone());
            return;
        }

        // try up
        if let Some((up_neighbor_pos, up_neighbor)) = get_neighbor_at_offset(trail_map, curr_pos, -1, 0) {
            if curr_trail_cell.can_climb_to(&up_neighbor) {
                find_trails_rec_helper(trail_map, &up_neighbor_pos, trailends);
            }
        }

        if let Some((down_neighbor_pos, down_neighbor)) = get_neighbor_at_offset(trail_map, curr_pos, 1, 0) {
            if curr_trail_cell.can_climb_to(&down_neighbor) {
                find_trails_rec_helper(trail_map, &down_neighbor_pos, trailends);
            }
        }
        
        if let Some((left_neighbor_pos, left_neighbor)) = get_neighbor_at_offset(trail_map, curr_pos, 0, -1) {
            if curr_trail_cell.can_climb_to(&left_neighbor) {
                find_trails_rec_helper(trail_map, &left_neighbor_pos, trailends);
            }
        }

        if let Some((right_neighbor_pos, right_neighbor)) = get_neighbor_at_offset(trail_map, curr_pos, 0, 1) {
            if curr_trail_cell.can_climb_to(&right_neighbor) {
                find_trails_rec_helper(trail_map, &right_neighbor_pos, trailends);
            }
        }
    }

    find_trails_rec_helper(trail_map, start_pos, &mut trailends);

    Vec::from_iter(trailends)
}

fn find_all_trails_pt1(trail_map: &TopographicTrailMap) -> std::collections::HashMap<GridPos, Vec<GridPos>> {
    let mut trails = std::collections::HashMap::<GridPos, Vec<GridPos>>::new();
    for r in 0..(trail_map.height as isize) {
        for c in 0..(trail_map.width as isize) {
            if trail_map.get_cell(r, c).is_trailhead() {
                let trailhead_pos = GridPos { row: r, col: c };
                trails.insert(trailhead_pos, find_trails_pt1(trail_map, &trailhead_pos));
            }
        }
    }

    trails
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

    {
        let trails = find_all_trails_pt1(&trail_map);
        let trailhead_scores: Vec<usize> = trails.iter().map(|(_trail_start, trail_ends)| trail_ends.len()).collect();
        let trailhead_score_sum: usize = trailhead_scores.iter().sum();
        println!("Pt 1: trailhead_score_sum = {}", trailhead_score_sum);
        if trails.len() < 20 {
            for trail in &trails {
                println!("- start={}; trail={:?}", trail.0, trail.1);
            }
        }
    }

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
