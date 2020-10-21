mod traverse;
mod find_exe;
mod runnable_file;
mod list_view;
mod selection_gui;
mod ordered_type;
mod filtered_list;

use std::{env, fs, io};
use std::error::Error;
use crate::runnable_file::runnable_file::{RunnableFileTrait, RunnableFile};
use std::io::Write;
use std::ffi::OsStr;
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::event::KeyCode::{Char, Backspace};
use std::i32::MAX;
use std::i8::MIN;
use std::cmp::{min, max, Ordering};
use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::{BinaryHeap, BTreeMap};
use std::ops::Deref;
use fuzzy_matcher::skim::SkimMatcherV2;


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

fn main() -> Result<(), Box<dyn Error>> {
    let finder = find_exe::print_start_menu()?;

    let selection: RunnableFile = selection_gui::run_selection_gui(finder.receiver)?;

    let file_name = selection.get_file_path()
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or("error getting file name")?;
    print!("& {} ", file_name);
    io::stdout().flush();
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
