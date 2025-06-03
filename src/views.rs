use std::fs::File;
use std::io::Write;
use maud::Render;

pub mod rules_page;

pub use rules_page::RulesPage;

pub trait View {
	fn to_file(&self, file: &mut File) -> Result<(), std::io::Error>;
}
impl<T: Render> View for T {
	fn to_file(&self, file: &mut File) -> Result<(), std::io::Error> {
		file.write_all(self.render().into_string().as_bytes())?;
		Ok(())
	}
}