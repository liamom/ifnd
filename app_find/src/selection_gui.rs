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

struct FilteredList<'a> {
    filtered_list: BTreeMap<i64, OrderedSearchMatch<'a>>,
}

impl <'a> FilteredList <'a>{
    fn empty() -> Self {
        FilteredList {
            filtered_list: BTreeMap::new(),
        }
    }

    fn new(commands: &'a Vec<RunnableFile>, search_str: &str, matcher: &SkimMatcherV2) -> Self {
        let filtered_list: BTreeMap<i64, OrderedSearchMatch> = commands.iter()
            .map(|i| {
                let choice = &i.get_file_path().to_str().unwrap();
                let (score, indices) = matcher.fuzzy_indices(choice, search_str)
                    .unwrap_or_else(|| (0, Vec::new()));
                return (score, OrderedSearchMatch {
                    score,
                    indices,
                    file: i
                });
            })
            .collect();

        FilteredList {
            filtered_list,
        }
    }

    fn iter(&'a self, page: usize) -> impl Iterator<Item = &'a OrderedSearchMatch> {
        let mut a = self.filtered_list.iter().rev()
            .skip(page * 25)
            .take(25)
            .map(|(a, b)| b);
        return a;
    }
}

// fn get_filtered_list<'a>(matcher: &SkimMatcherV2, commands: &'a Vec<RunnableFile>, search_str: &str, page: usize)
//     -> (BTreeMap<i64, OrderedSearchMatch<'a>>, impl Iterator<Item = (&'a OrderedSearchMatch<'a>)>)
// {
//     let filtered_list: BTreeMap<i64, OrderedSearchMatch> = commands.iter()
//         .map(|i| {
//             let choice = &i.get_file_path().to_str().unwrap();
//             let (score, indices) = matcher.fuzzy_indices(choice, search_str)
//                 .unwrap_or_else(|| (0, Vec::new()));
//             return (score, OrderedSearchMatch {
//                 score,
//                 indices,
//                 file: i
//             });
//         })
//         .collect();
//
//     let b = filtered_list.iter();
//     let mut a = filtered_list.iter().rev()
//         .skip(page * 25)
//         .take(25)
//         .map(|(a, b)| b);
//
//     return (filtered_list, a);
//     // let a =  FilteredListIter {
//     //     filtered_list,
//     // };
//     FilteredList::new()
// }

pub fn run_selection_gui(receiver: Receiver<RunnableFile>) -> Result<RunnableFile, Box<dyn Error>> {
    let matcher = SkimMatcherV2::default();

    let mut search_str = String::new();

    let mut need_rerender = true;
    let mut all_files_cache = Vec::new();

    let mut cursor: usize = 0;
    let mut page = 0;

    let mut items_displayed_on_current_page = 0;

    let selection: RunnableFile = 'outer: loop {
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
            let mut list = FilteredList::new(&all_files_cache, search_str.as_str(), &matcher);
            let mut a = list.iter(page);

            items_displayed_on_current_page = list_view::print_view(cursor,
                                                                    search_str.as_str(),
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
                            search_str.push(c);
                        },
                        Backspace => { search_str.pop(); },

                        KeyCode::Enter => {
                            let mut list = FilteredList::new(
                                &all_files_cache,
                                search_str.as_str(),
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