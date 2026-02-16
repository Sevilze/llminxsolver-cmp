use std::cell::RefCell;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};

static TEMPFILE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_temp_suffix() -> String {
    let pid = process::id();
    let count = TEMPFILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}", pid, count)
}

pub struct TempFile {
    path: PathBuf,
    writer: RefCell<Option<BufWriter<File>>>,
    count: usize,
}

impl TempFile {
    const FILE_PREFIX: &'static str = "llminx_solutions";

    pub fn new() -> Result<Self, String> {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join(format!(
            "{}_{}.txt",
            Self::FILE_PREFIX,
            unique_temp_suffix()
        ));

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
    if let Ok(entries) = fs::read_dir(temp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| {
                        name.starts_with(TempFile::FILE_PREFIX) && name.ends_with(".txt")
                    })
            {
                let _ = fs::remove_file(path);
            }
        }
    }
}

pub struct BatchTempFile {
    dir: PathBuf,
    counts: std::collections::HashMap<usize, usize>,
    total_count: usize,
}

impl BatchTempFile {
    const DIR_PREFIX: &'static str = "llminx_batch_solutions";
    const CASES_PER_SEGMENT: usize = 100;

    pub fn new() -> Result<Self, String> {
        let temp_dir = std::env::temp_dir();
        let dir = temp_dir.join(format!("{}_{}", Self::DIR_PREFIX, unique_temp_suffix()));

        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create batch temp directory: {}", e))?;

        Ok(Self {
            dir,
            counts: std::collections::HashMap::new(),
            total_count: 0,
        })
    }

    fn segment_path(&self, case_number: usize) -> PathBuf {
        let segment_index = case_number / Self::CASES_PER_SEGMENT;
        self.dir.join(format!("batch_seg_{}.txt", segment_index))
    }

    pub fn append(&mut self, case_number: usize, solution: &str) -> Result<(), String> {
        let path = self.segment_path(case_number);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("Failed to open segment file: {}", e))?;

        writeln!(file, "{}\t{}", case_number, solution)
            .map_err(|e| format!("Failed to write solution: {}", e))?;

        *self.counts.entry(case_number).or_insert(0) += 1;
        self.total_count += 1;
        Ok(())
    }

    pub fn read_case_page(
        &self,
        case_number: usize,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<String>, String> {
        let path = self.segment_path(case_number);

        if !path.exists() {
            return Ok(vec![]);
        }

        let file = File::open(&path).map_err(|e| format!("Failed to open segment file: {}", e))?;
        let reader = BufReader::new(file);

        let prefix = format!("{}\t", case_number);
        let solutions: Vec<String> = reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| line.starts_with(&prefix))
            .map(|line| line[prefix.len()..].to_string())
            .skip(offset)
            .take(limit)
            .collect();

        Ok(solutions)
    }

    pub fn case_count(&self, case_number: usize) -> usize {
        self.counts.get(&case_number).copied().unwrap_or(0)
    }

    pub fn count(&self) -> usize {
        self.total_count
    }

    pub fn get_path(&self) -> &Path {
        &self.dir
    }

    pub fn flush(&self) {}

    pub fn close(&mut self) {}

    pub fn delete(&mut self) {
        if self.dir.exists() {
            let _ = fs::remove_dir_all(&self.dir);
        }
        self.counts.clear();
        self.total_count = 0;
    }
}

impl Drop for BatchTempFile {
    fn drop(&mut self) {
        self.delete();
    }
}

pub fn cleanup_stale_batch_temp_files() {
    let temp_dir = std::env::temp_dir();
    if let Ok(entries) = fs::read_dir(temp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(BatchTempFile::DIR_PREFIX))
            {
                let _ = fs::remove_dir_all(path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn test_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn test_temp_file_creation() {
        let _guard = test_lock().lock().unwrap();
        let temp_file = TempFile::new();
        assert!(temp_file.is_ok());
    }

    #[test]
    fn test_temp_file_append() {
        let _guard = test_lock().lock().unwrap();
        let mut temp_file = TempFile::new().unwrap();
        let result = temp_file.append("test solution");
        assert!(result.is_ok());
        assert_eq!(temp_file.count(), 1);
    }

    #[test]
    fn test_temp_file_append_multiple() {
        let _guard = test_lock().lock().unwrap();
        let mut temp_file = TempFile::new().unwrap();
        for i in 0..5 {
            let result = temp_file.append(&format!("solution {}", i));
            assert!(result.is_ok());
        }
        assert_eq!(temp_file.count(), 5);
    }

    #[test]
    fn test_temp_file_get_path() {
        let _guard = test_lock().lock().unwrap();
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
        let _guard = test_lock().lock().unwrap();
        let mut temp_file = TempFile::new().unwrap();
        temp_file.append("test").unwrap();
        temp_file.close();
        let result = temp_file.append("another");
        assert!(result.is_err());
    }

    #[test]
    fn test_temp_file_flush_file() {
        let _guard = test_lock().lock().unwrap();
        let temp_file = TempFile::new().unwrap();
        temp_file.flush_file();
    }

    #[test]
    fn test_temp_file_delete_file() {
        let _guard = test_lock().lock().unwrap();
        let mut temp_file = TempFile::new().unwrap();
        temp_file.append("test").unwrap();
        let path = temp_file.get_path().to_path_buf();

        temp_file.delete_file();
        assert!(!path.exists() || temp_file.append("should fail").is_err());
    }

    #[test]
    fn test_temp_file_count() {
        let _guard = test_lock().lock().unwrap();
        let mut temp_file = TempFile::new().unwrap();
        assert_eq!(temp_file.count(), 0);
        temp_file.append("test").unwrap();
        assert_eq!(temp_file.count(), 1);
    }

    #[test]
    fn test_cleanup_stale_temp_files() {
        let _guard = test_lock().lock().unwrap();
        let temp_file = TempFile::new().unwrap();
        let _path = temp_file.get_path().to_path_buf();
        drop(temp_file);
        cleanup_stale_temp_files();
    }

    #[test]
    fn test_temp_file_append_after_close() {
        let _guard = test_lock().lock().unwrap();
        let mut temp_file = TempFile::new().unwrap();
        temp_file.append("before close").unwrap();
        temp_file.close();

        let result = temp_file.append("after close");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Writer is closed"));
    }

    #[test]
    fn test_batch_temp_file_creation() {
        let _guard = test_lock().lock().unwrap();
        let btf = BatchTempFile::new();
        assert!(btf.is_ok());
        let btf = btf.unwrap();
        assert!(btf.get_path().exists());
        assert_eq!(btf.count(), 0);
    }

    #[test]
    fn test_batch_temp_file_append_and_count() {
        let _guard = test_lock().lock().unwrap();
        let mut btf = BatchTempFile::new().unwrap();
        btf.append(0, "R U R' U'").unwrap();
        btf.append(0, "U R U' R'").unwrap();
        btf.append(1, "R U2 R'").unwrap();

        assert_eq!(btf.count(), 3);
        assert_eq!(btf.case_count(0), 2);
        assert_eq!(btf.case_count(1), 1);
        assert_eq!(btf.case_count(99), 0);
    }

    #[test]
    fn test_batch_temp_file_read_case_page() {
        let _guard = test_lock().lock().unwrap();
        let mut btf = BatchTempFile::new().unwrap();
        for i in 0..10 {
            btf.append(0, &format!("sol_{}", i)).unwrap();
        }
        btf.append(1, "other_case").unwrap();

        let page = btf.read_case_page(0, 0, 5).unwrap();
        assert_eq!(page.len(), 5);
        assert_eq!(page[0], "sol_0");
        assert_eq!(page[4], "sol_4");

        let page = btf.read_case_page(0, 5, 10).unwrap();
        assert_eq!(page.len(), 5);
        assert_eq!(page[0], "sol_5");

        let page = btf.read_case_page(1, 0, 100).unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0], "other_case");

        let page = btf.read_case_page(99, 0, 100).unwrap();
        assert!(page.is_empty());
    }

    #[test]
    fn test_batch_temp_file_segmentation() {
        let _guard = test_lock().lock().unwrap();
        let mut btf = BatchTempFile::new().unwrap();
        btf.append(0, "case_0_sol").unwrap();
        btf.append(99, "case_99_sol").unwrap();
        btf.append(100, "case_100_sol").unwrap();
        btf.append(250, "case_250_sol").unwrap();

        // Cases 0 and 99 share segment 0, case 100 is segment 1, case 250 is segment 2
        let seg0 = btf.dir.join("batch_seg_0.txt");
        let seg1 = btf.dir.join("batch_seg_1.txt");
        let seg2 = btf.dir.join("batch_seg_2.txt");
        assert!(seg0.exists());
        assert!(seg1.exists());
        assert!(seg2.exists());

        let page = btf.read_case_page(0, 0, 100).unwrap();
        assert_eq!(page, vec!["case_0_sol"]);

        let page = btf.read_case_page(99, 0, 100).unwrap();
        assert_eq!(page, vec!["case_99_sol"]);

        let page = btf.read_case_page(100, 0, 100).unwrap();
        assert_eq!(page, vec!["case_100_sol"]);

        let page = btf.read_case_page(250, 0, 100).unwrap();
        assert_eq!(page, vec!["case_250_sol"]);
    }

    #[test]
    fn test_batch_temp_file_delete() {
        let _guard = test_lock().lock().unwrap();
        let mut btf = BatchTempFile::new().unwrap();
        btf.append(0, "test").unwrap();
        let dir = btf.get_path().to_path_buf();
        assert!(dir.exists());

        btf.delete();
        assert!(!dir.exists());
        assert_eq!(btf.count(), 0);
    }

    #[test]
    fn test_cleanup_stale_batch_temp_files() {
        let _guard = test_lock().lock().unwrap();
        let btf = BatchTempFile::new().unwrap();
        let dir = btf.get_path().to_path_buf();
        // Prevent Drop from cleaning up before we test
        std::mem::forget(btf);

        assert!(dir.exists());
        cleanup_stale_batch_temp_files();
        assert!(!dir.exists());
    }
}
