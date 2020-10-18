mod traverse;
mod find_exe;
mod runnable_file;
mod list_view;

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
use indexmap::set::IndexSet;
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

#[derive(Copy, Clone)]
struct OrderedType<'a> {
    pub score: i64,
    pub file: &'a RunnableFile,
}

impl <'a> PartialOrd for OrderedType <'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a> Ord for OrderedType <'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl <'a> Eq for OrderedType <'_> {

}

impl <'a> PartialEq for OrderedType <'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.score == other.score;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.score != other.score;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let finder = find_exe::print_start_menu()?;

    // start at 1 cause that's where numbers start on the keyboard
    let mut i = 1;

    // let search_str: String = env::args().skip(1).collect();
    let mut search_str = String::new();

    let matcher = SkimMatcherV2::default();

    let mut commands = Vec::new();

    let mut cursor: usize = 0;
    let mut page = 0;

    let selection: &RunnableFile = 'outer: loop {
        while let Ok(result) = finder.receiver.try_recv() {
            let path = result.get_file_path();
            let path_str = match path.to_str() {
                Some(v) => v,
                None => continue,
            };

            commands.push(result);
        }

        // let a = BTreeMap::new();
        // a.insert(1, "hi");

        let filtered_list: BTreeMap <i64, OrderedType> = commands.iter()
            .map(|i| {
                // let choice = i.get_file_path().file_name().unwrap().to_str().unwrap();
                let choice = &i.get_file_path().to_str().unwrap();
                    // .unwrap()[6..]
                    // .replace("\\", " ");
                let score = matcher.fuzzy_match(choice, search_str.as_str()).unwrap_or(0);
                return (score, OrderedType{
                    score,
                    file: i
                });
            })
            // .filter(|(i, ot)| *i != 0i64)
            .collect();

/*
        let filtered_list: Vec<&RunnableFile> = commands.iter()
            .filter(|file| {
                let buf = file.get_file_path().to_str().unwrap();
                return buf.to_lowercase().contains(&search_str.to_lowercase())
            })
            .take(25)
            .collect();
*/
        let mut a = filtered_list.iter().rev()
            .skip(page*25)
            .take(25)
            .map(|(a,b)|b);
        list_view::print_view(cursor,
                              search_str.as_str(),
                              &mut a).unwrap();


        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        Char(c) => {
                            search_str.push(c);
                        },
                        Backspace => {search_str.pop();},

                        KeyCode::Enter => {
                            let it = filtered_list.iter().rev().skip(cursor - 1).next();

                            break 'outer it.unwrap().1.file.clone();
                            // break 'outer filtered_list[&index].file.clone();
                        },
                        KeyCode::Left => {}
                        KeyCode::Right => {}
                        KeyCode::Up => cursor = match cursor {
                            0|1 => 1,
                            _ => cursor - 1,
                        },
                        // KeyCode::Up => cursor = min(0, cursor as i64 - 1) as usize,
                        // KeyCode::Down => cursor = max(commands.len(), cursor + 1),
                        KeyCode::Down => {
                            let list_size = filtered_list.len();
                            cursor = if cursor < list_size {
                                cursor + 1
                            } else {
                                list_size
                            }
                        }
                        KeyCode::Home => {}
                        KeyCode::End => {}
                        KeyCode::PageUp => {
                            page = page - 1;
                        }
                        KeyCode::PageDown => {
                            page = page + 1;
                        }
                        KeyCode::Tab => {}
                        KeyCode::BackTab => {}
                        KeyCode::Delete => {}
                        KeyCode::Insert => {}
                        KeyCode::F(_) => {}
                        KeyCode::Null => {}
                        KeyCode::Esc => {}
                    }
                },
                // Event::Mouse(event) => println!("{:?}", event),
                // Event::Resize(width, height) => println!("New size {}x{}", width, height),
                _ => {},
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    };

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
