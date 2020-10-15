use parselnk::Lnk;
use std::path::{Path, PathBuf};
use std::fs;
use std::ffi::OsStr;
use std::convert::TryFrom;
use std::process::Command;
use std::error::Error;

pub struct LnkFile {
    lnk: Lnk,
    path: PathBuf,
}

impl LnkFile {
    pub fn new(path: PathBuf) -> Result<LnkFile, ()> {
        match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
            "lnk" => {
                assert_eq!(path.extension().unwrap(), "lnk");
                let lnk: Lnk = Lnk::try_from(path.as_path()).map_err(|a| ())?;
                Ok(LnkFile{
                    lnk,
                    path
                })
            }
            _ => Err(())
        }
    }

    pub fn get_path(&self) -> String {
        let rel_path: Option<PathBuf> = self.lnk.relative_path();
        let working_dir: Option<PathBuf> = self.lnk.working_dir();

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

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let args = self.path.clone().into_os_string().into_string().map_err(|e| "error")?;
        let full_path = Path::new(&args[4..]);

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
        let command = format!(r#"$Env:tmp\{}"#, file_name);
        println!("command: {}, env: {}", command, parent);

        let output = Command::new(r#"powershell"#)
            .arg("/c")
            .arg("start")
            .arg(command)
            .output()
            .expect("Failed to execture process");


        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Ok(())
    }
}