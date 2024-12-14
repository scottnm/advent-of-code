use input_helpers;
use std::{process::ExitCode, string};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{},y:{})", self.x, self.y)
    }
}


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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Robot {
    pos: Vec2,    
    vel: Vec2,
}

struct RobotArea {
    width: usize,
    height: usize,
}

fn dump_grid(title: &str, robot_area: &RobotArea, robots: &[Robot]) {
    println!("{}: ", title);
    let mut string_buf = String::with_capacity(robot_area.width);
    for r in 0..robot_area.height as isize {
        string_buf.clear();
        for c in 0..robot_area.width as isize {
            let pos = Vec2 {x: c, y: r};
            let robots_in_pos = robots.iter().filter(|r| r.pos == pos).count();
            let cell_char = match robots_in_pos {
                0 => '.',
                1..=9 => (('0' as usize) + robots_in_pos) as u8 as char, 
                10..=35=> (('A' as usize) + (robots_in_pos - 10)) as u8 as char,
                36..=61=> (('a' as usize) + (robots_in_pos - 36)) as u8 as char,
                _ => panic!("not enough chars to represent {}", robots_in_pos),
            };
            string_buf.push(cell_char);
        }
        println!("  {}", string_buf);
    }
}

fn read_robots(filename: &str) -> Result<(RobotArea, Vec<Robot>), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.is_empty() {
        return Err(format!("Need at least one line for width/height of robot area"));
    }

    let area_line = &lines[0];
    let mut area_line_split = area_line.split_ascii_whitespace();

    let width: usize = area_line_split
        .next()
        .unwrap()
        .parse()
        .map_err(|_| String::from("Failed to parse width"))?;

    let height: usize = area_line_split
        .next()
        .ok_or(String::from("Missing height value on first list"))?
        .parse()
        .map_err(|_| String::from("Failed to parse width"))?;

    let robot_area = RobotArea {width, height};

    let robot_line_re = regex::Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    
    let mut robots = vec![];

    for line in &lines[1..] {
        let captures = robot_line_re.captures(line);
        let robot_match = captures.ok_or(format!("line '{}' did not match expected robot format", line))?;

        let px: isize = robot_match
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| format!("X position could not be parsed as int! '{}'", line))?;

        let py: isize = robot_match
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| format!("Y position could not be parsed as int! '{}'", line))?;

        let vx: isize = robot_match
            .get(3)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| format!("X velocity could not be parsed as int! '{}'", line))?;

        let vy: isize = robot_match
            .get(4)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| format!("Y velocity could not be parsed as int! '{}'", line))?;
        
        let robot = Robot{pos: Vec2{x: px, y: py}, vel: Vec2{x: vx, y: vy}};
        robots.push(robot);
    }

    Ok((robot_area, robots))
}

fn step_by_step_simulation(robots: &mut [Robot], robot_area: &RobotArea, simulation_step_count: usize) {
    // FIXME:
    // This is horribly naive. There are much faster ways to do this. Namely, I don't actually have to loop.
    // I can just multiple all of the moves together and do one calculated adjustment back onto the grid that
    // does all wrapping at once. But I'm keeping it naive for now since I don't know what part 2 will be like.
    for robot in robots {
        for _ in 0..simulation_step_count {
            robot.pos.x += robot.vel.x;
            robot.pos.y += robot.vel.y;

            if robot.pos.x < 0 {
                robot.pos.x += (robot_area.width) as isize;
            }

            else if robot.pos.x >= robot_area.width as isize {
                robot.pos.x -= (robot_area.width) as isize;
            }

            if robot.pos.y < 0 {
                robot.pos.y += (robot_area.height) as isize;
            }

            else if robot.pos.x >= robot_area.width as isize {
                robot.pos.x -= (robot_area.height) as isize;
            }
        }
    }
}

fn count_robots_in_quadrants(robots: &[Robot], robot_area: &RobotArea) -> (usize, usize, usize, usize) {
    /*
    |----|-----
    | Q1 | Q2 |
    |----|-----
    | Q3 | Q4 |
    |----|-----
    */

    struct QuadrantBounds {
        topleft: Vec2,
        bottomright: Vec2,
    }

    fn in_quadrant_bounds(bounds: &QuadrantBounds, pos: &Vec2) -> bool {
        pos.x >= bounds.topleft.x &&
            pos.x <= bounds.bottomright.x &&
            pos.y >= bounds.topleft.y &&
            pos.y <= bounds.bottomright.y
    }

    // FIXME:  assume width and height are odd values for now. Its the sizes provided in both the sample problem 
    // and the real pt1 input. Assuming odd makes it easier to calculate area bounds or now
    assert!(robot_area.height % 2 == 1);
    assert!(robot_area.width % 2 == 1);
    let mid_x = (robot_area.width / 2) as isize;
    let mid_y = (robot_area.height / 2) as isize;
    let top_x = (robot_area.width - 1) as isize;
    let top_y = (robot_area.height - 1) as isize;

    let q1_bounds = QuadrantBounds { topleft: Vec2{x: 0, y: 0 }, bottomright: Vec2 {x: mid_x-1, y: mid_y-1}};
    let q2_bounds = QuadrantBounds { topleft: Vec2{x: mid_x+1, y: 0 }, bottomright: Vec2 {x: top_x, y: mid_y-1}};
    let q3_bounds = QuadrantBounds { topleft: Vec2{x: 0, y: mid_y+1 }, bottomright: Vec2 {x: mid_x-1, y: top_y}};
    let q4_bounds = QuadrantBounds { topleft: Vec2{x: mid_x+1, y: mid_y+1 }, bottomright: Vec2 {x: top_x, y: top_y}};

    let mut quadrant_counts = (0, 0, 0, 0);

    for robot in robots {
        if in_quadrant_bounds(&q1_bounds, &robot.pos) {
            quadrant_counts.0 += 1;
        } else if in_quadrant_bounds(&q2_bounds, &robot.pos) {
            quadrant_counts.1 += 1;
        } else if in_quadrant_bounds(&q3_bounds, &robot.pos) {
            quadrant_counts.2 += 1;
        } else if in_quadrant_bounds(&q4_bounds, &robot.pos) {
            quadrant_counts.3 += 1;
        }
    }

    quadrant_counts
}


fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_robots(filename);
    let (robot_area, robots)= match parse_result {
        Ok(result) => result,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    if robot_area.width * robot_area.height < 250 {
        dump_grid("start state", &robot_area, &robots);
    }

    {
        let mut simulated_robots = robots.clone();
        step_by_step_simulation(&mut simulated_robots, &robot_area, 100);
        let quadrant_counts = count_robots_in_quadrants(&simulated_robots, &robot_area);

        if robot_area.width * robot_area.height < 250 {
            dump_grid("end state", &robot_area, &simulated_robots);
        }

        let total_safety_factor = quadrant_counts.0 * quadrant_counts.1 * quadrant_counts.2 * quadrant_counts.3;
        println!("Pt 1. Total safety factor: {} = {} * {} * {} * {}",
            total_safety_factor, quadrant_counts.0 , quadrant_counts.1 , quadrant_counts.2 , quadrant_counts.3);
    }

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
