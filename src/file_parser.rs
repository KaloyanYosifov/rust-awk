use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::line::Line;

#[derive(Debug)]
pub struct FileParser;

impl FileParser {
    pub fn parse(file_path: &Path) -> Vec<crate::line::Line> {
        match Self::read_lines(file_path) {
            Ok(lines) => {
                let mut contents = Vec::new();

                for line in lines {
                    if let Ok(the_line) = line {
                        contents.push(Line::new(the_line));
                    }
                }

                contents
            }
            _ => Vec::new()
        }
    }

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use crate::file_parser::FileParser;

    #[test]
    fn it_parses_the_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("temp_file.txt");
        let mut file = File::create(file_path).unwrap();

        writeln!(file, "Hey there!").unwrap();
        writeln!(file, "Hey there 2!").unwrap();

        eprintln!("{:#?}", FileParser::parse(dir.path().join("temp_file.txt").as_path()));

        assert!(false)
    }
}
