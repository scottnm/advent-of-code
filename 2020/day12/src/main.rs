#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum RelativeDirection {
    Left,
    Right,
}

#[derive(Debug)]
enum Nav {
    Turn(RelativeDirection, usize),
    MoveCardinal(Direction, usize),
    MoveForward(usize),
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct ShipData {
    direction: Direction,
    ship_pos: Pos,
    waypoint_relative_pos: Pos,
}

impl ShipData {
    fn default() -> Self {
        ShipData {
            direction: Direction::East,
            ship_pos: Pos { x: 0, y: 0 },
            waypoint_relative_pos: Pos { x: 10, y: 1 },
        }
    }
}

impl ShipData {
    fn move_cardinal_ship(&mut self, direction: Direction, movement: usize) {
        let signed_move = movement as isize;
        let (x_move, y_move) = match direction {
            Direction::North => (0, signed_move),
            Direction::South => (0, signed_move * -1),
            Direction::East => (signed_move, 0),
            Direction::West => (signed_move * -1, 0),
        };

        self.ship_pos.x += x_move;
        self.ship_pos.y += y_move;
    }

    fn move_cardinal_waypoint(&mut self, direction: Direction, movement: usize) {
        unimplemented!();
        /*
        let signed_move = movement as isize;
        let (x_move, y_move) = match direction {
            Direction::North => (0, signed_move),
            Direction::South => (0, signed_move * -1),
            Direction::East => (signed_move, 0),
            Direction::West => (signed_move * -1, 0),
        };

        self.ship_pos.x += x_move;
        self.ship_pos.y += y_move;
        */
    }

    fn turn_ship(&mut self, relative_direction: RelativeDirection, degrees: usize) {
        assert!(degrees % 90 == 0); // the degrees are currently assumed to be in increments of 90
        let relative_turn_count = (degrees % 360) / 90;
        let abs_turn_count = match relative_direction {
            RelativeDirection::Left => 4 - relative_turn_count,
            RelativeDirection::Right => relative_turn_count,
        };

        for _ in 0..abs_turn_count {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
        }
    }

    fn turn_waypoint(&mut self, relative_direction: RelativeDirection, degrees: usize) {
        unimplemented!();
        /*
        assert!(degrees % 90 == 0); // the degrees are currently assumed to be in increments of 90
        let relative_turn_count = (degrees % 360) / 90;
        let abs_turn_count = match relative_direction {
            RelativeDirection::Left => 4 - relative_turn_count,
            RelativeDirection::Right => relative_turn_count,
        };

        for _ in 0..abs_turn_count {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
        }
        */
    }

    fn run_navigation_abs(&self, navigation_instructions: &[Nav]) -> ShipData {
        let mut final_ship_data = self.clone();
        for instruction in navigation_instructions {
            match instruction {
                Nav::MoveCardinal(direction, movement) => {
                    final_ship_data.move_cardinal_ship(*direction, *movement)
                }
                Nav::Turn(relative_direction, degrees) => {
                    final_ship_data.turn_ship(*relative_direction, *degrees)
                }
                Nav::MoveForward(movement) => {
                    final_ship_data.move_cardinal_ship(final_ship_data.direction, *movement)
                }
            }
        }
        final_ship_data
    }

    fn run_navigation_waypoint(&self, navigation_instructions: &[Nav]) -> ShipData {
        let mut final_ship_data = self.clone();
        for instruction in navigation_instructions {
            match instruction {
                Nav::MoveCardinal(direction, movement) => {
                    final_ship_data.move_cardinal_waypoint(*direction, *movement)
                }
                Nav::Turn(relative_direction, degrees) => {
                    final_ship_data.turn_waypoint(*relative_direction, *degrees)
                }
                Nav::MoveForward(movement) => {
                    unimplemented!(); // move forward moves to waypoint. it doesn't move the waypoint
                                      //final_ship_data.move_cardinal_waypoint(final_ship_data.direction, *movement)
                }
            }
        }
        final_ship_data
    }

    fn get_manhattan_distance(&self) -> usize {
        (self.ship_pos.x.abs() + self.ship_pos.y.abs()) as usize
    }
}

fn get_instructions_from_input(file_name: &str) -> Vec<Nav> {
    input_helpers::read_lines(file_name)
        .map(|line| {
            let movement = line[1..].parse().unwrap();
            match &line[0..1] {
                "N" => Nav::MoveCardinal(Direction::North, movement),
                "S" => Nav::MoveCardinal(Direction::South, movement),
                "E" => Nav::MoveCardinal(Direction::East, movement),
                "W" => Nav::MoveCardinal(Direction::West, movement),
                "L" => Nav::Turn(RelativeDirection::Left, movement),
                "R" => Nav::Turn(RelativeDirection::Right, movement),
                "F" => Nav::MoveForward(movement),
                invalid_char @ _ => panic!("Invalid character at start of line: {}", invalid_char),
            }
        })
        .collect()
}

fn main() {
    let file_name = input_helpers::get_input_file_from_args(&mut std::env::args());

    let initial_ship_data = ShipData::default();

    let navigation_instructions = get_instructions_from_input(&file_name);

    let pt1_ship_data = initial_ship_data.run_navigation_abs(&navigation_instructions);
    println!(
        "Pt1 ship: {:?}, Dist: {}",
        pt1_ship_data,
        pt1_ship_data.get_manhattan_distance(),
    );

    let pt2_ship_data = initial_ship_data.run_navigation_waypoint(&navigation_instructions);
    println!(
        "Pt2 ship: {:?}, Dist: {}",
        pt2_ship_data,
        pt2_ship_data.get_manhattan_distance(),
    );
}
