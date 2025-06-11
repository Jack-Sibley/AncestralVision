use crate::models::Section;
use rayon::prelude::*;
use regex::Regex;
use regex_split::RegexSplit;

#[derive(Debug)]
pub struct Rulebook {
    pub sections: Vec<Section>,
}

impl Rulebook {
    pub fn new(text: &str) -> Rulebook {
        let section_re = Regex::new(r"\n(\d{1,2}\.\s.+)").unwrap();
        let sections_text: Vec<&str> = section_re
            .split_inclusive_left(text)
            .map(|s| s.trim())
            .collect();
        Rulebook {
            sections: sections_text
                .par_iter()
                .enumerate()
                .map(|(index, text)| Section::new((index as u32) + 1, text))
                .collect(),
        }
    }
}
