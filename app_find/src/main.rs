mod traverse;
mod find_exe;
mod runnable_file;

use std::{env, fs, io};
use std::error::Error;
use crate::runnable_file::runnable_file::RunnableFileTrait;
use std::io::Write;
use std::ffi::OsStr;

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
    println!("Hello, world!");

    let finder = find_exe::print_start_menu()?;

    // start at 1 cause that's where numbers start on the keyboard
    let mut i = 1;

    let search_str: String = env::args().skip(1).collect();
    println!("searching for '{}'", search_str);

    let mut commands = Vec::new();
    loop {
        if let Ok(result) = finder.receiver.recv() {
            let path = result.get_file_path();
            let path_str = match path.to_str() {
                Some(v) => v,
                None => continue,
            };

            if path_str.to_lowercase().contains(&search_str.to_lowercase()) {
                println!("{:>3}: {:<50} - {}",
                         i,
                         path.file_name()
                             .and_then(|v| v.to_str())
                             .unwrap_or(""),
                         path.to_str().unwrap_or(""));
                i = i + 1;
                commands.push(result);
            }
        } else {
            break;
        }
    }

    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("failed");

    let choice: String = ret.chars()
        .filter(|c| c.is_numeric())
        .collect();
    println!("user chose {}", ret);

    let num = choice.parse::<usize>().unwrap();
    let runnableFile = &commands[num - 1];

    let file_name = runnableFile.get_file_path()
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or("error getting file name")?;
    print!("& {} ", file_name);
    io::stdout().flush();
    let mut args = String::new();
    io::stdin().read_line(&mut args).expect("failed to get args");

    // let args = args.split(' ').map(|a| OsStr::new(a));
    let mut command = runnableFile.run()?;
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
