#[macro_use]
extern crate lazy_static;
extern crate regex;

type Year = i64;

fn parse_year(input: &str) -> Option<Year> {
    input.parse::<i64>().ok()
}

#[derive(Clone, Copy)]
enum Height {
    Cm(u32),
    In(u32),
}

fn parse_height(input: &str) -> Option<Height> {
    lazy_static! {
        // 1-3 a: abcdef
        static ref HEIGHT_REGEX: regex::Regex =
            regex::Regex::new(r"(\d+)((cm)|(in))").unwrap();
    }

    HEIGHT_REGEX.captures(input).map(|captures| {
        let height_value = captures[1].parse().unwrap();
        let height_unit = &captures[2];
        match height_unit {
            "cm" => Height::Cm(height_value),
            "in" => Height::In(height_value),
            _ => unreachable!(), // regex will only match cm or in
        }
    })
}

type HairColor = String;

fn parse_hair_color(input: &str) -> Option<HairColor> {
    lazy_static! {
        // 1-3 a: abcdef
        static ref HAIR_COLOR_REGEX: regex::Regex =
            regex::Regex::new(r"#([0-9||a-f]{6})").unwrap();
    }

    HAIR_COLOR_REGEX
        .captures(input)
        .map(|captures| String::from(&captures[1]))
}

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

fn parse_eye_color(input: &str) -> Option<EyeColor> {
    match input {
        "amb" => Some(EyeColor::Amber),
        "blu" => Some(EyeColor::Blue),
        "brn" => Some(EyeColor::Brown),
        "gry" => Some(EyeColor::Gray),
        "grn" => Some(EyeColor::Green),
        "hzl" => Some(EyeColor::Hazel),
        "oth" => Some(EyeColor::Other),
        _ => None,
    }
}

type PassportId = String;

fn parse_passport_id(input: &str) -> Option<PassportId> {
    lazy_static! {
        // 1-3 a: abcdef
        static ref PASSPORT_ID_REGEX: regex::Regex =
            regex::Regex::new(r"(\d{9})").unwrap();
    }

    PASSPORT_ID_REGEX
        .captures(input)
        .map(|captures| String::from(&captures[0]))
}

struct PassportRecord {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Height>,
    hair_color: Option<HairColor>,
    eye_color: Option<EyeColor>,
    pid: Option<PassportId>,
}

impl PassportRecord {
    fn is_valid(&self) -> bool {
        let is_valid_birth_year = |y: Year| y >= 1920 && y <= 2002;
        let is_valid_issued_year = |y: Year| y >= 2010 && y <= 2020;
        let is_valid_expiration_year = |y: Year| y >= 2020 && y <= 2030;
        let is_valid_height = |h: Height| match h {
            Height::Cm(centimeters) => centimeters >= 150 && centimeters <= 193,
            Height::In(inches) => inches >= 59 && inches <= 76,
        };

        self.birth_year.is_some()
            && is_valid_birth_year(self.birth_year.unwrap())
            && self.issue_year.is_some()
            && is_valid_issued_year(self.issue_year.unwrap())
            && self.expiration_year.is_some()
            && is_valid_expiration_year(self.expiration_year.unwrap())
            && self.height.is_some()
            && is_valid_height(self.height.unwrap())
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.pid.is_some()
    }
}

fn get_input_passports(file_name: &str) -> Vec<PassportRecord> {
    let mut passport_inputs = Vec::new();
    let mut next_passport_inputs = Vec::new();
    for maybe_line in input_helpers::read_lines(file_name) {
        match maybe_line {
            Ok(line) => {
                if line.is_empty() {
                    passport_inputs.push(std::mem::replace(&mut next_passport_inputs, Vec::new()));
                } else {
                    let line_elements: Vec<String> =
                        line.split(' ').map(|e| String::from(e)).collect();
                    next_passport_inputs.extend(line_elements);
                }
            }
            Err(line_err) => println!("Line err: {}", line_err),
        }
    }

    if !next_passport_inputs.is_empty() {
        passport_inputs.push(std::mem::replace(&mut next_passport_inputs, Vec::new()));
    }

    dbg!(passport_inputs.last());

    let mut passports = Vec::new();
    for passport_input in passport_inputs {
        let mut birth_year: Option<Year> = None;
        let mut issue_year: Option<Year> = None;
        let mut expiration_year: Option<Year> = None;
        let mut height: Option<Height> = None;
        let mut hair_color: Option<HairColor> = None;
        let mut eye_color: Option<EyeColor> = None;
        let mut pid: Option<PassportId> = None;
        for passport_input_element in passport_input {
            let mut element_split = passport_input_element.split(':');
            let element_key = element_split.next().unwrap();
            let element_val = element_split.next().unwrap();
            match element_key {
                "byr" => {
                    birth_year = parse_year(element_val);
                }
                "iyr" => {
                    issue_year = parse_year(element_val);
                }
                "eyr" => {
                    expiration_year = parse_year(element_val);
                }
                "hgt" => {
                    height = parse_height(element_val);
                }
                "hcl" => {
                    hair_color = parse_hair_color(element_val);
                }
                "ecl" => {
                    eye_color = parse_eye_color(element_val);
                }
                "pid" => {
                    pid = parse_passport_id(element_val);
                }
                "cid" => (), // CIDs are ignored entirey
                _ => (),
            }
        }

        passports.push(PassportRecord {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            pid,
        });
    }
    passports
}

fn main() {
    let passports = get_input_passports("src/input.txt");
    let valid_count = passports.iter().filter(|p| p.is_valid()).count();
    println!("Valid passport count={}", valid_count);
}
