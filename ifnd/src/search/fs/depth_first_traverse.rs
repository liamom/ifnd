use std::{fs};
use std::error::Error;
use std::path::Path;
use std::fs::{DirEntry, ReadDir};

#[derive(Debug)]
pub struct DepthFirstRecursiveDirTraversal {
    dirs: Vec<ReadDir>,
}

impl DepthFirstRecursiveDirTraversal {
    pub fn new<S>(path: S) -> Result<Self, Box<dyn Error>> where S: AsRef<Path>{
        let a = fs::read_dir(path)?;
        Ok(DepthFirstRecursiveDirTraversal {
            dirs: vec![a]
        })
    }
}

impl Iterator for DepthFirstRecursiveDirTraversal {
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

        let mut iter = DepthFirstRecursiveDirTraversal::new("test")?;
        let mut next_path = || -> Result<PathBuf, Box<dyn Error>> {
            Ok(iter.next().ok_or("empty iter")?.path())
        };

        assert_eq!(next_path()?, PathBuf::from("test/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/2.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a1/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a1/2.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a1/b1/1.txt"));

        assert_eq!(next_path()?, PathBuf::from("test/a2/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a2/b2/1.txt"));
        
        assert_eq!(next_path()?, PathBuf::from("test/a3/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a3/b2/1.txt"));

        assert_eq!(next_path()?, PathBuf::from("test/a4/1.txt"));
        assert_eq!(next_path()?, PathBuf::from("test/a4/b2/1.txt"));



        assert!(iter.next().is_none());

        fs::remove_dir_all("test")?;

        Ok(())
    }
}

