use std::path::PathBuf;

pub trait FileBase {
    fn get_path(&self) -> &PathBuf;
    fn get_search_path(&self) -> &str;
}
