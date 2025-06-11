use std::error::Error;
use cr_parse::models::Rulebook;
use std::fs;
use std::path::Path;
use cr_html::views::PageGenerator;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_contents = fs::read_to_string("in/mcr_slice.txt")?;
    let contents = raw_contents.trim_start_matches('\u{FEFF}');
    let out_dir = Path::new("dist");
    
    Rulebook::new(contents).generate_web_pages(out_dir)?;
    
    Ok(())
}
