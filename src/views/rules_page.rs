use std::io::Error;
use std::path::Path;
use cr_parse::models::{Rulebook, Section, Subsection};
use cr_maud::RuleComponent;
use crate::views::{IntoFile, PageGenerator};
use maud::{DOCTYPE, Markup, Render, html};
use crate::components::Sidebar;
use crate::to_kebab_case;

pub struct RulesPage<'a> {
    section: &'a Section,
    subsection: &'a Subsection,
}

impl<'a> RulesPage<'a> {
    pub fn new(book: &Rulebook, section_index: usize, subsection_index: usize) -> RulesPage {
        let section = &book.sections[section_index];
        let subsection = &section.subsections[subsection_index];
        RulesPage {
            section,
            subsection,
        }
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
                }
                body {
                    main .container {
                        (Sidebar {})
                        section {
                            h1 { "MTG Rules: "(self.section.title)" - "(self.subsection.title)}
                            (RuleComponent(self.subsection))
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
            to_kebab_case(&self.section.title),
            to_kebab_case(&self.subsection.title)
        )
    }
}

impl PageGenerator for Rulebook {
    fn generate_web_pages(&self, dir_path: &Path) -> Result<(), Error> {
        for (i, section) in self.sections.iter().enumerate() {
            for (j, _subsection) in section.subsections.iter().enumerate() {
                RulesPage::new(&self, i, j).to_file(dir_path)?
            }
        }
        Ok(())
    }
}