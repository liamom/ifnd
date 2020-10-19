use std::collections::BTreeMap;
use crate::ordered_type::OrderedSearchMatch;
use crate::runnable_file::runnable_file::{RunnableFile, RunnableFileTrait};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct FilteredList<'a> {
    filtered_list: BTreeMap<i64, OrderedSearchMatch<'a>>,
}

impl <'a> FilteredList <'a>{
    pub fn empty() -> Self {
        FilteredList {
            filtered_list: BTreeMap::new(),
        }
    }

    pub fn new(commands: &'a Vec<RunnableFile>, search_str: &str, matcher: &SkimMatcherV2) -> Self {
        let filtered_list: BTreeMap<i64, OrderedSearchMatch> = commands.iter()
            .map(|i| {
                let choice = &i.get_file_path().to_str().unwrap();
                let (score, indices) = matcher.fuzzy_indices(choice, search_str)
                    .unwrap_or_else(|| (0, Vec::new()));
                return (score, OrderedSearchMatch {
                    score,
                    indices,
                    file: i
                });
            })
            .collect();

        FilteredList {
            filtered_list,
        }
    }

    pub fn iter(&'a self, page: usize) -> impl Iterator<Item = &'a OrderedSearchMatch> {
        let mut a = self.filtered_list.iter().rev()
            .skip(page * 25)
            .take(25)
            .map(|(a, b)| b);
        return a;
    }
}