use crate::runnable_file::runnable_file::{RunnableFile, RunnableFileTrait};
use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::BTreeMap;
use crate::{ordered_type, list_view, find_exe};
use fuzzy_matcher::FuzzyMatcher;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::event::KeyCode::{Char, Backspace};
use std::time::Duration;
use std::error::Error;
use std::sync::mpsc::Receiver;
use crate::ordered_type::OrderedSearchMatch;
use crate::filtered_list::FilteredList;

struct AppSearch {
    cache: Vec<RunnableFile>,
    receiver: Receiver<RunnableFile>,
}

impl AppSearch {
    fn new() -> Self {
        let finder = find_exe::print_start_menu().unwrap();
        AppSearch {
            cache: Vec::new(),
            receiver: finder.receiver,
        }
    }

    fn check_for_updates(&mut self) -> bool {
        while let Ok(result) = self.receiver.try_recv() {
            let path = result.get_file_path();
            let path_str = match path.to_str() {
                Some(v) => v,
                None => continue,
            };

            self.cache.push(result);
            return true;
        }

        return false;
    }
}

enum SearchMode {
    None,
    AppSerach(AppSearch),
}

pub fn run_selection_gui(receiver: Receiver<RunnableFile>) -> Result<RunnableFile, Box<dyn Error>> {
    let matcher = SkimMatcherV2::default();

    let mut search_mode = SearchMode::None;

    // will have single letter prefix for mode
    let mut input_search_str = String::new();

    let mut need_rerender = true;
    let mut all_files_cache = Vec::new();

    let mut cursor: usize = 0;
    let mut page = 0;

    let mut items_displayed_on_current_page = 0;

    let selection: RunnableFile = 'outer: loop {
        let mut iter = input_search_str.splitn(2, ' ');
        let command_str = iter.next().unwrap_or("");
        let search_str = iter.next().unwrap_or("");

        // set search mode
        match command_str {
            "a" => if let SearchMode::AppSerach(_) = search_mode {
                    search_mode = SearchMode::AppSerach(AppSearch::new());
            },
            _ => (),
        };

        while let Ok(result) = receiver.try_recv() {
            let path = result.get_file_path();
            let path_str = match path.to_str() {
                Some(v) => v,
                None => continue,
            };

            all_files_cache.push(result);
            need_rerender = true;
        }

        if need_rerender {
            let mut list = FilteredList::new(&all_files_cache, search_str, &matcher);
            let mut a = list.iter(page);

            items_displayed_on_current_page = list_view::print_view(cursor,
                                                                    input_search_str.as_str(),
                                                                    &mut a).unwrap();
            need_rerender = false;
        }


        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            need_rerender = true;
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        Char(c) => {
                            input_search_str.push(c);
                        },
                        Backspace => { input_search_str.pop(); },

                        KeyCode::Enter => {
                            let mut list = FilteredList::new(
                                &all_files_cache,
                                input_search_str.as_str(),
                                &matcher);
                            let mut a = list.iter(page);
                            let it = a.skip(cursor - 1).next();
                            break 'outer it.unwrap().file.clone();
                            // break 'outer filtered_list[&index].file.clone();
                        },
                        KeyCode::Left => {}
                        KeyCode::Right => {}
                        KeyCode::Up => {
                            cursor = match cursor {
                                0 | 1 => 1,
                                _ => cursor - 1,
                            }
                        },
                        // KeyCode::Up => cursor = min(0, cursor as i64 - 1) as usize,
                        // KeyCode::Down => cursor = max(commands.len(), cursor + 1),
                        KeyCode::Down => {
                            let list_size = items_displayed_on_current_page;
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
            need_rerender = false;
            // Timeout expired and no `Event` is available
        }
    };

    return Ok(selection);
}