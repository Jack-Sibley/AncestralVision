use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;
use cr_parse::models::{Rulebook, Section, Subsection};
use cr_maud::RuleComponent;
use crate::views::{IntoFile, PageGenerator};
use maud::{DOCTYPE, Markup, Render, html};
use crate::components::Sidebar;

fn to_kebab_case(str: &str) -> String {
    str.to_lowercase()
        .replace(" ", "-")
        .replace(",", "")
        .replace("/", "")
}

pub struct RulesPage<'a> {
    pub rulebook: &'a Rulebook,
    pub section_index: usize, 
    pub subsection_index: usize,
}

impl<'a> RulesPage<'a> {
    pub fn new(rulebook: &Rulebook, section_index: usize, subsection_index: usize) -> RulesPage {
        RulesPage {
            rulebook,
            section_index,
            subsection_index,
        }
    }
    
    pub fn section(&self) -> &Section {
        &self.rulebook.sections[self.section_index]
    }
    
    pub fn subsection(&self) -> &Subsection {
        &self.section().subsections[self.subsection_index]
    }
    
}

impl<'a> Render for RulesPage<'a> {
    fn render(&self) -> Markup {

        html! {
            (DOCTYPE);
            html lang="en" {
                head {
                    meta charset="UTF-8";
                    title {"MTG Rules"}
                    meta name="viewport" content="width=device-width,initial-scale=1";
                    meta name="description" content "";
                    link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css";
                    link rel="stylesheet" href="/css/rules.css";
                }
                body {
                    main .with-sidebar {
                        (Sidebar {rule_page: self})
                        section .container {
                            h1 { "MTG Rules: "(self.section().title)" - "(self.subsection().title)}
                            (RuleComponent(self.subsection()))
                        }
                    }
                }
            }
        }
    }
}

impl<'a> IntoFile for RulesPage<'a> {
    fn get_file_path(&self) -> String {
        format!(
            "{}/{}/index.html",
            to_kebab_case(&self.section().title),
            to_kebab_case(&self.subsection().title)
        )
    }
}

turf::style_sheet!("src/styles/styles.scss");
impl PageGenerator for Rulebook {
    fn generate_web_pages(&self, dir_path: &Path) -> Result<(), Error> {
        for (i, section) in self.sections.iter().enumerate() {
            for (j, _subsection) in section.subsections.iter().enumerate() {
                RulesPage::new(&self, i, j).to_file(dir_path)?
            }
        }
        
        let css_path = dir_path.join("css/rules.css");
        fs::create_dir_all(css_path.parent().unwrap())?;
        let mut css_file = File::create(css_path)?;
        css_file.write_all(STYLE_SHEET.as_ref())?;
        Ok(())
    }
}