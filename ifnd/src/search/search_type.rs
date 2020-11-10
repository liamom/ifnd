use crate::search::app_search::AppSearch;
use crate::files::runnable_file::RunnableFile;
use crate::files::file_base::FileBase;
use crate::search::file_search::FileSearch;

pub trait SearchType<T: FileBase> {
    fn check_for_updates(&mut self) -> bool;
    fn get_cache(&self) -> &Vec<T>;
}

pub enum SearchMode {
    None,
    AppSerach(AppSearch),
    FileSearch(FileSearch),
}

// impl SearchMode {
//     pub(crate) fn as_trait(&mut self) -> Option<&mut dyn SearchType> {
//         match self {
//             SearchMode::None => None,
//             SearchMode::AppSerach(a) => Some(a),
//         }
//     }
// }