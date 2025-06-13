mod views;
mod components;

use std::{
    error::Error,
    fs,
    path::Path
};
use cr_parse::models::Rulebook;
use views::PageGenerator;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_contents = fs::read_to_string("in/mcr_slice.txt")?;
    let contents = raw_contents.trim_start_matches('\u{FEFF}');
    let out_dir = Path::new("dist");
    
    Rulebook::new(contents)?.generate_web_pages(out_dir)?;

    Ok(())
}
