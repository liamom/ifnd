use crate::runnable_file::runnable_file::{RunnableFile, RunnableFileTrait};
use std::io::{stdout, Write};
use crossterm::{execute, queue, style::{self, Colorize}, cursor, terminal, QueueableCommand, ExecutableCommand};
use crossterm;
use std::error::Error;
use crossterm::cursor::{SavePosition, RestorePosition};

// pub fn print_view<'a, I>(cursor_pos: usize, search_text: &str, list: &mut I) -> crossterm::Result<()>
pub fn print_view(cursor_pos: usize,
                      search_text: &str,
                      list: &Vec<&RunnableFile>)
    -> crossterm::Result<()>
{
    let mut stdout = stdout();

    let (total_cols, _) = crossterm::terminal::size()?;
    let (start_col, start_row) = crossterm::cursor::position()?;
    stdout
        // .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, start_row))?
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
        .queue(style::PrintStyledContent( "a ".magenta()))?
        .queue(style::PrintStyledContent( search_text.cyan()))?
        .queue(SavePosition)?;

    let mut counter = 1;
    for file in list {
        let pysical_row = start_row + counter;
        let mut list_text = format!("{:>3}: {:<50} - {}",
                counter,
                file.get_file_path().file_name()
                     .and_then(|v| v.to_str())
                     .unwrap_or(""),
                 file.get_file_path().to_str().unwrap_or(""));
        list_text.truncate(total_cols as usize);

        if cursor_pos as u16 == counter {
            stdout.queue(cursor::MoveTo(0, pysical_row))?
                .queue(style::PrintStyledContent( "→".magenta()))?
                .queue(cursor::MoveTo(1, pysical_row))?
                .queue(style::Print(list_text))?;
        } else {
            stdout.queue(cursor::MoveTo(0, pysical_row))?
                .queue(style::Print(" "))?
                .queue(style::Print(list_text))?;
        }



        counter = counter + 1;
    }
    stdout
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
        .queue(RestorePosition)?
        .flush()?;

    Ok(())
}