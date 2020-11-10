use std::path::PathBuf;
use std::error::Error;
use std::process::Command;
use crate::files::runnable_file::RunnableFileTrait;
use crate::files::file_base::FileBase;

#[derive(Debug, Clone)]
pub struct ExeFile {
    path: PathBuf
}

impl ExeFile {
    pub fn new (path: PathBuf) -> Self{
        ExeFile {
            path: path
        }
    }
}

impl RunnableFileTrait for ExeFile {
    fn run(&self) -> Result<Command, Box<dyn Error>>
    {
        let output = Command::new(self.path.to_str().ok_or("could not get path")?);
        Ok(output)
    }
}

impl FileBase for ExeFile {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }

    fn get_search_path(&self) -> &str {
        self.path.to_str().unwrap()
    }
}