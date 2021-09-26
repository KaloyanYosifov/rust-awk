enum Instruction {
    NONE,
    PRINT(i32),
}

pub struct Parser {
    code: String,
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn parse(code: String) -> Self {
        let instructions = Self::parse_code(&code);

        Self {
            code,
            instructions,
        }
    }

    fn parse_code(code: &str) -> Vec<Instruction> {
        code.split(';')
            .into_iter()
            .map(|mut line| {
                line = line.trim();

                let mut instruction = Instruction::NONE;

                if line.starts_with("print") {
                    line = &line[5..].trim();

                    if !line.starts_with('$') {
                        panic!("Unknown code!");
                    }

                    line = &line[1..].trim();

                    instruction = Instruction::PRINT(line.parse::<i32>().unwrap())
                }

                instruction
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::parser::Instruction;

    #[test]
    fn it_parses_each_line_separated_by_semicolon() {
        let parser = Parser::parse("print $3; print $5;".to_owned());

        assert!(matches!(parser.instructions[0], Instruction::PRINT(3)));
        assert!(matches!(parser.instructions[1], Instruction::PRINT(5)));

        assert!(matches!(parser.instructions.get(2).unwrap(), Instruction::NONE));
    }
}
