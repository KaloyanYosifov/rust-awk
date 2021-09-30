use std::env;
use std::path::Path;
use rust_awk_lib::file_parser::FileParser;
use code_parser::parser::Instruction;
use code_parser::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("{}", "Please use this as an example! rust-awk '{ print $0 }' ./test.txt")
    }

    let code = args.get(1).unwrap();
    let path = Path::new(args.get(2).unwrap());
    let lines = FileParser::parse(&path);
    let code_parser = Parser::parse(code.clone());

    if let Some(Instruction::REGEX(pattern)) = code_parser.instructions().get(0) {
        code_parser
            .instructions()
            .iter()
            .skip(1)
            .for_each(|instruction| {
                match instruction {
                    Instruction::PRINT(index) => {
                        if *index == 0 {
                            lines.lines()
                                .iter()
                                .filter(|line| {
                                    let regex = regex::Regex::new(pattern).unwrap();

                                    regex.is_match(&line.line())
                                })
                                .for_each(|line| println!("{}", line.line()))
                        } else {
                            lines
                                .word((*index) as usize)
                                .iter()
                                .filter(|line| {
                                    let regex = regex::Regex::new(pattern).unwrap();

                                    regex.is_match(line)
                                })
                                .for_each(|line| println!("{}", line))
                        }
                    }
                    _ => ()
                };
            });
    } else {
        code_parser
            .instructions()
            .iter()
            .for_each(|instruction| {
                match instruction {
                    Instruction::PRINT(index) => {
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
}
