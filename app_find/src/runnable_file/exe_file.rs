use std::path::PathBuf;
use crate::runnable_file::runnable_file::RunnableFileTrait;
use std::error::Error;
use std::ffi::OsStr;
use std::process::Command;

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

    fn get_file_path(&self) -> &PathBuf {
        return &self.path;
    }
}