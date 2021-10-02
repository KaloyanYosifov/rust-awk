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
        // get the removed code and put it as a &str in parsed code as rust
        // doesn't allow doint .as_str() inline, as for some reason it shows that the call will go out of scope
        // makes no sense to me since we are calling it inside the function which has the whole scope
        // but ok.
        let removed_code = Self::remove_unused_characters_in_code(code);
        let mut parsed_code = removed_code.as_str();

        if parsed_code.starts_with('/') {
            let other_slash_index = parsed_code.rfind('/').unwrap();
            let regex = (&parsed_code[1..other_slash_index]).to_owned();
            parsed_code = &parsed_code[other_slash_index + 1..];

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

    fn remove_unused_characters_in_code(code: &str) -> String {
        let re = regex::Regex::new(r"\{|\}").unwrap();
        let removed = re.replace_all(code, "").to_string();

        removed
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
