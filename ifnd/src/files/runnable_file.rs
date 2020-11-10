use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use crate::files::file_base::FileBase;
use crate::files::lnk_parser::LnkFile;
use crate::files::exe_file::ExeFile;

pub trait RunnableFileTrait {
    fn run(&self) -> Result<Command, Box<dyn Error>>;
}

#[derive(Debug, Clone)]
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
}

impl FileBase for RunnableFile {
    fn get_path(&self) -> &PathBuf {
        match self {
            RunnableFile::LnkFile(v) => v.get_path(),
            RunnableFile::ExeFile(v) => v.get_path(),
        }
    }

    fn get_search_path(&self) -> &str {
        match self {
            RunnableFile::LnkFile(v) => v.get_search_path(),
            RunnableFile::ExeFile(v) => v.get_search_path(),
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