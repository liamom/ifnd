use crate::runnable_file::runnable_file::RunnableFile;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct OrderedSearchMatch<'a> {
    pub score: i64,
    pub indices: Vec<usize>,
    pub file: &'a RunnableFile,
}

impl <'a> PartialOrd for OrderedSearchMatch<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a> Ord for OrderedSearchMatch<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl <'a> Eq for OrderedSearchMatch<'_> {

}

impl <'a> PartialEq for OrderedSearchMatch<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.score == other.score;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.score != other.score;
    }
}