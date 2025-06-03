use crate::models::{Book, Section, Subsection};
use maud::{DOCTYPE, Markup, Render, html};
use crate::views::IntoFile;
use crate::helpers::*;

pub struct RulesPage<'a> {
    section: &'a Section,
    subsection: &'a Subsection,
}

impl<'a> RulesPage<'a> {
    pub fn new(book: &Book, section_index: usize, subsection_index: usize) -> RulesPage {
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
                        h1 { "MTG Rules: "(&self.section.title)" - "(&self.subsection.title)}
                        (self.subsection)
                    }
                }
            }
        }
    }
}

impl<'a> IntoFile for RulesPage<'a> {
    fn get_file_path(&self) -> String {
        format!("{}/{}/index.html", to_kebab_case(&self.section.title), to_kebab_case(&self.subsection.title))
    }
}