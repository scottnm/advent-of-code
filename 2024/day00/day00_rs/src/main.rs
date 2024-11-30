struct FloorCalc {
    result_floor: i32,
    first_basement_instruction: Option<usize>,
}

fn run() -> Result<FloorCalc, std::io::Error> {
    print!("Input floor instructions: ");

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let line = buffer.trim_end();

    let mut floor = 0;
    let mut first_basement_instruction = None;
    for (instr_idx, instruction) in line.chars().enumerate() {
        match instruction {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Bad instruction '{}'", instruction),
                ))
            }
        }
        if first_basement_instruction.is_none() && floor < 0 {
            first_basement_instruction = Some(instr_idx + 1);
        }
    }

    return Ok(FloorCalc {
        result_floor: floor,
        first_basement_instruction: first_basement_instruction,
    });
}

fn main() {
    let result = run();
    match result {
        Ok(floor_calc) => {
            println!("Result floor: {}", floor_calc.result_floor);
            if let Some(first_basement_instruction) = floor_calc.first_basement_instruction {
                println!("First basement instruction: {}", first_basement_instruction);
            } else {
                println!("No instructions go to basement");
            }
        }
        Err(err) => {
            println!("Fail! {}", err);
        }
    }
}
