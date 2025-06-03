use crate::models::subsection::Subsection;
use rayon::prelude::*;
use regex::Regex;
use regex_split::RegexSplit;

#[derive(Debug)]
pub struct Section {
    pub number: u32,
    pub title: String,
    pub subsections: Vec<Subsection>,
}
thread_local! {
    static CAPTURE_SECTION: Regex = Regex::new(r"^\d{1,2}\.\s(.++)([\s\S]+)").unwrap();
    static SPLIT_SUBSECTION: Regex = Regex::new(r"\n\d{3}\.\s.+").unwrap();
}
impl Section {
    pub fn from_text(section_no: u32, text: &str) -> Section {
        let section_captures = CAPTURE_SECTION.with(|regex| regex.captures(text).unwrap());
        let subsection_texts: Vec<String> = SPLIT_SUBSECTION.with(|re| {
            re.split_inclusive_left(section_captures[2].trim())
                .map(|s| s.trim().to_string())
                .collect()
        });
        Section {
            number: section_no,
            title: section_captures[1].to_string(),
            //subsections: Subsection::from_section_text(section_captures[2].trim()),
            subsections: subsection_texts
                .par_iter()
                .map(Subsection::from_text)
                .collect(),
        }
    }
}
