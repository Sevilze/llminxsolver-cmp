use std::path::PathBuf;
use std::sync::OnceLock;

static DATA_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

pub fn set_data_directory(path: &str) {
    let path_buf = PathBuf::from(path);
    let _ = DATA_DIRECTORY.set(path_buf);
}

pub fn get_data_directory() -> Option<&'static PathBuf> {
    DATA_DIRECTORY.get()
}
