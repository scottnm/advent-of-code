#[macro_use]
extern crate lazy_static;
extern crate regex;

#[derive(Debug)]
struct PasswordPolicy {
    character: char,
    character_policy: (usize, usize),
}

#[derive(Debug)]
struct DatabaseRecord {
    password: String,
    policy: PasswordPolicy,
}

impl DatabaseRecord {
    fn new<S>(character_policy: (usize, usize), character: char, password: S) -> Self
    where
        S: AsRef<str>,
    {
        DatabaseRecord {
            password: String::from(password.as_ref()),
            policy: PasswordPolicy {
                character,
                character_policy,
            },
        }
    }

    fn check_range_policy(&self) -> bool {
        let character_count = self
            .password
            .chars()
            .filter(|c| *c == self.policy.character)
            .count();

        let (start_range, end_range) = self.policy.character_policy;
        character_count >= start_range && character_count <= end_range
    }

    fn check_index_policy(&self) -> bool {
        let (first_index, second_index) = self.policy.character_policy;
        let nth_char = |s: &str, i: usize| s.chars().nth(i).unwrap();
        let first_match = nth_char(&self.password, first_index - 1) == self.policy.character;
        let second_match = nth_char(&self.password, second_index - 1) == self.policy.character;

        return first_match != second_match;
    }
}

impl std::str::FromStr for DatabaseRecord {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // 1-3 a: abcdef
            static ref RECORD_REGEX: regex::Regex =
                regex::Regex::new(r"(\d+)-(\d+)\s+([[:alpha:]]):\s+([[:alpha:]]+)").unwrap();
        }

        let captures = RECORD_REGEX
            .captures(s)
            .ok_or("Failed to parse database record")?;
        let range_start: usize = captures[1].parse().unwrap();
        let range_end: usize = captures[2].parse().unwrap();
        let character: char = captures[3].parse().unwrap();
        let password: String = String::from(&captures[4]);

        Ok(DatabaseRecord::new(
            (range_start, range_end),
            character,
            password,
        ))
    }
}

fn get_database_from_file(file_name: &str) -> Vec<DatabaseRecord> {
    let mut database = Vec::<DatabaseRecord>::new();

    for read_line in input_helpers::read_lines(file_name) {
        match read_line {
            Err(line_err) => println!("Bad line! {}", line_err),
            Ok(line) => database.push(line.parse().unwrap()),
        }
    }

    database
}

fn main() {
    let database = get_database_from_file("src/input.txt");
    let valid_entry_count_old_company = database.iter().filter(|e| e.check_range_policy()).count();
    let valid_entry_count_current_company =
        database.iter().filter(|e| e.check_index_policy()).count();

    println!("Database count: {}", database.len());
    println!("Valid entry count (old): {}", valid_entry_count_old_company);
    println!(
        "Valid entry count (current): {}",
        valid_entry_count_current_company
    );
}
