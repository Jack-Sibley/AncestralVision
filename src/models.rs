mod book;
mod rule;
mod section;
mod subsection;

use std::path::Path;
pub use book::Book;
pub use rule::{ChildRule, Rule};
pub use section::Section;
pub use subsection::Subsection;

pub trait Routable {
	fn generate_pages(&self, dir_path: &Path) -> Result<(), std::io::Error>;
}