use walkdir::WalkDir;

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

    pub fn crawl_and_kill(&self) {
        for entry in WalkDir::new(&self.root_directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == ".vs" {
                println!(".vs folder found: {:?}", entry.path());
            }
        }
    }
}
