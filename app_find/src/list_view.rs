use crate::runnable_file::runnable_file::{RunnableFile, RunnableFileTrait};
use std::io::{stdout, Write};
use crossterm::{execute, queue, style::{self, Colorize}, cursor, terminal, QueueableCommand, ExecutableCommand};
use crossterm;
use std::error::Error;
use crossterm::cursor::{SavePosition, RestorePosition};
use crate::OrderedType;

// pub fn print_view<'a, I>(cursor_pos: usize, search_text: &str, list: &mut I) -> crossterm::Result<()>
pub fn print_view<'a, I>(cursor_pos: usize,
                      search_text: &str,
                      list: &mut I)
    -> crossterm::Result<()>
    where I: Iterator<Item = &'a OrderedType<'a>>
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
    for ot in list {
        let file = ot.file;
        let pysical_row = start_row + counter;
        let mut list_text = format!("{:>3} ({:>3}): {:<50} - {}",
                counter,
                ot.score,
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