use std::error::Error;
use std::path::PathBuf;
use crate::runnable_file::exe_file::ExeFile;
use crate::runnable_file::lnk_parser::LnkFile;
use std::ffi::OsStr;
use std::process::Command;

pub trait RunnableFileTrait {
    fn run(&self) -> Result<Command, Box<dyn Error>>;
    fn get_file_path(&self) -> &PathBuf;
}

pub enum RunnableFile {
    LnkFile(LnkFile),
    ExeFile(ExeFile),
}

impl RunnableFileTrait for RunnableFile {
    fn run(&self) -> Result<Command, Box<dyn Error>> {
        match self {
            RunnableFile::LnkFile(v) => v.run(),
            RunnableFile::ExeFile(v) => v.run(),
        }
    }

    fn get_file_path(&self) -> &PathBuf {
        match self {
            RunnableFile::LnkFile(v) => v.get_file_path(),
            RunnableFile::ExeFile(v) => v.get_file_path(),
        }
    }
}

pub fn to_file(path: PathBuf) -> Result<RunnableFile, ()> {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
        "lnk" => Ok(RunnableFile::LnkFile(LnkFile::new(path)?)),
        "exe" => Ok(RunnableFile::ExeFile(ExeFile::new(path))),
        _ => Err(())
    }
}