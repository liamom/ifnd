use std::collections::BTreeMap;
use crate::ordered_type::OrderedSearchMatch;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use crate::files::runnable_file::RunnableFile;
use crate::files::file_base::FileBase;

pub struct FilteredList<'a, T: FileBase> {
    filtered_list: BTreeMap<i64, OrderedSearchMatch<&'a T>>,
}

impl <'a, T: FileBase> FilteredList <'a, T>{
    pub fn new(commands: &'a Vec<T>, search_str: &str, matcher: &SkimMatcherV2) -> Self {
        let filtered_list: BTreeMap<i64, OrderedSearchMatch<&'a T>> = commands.iter()
            .map(|i| {
                let choice = i.get_search_path();
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

    pub fn
    iter(&'a self, page: usize) -> impl Iterator<Item = &'a OrderedSearchMatch<&'a T>> {
        let a = self.filtered_list.iter().rev()
            .skip(page * 25)
            .take(25)
            .map(|(_, b)| b);
        return a;
    }
}