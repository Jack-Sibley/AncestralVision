use maud::Render;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) mod rules_page;

pub trait View {
    fn to_file(&self, file: &mut File) -> Result<(), std::io::Error>;
}
impl<T: Render> View for T {
    fn to_file(&self, file: &mut File) -> Result<(), std::io::Error> {
        file.write_all(self.render().into_string().as_bytes())?;
        Ok(())
    }
}
pub trait IntoFile: View {
    fn get_file_path(&self) -> String;
    fn to_file(&self, dir_path: &Path) -> Result<(), std::io::Error> {
        let path = Path::new(dir_path).join(self.get_file_path());
        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        <Self as View>::to_file(self, &mut file)
    }
}

pub trait PageGenerator {
    fn generate_web_pages(&self, dir_path: &Path) -> Result<(), std::io::Error>;
}