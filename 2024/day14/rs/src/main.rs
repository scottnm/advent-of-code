use input_helpers;
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

fn dump_grid(garden: &Grid<()>) {
    for r in 0..(garden.height as isize) {
        for c in 0..(garden.width as isize) {
            //print!("{}", garden.get_cell(r, c).plant_type);
        }
        //println!("");
    }
}

fn read_(filename: &str) -> Result<Grid<()>, String> {
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

    let mut cells: Vec<()> = vec![];
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
                'A'..='Z' => (),
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

    let parse_result = read_(filename);
    let garden = match parse_result {
        Ok(garden) => garden,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    dump_grid(&garden);

    /*
    {
        let regions = split_regions(&garden);
        let mut total_fence_price = 0;
        let print_region_info = regions.len() < 20;
        for (i, region) in regions.iter().enumerate() {
            let area = calculate_region_area(&garden, region);
            let perimeter = calculate_region_perimeter(&garden, region);
            let price = area * perimeter;
            total_fence_price += price;
            if print_region_info {
                println!(
                    " {:02}. {} ${} = {}(area) x {}(peri)  ::  {:?}",
                    i, region.plant_type, price, area, perimeter, region.plot_positions
                );
            }
        }

        println!("Pt 1: total fence price = {}", total_fence_price);
    } */

    println!("");

    /*{
        let regions = split_regions(&garden);
        let mut total_fence_price = 0;
        let print_region_info = regions.len() < 20;
        for (i, region) in regions.iter().enumerate() {
            let area = calculate_region_area(&garden, region);
            let side_count = count_region_sides(&garden, region);
            let price = area * side_count;
            total_fence_price += price;
            if print_region_info {
                println!(
                    " {:02}. {} ${} = {}(area) x {}(sides)  ::  {:?}",
                    i, region.plant_type, price, area, side_count, region.plot_positions
                );
            }
        }

        println!("Pt 2: total fence price = {}", total_fence_price);
    }*/

    return ExitCode::SUCCESS;
}
