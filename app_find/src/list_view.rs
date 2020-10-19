use crate::runnable_file::runnable_file::{RunnableFile, RunnableFileTrait};
use std::io::{stdout, Write};
use crossterm::{execute, queue, style::{self, Colorize}, cursor, terminal, QueueableCommand, ExecutableCommand};
use crossterm;
use std::error::Error;
use crossterm::cursor::{SavePosition, RestorePosition};
use crate::ordered_type;
use crate::ordered_type::OrderedSearchMatch;

pub fn print_view<'a, I>(cursor_pos: usize,
                      search_text: &str,
                      list: &mut I)
    -> Result<usize, Box<dyn Error>>
    where I: Iterator<Item = &'a OrderedSearchMatch<'a>>
{
    let mut stdout = stdout();

    let (total_cols, _) = crossterm::terminal::size()?;
    let (start_col, start_row) = crossterm::cursor::position()?;
    stdout
        // .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(terminal::DisableLineWrap)?
        .queue(cursor::MoveTo(0, start_row))?
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
        .queue(style::PrintStyledContent( "a ".magenta()))?
        .queue(style::PrintStyledContent( search_text.cyan()))?
        .queue(SavePosition)?;

    let mut counter = 1;
    for ot in list {
        let file = ot.file;
        let pysical_row = start_row + counter;


        let mut list_text = format!("{:>3} ({:>3}): ",
                counter,
                ot.score);

        if cursor_pos as u16 == counter {
            stdout
                .queue(cursor::MoveTo(0, pysical_row))?
                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
                .queue(style::PrintStyledContent( "→".magenta()))?
                .queue(cursor::MoveTo(1, pysical_row))?
                .queue(style::Print(list_text))?;
        } else {
            stdout
                .queue(cursor::MoveTo(0, pysical_row))?
                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
                .queue(style::Print(" "))?
                .queue(style::Print(list_text))?;
        }

        let full_path = file.get_file_path().to_str().unwrap_or("");

        let file_name = file.get_file_path().file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("");

        let offset = full_path.len() - file_name.len();
        for i in 0..file_name.len() {
            let char: char = file_name.chars().nth(i).unwrap();
            if ot.indices.contains(&(i + offset)) {
                stdout.queue(style::PrintStyledContent( char.magenta()))?;
            } else {
                stdout.queue(style::Print( char))?;
            }
        }

        stdout.queue(cursor::MoveTo(50, pysical_row))?
              .queue(style::Print( "| "))?;

        for i in 4..full_path.len() {
            let char: char = full_path.chars().nth(i).unwrap();
            if ot.indices.contains(&i) {
                stdout.queue(style::PrintStyledContent( char.magenta()))?;
            } else {
                stdout.queue(style::Print( char))?;
            }
        }


        counter = counter + 1;
    }
    stdout
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
        .queue(RestorePosition)?
        .flush()?;

    Ok(counter as usize)
}