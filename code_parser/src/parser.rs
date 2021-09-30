#[derive(Debug)]
pub enum Instruction {
    NONE,
    PRINT(i32),
    REGEX(String),
}

#[derive(Debug)]
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
        let mut instructions: Vec<Instruction> = vec![];

        let mut parsed_code = code
            .replace("{", "")
            .replace("}", "");

        if parsed_code.starts_with('/') {
            let other_slash_index = parsed_code.rfind('/').unwrap();
            let regex = (&parsed_code[1..other_slash_index]).to_owned();
            parsed_code = (&parsed_code[other_slash_index + 1..]).to_owned();

            instructions.push(Instruction::REGEX(regex));
        }

        instructions.append(
            &mut parsed_code.split(';')
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
                .filter(|instruction| !matches!(instruction, Instruction::NONE))
                .collect::<Vec<Instruction>>()
        );

        instructions
    }
}

impl Parser {
    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
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

        assert!(matches!(parser.instructions.get(2), None));
    }

    #[test]
    fn it_has_no_instructions_if_they_are_not_recognized() {
        let parser = Parser::parse("hallo $3; print $5;".to_owned());

        assert!(matches!(parser.instructions[0], Instruction::PRINT(5)));
        assert!(matches!(parser.instructions.get(1), None));
    }

    #[test]
    fn it_parses_the_code_even_if_it_has_surrounding_curly_brackets() {
        let parser = Parser::parse("{hallo $3; print $5};".to_owned());

        assert!(matches!(parser.instructions[0], Instruction::PRINT(5)));
        assert!(matches!(parser.instructions.get(1), None));
    }

    #[test]
    fn it_parses_the_print_instruction() {
        let parser = Parser::parse("{print $3;}".to_owned());

        assert!(matches!(parser.instructions[0], Instruction::PRINT(3)));
    }
}
