use parselnk::Lnk;
use std::path::{Path, PathBuf};
use std::fs;
use std::convert::TryFrom;
use std::process::Command;
use std::error::Error;
use crate::runnable_file::runnable_file::RunnableFileTrait;
use std::ffi::OsStr;
use std::ops::Deref;

pub struct LnkFile {
    _lnk: Lnk,
    path: PathBuf,
}

impl RunnableFileTrait for LnkFile {
    fn run(&self) -> Result<Command, Box<dyn Error>> {
        let path  = self.path.clone().into_os_string().into_string().map_err(|_| "error")?;
        let full_path = Path::new(&path[4..]);

        // hack to get around argument length limit
        let parent = full_path.parent()
            .ok_or("error getting parent")?
            .to_str()
            .ok_or("error converting parent to string")?;
        std::env::set_var("tmp", parent);

        let file_name = full_path.file_name()
            .ok_or("error getting file name")?
            .to_str()
            .ok_or("error converting file name to string")?;
        let command_str = format!(r#"$Env:tmp\{}"#, file_name);
        println!("command: {}, env: {}", command_str, parent);

        let mut output = Command::new(r#"powershell"#);
        output.arg("/c")
            .arg("start")
            .arg(command_str);

        Ok(output)
    }


    fn get_file_path(&self) -> &PathBuf {
        return &self.path;
    }
}

impl LnkFile {
    pub fn new(path: PathBuf) -> Result<LnkFile, ()> {
        assert_eq!(path.extension().unwrap(), "lnk");
        let lnk: Lnk = Lnk::try_from(path.as_path()).map_err(|_| ())?;
        Ok(LnkFile{
            _lnk: lnk,
            path
        })
    }

    pub fn _get_internal_path(&self) -> String {
        let rel_path: Option<PathBuf> = self._lnk.relative_path();
        let working_dir: Option<PathBuf> = self._lnk.working_dir();

        // working_dir
        return rel_path.clone()
            .and_then(|p| {
                working_dir.map(move |wd| wd.join(p))
            })
            .and_then(|p| fs::canonicalize(p).ok())
            .map(|p| p.into_os_string())
            .and_then(|p| p.into_string().ok())
            .unwrap_or_else(|| {
                rel_path.and_then(|p| {
                    p.into_os_string()
                     .into_string()
                     .ok()
                })
                .unwrap_or("".into())
            });
    }
}