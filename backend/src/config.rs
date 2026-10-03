use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub storage_dir: PathBuf,
    pub uploads_dir: PathBuf,
    pub extracted_dir: PathBuf,
    pub generated_dir: PathBuf,
    pub previews_dir: PathBuf,
    pub max_upload_size: usize,
    pub max_extracted_size: u64,
    pub max_file_count: usize,
}

impl Default for Config {
    fn default() -> Self {
        let storage_dir = if PathBuf::from("../storage").exists() {
            PathBuf::from("../storage")
        } else {
            PathBuf::from("storage")
        };

        Self {
            port: 3001,
            uploads_dir: storage_dir.join("uploads"),
            extracted_dir: storage_dir.join("extracted"),
            generated_dir: storage_dir.join("generated"),
            previews_dir: storage_dir.join("previews"),
            storage_dir,
            max_upload_size: 25 * 1024 * 1024,   // 25 MB
            max_extracted_size: 100 * 1024 * 1024, // 100 MB
            max_file_count: 500,
        }
    }
}
