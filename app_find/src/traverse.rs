use std::{fs, io};
use std::error::Error;
use std::path::Path;
use std::fs::{DirEntry, ReadDir, Metadata};

#[derive(Debug)]
pub struct RecursiveDirTraversal {
    dirs: Vec<ReadDir>,
}

impl RecursiveDirTraversal {
    pub fn new<S>(path: S) -> Result<Self, Box<dyn Error>> where S: AsRef<Path>{
        let a = fs::read_dir(path)?;
        Ok(RecursiveDirTraversal {
            dirs: vec![a]
        })
    }
}

impl Iterator for RecursiveDirTraversal {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            let iter = match self.dirs.last_mut() {
                None => return None,
                Some(a) => a,
            };

            while let Some(read_dir_result) = iter.next() {
                let read_dir = match read_dir_result {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error reading file {:?}", e);
                        continue
                    },
                };

                let metadata = match fs::metadata(&read_dir.path()) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error reading file metadata {:?}", e);
                        continue
                    }
                };

                if metadata.is_file() {
                    return Some(read_dir);
                } else if metadata.is_dir() {
                    let new_sub_folder = match fs::read_dir(&read_dir.path()) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("error reading folder {:?}", e);
                            continue
                        }
                    };

                    self.dirs.push(new_sub_folder);
                    continue 'outer;
                }
            }

            self.dirs.pop();
        }
    }
}

