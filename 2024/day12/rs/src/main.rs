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
struct GardenPlot {
    plant_type: char,
}

struct GardenRegion {
    plant_type: char,
    plot_positions: Vec<GridPos>,
}

fn dump_garden(garden: &Grid<GardenPlot>) {
    for r in 0..(garden.height as isize) {
        for c in 0..(garden.width as isize) {
            print!("{}", garden.get_cell(r, c).plant_type);
        }
        println!("");
    }
}

fn split_regions(garden: &Grid<GardenPlot>) -> Vec<GardenRegion> {
    unimplemented!();
}

fn calculate_region_area(garden: &Grid<GardenPlot>, region: &GardenRegion) -> usize {
    unimplemented!();
}

fn calculate_region_perimeter(garden: &Grid<GardenPlot>, region: &GardenRegion) -> usize {
    unimplemented!();
}

fn read_garden_map(filename: &str) -> Result<Grid<GardenPlot>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() == 0 {
        return Ok(Grid {
            width: 0,
            height: 0,
            cells: vec![],
        });
    }

    let height = lines.len();
    let width = lines[0].len();

    let mut cells: Vec<GardenPlot> = vec![];
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
                'A'..='Z' => GardenPlot {plant_type: c },
                _ => return Err(format!("Invalid garden plot char! {}", c)),
            };
            cells.push(cell);
        }
    }

    Ok(Grid {
        width,
        height,
        cells,
    })
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_garden_map(filename);
    let garden = match parse_result {
        Ok(garden) => garden,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    dump_garden(&garden);

    {
        let regions = split_regions(&garden);
        let fence_prices: Vec<usize> = regions
            .iter()
            .map(|region| calculate_region_area(&garden, region) * calculate_region_perimeter(&garden, region))
            .collect(); 
        let total_fence_price: usize = fence_prices.iter().sum();
        println!("Pt 1: total fence price = {}", total_fence_price);
        if regions.len() < 20 {
            for (i, (region, price)) in regions.iter().zip(fence_prices.iter()).enumerate() {
                println!(" {:02}. {} ${}: {:?}", i, region.plant_type, price, region.plot_positions);
            }
        }
    }

    println!("");

    /*
    {
        let trails = find_all_trails_pt2(&trail_map);
        let trailhead_ratings: Vec<usize> = trails
            .iter()
            .map(|(_trail_start, trail_ends)| trail_ends.len())
            .collect();
        let trailhead_rating_sum: usize = trailhead_ratings.iter().sum();
        println!("Pt 2: trailhead_rating_sum = {}", trailhead_rating_sum);
        if trails.len() < 20 {
            for trail in &trails {
                println!("- start={}; trail={:?}", trail.0, trail.1);
            }
        }
    } */

    return ExitCode::SUCCESS;
}
