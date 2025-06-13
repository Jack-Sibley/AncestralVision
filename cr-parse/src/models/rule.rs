use crate::error::ParseError;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub rule: ChildRule,
    pub subrules: Vec<ChildRule>,
}

#[derive(Debug, Clone, PartialEq)]
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
    pub fn vec_from_text(text: &str) -> Result<Vec<Rule>, ParseError> {
        let mut stack = Vec::new();
        for (line_no, line) in text.lines().enumerate() {
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
            } else if !line.is_empty() {
                let last_valid_rule: &str = match stack.last() {
                    Some(rule) => &rule.rule.code,
                    None => "None",
                };
                return Err(ParseError {
                    culprit: format!("Last Valid Rule: {} > {}", last_valid_rule, line),
                    message: format!(
                        "Unexpected syntax on line {}, rules text should be one of the following:\n\
                    A Rule: 100.1. These Magic rules apply to any Magic game with two or more players, including two-player games and multiplayer games.\n\
                    A Subrule: 100.1a These Magic rules apply to Magic: the Gathering games with two or more players, including two-player games and multiplayer games.\n\
                    An Example: Example: \"This creature can’t block\" is an ability.\"",
                        line_no + 1
                    ),
                });
            }
        }
        Ok(stack)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;
    use crate::models::{ChildRule, Rule};

    #[test]
    fn it_works() {
        let rules = Rule::vec_from_text(
            "100.1. These Magic rules apply to any Magic game with two or more players, including two-player games and multiplayer games.\n\
                100.1a These Magic rules do not apply to other games.\n\
                Example: They won't be much help in Monopoly.",
        );

        let intended: Result<Vec<Rule>, ParseError> = Ok(vec![Rule {
            rule: ChildRule {
                code: "100.1.".to_string(),
                contents: "These Magic rules apply to any Magic game with two or more players, including two-player games and multiplayer games.".to_string(),
                examples: vec![],
            },
            subrules: vec![ChildRule {
                code: "100.1a".to_string(),
                contents: "These Magic rules do not apply to other games.".to_string(),
                examples: vec!["They won't be much help in Monopoly.".to_string()],
            }],
        }]);
        assert_eq!(rules.unwrap()[0], intended.unwrap()[0]);
    }

    #[test]
    fn invalid_syntax() {
        let rules = Rule::vec_from_text("This is not valid syntax");
        let err = rules.unwrap_err();
        assert_eq!(err.culprit, "Last Valid Rule: None > This is not valid syntax");
        assert!(err.message.starts_with(
            "Unexpected syntax on line 1, rules text should be one of the following:"
        ));
    }
}
