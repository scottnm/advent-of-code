use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
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

pub fn get_input_file_from_args(args: &mut std::env::Args) -> String {
    let program_name = args.nth(0).unwrap();
    let input_file = match args.nth(0).as_ref().map(|s| s.as_str()) {
        Some("simple") => "src/simple_input.txt",
        Some("simple2") => "src/simple2_input.txt",
        Some("real") => "src/input.txt",
        _ => panic!("USAGE: ./{} [simple|simple2|real]", &program_name),
    };

    String::from(input_file)
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
