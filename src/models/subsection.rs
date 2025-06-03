use crate::models::Rule;
use regex::Regex;

#[derive(Debug)]
pub struct Subsection {
    pub number: usize,
    pub title: String,
    pub rules: Vec<Rule>,
}
thread_local! {
    static SPLIT_SUBSECTION: Regex = Regex::new(r"\n\d{3}\.\s.+").unwrap();
    static CAPTURE_SUBSECTION: Regex = Regex::new(r"^(\d{3})\.\s(.++)([\s\S]+)").unwrap();
}
impl Subsection {
    pub fn from_text(text: &String) -> Subsection {
        let subsection_captures = CAPTURE_SUBSECTION.with(|regex| regex.captures(text).unwrap());
        Subsection {
            number: subsection_captures[1].parse().unwrap(),
            title: subsection_captures[2].to_string(),
            rules: Rule::vec_from_text(subsection_captures[3].trim()),
        }
    }
}
