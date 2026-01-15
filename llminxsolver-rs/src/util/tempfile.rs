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
