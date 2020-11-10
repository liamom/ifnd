use std::path::PathBuf;
use crate::files::file_base::FileBase;

#[derive(Debug, Clone)]
pub struct AnyFile {
    pub path: PathBuf,
}

impl FileBase for AnyFile {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }

    fn get_search_path(&self) -> &str {
        let buf = std::env::current_dir().unwrap();
        let option = self.path.strip_prefix(buf).unwrap().to_str();
        return option.unwrap();

    }
}