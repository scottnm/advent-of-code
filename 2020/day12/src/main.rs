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
struct ShipData {
    direction: Direction,
    x_pos: isize,
    y_pos: isize,
}

impl ShipData {
    fn move_cardinal(&mut self, direction: Direction, movement: usize) {
        let signed_move = movement as isize;
        let (x_move, y_move) = match direction {
            Direction::North => (0, signed_move),
            Direction::South => (0, signed_move * -1),
            Direction::East => (signed_move, 0),
            Direction::West => (signed_move * -1, 0),
        };

        self.x_pos += x_move;
        self.y_pos += y_move;
    }

    fn turn(&mut self, relative_direction: RelativeDirection, degrees: usize) {
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

    fn run_navigation(&self, navigation_instructions: &[Nav]) -> ShipData {
        let mut final_ship_data = self.clone();
        for instruction in navigation_instructions {
            match instruction {
                Nav::MoveCardinal(direction, movement) => {
                    final_ship_data.move_cardinal(*direction, *movement)
                }
                Nav::Turn(relative_direction, degrees) => {
                    final_ship_data.turn(*relative_direction, *degrees)
                }
                Nav::MoveForward(movement) => {
                    final_ship_data.move_cardinal(final_ship_data.direction, *movement)
                }
            }
        }
        final_ship_data
    }

    fn get_manhattan_distance(&self) -> usize {
        (self.x_pos.abs() + self.y_pos.abs()) as usize
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

    let initial_ship_data = ShipData {
        direction: Direction::East,
        x_pos: 0,
        y_pos: 0,
    };

    let navigation_instructions = get_instructions_from_input(&file_name);

    let final_ship_data = initial_ship_data.run_navigation(&navigation_instructions);
    let final_manhattan_distance = final_ship_data.get_manhattan_distance();

    println!(
        "start: {:?}, end: {:?}, final md: {}",
        initial_ship_data, final_ship_data, final_manhattan_distance
    );
}
