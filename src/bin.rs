use rust_awk_lib::file_parser::FileParser;
use std::path::Path;

fn main() {
    let path = Path::new("./build.txt");
    let lines = FileParser::parse(&path);

    eprintln!("{:#?}", lines);
}
