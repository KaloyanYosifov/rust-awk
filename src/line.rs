#[derive(Debug)]
pub struct Line {
    line: String,
    words: Vec<String>,
}

impl Line {
    pub fn new(line: String) -> Self {
        Self {
            line,
            words: line.split(' ')
                .collect(),
        }
    }
}
