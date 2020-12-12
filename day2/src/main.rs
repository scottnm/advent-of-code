#[derive(Debug)]
struct PasswordPolicy {
    required_character: char,
    required_character_range: (usize, usize),
}

#[derive(Debug)]
struct DatabaseRecord {
    password: String,
    policy: PasswordPolicy,
}

impl DatabaseRecord {
    fn new<S>(
        required_character_range: (usize, usize),
        required_character: char,
        password: S,
    ) -> Self
    where
        S: AsRef<str>,
    {
        DatabaseRecord {
            password: String::from(password.as_ref()),
            policy: PasswordPolicy {
                required_character,
                required_character_range,
            },
        }
    }

    fn check_password(&self) -> bool {
        let required_character_count = self
            .password
            .chars()
            .filter(|c| *c == self.policy.required_character)
            .count();

        required_character_count >= self.policy.required_character_range.0
            && required_character_count <= self.policy.required_character_range.1
    }
}

fn get_database_from_file(_file_name: &str) -> Vec<DatabaseRecord> {
    let mut database = Vec::<DatabaseRecord>::new();

    database.push(DatabaseRecord::new((2, 4), 'a', "apple"));
    database.push(DatabaseRecord::new((1, 3), 'p', "apple"));
    /*
    for read_line in input_helpers::read_lines(file_name) {
        match read_line {
            Err(line_err) => println!("Bad line! {}", line_err),
            Ok(line) => report_entries.push(line.parse().unwrap()),
        }
    }
    */

    database
}

fn main() {
    let database = get_database_from_file("simple_input.txt");
    let invalid_entry_count = database.iter().filter(|e| !e.check_password()).count();

    println!("Database count: {}", database.len());
    println!("Invalid entry count: {}", invalid_entry_count);
}
