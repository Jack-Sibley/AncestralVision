use crate::error::ParseError;
use crate::models::Rule;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Subsection {
    pub number: usize,
    pub title: String,
    pub rules: Vec<Rule>,
}
thread_local! {
    static SPLIT_SUBSECTION: Regex = Regex::new(r"\n\d{3}\.\s.+").unwrap();
    static CAPTURE_SUBSECTION: Regex = Regex::new(r"^(\d{3})\.\s(.+)([\s\S]*)").unwrap();
}
impl Subsection {
    pub fn new(text: &str) -> Result<Subsection, ParseError> {
        let subsection_captures = match CAPTURE_SUBSECTION.with(|regex| regex.captures(text)) {
            Some(c) => c,
            None => return Err(ParseError { 
                culprit: text.to_string(),
                message: "Each subsection must begin with a 3-digit subsection number and a title like so:\n111. Tokens".to_string() 
            })
        };

        let ret = Subsection {
            // This is safe because we checked for a match above.
            number:subsection_captures[1].parse().unwrap(),
            title: subsection_captures[2].to_string(),
            rules: match Rule::vec_from_text(subsection_captures[3].trim()) {
                Ok(r) => r,
                Err(e) => {
                    let mut err = e.clone();
                    err.culprit = format!(
                        "Subsection: {} > {}",
                        subsection_captures[1].to_string(),
                        err.culprit
                    );
                    return Err(err);
                }
            },
        };
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;
    use crate::models::{ChildRule, Rule, Subsection};

    #[test]
    fn it_works() {
        let subsection = Subsection::new(
            "111. Tokens\n\
                \n\
                111.1. Some effects put tokens onto the battlefield. A token is a marker used to represent any permanent that isn’t represented by a card.",
        );
        let intended: Result<_, ParseError> = Ok(Subsection {
            number: 111,
            title: "Tokens".to_string(),
            rules: vec![Rule {
                rule: ChildRule {
                    code: "111.1.".to_string(),
                    contents: "Some effects put tokens onto the battlefield. A token is a marker used to represent any permanent that isn’t represented by a card.".to_string(),
                    examples: vec![],
                },
                subrules: vec![],
            }],
        });
        assert_eq!(subsection, intended);
    }

    #[test]
    fn incorrect_syntax () {
        let result = Subsection::new("111.Tokens");
        let err = result.unwrap_err();
        assert_eq!(err.message, "Each subsection must begin with a 3-digit subsection number and a title like so:\n111. Tokens");
    }

    #[test]
    fn non_numeric_subsection() {
        let result = Subsection::new("ABC. Tokens");
        let err = result.unwrap_err();
        assert_eq!(err.message, "Each subsection must begin with a 3-digit subsection number and a title like so:\n111. Tokens");
    }

}
