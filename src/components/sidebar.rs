use maud::{Markup, Render, html};
use crate::views::rules_page::RulesPage;

pub struct Sidebar<'a> {
	pub rule_page: &'a RulesPage<'a>,
}
impl Render for Sidebar<'_> {
    fn render(&self) -> Markup {
	    let rulebook = self.rule_page.rulebook;
        html! {
            aside {
                @for (section_index, section) in rulebook.sections.iter().enumerate() {
	            nav {
				        p { bold { (section.number)". "(section.title)} }
				        @if section.subsections.len() > 0 {
					        ul {
						        @for (subsection_index, subsection) in section.subsections.iter().enumerate() {
                                    @let section_name = format!("{}. {}", subsection.number, subsection.title);
							        li {
								        @if section_index == self.rule_page.section_index && subsection_index == self.rule_page.subsection_index  {
									        a {(section_name)}
								        }
								        @else {
									        a href="" {(section_name)}
								        }
							        }
						        }
					        }
				        }
			        }
	            }
            }
        }
    }
}
