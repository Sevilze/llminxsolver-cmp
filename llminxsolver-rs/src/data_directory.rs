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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_data_directory() {
        // DATA_DIRECTORY is a static OnceLock, it can only be set once
        // This test may fail if another test sets it first
        let test_path = "/tmp/test_data";
        set_data_directory(test_path);

        let result = get_data_directory();
        assert!(result.is_some());
        assert_eq!(result.unwrap().to_str().unwrap(), test_path);
    }
}
