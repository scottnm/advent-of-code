use input_helpers;
use std::process::ExitCode;

#[derive(Clone, Copy, Debug)]
struct Lock {
    pin_heights: (u8, u8, u8, u8, u8),
}

impl std::fmt::Display for Lock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Lock(pins={},{},{},{},{})",
            self.pin_heights.0,
            self.pin_heights.1,
            self.pin_heights.2,
            self.pin_heights.3,
            self.pin_heights.4
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct Key {
    notch_heights: (u8, u8, u8, u8, u8),
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Key(notches={},{},{},{},{})",
            self.notch_heights.0,
            self.notch_heights.1,
            self.notch_heights.2,
            self.notch_heights.3,
            self.notch_heights.4
        )
    }
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

    let (locks, keys) = read_input(filename)?;

    // dbg!(&locks);
    // dbg!(&keys);

    {
        let mut compatible_lock_key_pairs = vec![];
        for lock in &locks {
            for key in &keys {
                if do_key_lock_pair_fit(lock, key) {
                    compatible_lock_key_pairs.push((lock, key));
                    if verbose {
                        println!("{} and {} fit together!", lock, key);
                    }
                } else {
                    if verbose {
                        println!("{} and {} do not fit together.", lock, key);
                    }
                }
            }
        }

        println!(
            "Pt 1. # compatible pairs = {}",
            compatible_lock_key_pairs.len()
        );
    }

    if do_pt2 {
        unimplemented!();
    }

    Ok(())
}

fn read_input(filename: &str) -> Result<(Vec<Lock>, Vec<Key>), String> {
    let lines: Vec<String> = input_helpers::read_lines(filename).collect();

    let mut locks = vec![];
    let mut keys = vec![];

    let mut line_idx = 0;
    while line_idx <= lines.len() - 7 {
        let item_lines = &lines[line_idx..line_idx + 7];
        line_idx += 7;

        if line_idx < lines.len() {
            if lines[line_idx] != "" {
                return Err(format!(
                    "Unexpected line at {}! Expected empty separator. Found {}",
                    line_idx, lines[line_idx]
                ));
            }
            line_idx += 1;
        }

        if item_lines.first().unwrap() == "#####" {
            let lock = read_lock_lines(&item_lines[1..])?;
            locks.push(lock);
        } else if item_lines.last().unwrap() == "#####" {
            let key = read_key_lines(&item_lines[..item_lines.len() - 1])?;
            keys.push(key);
        } else {
            return Err(format!(
                "Invalid lines! Either first or last line in group should be all #'s! lines={:?}",
                item_lines
            ));
        }
    }

    Ok((locks, keys))
}

fn read_column_height<'a>(
    row_iter: impl Iterator<Item = &'a String>,
    column_idx: usize,
) -> Result<u8, String> {
    let mut column_height = 0;

    for row in row_iter {
        let column_char: char = row.as_bytes()[column_idx] as char;
        match column_char {
            '#' => column_height += 1,
            '.' => break,
            _ => return Err(format!("Invalid column char {}", column_char)),
        }
    }

    Ok(column_height)
}

fn read_lock_lines(lines: &[String]) -> Result<Lock, String> {
    if lines.len() != 6 {
        return Err(format!(
            "Invalid number of lines for lock! Found {}",
            lines.len()
        ));
    }

    if lines.iter().any(|line| line.len() != 5) {
        return Err(format!(
            "Invalid lock lines! All lines must have 5 chars {:?}",
            lines
        ));
    }

    let pin_heights = (
        read_column_height(lines.iter(), 0)?,
        read_column_height(lines.iter(), 1)?,
        read_column_height(lines.iter(), 2)?,
        read_column_height(lines.iter(), 3)?,
        read_column_height(lines.iter(), 4)?,
    );

    Ok(Lock { pin_heights })
}

fn read_key_lines(lines: &[String]) -> Result<Key, String> {
    if lines.len() != 6 {
        return Err(format!(
            "Invalid number of lines for key! Found {}",
            lines.len()
        ));
    }

    if lines.iter().any(|line| line.len() != 5) {
        return Err(format!(
            "Invalid key lines! All lines must have 5 chars {:?}",
            lines
        ));
    }

    let notch_heights = (
        read_column_height(lines.iter().rev(), 0)?,
        read_column_height(lines.iter().rev(), 1)?,
        read_column_height(lines.iter().rev(), 2)?,
        read_column_height(lines.iter().rev(), 3)?,
        read_column_height(lines.iter().rev(), 4)?,
    );

    Ok(Key { notch_heights })
}

fn do_key_lock_pair_fit(lock: &Lock, key: &Key) -> bool {
    (lock.pin_heights.0 + key.notch_heights.0 <= 5)
        && (lock.pin_heights.1 + key.notch_heights.1 <= 5)
        && (lock.pin_heights.2 + key.notch_heights.2 <= 5)
        && (lock.pin_heights.3 + key.notch_heights.3 <= 5)
        && (lock.pin_heights.4 + key.notch_heights.4 <= 5)
}
