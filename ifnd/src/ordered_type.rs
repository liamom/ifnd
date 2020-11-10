use std::cmp::Ordering;
use crate::files::runnable_file::RunnableFile;
use crate::files::file_base::FileBase;

#[derive(Clone)]
pub struct OrderedSearchMatch<T> {
    pub score: i64,
    pub indices: Vec<usize>,
    pub file: T,
    // pub file: &'a T,
}

impl <T> PartialOrd for OrderedSearchMatch<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <T> Ord for OrderedSearchMatch<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl <T> Eq for OrderedSearchMatch<T> {

}

impl <T> PartialEq for OrderedSearchMatch<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.score == other.score;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.score != other.score;
    }
}