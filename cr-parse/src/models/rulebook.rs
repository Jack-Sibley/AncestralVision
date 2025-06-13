use crate::error::ParseError;
use crate::models::Section;
use rayon::prelude::*;
use regex::Regex;
use regex_split::RegexSplit;

#[derive(Debug, Clone, PartialEq)]
pub struct Rulebook {
    pub sections: Vec<Section>,
}
thread_local! {
    static SPLIT_SECTION: Regex = Regex::new(r"\n(\d{1,2}\.\s.+)").unwrap();
    static MATCH_SECTION_HEADER: Regex = Regex::new(r"^(\d{1,2}\.\s)").unwrap();   
}
impl Rulebook {
    pub fn new(text: &str) -> Result<Rulebook, ParseError> {
        if text.is_empty() {
            return Err(ParseError {
                culprit: text.to_string(),
                message: "The rulebook is empty".to_string(),
            });
        }
        
        let sections_text: Vec<&str> = SPLIT_SECTION.with(|re| {
            re
                .split_inclusive_left(text)
                .map(|s| s.trim())
                .collect()
        });
        if sections_text.len() == 1 && MATCH_SECTION_HEADER.with(|re| !re.is_match(sections_text[0])) {
            return Err(ParseError { 
                culprit: sections_text[0].to_string(),
                message: "Each section must begin with a 1 or 2 digit section number and a title like so:\n1. Game Concepts".to_string()
            });
        }
        let ret = Rulebook {
            sections: sections_text
                .par_iter()
                .enumerate()
                .map(|(index, text)| Section::new((index as u32) + 1, text))
                .collect::<Result<Vec<Section>, ParseError>>()?,
        };
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ChildRule, Rule, Rulebook, Section, Subsection};

    #[test]
    fn it_works() {
        let rulebook = Rulebook::new(
            "1. Game Concepts\n\
                111. Tokens\n\
                \n\
                111.1. Some effects put tokens onto the battlefield. A token is a marker used to represent any permanent that isn’t represented by a card.",
        );
        
        let intended = Ok(Rulebook {
            sections: vec![Section {
                number: 1,
                title: "Game Concepts".to_string(),
                subsections: vec![ Subsection {
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
                }],
            }]
        });
        
        assert_eq!(rulebook, intended);
    }
    #[test]
    fn invalid_first_section() {
        let section = Rulebook::new("Invalid syntax");
        let err = section.unwrap_err();
        assert_eq!(err.culprit, "Invalid syntax");
        assert_eq!(err.message, "Each section must begin with a 1 or 2 digit section number and a title like so:\n1. Game Concepts");
    }
    
    #[test]
    fn empty_input() {
        let section = Rulebook::new("");
        let err = section.unwrap_err();
        assert_eq!(err.culprit, "");
        assert_eq!(err.message, "The rulebook is empty");
    }
}