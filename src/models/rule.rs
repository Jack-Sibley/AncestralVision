use maud::{html, Markup, Render};
use regex::Regex;

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
    pub fn vec_from_text(text: &str) -> Vec<Rule> {
        let mut stack= Vec::new();
        for line in text.lines() {
            if let Some(caps) = IS_BASE_RULE.with(|re| re.captures(line)) {
                stack.push(Rule {
                    rule: ChildRule {
                        code: caps[1].to_string(),
                        contents: caps[2].to_string(),
                        examples: vec![],
                    },
                    subrules: vec![],
                })
            } else if let Some(current) = stack.last_mut() {
                if let Some(caps) = IS_SUBRULE.with(|re| re.captures(line)) {
                    current.subrules.push(ChildRule {
                        code: caps[1].to_string(),
                        contents: caps[2].to_string(),
                        examples: vec![],
                    })
                } else if let Some(caps) = IS_EXAMPLE.with(|re| re.captures(line)) {
                    if let Some(current_subrule) = current.subrules.last_mut() {
                        current_subrule.examples.push(caps[1].to_string());
                    } else {
                        current.rule.examples.push(caps[1].to_string())
                    }
                }
            }
        }
	    stack
    }
}

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