use cr_html::models::{Book, Routable};
use std::fs;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let raw_contents = fs::read_to_string("in/mcr_slice.txt")?;
    let contents = raw_contents.trim_start_matches('\u{FEFF}');
    let out_dir = Path::new("dist");
    Book::from_text(contents).generate_pages(out_dir)?;
    Ok(())
}
