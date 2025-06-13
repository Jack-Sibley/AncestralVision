use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub culprit: String,
    pub message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {
            f,
            "There was an error parsing the following:\n{}\n\nError: {}",
            self.culprit,
            self.message
        }
    }
}

impl std::error::Error for ParseError {}
