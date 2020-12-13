type Year = i64;
type Height = u32;
type Color = u32;
type PassportId = u64;
type CountryId = u64;

struct PassportRecord {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Height>,
    hair_color: Option<Color>,
    eye_color: Option<Color>,
    pid: Option<PassportId>,
    cid: Option<CountryId> // only actually opt field
}

impl PassportRecord {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some() &&
        self.issue_year.is_some() &&
        self.expiration_year.is_some() &&
        self.height.is_some() &&
        self.hair_color.is_some() &&
        self.eye_color.is_some() &&
        self.pid.is_some()
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
                    let line_elements: Vec<String> = line.split(' ').map(|e| String::from(e)).collect();
                    next_passport_inputs.extend(line_elements);
                }
            },
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
        let mut hair_color: Option<Color> = None;
        let mut eye_color: Option<Color> = None;
        let mut pid: Option<u64> = None;
        let mut cid: Option<u64> = None;
        for passport_input_element in passport_input {
            let element_key = passport_input_element.split(':').nth(0).unwrap();
            match element_key {
                "byr" => { birth_year = Some(Year::default()); },
                "iyr" => { issue_year = Some(Year::default()); },
                "eyr" => { expiration_year = Some(Year::default()); },
                "hgt" => { height = Some(Height::default()); },
                "hcl" => { hair_color = Some(Color::default()); },
                "ecl" => { eye_color = Some(Color::default()); },
                "pid" => { pid = Some(PassportId::default()); },
                "cid" => { cid = Some(CountryId::default()); },
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
            cid,
        });
    }
    passports
}

fn main() {
    let passports = get_input_passports("src/input.txt");
    let valid_count = passports.iter().filter(|p| p.is_valid()).count();
    println!("Valid passport count={}", valid_count);
}