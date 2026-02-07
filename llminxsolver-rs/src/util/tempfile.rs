use std::cell::RefCell;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

pub struct TempFile {
    path: PathBuf,
    writer: RefCell<Option<BufWriter<File>>>,
    count: usize,
}

impl TempFile {
    const FILENAME: &'static str = "llminx_solutions.txt";

    pub fn new() -> Result<Self, String> {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join(Self::FILENAME);

        if path.exists() {
            let _ = fs::remove_file(&path);
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        Ok(Self {
            path,
            writer: RefCell::new(Some(BufWriter::new(file))),
            count: 0,
        })
    }

    pub fn append(&mut self, solution: &str) -> Result<(), String> {
        let mut writer_ref = self.writer.borrow_mut();
        if let Some(ref mut writer) = *writer_ref {
            writeln!(writer, "{}", solution)
                .map_err(|e| format!("Failed to write solution: {}", e))?;
            self.count += 1;

            if self.count.is_multiple_of(100) {
                let _ = writer.flush();
            }
            Ok(())
        } else {
            Err("Writer is closed".to_string())
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn close(&mut self) {
        let mut writer_ref = self.writer.borrow_mut();
        if let Some(mut writer) = writer_ref.take() {
            let _ = writer.flush();
        }
    }

    pub fn flush_file(&self) {
        let mut writer_ref = self.writer.borrow_mut();
        if let Some(ref mut writer) = *writer_ref {
            let _ = writer.flush();
        }
    }

    pub fn delete_file(&mut self) {
        self.close();
        let _ = fs::remove_file(&self.path);
    }

    pub fn read_page(&self, offset: usize, limit: usize) -> Result<Vec<String>, String> {
        self.flush_file();

        let file =
            File::open(&self.path).map_err(|e| format!("Failed to open solutions file: {}", e))?;
        let reader = BufReader::new(file);
        let solutions: Vec<String> = reader
            .lines()
            .skip(offset)
            .take(limit)
            .filter_map(Result::ok)
            .collect();
        Ok(solutions)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        self.delete_file();
    }
}

pub fn cleanup_stale_temp_files() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join(TempFile::FILENAME);
    if path.exists() {
        let _ = fs::remove_file(&path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_file_creation() {
        let temp_file = TempFile::new();
        assert!(temp_file.is_ok());
    }

    #[test]
    fn test_temp_file_append() {
        let mut temp_file = TempFile::new().unwrap();
        let result = temp_file.append("test solution");
        assert!(result.is_ok());
        assert_eq!(temp_file.count(), 1);
    }

    #[test]
    fn test_temp_file_append_multiple() {
        let mut temp_file = TempFile::new().unwrap();
        for i in 0..5 {
            let result = temp_file.append(&format!("solution {}", i));
            assert!(result.is_ok());
        }
        assert_eq!(temp_file.count(), 5);
    }

    #[test]
    fn test_temp_file_get_path() {
        let temp_file = TempFile::new().unwrap();
        let path = temp_file.get_path();
        assert!(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains("llminx")
        );
    }

    #[test]
    fn test_temp_file_close() {
        let mut temp_file = TempFile::new().unwrap();
        temp_file.append("test").unwrap();
        temp_file.close();
        // After close, append should fail
        let result = temp_file.append("another");
        assert!(result.is_err());
    }

    #[test]
    fn test_temp_file_flush_file() {
        let temp_file = TempFile::new().unwrap();
        temp_file.flush_file();
    }

    #[test]
    fn test_temp_file_delete_file() {
        let mut temp_file = TempFile::new().unwrap();
        temp_file.append("test").unwrap();
        let path = temp_file.get_path().to_path_buf();

        temp_file.delete_file();
        assert!(!path.exists() || temp_file.append("should fail").is_err());
    }

    #[test]
    fn test_temp_file_count() {
        let mut temp_file = TempFile::new().unwrap();
        assert_eq!(temp_file.count(), 0);
        temp_file.append("test").unwrap();
        assert_eq!(temp_file.count(), 1);
    }

    #[test]
    fn test_cleanup_stale_temp_files() {
        // Create a temp file first
        let temp_file = TempFile::new().unwrap();
        let _path = temp_file.get_path().to_path_buf();

        // Manually delete to trigger cleanup (drop will also delete)
        drop(temp_file);

        // Cleanup should not panic even if file doesn't exist
        cleanup_stale_temp_files();
    }

    #[test]
    fn test_temp_file_append_after_close() {
        let mut temp_file = TempFile::new().unwrap();
        temp_file.append("before close").unwrap();
        temp_file.close();

        let result = temp_file.append("after close");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Writer is closed"));
    }
}
