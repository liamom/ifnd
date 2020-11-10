use std::{env, fs, thread};
use std::error::Error;
use std::path::{PathBuf, Path};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use crate::search::fs::depth_first_traverse::DepthFirstRecursiveDirTraversal;
use crate::files::runnable_file::{RunnableFile, to_file};

pub struct AsyncFinder {
    pub receiver: Receiver<RunnableFile>,
}

fn deep_search_thread_inner<P>(search_path: P, tx: &Sender<RunnableFile>) -> Result<(), Box<dyn Error>>
    where P: AsRef<Path>
{
    for entry in DepthFirstRecursiveDirTraversal::new(search_path)? {
        if let Ok(file) = to_file(entry.path()) {
            tx.send(file)?;
        }
    }

    Ok(())
}

fn shallow_search_thread_inner<P>(search_path: P, tx: &Sender<RunnableFile>) -> Result<(), Box<dyn Error>>
    where P: AsRef<Path>
{
    for entry in fs::read_dir(&search_path)? {
        let entry = entry?;
        if let Ok(file) = to_file(entry.path()) {
            tx.send(file)?;
        }
    }

    Ok(())
}

fn to_canonicalized(a: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for path in a {
        match fs::canonicalize(path) {
            Ok(v) => paths.push(v),
            Err(_) => {
                //eprintln!("*** could not find {:?},  {:?}",  path, e)
            },
        }
    }

    paths
}

impl AsyncFinder {
    pub fn new(deep_search_paths: Vec<PathBuf>, shallow_search_paths: Vec<PathBuf>) -> Self {
        let (tx, rx): (Sender<RunnableFile>, Receiver<RunnableFile>) = mpsc::channel();

        let tx2 = tx.clone();
        thread::spawn(move || {
            let paths = to_canonicalized(&deep_search_paths);

            for path in paths {
                if let Err(e) = deep_search_thread_inner(path, &tx2) {
                    eprintln!("Error running traverse {:?}", e);
                }
            }
        });

        thread::spawn(move || {
            let paths = to_canonicalized(&shallow_search_paths);

            for path in paths {
                if let Err(e) = shallow_search_thread_inner(path, &tx) {
                    eprintln!("Error running traverse {:?}", e);
                }
            }
        });

        AsyncFinder {
            receiver: rx
        }
    }
}

// pub fn print_start_menu() -> Result<(), Box<dyn Error>> {
pub fn print_start_menu() -> Result<AsyncFinder, Box<dyn Error>> {
    let start_menu_dirs = vec![
        format!(r"{}\Microsoft\Windows\Start Menu\Programs", env::var("ProgramData")?),
        format!(r"{}\Microsoft\Windows\Start Menu\Programs", env::var("AppData")?),
        format!(r"{}\OneDrive\Desktop", env::var("USERPROFILE")?),
        format!(r"{}\Desktop", env::var("PUBLIC")?),
    ]
        .iter()
        .map(PathBuf::from)
        .collect();

    let paths = env::var("PATH")?;
    let shallow_search_paths = env::split_paths(&paths)
        .collect();

    Ok(AsyncFinder::new(start_menu_dirs, shallow_search_paths))
}
