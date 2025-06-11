use maud::{html, Markup};
use cr_parse::models::{ChildRule, Rule, Subsection};

pub trait RuleComponentRender {
	fn render(&self) -> Markup;
}

pub struct RuleComponent<T: RuleComponentRender>(pub T);

impl<T: RuleComponentRender> maud::Render for RuleComponent<T> {
	fn render(&self) -> Markup {
		self.0.render()
	}
}

impl RuleComponentRender for &ChildRule {
	fn render(&self) -> Markup {
		html! {
            p { strong { (self.code) }" "(self.contents) }
            @for example in &self.examples {
                article {
                    header {  strong { "Example" } }
                    (example)
                }
            }
        }
	}
}

impl RuleComponentRender for &Rule {
	fn render(&self) -> Markup {
		let Rule { rule, subrules } = self;
		html! {
            (RuleComponent(rule))
            @if !self.subrules.is_empty() {
                ul {
                    @for subrule in subrules {
                        li { (RuleComponent(subrule)) }
                    }
                }
            }
        }
	}
}

impl RuleComponentRender for &Subsection {
	fn render(&self) -> Markup {
		html! {
            h2 {(self.number)". "(self.title) }
            @for rule in &self.rules {
                (RuleComponent(rule))
            }
        }
	}
}