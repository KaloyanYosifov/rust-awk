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

    pub fn words_len(&self) -> usize {
        self.words.len()
    }

    pub fn word(&self, mut index: usize) -> Option<&String> {
        index = if index == 0 { 1 } else { index };

        self.words.get(index - 1)
    }

    pub fn line(&self) -> String {
        let string_capacity = self.words()
            .iter()
            .fold(0, |acc, word| {
                // the plus one is for the space we have between words
                acc + word.len() + 1
            });

        self.words
            .iter()
            .fold(
                String::with_capacity(string_capacity),
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
    fn it_returns_empty_line_if_line_is_empty() {
        let line = String::from("");
        let parsed_line = Line::new(line.clone());

        assert_eq!(line, parsed_line.line());
    }

    #[test]
    fn it_parses_the_line_into_words() {
        let line = String::from("Hey there stranger!");
        let parsed_line = Line::new(line.clone());

        assert_eq!("Hey", parsed_line.word(1).unwrap());
        assert_eq!("there", parsed_line.word(2).unwrap());
        assert_eq!("stranger!", parsed_line.word(3).unwrap());
    }

    #[test]
    fn it_returns_none_if_the_line_doesnt_have_a_word_at_index() {
        let line = String::from("Hey");
        let parsed_line = Line::new(line.clone());

        assert_eq!("Hey", parsed_line.word(1).unwrap());
        assert!(matches!(parsed_line.word(2), None));
    }

    #[test]
    fn it_returns_empty_string_for_first_word_if_line_has_nothing() {
        let line = String::from("");
        let parsed_line = Line::new(line.clone());

        assert_eq!("", parsed_line.word(1).unwrap());
    }
}
