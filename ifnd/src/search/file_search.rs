use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::sync::mpsc;
use crate::search::fs::bredth_first_traverse::BredthFirstRecursiveDirTraversal;
use crate::search::search_type::SearchType;
use crate::files::any_file::AnyFile;
use crate::files::runnable_file::RunnableFile;
use crate::files::file_base::FileBase;
use std::error::Error;

pub struct FileSearch {
    cache: Vec<AnyFile>,
    receiver: Receiver<AnyFile>,
}

fn thread_internal(tx: Sender<AnyFile>) -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let result = BredthFirstRecursiveDirTraversal::new(current_dir);

    for path in result? {
        tx.send(AnyFile {
            path: path.path()
        });
    }

    Ok(())
}

pub fn create_finder() -> Receiver<AnyFile> {
    let (tx, rx): (Sender<AnyFile>, Receiver<AnyFile>) = mpsc::channel();

    thread::spawn(move || {
        match thread_internal(tx) {
            Ok(_) => {}
            Err(_) => {}
        }
    });

    rx
}

impl FileSearch {
    pub fn new() -> Self {
        FileSearch {
            cache: Vec::new(),
            receiver: create_finder()
        }
    }
}

impl SearchType<AnyFile> for FileSearch {
    fn check_for_updates(&mut self) -> bool {
        let mut need_rerender = false;
        while let Ok(result) = self.receiver.try_recv() {
            self.cache.push(result);
            need_rerender = true;
        }

        need_rerender
    }

    fn get_cache(&self) -> &Vec<AnyFile> {
        &self.cache
    }
}