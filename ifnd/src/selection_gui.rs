use fuzzy_matcher::skim::{SkimMatcherV2, SkimScoreConfig};
use crate::{list_view};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::event::KeyCode::{Char, Backspace};
use std::time::Duration;
use std::error::Error;
use crate::filtered_list::FilteredList;
use crate::search::search_type::{SearchMode, SearchType};
use crate::search::app_search::AppSearch;
use crate::files::runnable_file::RunnableFile;
use crate::files::any_file::AnyFile;
use crate::files::file_base::FileBase;
use std::ops::{Deref, DerefMut};
use crate::list_view::print_input;
use std::io::stdout;
use crate::search::file_search::FileSearch;

pub enum FileTypes {
    RunnableFile(RunnableFile),
    AnyFile(AnyFile),
}

enum SelectRet {
    MakeSelection,
    Rerender,
    Norerender
}

fn select_internal(state: &mut State) -> Result<SelectRet, crossterm::ErrorKind>
{
    // `poll()` waits for an `Event` for a given time period
    if poll(Duration::from_millis(500))? {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        match read()? {
            Event::Key(event) => {
                match event.code {
                    Char(c) => {
                        state.input_search_str.push(c);
                    },
                    Backspace => { state.input_search_str.pop(); },

                    KeyCode::Enter => {
                        return Ok(SelectRet::MakeSelection);
                    },
                    KeyCode::Left => {}
                    KeyCode::Right => {}
                    KeyCode::Up => {
                        state.cursor = match state.cursor {
                            0 | 1 => 1,
                            _ => state.cursor - 1,
                        }
                    },
                    // KeyCode::Up => cursor = min(0, cursor as i64 - 1) as usize,
                    // KeyCode::Down => cursor = max(commands.len(), cursor + 1),
                    KeyCode::Down => {
                        let list_size = state.items_displayed_on_current_page;
                        state.cursor = if state.cursor < list_size {
                            state.cursor + 1
                        } else {
                            list_size
                        }
                    }
                    KeyCode::Home => {}
                    KeyCode::End => {}
                    KeyCode::PageUp => {
                        state.page = state.page - 1;
                    }
                    KeyCode::PageDown => {
                        state.page = state.page + 1;
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

        Ok(SelectRet::Rerender)
    } else {
        Ok(SelectRet::Norerender)
    }
}

fn render_results_list<FT: FileBase + 'static, T: SearchType<FT>>(mode: &mut T, state: &mut State)
{
    state.need_rerender = state.need_rerender || mode.check_for_updates();

    if state.need_rerender {
        let (_, search_str) = state.split_input();
        let list = FilteredList::new(&mode.get_cache(), search_str, &state.matcher);
        let mut a = list.iter(state.page);

        let items_displayed_on_current_page
            = list_view::print_results(state.cursor,&mut a, state.start_row).unwrap();

        state.items_displayed_on_current_page = items_displayed_on_current_page;
    }
}

struct State {
    matcher: SkimMatcherV2,
    input_search_str: String,
    cursor: usize,
    page: usize,
    items_displayed_on_current_page: usize,
    need_rerender: bool,
    start_row: u16,
}

impl State {
    fn new() -> Result<Self, crossterm::ErrorKind>{
        let (_start_col, start_row) = crossterm::cursor::position()?;

        let score_config = SkimScoreConfig::default();

        let matcher = SkimMatcherV2::default()
            .element_limit(2000)
            .score_config(score_config);

        Ok(State {
            matcher,
            input_search_str: "".to_string(),
            cursor: 0,
            page: 0,
            items_displayed_on_current_page: 0,
            need_rerender: true,
            start_row: start_row
        })
    }

    fn split_input(&self) -> (&str, &str) {
        let mut iter = self.input_search_str.splitn(2, ' ');
        let command_str = iter.next().unwrap_or("");
        let search_str = iter.next().unwrap_or("");
        (command_str, search_str)
    }
}

fn make_selection<FT: FileBase + Clone + 'static, ST: SearchType<FT>>(search_mode: &mut ST, state: &mut State) -> Result<FT, Box<dyn Error>> {
    let cache = search_mode.get_cache();

    let (_, search_str) = state.split_input();
    let list: FilteredList<FT> = FilteredList::new(
        &cache,
        search_str,
        &state.matcher);
    let a = list.iter(state.page);
    let it = a.skip(state.cursor - 1).next();
    let file: FT = it.ok_or("could not find search match")?.file.clone();
    return Ok(file);
}

pub fn run_selection_gui() -> Result<FileTypes, Box<dyn Error>> {
    let mut state = State::new()?;
    let mut search_mode: SearchMode = SearchMode::None;

    let selection: FileTypes = 'outer: loop {
        let (command_str, search_str) = state.split_input();

        // set search mode
        match command_str {
            "a" => match search_mode {
                SearchMode::AppSerach(_) => {}
                _ => search_mode = SearchMode::AppSerach(AppSearch::new())
            },
            "f" => match search_mode {
                SearchMode::FileSearch(_) => {}
                _ => search_mode = SearchMode::FileSearch(FileSearch::new())
            },
            _ => (),
        };

        if state.need_rerender {
            print_input(&state.input_search_str, state.start_row)?;
        }

        match &mut search_mode {
            SearchMode::None => {},
            SearchMode::AppSerach(mode) => render_results_list(mode, &mut state),
            SearchMode::FileSearch(mode) => render_results_list(mode, &mut state),
        };

        if state.need_rerender {
            list_view::flush()?;
        }

        let r = select_internal(&mut state)?;

        match r {
            SelectRet::MakeSelection => {
                let ret = match &mut search_mode {
                    SearchMode::None => {
                        panic!("wtf how");
                    }
                    SearchMode::AppSerach(mode) => {
                        let file = make_selection(mode, &mut state);
                        FileTypes::RunnableFile(file?)
                    }
                    SearchMode::FileSearch(mode) => {
                        let file = make_selection(mode, &mut state);
                        FileTypes::AnyFile(file?)
                    }
                };

                list_view::reset_terminal(state.start_row);
                break 'outer ret;
            },
            SelectRet::Rerender => state.need_rerender = true,
            SelectRet::Norerender => state.need_rerender = false,
        }



    };

    return Ok(selection);
}