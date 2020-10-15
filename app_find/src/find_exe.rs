use std::{env, fs, io};
use std::error::Error;
use crate::traverse::RecursiveDirTraversal;
use std::path::{PathBuf, Path};
use parselnk::Lnk;
use std::convert::TryFrom;
use std::env::join_paths;
use crate::lnk_parser::LnkFile;

pub fn print_start_menu() -> Result<(), Box<dyn Error>> {
    let start_menu_dirs = [
        format!(r"{}\Microsoft\Windows\Start Menu\Programs", env::var("ProgramData")?),
        format!(r"{}\Microsoft\Windows\Start Menu\Programs", env::var("AppData")?),
        format!(r"{}\OneDrive\Desktop", env::var("USERPROFILE")?),
        format!(r"{}\Desktop", env::var("PUBLIC")?),
        format!(r"D:\"),
    ];

    let mut commands: Vec<LnkFile> = Vec::new();

    let mut i: u32 = 1;
    for dir_str in start_menu_dirs.iter() {
        let current_dir = match fs::canonicalize(PathBuf::from(dir_str)) {
            Ok(c) => {c},
            Err(e) => {
                eprintln!("*** could not find {}", dir_str);
                continue;
            },
        };

        println!("searching in dir {:?}", current_dir);
        // for entry in fs::read_dir(current_dir)? {
        for entry in RecursiveDirTraversal::new(current_dir)? {
            // println!("    {:?}", entry);

            // let lnk_path = std::path::Path::new(r"c:\users\me\desktop\slack.lnk");
            // let lnk_path = ;
            if let Ok(file) = LnkFile::new(entry.path()) {
                println!("{:>3}: {:<50} - {}",
                         i,
                         entry.file_name().to_str().unwrap_or(""),
                         file.get_path());
                i = i + 1;

                commands.push(file);
            }
        }
    }

    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("failed");

    let choice: String = ret.chars()
        .filter(|c| c.is_numeric())
        .collect();
    println!("user chose {}", ret);

    let num = choice.parse::<usize>().unwrap();
    commands[num - 1].run()?;

    Ok(())
}
