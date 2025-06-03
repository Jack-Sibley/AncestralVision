use std::io::Error;
use std::path::Path;
use crate::models::{Routable, Section};
use rayon::prelude::*;
use regex::Regex;
use regex_split::RegexSplit;
use crate::views::{IntoFile, RulesPage};

#[derive(Debug)]
pub struct Book {
    pub sections: Vec<Section>,
}

impl Book {
    pub fn from_text(text: &str) -> Book {
        let section_re = Regex::new(r"\n(\d{1,2}\.\s.+)").unwrap();
        let sections_text: Vec<&str> = section_re
            .split_inclusive_left(text)
            .map(|s| s.trim())
            .collect();
        Book {
            sections: sections_text
                .par_iter()
                .enumerate()
                .map(|(index, text)| Section::from_text((index as u32) + 1, text))
                .collect(),
        }
    }
}

impl Routable for Book {
    fn generate_pages(&self, dir_path: &Path) -> Result<(), Error> {
        for (i, section) in self.sections.iter().enumerate() {
            for (j, _subsection) in section.subsections.iter().enumerate() {
                RulesPage::new(&self, i, j).to_file(dir_path)?
            }
        }
        Ok(())
    }
}
