use std::env;
use std::path::Path;
use rust_awk_lib::file_parser::FileParser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("{}", "Please use this as an example! rust-awk '{ print $0 }' ./test.txt")
    }

    let code = args.get(1).unwrap();
    let path = Path::new(args.get(2).unwrap());
    let lines = FileParser::parse(&path);
    let code_parser = code_parser::parser::Parser::parse(code.clone());

    code_parser
        .instructions()
        .iter()
        .for_each(|instruction| {
            match instruction {
                code_parser::parser::Instruction::PRINT(index) => {
                    if *index == 0 {
                        lines.lines()
                            .iter()
                            .for_each(|line| println!("{}", line.line()))
                    } else {
                        lines.word((*index) as usize).iter().for_each(|line| println!("{}", line))
                    }
                }
                _ => ()
            };
        });
}
