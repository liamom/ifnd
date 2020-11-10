use std::io::{stdout, Write};
use crossterm::{style::{self, Colorize}, cursor, terminal, QueueableCommand};
use crossterm;
use std::error::Error;
use crossterm::cursor::{SavePosition, RestorePosition};
use crate::ordered_type::OrderedSearchMatch;
use crate::files::file_base::FileBase;

pub fn print_input(search_text: &str, start_row: u16) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    stdout
        .queue(terminal::DisableLineWrap)?
        .queue(cursor::MoveTo(0, start_row))?
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?;

    {
        let mut iter = search_text.splitn(2, ' ');

        if let Some(command) = iter.next() {
            stdout.queue(style::PrintStyledContent("> ".white()))?
                  .queue(style::PrintStyledContent(command.magenta()))?;
        }
        if let Some(search_str) = iter.next() {
            stdout
                .queue(style::Print(" "))?
                .queue(style::PrintStyledContent(search_str.cyan()))?;
        }
    }
    stdout.queue(SavePosition)?;

    Ok(())
}

pub fn print_results<'a, T, I>(cursor_pos: usize,
                               list: &mut I,
                               start_row: u16
)
                               -> Result<usize, Box<dyn Error>>
    where
        T: FileBase + 'static,
        I: Iterator<Item = &'a OrderedSearchMatch<&'a T>>
{
    let mut stdout = stdout();

    let mut counter = 1;
    for ot in list {
        let file = ot.file;
        let pysical_row = start_row + counter;


        let list_text = format!("{:>3} ({:>3}): ",
                counter,
                ot.score);

        if cursor_pos as u16 == counter {
            stdout
                .queue(cursor::MoveTo(0, pysical_row))?
                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
                .queue(style::PrintStyledContent( ">".dark_grey()))?
                .queue(cursor::MoveTo(1, pysical_row))?
                .queue(style::Print(list_text))?;
        } else {
            stdout
                .queue(cursor::MoveTo(0, pysical_row))?
                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
                .queue(style::Print(" "))?
                .queue(style::Print(list_text))?;
        }

        let full_path = file.get_search_path();

        // let file_name = file.get_search_path().file_name()
        //     .and_then(|v| v.to_str())
        //     .unwrap_or("");
        //
        // let offset = full_path.len() - file_name.len();
        // for i in 0..file_name.len() {
        //     let char: char = file_name.chars().nth(i).unwrap();
        //     if ot.indices.contains(&(i + offset)) {
        //         stdout.queue(style::PrintStyledContent( char.magenta()))?;
        //     } else {
        //         stdout.queue(style::Print( char))?;
        //     }
        // }

        // stdout
        //     .queue(cursor::MoveTo(50, pysical_row))?
        //     .queue(style::Print( "| "))?;

        for i in 0..full_path.len() {
            let char: char = full_path.chars().nth(i).unwrap();
            if ot.indices.contains(&i) {
                stdout.queue(style::PrintStyledContent( char.magenta()))?;
            } else {
                stdout.queue(style::Print( char))?;
            }
        }


        counter = counter + 1;
    }


    Ok(counter as usize)
}

pub fn flush() -> Result<(), Box<dyn Error>> {
    stdout()
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
        .queue(RestorePosition)?
        .flush()?;

    Ok(())
}

pub fn reset_terminal(start_row: u16) -> Result<(), Box<dyn Error>> {
    stdout()
        .queue(RestorePosition)?
        .queue(cursor::MoveTo(0, start_row))?
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
        .flush()?;

    Ok(())
}