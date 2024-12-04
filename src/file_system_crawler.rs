pub struct FileSystemCrawler {
    root_directory: String,
    verbose: bool,
}

impl FileSystemCrawler {
    pub fn new(root_directory: String, verbose: bool) -> FileSystemCrawler {
        FileSystemCrawler {
            root_directory,
            verbose,
        }
    }

    pub fn crawl_and_kill(&self) {}
}
