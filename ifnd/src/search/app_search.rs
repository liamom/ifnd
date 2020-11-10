use std::sync::mpsc::Receiver;
use crate::search::find_exe;
use crate::search::search_type::SearchType;
use crate::files::runnable_file::RunnableFile;
use crate::files::file_base::FileBase;

pub struct AppSearch {
    cache: Vec<RunnableFile>,
    receiver: Receiver<RunnableFile>,
}

impl AppSearch {
    pub fn new() -> Self {
        let finder = find_exe::print_start_menu().unwrap();
        AppSearch {
            cache: Vec::new(),
            receiver: finder.receiver,
        }
    }

}

impl SearchType<RunnableFile> for &AppSearch {
    fn check_for_updates(&mut self) -> bool {
        unimplemented!()
    }

    fn get_cache(&self) -> &Vec<RunnableFile> {
        unimplemented!()
    }
}


impl SearchType<RunnableFile> for AppSearch {
    fn check_for_updates(&mut self) -> bool {
        let mut need_rerender = false;
        while let Ok(result) = self.receiver.try_recv() {
            self.cache.push(result);
            need_rerender = true;
        }

        need_rerender
    }

    fn get_cache(&self) -> &Vec<RunnableFile> {
        &self.cache
    }
}