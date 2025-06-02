pub mod html_gen;

use regex::Regex;
use rayon::prelude::*;
use regex_split::RegexSplit;

#[derive(Debug)]
pub struct Section {
    pub number: usize,
    pub title: String,
    pub subsections: Vec<Subsection>,
}
thread_local! {
    static CAPTURE_SECTION: Regex = Regex::new(r"^\d{1,2}\.\s(.++)([\s\S]+)").unwrap();
}
impl Section {
    pub fn from_sections_text(sections: Vec<&str>) -> Vec<Section> {
        sections
            .par_iter()
            .enumerate()
            .map(|(section_index, section_text)| {
                let section_captures = CAPTURE_SECTION.with(|regex| {
                    regex
                        .captures(section_text)
                        .unwrap()
                });
                Section {
                    number: section_index + 1,
                    title: section_captures[1].to_string(),
                    subsections: Subsection::from_section_text(section_captures[2].trim()),
                }
            })
            .collect()
    }
}

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
    pub fn from_section_text(section_text: &str) -> Vec<Subsection> {
        let split_subsections: Vec<String> = SPLIT_SUBSECTION.with(|re| {
            re.split_inclusive_left(section_text)
                .map(|s| s.trim().to_string())
                .collect()
        });

        split_subsections
            .par_iter()
            .map(|subsection_text| {
                let subsection_captures = CAPTURE_SUBSECTION.with(|regex| {
                    regex
                        .captures(subsection_text)
                        .unwrap()
                });
                Subsection {
                    number: subsection_captures[1].parse().unwrap(),
                    title: subsection_captures[2].to_string(),
                    rules: Rule::from_subsection_text(subsection_captures[3].trim()),
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Rule {
    pub rule: ChildRule,
    pub subrules: Vec<ChildRule>,
}

#[derive(Debug)]
pub struct ChildRule {
    pub code: String,
    pub contents: String,
    pub examples: Vec<String>,
}

thread_local! {
        static IS_BASE_RULE: Regex = Regex::new(
            // language=RegExp
            r"(^\d{3}\.\d+\.)\s(.+)$"
        ).unwrap();

        static IS_SUBRULE: Regex = Regex::new(
            // language=RegExp
            r"(^\d{3}\.\d+[a-z]+)\s(.+)$"
        ).unwrap();

        static IS_EXAMPLE: Regex = Regex::new(
            // language=RegExp
            r"Example:\s(.+)$"
        ).unwrap();
}
impl Rule {
    pub fn from_subsection_text(text: &str) -> Vec<Rule> {
        let mut out: Vec<Rule> = Vec::new();
        for line in text.lines() {
            if let Some(caps) = IS_BASE_RULE.with(|re| re.captures(line))/*.unwrap()*/ {
                out.push(Rule {
                    rule: ChildRule {
                        code: caps[1].to_string(),
                        contents: caps[2].to_string(),
                        examples: vec![],
                    },
                    subrules: vec![],
                })
            } else if let Some(current) = out.last_mut() {
                if let Some(caps) = IS_SUBRULE.with(|re| re.captures(line))/*.unwrap()*/ {
                    current.subrules.push(ChildRule {
                        code: caps[1].to_string(),
                        contents: caps[2].to_string(),
                        examples: vec![],
                    })
                } else if let Some(caps) = IS_EXAMPLE.with(|re| re.captures(line))/*.unwrap()*/ {
                    if let Some(current_subrule) = current.subrules.last_mut() {
                        current_subrule.examples.push(caps[1].to_string());
                    } else {
                        current.rule.examples.push(caps[1].to_string())
                    }
                }
            }
        }
        out
    }
}
