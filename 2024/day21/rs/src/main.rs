use input_helpers;
use core::num;
use std::{mem::discriminant, process::ExitCode};

#[derive(Clone, Copy, Debug, Hash)]
enum NumpadButton {
    Btn0,
    Btn1,
    Btn2,
    Btn3,
    Btn4,
    Btn5,
    Btn6,
    Btn7,
    Btn8,
    Btn9,
    BtnA,
}

impl NumpadButton {
    fn as_char(&self) -> char {
        match *self {
            NumpadButton::Btn0 => '0',
            NumpadButton::Btn1 => '1',
            NumpadButton::Btn2 => '2',
            NumpadButton::Btn3 => '3',
            NumpadButton::Btn4 => '4',
            NumpadButton::Btn5 => '5',
            NumpadButton::Btn6 => '6',
            NumpadButton::Btn7 => '7',
            NumpadButton::Btn8 => '8',
            NumpadButton::Btn9 => '9',
            NumpadButton::BtnA => 'A',
        }
    }
}

#[derive(Clone, Copy, Debug, Hash)]
enum DirButton {
    BtnUp,
    BtnDown,
    BtnLeft,
    BtnRight,
    BtnA,
}

impl DirButton {
    fn as_char(&self) -> char {
        match *self {
            DirButton::BtnUp => '^',
            DirButton::BtnDown => 'v',
            DirButton::BtnLeft => '<',
            DirButton::BtnRight => '>',
            DirButton::BtnA => 'A',
        }
    }
}

/*
Numpad
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
 */

fn get_numpad_move_btn_sequence(start_btn: NumpadButton, end_btn: NumpadButton) -> Vec<DirButton> {
    match start_btn {
        NumpadButton::Btn0 => match end_btn {
            NumpadButton::Btn0 => vec![],
            NumpadButton::Btn1 => vec![DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn2 => vec![DirButton::BtnUp],
            NumpadButton::Btn3 => vec![DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::Btn4 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn5 => vec![DirButton::BtnUp, DirButton::BtnUp],
            NumpadButton::Btn6 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::Btn7 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn8 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnUp],
            NumpadButton::Btn9 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::BtnA => vec![DirButton::BtnRight],
        },
        NumpadButton::Btn1 => match end_btn {
            NumpadButton::Btn0 => vec![DirButton::BtnRight, DirButton::BtnDown],
            NumpadButton::Btn1 => vec![],
            NumpadButton::Btn2 => vec![DirButton::BtnRight],
            NumpadButton::Btn3 => vec![DirButton::BtnRight],
            NumpadButton::Btn4 => vec![DirButton::BtnUp],
            NumpadButton::Btn5 => vec![DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::Btn6 => vec![DirButton::BtnUp, DirButton::BtnRight, DirButton::BtnRight],
            NumpadButton::Btn7 => vec![DirButton::BtnUp, DirButton::BtnUp],
            NumpadButton::Btn8 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::Btn9 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnRight, DirButton::BtnRight],
            NumpadButton::BtnA => vec![DirButton::BtnRight, DirButton::BtnRight, DirButton::BtnDown],
        },
        NumpadButton::Btn2 => match end_btn {
            NumpadButton::Btn0 => vec![DirButton::BtnDown],
            NumpadButton::Btn1 => vec![DirButton::BtnLeft],
            NumpadButton::Btn2 => vec![],
            NumpadButton::Btn3 => vec![DirButton::BtnRight],
            NumpadButton::Btn4 => vec![DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn5 => vec![DirButton::BtnUp],
            NumpadButton::Btn6 => vec![DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::Btn7 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn8 => vec![DirButton::BtnUp, DirButton::BtnUp],
            NumpadButton::Btn9 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::BtnA => vec![DirButton::BtnRight, DirButton::BtnDown],
        },
        NumpadButton::Btn3 => match end_btn {
            NumpadButton::Btn0 => vec![DirButton::BtnLeft, DirButton::BtnDown],
            NumpadButton::Btn1 => vec![DirButton::BtnLeft, DirButton::BtnLeft],
            NumpadButton::Btn2 => vec![DirButton::BtnLeft],
            NumpadButton::Btn3 => vec![],
            NumpadButton::Btn4 => vec![DirButton::BtnUp, DirButton::BtnLeft, DirButton::BtnLeft],
            NumpadButton::Btn5 => vec![DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn6 => vec![DirButton::BtnUp],
            NumpadButton::Btn7 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnLeft, DirButton::BtnLeft],
            NumpadButton::Btn8 => vec![DirButton::BtnUp, DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn9 => vec![DirButton::BtnUp, DirButton::BtnUp,],
            NumpadButton::BtnA => vec![DirButton::BtnDown],
        },
        NumpadButton::Btn4 => match end_btn {
            NumpadButton::Btn0 => vec![DirButton::BtnRight, DirButton::BtnDown, DirButton::BtnDown],
            NumpadButton::Btn1 => vec![DirButton::BtnDown],
            NumpadButton::Btn2 => vec![DirButton::BtnRight, DirButton::BtnDown],
            NumpadButton::Btn3 => vec![DirButton::BtnRight, DirButton::BtnRight, DirButton::BtnDown],
            NumpadButton::Btn4 => vec![],
            NumpadButton::Btn5 => vec![DirButton::BtnRight],
            NumpadButton::Btn6 => vec![DirButton::BtnRight, DirButton::BtnRight],
            NumpadButton::Btn7 => vec![DirButton::BtnUp],
            NumpadButton::Btn8 => vec![DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::Btn9 => vec![DirButton::BtnUp, DirButton::BtnRight, DirButton::BtnRight],
            NumpadButton::BtnA => vec![DirButton::BtnRight, DirButton::BtnRight, DirButton::BtnDown, DirButton::BtnDown],
        },
        NumpadButton::Btn5 => match end_btn {
            NumpadButton::Btn0 => vec![DirButton::BtnDown, DirButton::BtnDown],
            NumpadButton::Btn1 => vec![DirButton::BtnDown, DirButton::BtnLeft],
            NumpadButton::Btn2 => vec![DirButton::BtnDown],
            NumpadButton::Btn3 => vec![DirButton::BtnDown, DirButton::BtnRight],
            NumpadButton::Btn4 => vec![DirButton::BtnLeft],
            NumpadButton::Btn5 => vec![],
            NumpadButton::Btn6 => vec![DirButton::BtnRight],
            NumpadButton::Btn7 => vec![DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn8 => vec![DirButton::BtnUp],
            NumpadButton::Btn9 => vec![DirButton::BtnUp, DirButton::BtnRight],
            NumpadButton::BtnA => vec![DirButton::BtnRight, DirButton::BtnDown, DirButton::BtnDown],
        },
        NumpadButton::Btn6 => match end_btn {
            NumpadButton::Btn0 => vec![DirButton::BtnDown, DirButton::BtnDown, DirButton::BtnLeft],
            NumpadButton::Btn1 => vec![DirButton::BtnDown, DirButton::BtnLeft, DirButton::BtnLeft],
            NumpadButton::Btn2 => vec![DirButton::BtnDown, DirButton::BtnLeft],
            NumpadButton::Btn3 => vec![DirButton::BtnLeft],
            NumpadButton::Btn4 => vec![DirButton::BtnLeft, DirButton::BtnLeft],
            NumpadButton::Btn5 => vec![DirButton::BtnLeft],
            NumpadButton::Btn6 => vec![],
            NumpadButton::Btn7 => vec![DirButton::BtnUp, DirButton::BtnLeft, DirButton::BtnLeft],
            NumpadButton::Btn8 => vec![DirButton::BtnUp, DirButton::BtnLeft],
            NumpadButton::Btn9 => vec![DirButton::BtnUp],
            NumpadButton::BtnA => vec![DirButton::BtnDown, DirButton::BtnDown],
        },
        NumpadButton::Btn7 => match end_btn {
            NumpadButton::Btn0 => vec![],
            NumpadButton::Btn1 => vec![],
            NumpadButton::Btn2 => vec![],
            NumpadButton::Btn3 => vec![],
            NumpadButton::Btn4 => vec![],
            NumpadButton::Btn5 => vec![],
            NumpadButton::Btn6 => vec![],
            NumpadButton::Btn7 => vec![],
            NumpadButton::Btn8 => vec![],
            NumpadButton::Btn9 => vec![],
            NumpadButton::BtnA => vec![],
        },
        NumpadButton::Btn8 => match end_btn {
            NumpadButton::Btn0 => vec![],
            NumpadButton::Btn1 => vec![],
            NumpadButton::Btn2 => vec![],
            NumpadButton::Btn3 => vec![],
            NumpadButton::Btn4 => vec![],
            NumpadButton::Btn5 => vec![],
            NumpadButton::Btn6 => vec![],
            NumpadButton::Btn7 => vec![],
            NumpadButton::Btn8 => vec![],
            NumpadButton::Btn9 => vec![],
            NumpadButton::BtnA => vec![],
        },
        NumpadButton::Btn9 => match end_btn {
            NumpadButton::Btn0 => vec![],
            NumpadButton::Btn1 => vec![],
            NumpadButton::Btn2 => vec![],
            NumpadButton::Btn3 => vec![],
            NumpadButton::Btn4 => vec![],
            NumpadButton::Btn5 => vec![],
            NumpadButton::Btn6 => vec![],
            NumpadButton::Btn7 => vec![],
            NumpadButton::Btn8 => vec![],
            NumpadButton::Btn9 => vec![],
            NumpadButton::BtnA => vec![],
        },
        NumpadButton::BtnA => match end_btn {
            NumpadButton::Btn0 => vec![],
            NumpadButton::Btn1 => vec![],
            NumpadButton::Btn2 => vec![],
            NumpadButton::Btn3 => vec![],
            NumpadButton::Btn4 => vec![],
            NumpadButton::Btn5 => vec![],
            NumpadButton::Btn6 => vec![],
            NumpadButton::Btn7 => vec![],
            NumpadButton::Btn8 => vec![],
            NumpadButton::Btn9 => vec![],
            NumpadButton::BtnA => vec![],
        },
    }
}

type Code = [NumpadButton; 4];

fn read_char_as_numpad_btn(c: char) -> Result<NumpadButton, String> {
    let btn = match c {
        '0' => NumpadButton::Btn0,
        '1' => NumpadButton::Btn1,
        '2' => NumpadButton::Btn2,
        '3' => NumpadButton::Btn3,
        '4' => NumpadButton::Btn4,
        '5' => NumpadButton::Btn5,
        '6' => NumpadButton::Btn6,
        '7' => NumpadButton::Btn7,
        '8' => NumpadButton::Btn8,
        '9' => NumpadButton::Btn9,
        'A' => NumpadButton::BtnA,
        _ => return Err(format!("Invalid btn char! {}", c)),
    };

    Ok(btn)
}

fn read_code_line(line: &str) -> Result<Code, String> {
    if line.len() != 4 {
        return Err(format!(
            "Invalid line! Require exactly 4 chars. Had {}",
            line.len()
        ));
    }

    let mut chars_itr = line.chars();
    let numpad_btns = [ 
        read_char_as_numpad_btn(chars_itr.next().unwrap())?,
        read_char_as_numpad_btn(chars_itr.next().unwrap())?,
        read_char_as_numpad_btn(chars_itr.next().unwrap())?,
        read_char_as_numpad_btn(chars_itr.next().unwrap())?,
        ];
    for (i, btn) in numpad_btns[0..3].iter().enumerate() {
        if let NumpadButton::BtnA = btn {
            return Err(format!("Invalid BtnA in position {} in code {}{}{}{}", 
                i, 
                numpad_btns[0].as_char(),
                numpad_btns[1].as_char(),
                numpad_btns[2].as_char(),
                numpad_btns[3].as_char()))
        }
    }

    match numpad_btns[3] {
        NumpadButton::BtnA => (),
        _ => return Err(format!("Invalid code {}{}{}{}! Must end in BtnA",
                numpad_btns[0].as_char(),
                numpad_btns[1].as_char(),
                numpad_btns[2].as_char(),
                numpad_btns[3].as_char())),
    }
    
    Ok(numpad_btns)
}

fn read_input(filename: &str) -> Result<Vec<Code>, String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    if lines.len() != 5 {
        return Err(format!(
            "Invalid input! Require exactly 5 lines. Had {}",
            lines.len()
        ));
    }


    let codes = vec![
        read_code_line(&lines[0])?,
        read_code_line(&lines[1])?,
        read_code_line(&lines[2])?,
        read_code_line(&lines[3])?,
        read_code_line(&lines[4])?,
    ];

    Ok(codes)
}

fn run(args: &[String]) -> Result<(), String> {
    let filename: &str = input_helpers::get_nth_string_arg(args, 0)?;
    let verbose = args
        .iter()
        .find(|a| a.as_str() == "-v" || a.as_str() == "--verbose")
        .is_some();
    let do_pt2 = args
        .iter()
        .find(|a| a.as_str() == "-2" || a.as_str() == "--pt2")
        .is_some();

    let codes = read_input(filename)?;

    dbg!(&codes);

    {
        /*
        let mut design_test_memo = DesignTestMemoizer::new();
        let possible_designs: Vec<TargetDesign> = target_designs
            .iter()
            .filter(|design| {
                is_target_design_possible(design, &available_patterns, &mut design_test_memo)
            })
            .cloned()
            .collect();

        println!("Pt 1: {} designs possible", possible_designs.len());
        if verbose {
            println!("possible designs:");
            for design in &possible_designs {
                println!("  - {}", design);
            }
        }

        possible_designs
        */
    }

    if do_pt2 {
        unimplemented!();
    }

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Err: {}", e);
            ExitCode::FAILURE
        }
    }
}
