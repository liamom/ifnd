use std::{fs};
use std::error::Error;
use std::path::Path;
use std::fs::{DirEntry, ReadDir};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct BredthFirstRecursiveDirTraversal {
    dirs: VecDeque<ReadDir>,
}

impl BredthFirstRecursiveDirTraversal {
    pub fn new<S>(path: S) -> Result<Self, Box<dyn Error>> where S: AsRef<Path>{
        let a = fs::read_dir(path)?;

        let mut dirs = VecDeque::new();
        dirs.push_back(a);
        Ok(BredthFirstRecursiveDirTraversal {
            dirs
        })
    }
}

impl Iterator for BredthFirstRecursiveDirTraversal {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let iter = match self.dirs.front_mut() {
                None => return None,
                Some(a) => a,
            };

            let new_sub_folder: Option<ReadDir> = if let Some(read_dir_result) = iter.next() {
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
                }

                if metadata.is_dir() {
                    let new_sub_folder = match fs::read_dir(&read_dir.path()) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("error reading folder {:?}", e);
                            continue
                        }
                    };

                    Some(new_sub_folder)
                } else {
                    None
                }
            } else {
                self.dirs.pop_front();
                None

            };


            if let Some(folder) = new_sub_folder {
                self.dirs.push_back(folder);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_search_dris() ->Result<(), Box<dyn Error>> {
        fs::create_dir_all("test/a1/b1")?;
        fs::create_dir_all("test/a2/b2")?;
        fs::create_dir_all("test/a3/b2/c1")?;
        fs::create_dir_all("test/a4/b2/c1")?;
        File::create("test/1.txt")?.write(b"")?;
        File::create("test/2.txt")?.write(b"")?;
        File::create("test/a1/1.txt")?.write(b"")?;
        File::create("test/a1/2.txt")?.write(b"")?;
        File::create("test/a1/b1/1.txt")?.write(b"")?;

        File::create("test/a2/1.txt")?.write(b"")?;
        File::create("test/a2/b2/1.txt")?.write(b"")?;

        File::create("test/a3/1.txt")?.write(b"")?;
        File::create("test/a3/b2/1.txt")?.write(b"")?;
        File::create("test/a4/1.txt")?.write(b"")?;
        File::create("test/a4/b2/1.txt")?.write(b"")?;

        let mut iter = BredthFirstRecursiveDirTraversal::new("test")?;
        let mut next_path = || -> Result<PathBuf, Box<dyn Error>> {
            Ok(iter.next().ok_or("empty iter")?.path())
        };

        assert_eq!(next_path()?, PathBuf::from("test/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/2.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a1/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a1/2.txt"));

        assert_eq!(next_path()?, PathBuf::from("test/a2/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a3/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a4/1.txt"));


        assert_eq!(next_path()?, PathBuf::from("test/a1/b1/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a2/b2/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a3/b2/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a4/b2/1.txt"));

        assert!(iter.next().is_none());

        fs::remove_dir_all("test")?;

        Ok(())
    }
}

