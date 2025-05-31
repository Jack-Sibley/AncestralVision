use std::fs;
use std::io::{ErrorKind, Write};
use regex::Regex;
use rayon::prelude::*;

fn main() {
    let untrimmed_contents = fs::read_to_string("in/mcr_slice.txt")
        .expect("Should have been able to read the file");
    let contents = untrimmed_contents.trim_start_matches('\u{FEFF}');

    let lines: Vec<&str> = contents
        .lines()
        .collect();

    let html_lines = lines
        .par_iter()
        .filter_map(|line| {
            match line {
                &"" => None,
                _ => Some(handle_cr_line(line)),
            }
        })
    .collect::<Vec<String>>();

    match fs::remove_file("out.html") {
        Err(error) => match error.kind() {
            ErrorKind::NotFound => (),
            _ => panic!("Should have been able to delete the file")
        },
        Ok(_) => ()
    }
    let mut out_file = fs::File::create("out.html").unwrap();

    let template = fs::read_to_string("in/template.html").unwrap();
    let html = html_lines.join("\n");

    out_file.write_all(template.replace("<!--CONTENTS-->", html.as_str()).as_bytes()).unwrap();
}

const MARKUP_RULES: [(&str, &str); 4] = [
    (
        // language=RegExp
        r"(^\d{1,2}\.\s.+)",
        // language=HTML
        "<h1>$0</h1>"
    ),
    (
        // language=RegExp
        r"(^\d{3}\.\s.+)",
        // language=HTML
        "<h2>$0</h2>"
    ),
    (
        // language=RegExp
        r"(^\d{3}\.\d\S+)(\s.+)",
        // language=HTML
        "<p><strong>$1</strong>$2</p>"
    ),
    (
        // language=RegExp
        r"(^Example:)(\s.+)",
        // language=HTML
        r"<article><header><strong>Example</strong></header>$2</article>"
    )

];

fn handle_cr_line(s: &&str) -> String {
    let mut line: String = s.to_string();
    for rule in MARKUP_RULES {
        line = Regex::new(rule.0).unwrap()
            .replace_all(&line, rule.1).to_string();
    }
    line
}
