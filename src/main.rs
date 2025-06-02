use cr_html::Section;
use cr_html::html_gen::{ToHtml, ToKebabCase};
use regex::Regex;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::{Write, Result};
use regex_split::RegexSplit;


fn main() -> Result<()>{
    let untrimmed_contents =
        fs::read_to_string("in/mcr_slice.txt")?;
    let contents = untrimmed_contents.trim_start_matches('\u{FEFF}');
    let section_re = Regex::new(r"\n(\d{1,2}\.\s.+)").unwrap();
    let sections: Vec<&str> = section_re.split_inclusive_left(contents).map(|s| s.trim()).collect();

    let template = fs::read_to_string("in/template.html")?;

    let all_rules = Section::from_sections_text(sections);
    for section in all_rules {
        let section_slug = section.title.to_kebab_case();
        for subsection in section.subsections {
            let subsection_slug = subsection.title.to_kebab_case();

            create_dir_all(format!("dist/{}/{}", section_slug, subsection_slug))?;
            let mut file = File::create(format!("dist/{}/{}/index.html", section_slug, subsection_slug))?;

            let out_html = template.replace(r"<!--CONTENTS-->", &subsection.to_html());
            file.write_all(&out_html.into_bytes())?;
        }
    }
    Ok(())
}
