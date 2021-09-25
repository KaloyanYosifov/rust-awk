use crate::line::Line;

type LineList = Vec<Line>;

pub struct Lines {
    lines: LineList,
}

impl Lines {
    pub fn new(lines: LineList) -> Self {
        Self {
            lines
        }
    }
}

impl Lines {
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn total_lines_len(&self) -> usize {
        self
            .lines
            .iter()
            .fold(0, |mut acc, line| acc + line.words_len())
    }

    pub fn line(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }

    pub fn word(&self, index: usize) -> Vec<String> {
        self
            .lines
            .iter()
            .map(|line| line.word(index).unwrap_or(&"".to_owned()).clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::line::Line;
    use crate::lines::Lines;

    #[test]
    fn it_can_be_created_with_collection_of_lines() {
        let lines: Vec<Line> = vec![
            Line::new("Hey There".to_owned()),
            Line::new("Hallo welcome!".to_owned()),
        ];
        let line_list = Lines::new(lines);

        assert_eq!(2, line_list.len());
    }

    #[test]
    fn it_can_get_all_the_words_at_a_location_from_lines() {
        let lines: Vec<Line> = vec![
            Line::new("Hey There".to_owned()),
            Line::new("Hallo welcome!".to_owned()),
        ];
        let line_list = Lines::new(lines);
        let words_at_location_1 = line_list.word(1);
        let words_at_location_2 = line_list.word(2);

        assert_eq!("Hey".to_owned(), words_at_location_1[0]);
        assert_eq!("Hallo".to_owned(), words_at_location_1[1]);

        assert_eq!("There".to_owned(), words_at_location_2[0]);
        assert_eq!("welcome!".to_owned(), words_at_location_2[1]);
    }

    #[test]
    fn it_returns_an_empty_string_if_there_is_nothing_on_the_line_on_the_location() {
        let lines: Vec<Line> = vec![
            Line::new("Hey There".to_owned()),
            Line::new("Hallo".to_owned()),
        ];
        let line_list = Lines::new(lines);
        let words_at_location = line_list.word(2);

        assert_eq!("There".to_owned(), words_at_location[0]);
        assert_eq!("".to_owned(), words_at_location[1]);
    }
}
