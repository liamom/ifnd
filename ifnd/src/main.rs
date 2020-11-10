mod list_view;
mod selection_gui;
mod ordered_type;
mod filtered_list;
mod search;
mod files;

use std::{env, fs, io};
use std::error::Error;
use std::io::Write;
use std::ffi::OsStr;
use crate::selection_gui::FileTypes;
use crate::files::runnable_file::{RunnableFile, RunnableFileTrait};
use crate::files::file_base::FileBase;

fn _old() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}

fn handle_runnable(selection: RunnableFile) -> Result<(), Box<dyn Error>>{
    let file_name = selection.get_path()
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or("error getting file name")?;
    print!("& {} ", file_name);
    io::stdout().flush()?;
    let mut args = String::new();
    io::stdin().read_line(&mut args).expect("failed to get args");

    // let args = args.split(' ').map(|a| OsStr::new(a));
    let mut command = selection.run()?;
    for arg in args.split(' ').map(|i| i.trim()).filter(|s| !s.is_empty()) {
        println!("adding arg '{}'", arg);
        command.arg(OsStr::new(arg));
    }

    let output = command.output()
        .expect("Failed to execture process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let selection: selection_gui::FileTypes = selection_gui::run_selection_gui()?;
    match selection {
        FileTypes::RunnableFile(f) => handle_runnable(f)?,
        FileTypes::AnyFile(f) => {
            println!("{}", f.get_path().to_str().ok_or("error")?);
        },
    };

    return Ok(());
}
