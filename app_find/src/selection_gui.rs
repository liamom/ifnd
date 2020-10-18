use crate::runnable_file::runnable_file::{RunnableFile, RunnableFileTrait};
use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::BTreeMap;
use crate::{ordered_type, list_view};
use fuzzy_matcher::FuzzyMatcher;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::event::KeyCode::{Char, Backspace};
use std::time::Duration;
use std::error::Error;
use std::sync::mpsc::Receiver;
use crate::ordered_type::OrderedSearchMatch;

pub fn run_selection_gui(receiver: Receiver<RunnableFile>) -> Result<RunnableFile, Box<dyn Error>> {
    let mut search_str = String::new();

    let matcher = SkimMatcherV2::default();

    let mut commands = Vec::new();

    let mut cursor: usize = 0;
    let mut page = 0;

    let selection: RunnableFile = 'outer: loop {
        while let Ok(result) = receiver.try_recv() {
            let path = result.get_file_path();
            let path_str = match path.to_str() {
                Some(v) => v,
                None => continue,
            };

            commands.push(result);
        }

        // let a = BTreeMap::new();
        // a.insert(1, "hi");

        let filtered_list: BTreeMap<i64, OrderedSearchMatch> = commands.iter()
            .map(|i| {
                // let choice = i.get_file_path().file_name().unwrap().to_str().unwrap();
                let choice = &i.get_file_path().to_str().unwrap();
                // .unwrap()[6..]
                // .replace("\\", " ");
                let (score, indices) = matcher.fuzzy_indices(choice, search_str.as_str())
                    .unwrap_or_else(||(0, Vec::new()));
                return (score, OrderedSearchMatch {
                    score,
                    indices,
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

    return Ok(selection);
}