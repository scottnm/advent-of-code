use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub struct Lines {
    lines: std::io::Lines<BufReader<File>>,
}

impl Iterator for Lines {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.lines
            .next()
            .map(|result_string| result_string.unwrap())
    }
}

pub fn read_lines<P>(file_name: P) -> Lines
where
    P: AsRef<Path>,
{
    let file = File::open(file_name).unwrap();
    Lines {
        lines: BufReader::new(file).lines(),
    }
}

pub fn read_file_to_string<P>(file_name: P) -> Result<String, std::io::Error>
where
    P: AsRef<Path>,
{
    let mut file_data = String::new();
    match File::open(file_name)?.read_to_string(&mut file_data) {
        Ok(_) => Ok(file_data),
        Err(e) => Err(e), 
    }
}

pub fn get_input_file_from_args() -> String {
    let mut args = std::env::args();
    let program_name = args.nth(0).unwrap();
    let input_file = match args.nth(0).as_ref().map(|s| s.as_str()) {
        Some("simple") => "src/simple_input.txt",
        Some("simple2") => "src/simple2_input.txt",
        Some("real") => "src/input.txt",
        _ => panic!("USAGE: ./{} [simple|simple2|real]", &program_name),
    };

    String::from(input_file)
}

pub fn get_nth_string_arg<'a>(args: &'a [String], n: usize) -> Result<&'a str, String> {
    if args.len() <= n {
        return Err(format!(
            "Too few args! needed {}; had {}",
            n + 1,
            args.len()
        ));
    }

    Ok(&args[n])
}

pub fn get_nth_string_arg_or_default<'a>(args: &'a [String], n: usize, default: &'static str) -> Result<&'a str, String> {
    if args.len() <= n {
        return Ok(default);
    }

    Ok(&args[n])
}

pub fn get_nth_parsed_arg<T>(args: &[String], n: usize) -> Result<T, String>
where
    T: std::str::FromStr,
{
    if args.len() <= n {
        return Err(format!(
            "Too few args! needed {}; had {}",
            n + 1,
            args.len()
        ));
    }

    match args[n].parse() {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Failed to parse arg! '{}'", &args[n])),
    }
}

pub fn get_nth_parsed_arg_or_default<T>(args: &[String], n: usize, default: T) -> Result<T, String>
where
    T: std::str::FromStr,
{
    if args.len() <= n {
        return Ok(default);
    }

    match args[n].parse() {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Failed to parse arg! '{}'", &args[n])),
    }
}

pub fn get_parsed_arg_by_key<T>(args: &[String], key: &str) -> Result<Option<T>, String>
where
    T: std::str::FromStr,
{
    let key_prefix = format!("{}=", key);
    let mut arg_value = None;
    for a in args {
        if a.starts_with(&key_prefix) {
            arg_value = Some(a[key_prefix.len()..].to_string());
            break;
        }
    }

    if let Some(arg_value) = arg_value {
        match arg_value.parse() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(format!("Failed to parse arg value! '{}'", arg_value)),
        }
    } else {
        Ok(None)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut lines = super::read_lines("src/test.txt");
        for i in 0..6 {
            let maybe_line = lines.next();
            assert!(maybe_line.is_some());
            let line = maybe_line.unwrap();

            let line_value = line.parse::<u8>();
            assert!(line_value.is_ok());

            let line_value = line_value.unwrap();
            assert_eq!(line_value, i);
        }

        let no_line = lines.next();
        assert!(no_line.is_none());
    }
}
