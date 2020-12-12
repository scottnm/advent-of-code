use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;

pub fn read_lines<P>(file_name: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut lines = super::read_lines("src/test.txt");
        for i in 0..6 {
            let line = lines.next();
            assert!(line.is_some());

            let line = line.unwrap();
            assert!(line.is_ok());

            let line_value = line.unwrap().parse::<u8>();
            assert!(line_value.is_ok());

            let line_value = line_value.unwrap();
            assert_eq!(line_value, i);
        }

        let no_line = lines.next();
        assert!(no_line.is_none());
    }
}
