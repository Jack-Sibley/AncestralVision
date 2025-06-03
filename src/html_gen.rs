use crate::models::{ChildRule, Rule, Subsection};
use maud::{Markup, Render, html};

impl Render for ChildRule {
    fn render(&self) -> Markup {
        html! {
            p { strong { (self.code) } (self.contents) }
            @for example in &self.examples {
                article {
                    header {  strong { "Example" } }
                    (example)
                }
            }
        }
    }
}

impl Render for Rule {
    fn render(&self) -> Markup {
        html! {
            (self.rule)
            @if !self.subrules.is_empty() {
                ul {
                    @for subrule in &self.subrules {
                        li { (subrule) }
                    }
                }
            }
        }
    }
}

impl Render for Subsection {
    fn render(&self) -> Markup {
        html! {
            h1 {(self.number)". "(self.title) }
            @for rule in &self.rules {
                (rule)
            }
        }
    }
}
