use crate::{ChildRule, Rule, Subsection};

pub trait ToHtml {
    fn to_html(&self) -> String;
}

pub trait PagesOut {
    fn to_web_pages(&self) -> Vec<String>;
}

impl ToHtml for ChildRule {
    fn to_html(&self) -> String {
        let mut html = format!(r"<p><strong>{}</strong> {}</p>", self.code, self.contents);
        for example in &self.examples {
            html.push_str(&format!(
                "<article><header><strong>Example</strong></header>{}</article>",
                example
            ));
        }
        html
    }
}

impl ToHtml for Rule {
    fn to_html(&self) -> String {
        let mut html = self.rule.to_html();
        if !self.subrules.is_empty() {
            html.push_str(&format!(
                "<ul>{}</ul>",
                self.subrules
                    .iter()
                    .map(|r| r.to_html())
                    .fold(String::new(), |a, b| a + "<li>" + &b + "</li>")
                    .as_str()
            ));
        }
        html
    }
}

impl ToHtml for Subsection {
    fn to_html(&self) -> String {
        let mut html = format!("<h1>{}. {}</h1>", self.number, self.title);
        for rule in &self.rules {
            html.push_str(&rule.to_html());
        }
        html
    }
}

pub trait ToKebabCase {
    fn to_kebab_case(&self) -> String;
}

impl ToKebabCase for String {
    fn to_kebab_case(&self) -> String {
        self.to_lowercase().replace(" ","-").replace(",","").replace("/","")
    }
}