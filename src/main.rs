use cr_html::models::Book;
use cr_html::template::page;
use std::fs;
use std::io::{Result, Write};

fn main() -> Result<()> {
    let untrimmed_contents = fs::read_to_string("in/mcr_slice.txt")?;
    let contents = untrimmed_contents.trim_start_matches('\u{FEFF}');
    let book = Book::from_text(contents);

    for section in book.sections {
        let section_slug = to_kebab_case(&section.title);
        for subsection in section.subsections {
            let subsection_slug = to_kebab_case(&subsection.title);
            fs::create_dir_all(format!("dist/{section_slug}/{subsection_slug}"))?;
            let mut file =
                fs::File::create(format!("dist/{section_slug}/{subsection_slug}/index.html"))?;

            let out_html = page(&subsection).into_string();
            file.write_all(&out_html.into_bytes())?;
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
