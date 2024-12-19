use walkdir::WalkDir;

pub struct FileSystemCrawler {
    root_directory: String,
    verbose: bool,
    deleted_files: isize,
}

impl FileSystemCrawler {
    pub fn new(root_directory: String, verbose: bool) -> FileSystemCrawler {
        FileSystemCrawler {
            root_directory,
            verbose,
            deleted_files: 0,
        }
    }

    fn print_summary(&self) {
        println!("*******************************************************************************");
        println!("Deleted .vs folders: {}", self.deleted_files);
        println!();
    }

    pub fn crawl_and_kill(&mut self) {
        for entry in WalkDir::new(&self.root_directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == ".vs" {
                if self.verbose {
                    println!(".vs folder found: {:?}", entry.path());
                    println!("Attempting deletion");
                    println!();
                }

                if let Err(e) = std::fs::remove_dir_all(entry.path()) {
                    eprintln!("Failed to remove directory: {:?}", entry);
                    eprintln!("Reported Error: {}", e);
                    eprintln!();
                } else {
                    self.deleted_files += 1;
                }
            }
        }

        self.print_summary();
    }
}
