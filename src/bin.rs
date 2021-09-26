use std::env;
use std::path::Path;
use rust_awk_lib::file_parser::FileParser;

fn main() {
    let path = Path::new("./test.txt");
    let lines = FileParser::parse(&path);
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("{}", "Please use this as an example! rust-awk '{ print $0 }' ./test.txt")
    }

    let code = args.get(1).unwrap();
}
