use crate::error::ParseError;
use crate::models::subsection::Subsection;
use rayon::prelude::*;
use regex::Regex;
use regex_split::RegexSplit;

#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub number: u32,
    pub title: String,
    pub subsections: Vec<Subsection>,
}
thread_local! {
    static CAPTURE_SECTION: Regex = Regex::new(r"^\d{1,2}\.\s(.+)([\s\S]*)").unwrap();
    static SPLIT_SUBSECTION: Regex = Regex::new(r"\n\d{3}\.\s.+").unwrap();
    static MATCH_SUBSECTION: Regex = Regex::new(r"^\d{3}\.\s").unwrap();
}
impl Section {
    pub fn new(section_no: u32, text: &str) -> Result<Section, ParseError> {
        let section_captures_opt = CAPTURE_SECTION.with(|regex| regex.captures(text));
        let section_captures = match section_captures_opt {
            Some(c) => c,
            None => return Err(ParseError {
                culprit: text.to_string(),
                message: "Each section must begin with a 1 or 2 digit section number and a title like so:\n1. Game Concepts".to_string()
            })
        };
        let subsection_texts: Vec<String> = SPLIT_SUBSECTION.with(|re| {
            re.split_inclusive_left(section_captures[2].trim())
                .map(|s| s.trim().to_string())
                .collect()
        });
        
        if subsection_texts.len() == 1 && subsection_texts[0].is_empty() {
            return Err(ParseError {
                culprit: text.to_string(),
                message: "Each section must contain at least one subsection.".to_string()
            });
        }
        
        MATCH_SUBSECTION.with(|re| {
            if !re.is_match(&subsection_texts[0]) {
                return Err(ParseError {
                    culprit: text.to_string(),
                    message: "Each subsection must begin with a 3-digit subsection number and a title like so:\n111. Tokens".to_string()
                });
            }
            Ok(())
        })?;
        
        let ret = Section {
            number: section_no,
            title: section_captures[1].to_string(),
            subsections: match subsection_texts
                .par_iter()
                .map(|x| Subsection::new(x))
                .collect::<Result<Vec<Subsection>, ParseError>>()
            {
                Ok(v) => v,
                Err(e) => {
                    let mut err = e.clone();
                    err.culprit = format!("Section: {} > {}", section_no, err.culprit);
                    return Err(err);
                }
            },
        };
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ChildRule, Rule, Section, Subsection};

    #[test]
    fn it_works() {
        let section = Section::new(
            1,
            "1. Game Concepts\n\
                111. Tokens\n\
                \n\
                111.1. Some effects put tokens onto the battlefield. A token is a marker used to represent any permanent that isn’t represented by a card.",
        );
        let intended = Ok(Section {
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
        });
        
        assert_eq!(section, intended);
    }

    #[test]
    fn invalid_syntax() {
        let section = Section::new(1, "Invalid Syntax");
        let err = section.unwrap_err();
        
        assert_eq!(err.message, "Each section must begin with a 1 or 2 digit section number and a title like so:\n1. Game Concepts")
    }

    #[test]
    fn invalid_subsection() {
        let section = Section::new(1, "1. Game Concepts\n\n100 General");
        let err = section.unwrap_err();
        
        assert_eq!(err.message, "Each subsection must begin with a 3-digit subsection number and a title like so:\n111. Tokens")
    }
    

    #[test]
    fn no_subsections() {
        let section = Section::new(1, "1. Game Concepts");
        let err = section.unwrap_err();
        
        assert_eq!(err.message.as_str(), "Each section must contain at least one subsection.");
    }
}
