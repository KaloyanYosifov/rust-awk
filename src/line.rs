#[derive(Debug)]
pub struct Line {
    words: Vec<String>,
}

impl Line {
    pub fn new(line: String) -> Self {
        Self {
            words: line.split(' ')
                .map(|word| word.to_owned())
                .collect(),
        }
    }
}

// getters
impl Line {
    pub fn words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn line(&self) -> String {
        self.words
            .iter()
            .map(|value| value)
            .fold(
                String::from(""),
                |mut acc, word| {
                    // add a space between words if we are not in the beginning
                    if acc.len() > 0 {
                        acc.push_str(" ");
                    }

                    acc.push_str(word);

                    acc
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::line::Line;

    #[test]
    fn it_can_get_the_line() {
        let line = String::from("Hey there stranger!");
        let parsed_line = Line::new(line.clone());

        assert_eq!(line, parsed_line.line());
    }

    #[test]
    fn it_parses_the_line_into_words() {
        let line = String::from("Hey there stranger!");
        let parsed_line = Line::new(line.clone());

        assert_eq!("Hey", parsed_line.words[0]);
        assert_eq!("there", parsed_line.words[1]);
        assert_eq!("stranger!", parsed_line.words[2]);
    }
}
