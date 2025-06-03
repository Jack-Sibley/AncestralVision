use cr_html::models::Book;
use std::fs;
use std::io::Result;
use cr_html::views::{View, RulesPage};

fn main() -> Result<()> {
    let untrimmed_contents = fs::read_to_string("in/mcr_slice.txt")?;
    let contents = untrimmed_contents.trim_start_matches('\u{FEFF}');
    let book = Book::from_text(contents);

    for section in book.sections {
        let section_slug = to_kebab_case(&section.title);
        for subsection in &section.subsections {
            let subsection_slug = to_kebab_case(&subsection.title);
            fs::create_dir_all(format!("dist/{section_slug}/{subsection_slug}"))?;
            let mut file =
                fs::File::create(format!("dist/{section_slug}/{subsection_slug}/index.html"))?;

            RulesPage { subsection: &subsection }.to_file(&mut file)?
        }
    }
    Ok(())
}

fn to_kebab_case(str: &str) -> String {
    str.to_lowercase()
        .replace(" ", "-")
        .replace(",", "")
        .replace("/", "")
}