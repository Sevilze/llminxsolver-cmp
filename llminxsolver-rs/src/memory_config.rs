use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const DEFAULT_MOBILE_BUDGET_MB: usize = 256;
const DEFAULT_DESKTOP_BUDGET_PERCENT: f64 = 0.5;
const MIN_THREADS: usize = 1;
const BYTES_PER_MB: usize = 1024 * 1024;
const MEMORY_WARNING_THRESHOLD: f64 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub total_budget_bytes: usize,
    pub table_generation_threads: usize,
    pub search_threads: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self::for_desktop()
    }
}

impl MemoryConfig {
    pub fn new(budget_mb: usize, table_gen_threads: usize, search_threads: usize) -> Self {
        Self {
            total_budget_bytes: budget_mb * BYTES_PER_MB,
            table_generation_threads: table_gen_threads.max(MIN_THREADS),
            search_threads: search_threads.max(MIN_THREADS),
        }
    }

    pub fn for_desktop() -> Self {
        let num_cpus = num_cpus::get();
        let total_memory = get_system_memory_bytes();
        let budget_bytes = (total_memory as f64 * DEFAULT_DESKTOP_BUDGET_PERCENT) as usize;

        Self {
            total_budget_bytes: budget_bytes,
            table_generation_threads: num_cpus,
            search_threads: num_cpus,
        }
    }

    pub fn for_mobile(budget_mb: usize) -> Self {
        Self {
            total_budget_bytes: budget_mb * BYTES_PER_MB,
            table_generation_threads: 2,
            search_threads: 4,
        }
    }

    pub fn for_mobile_default() -> Self {
        Self::for_mobile(DEFAULT_MOBILE_BUDGET_MB)
    }

    pub fn with_budget(budget_mb: usize, threads: usize) -> Self {
        Self {
            total_budget_bytes: budget_mb * BYTES_PER_MB,
            table_generation_threads: threads.max(MIN_THREADS),
            search_threads: threads.max(MIN_THREADS),
        }
    }

    pub fn budget_mb(&self) -> usize {
        self.total_budget_bytes / BYTES_PER_MB
    }

    pub fn set_budget_mb(&mut self, budget_mb: usize) {
        self.total_budget_bytes = budget_mb * BYTES_PER_MB;
    }

    pub fn set_table_generation_threads(&mut self, threads: usize) {
        self.table_generation_threads = threads.max(MIN_THREADS);
    }

    pub fn set_search_threads(&mut self, threads: usize) {
        self.search_threads = threads.max(MIN_THREADS);
    }

    pub fn available_cpus() -> usize {
        num_cpus::get()
    }

    pub fn available_memory_mb() -> usize {
        get_system_memory_bytes() / BYTES_PER_MB
    }
}

fn get_system_memory_bytes() -> usize {
    #[cfg(target_os = "linux")]
    {
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 && let Ok(kb) = parts[1].parse::<usize>() {
                        return kb * 1024;
                    }
                }
            }
        }
        4 * 1024 * BYTES_PER_MB
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("sysctl").args(["-n", "hw.memsize"]).output() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(bytes) = s.trim().parse::<usize>() {
                    return bytes;
                }
            }
        }
        8 * 1024 * BYTES_PER_MB
    }

    #[cfg(target_os = "windows")]
    {
        8 * 1024 * BYTES_PER_MB
    }

    #[cfg(target_os = "android")]
    {
        2 * 1024 * BYTES_PER_MB
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
        target_os = "android"
    )))]
    {
        4 * 1024 * BYTES_PER_MB
    }
}

pub fn get_available_system_memory_bytes() -> usize {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("MemAvailable:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 && let Ok(kb) = parts[1].parse::<usize>() {
                        return kb * 1024;
                    }
                }
            }
        }
        get_system_memory_bytes() / 2
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("vm_stat").output() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let mut free_pages = 0u64;
                let mut inactive_pages = 0u64;
                for line in s.lines() {
                    if line.starts_with("Pages free:") {
                        if let Some(val) = line.split(':').nth(1) {
                            free_pages = val.trim().trim_end_matches('.').parse().unwrap_or(0);
                        }
                    } else if line.starts_with("Pages inactive:") {
                        if let Some(val) = line.split(':').nth(1) {
                            inactive_pages = val.trim().trim_end_matches('.').parse().unwrap_or(0);
                        }
                    }
                }
                let page_size = 4096u64;
                return ((free_pages + inactive_pages) * page_size) as usize;
            }
        }
        get_system_memory_bytes() / 2
    }

    #[cfg(target_os = "windows")]
    {
        get_system_memory_bytes() / 2
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
        target_os = "android"
    )))]
    {
        get_system_memory_bytes() / 2
    }
}

pub fn get_available_memory_mb() -> usize {
    get_available_system_memory_bytes() / BYTES_PER_MB
}

#[derive(Clone)]
pub struct MemoryTracker {
    budget_bytes: usize,
    used_bytes: Arc<AtomicUsize>,
}

impl MemoryTracker {
    pub fn new(budget_mb: usize) -> Self {
        Self {
            budget_bytes: budget_mb * BYTES_PER_MB,
            used_bytes: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn from_config(config: &MemoryConfig) -> Self {
        Self {
            budget_bytes: config.total_budget_bytes,
            used_bytes: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn can_allocate(&self, bytes: usize) -> bool {
        let current = self.used_bytes.load(Ordering::Relaxed);
        current + bytes <= self.budget_bytes
    }

    pub fn try_allocate(&self, bytes: usize) -> bool {
        loop {
            let current = self.used_bytes.load(Ordering::Relaxed);
            if current + bytes > self.budget_bytes {
                return false;
            }
            if self.used_bytes.compare_exchange_weak(
                current,
                current + bytes,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ).is_ok() {
                return true;
            }
        }
    }

    pub fn allocate(&self, bytes: usize) {
        self.used_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn deallocate(&self, bytes: usize) {
        self.used_bytes.fetch_sub(bytes, Ordering::Relaxed);
    }

    pub fn used_bytes(&self) -> usize {
        self.used_bytes.load(Ordering::Relaxed)
    }

    pub fn used_mb(&self) -> usize {
        self.used_bytes() / BYTES_PER_MB
    }

    pub fn remaining_bytes(&self) -> usize {
        self.budget_bytes.saturating_sub(self.used_bytes())
    }

    pub fn remaining_mb(&self) -> usize {
        self.remaining_bytes() / BYTES_PER_MB
    }

    pub fn usage_percentage(&self) -> f64 {
        if self.budget_bytes == 0 {
            return 100.0;
        }
        (self.used_bytes() as f64 / self.budget_bytes as f64) * 100.0
    }

    pub fn is_at_warning_threshold(&self) -> bool {
        self.usage_percentage() >= MEMORY_WARNING_THRESHOLD * 100.0
    }

    pub fn budget_mb(&self) -> usize {
        self.budget_bytes / BYTES_PER_MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_config_creation() {
        let config = MemoryConfig::new(512, 4, 8);
        assert_eq!(config.budget_mb(), 512);
        assert_eq!(config.table_generation_threads, 4);
        assert_eq!(config.search_threads, 8);
    }

    #[test]
    fn test_mobile_defaults() {
        let config = MemoryConfig::for_mobile_default();
        assert_eq!(config.budget_mb(), 256);
        assert_eq!(config.table_generation_threads, 2);
        assert_eq!(config.search_threads, 4);
    }

    #[test]
    fn test_min_threads() {
        let config = MemoryConfig::new(256, 0, 0);
        assert_eq!(config.table_generation_threads, 1);
        assert_eq!(config.search_threads, 1);
    }
}

