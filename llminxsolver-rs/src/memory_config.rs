use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2
                        && let Ok(kb) = parts[1].parse::<usize>()
                    {
                        return kb * 1024;
                    }
                }
            }
        }
        2 * 1024 * BYTES_PER_MB
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
        use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

        let mut mem_status = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };

        unsafe {
            GlobalMemoryStatusEx(&mut mem_status).expect("Failed to get system memory info");
            mem_status.ullTotalPhys as usize
        }
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
                    if parts.len() >= 2
                        && let Ok(kb) = parts[1].parse::<usize>()
                    {
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
        use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

        let mut mem_status = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };

        unsafe {
            GlobalMemoryStatusEx(&mut mem_status).expect("Failed to get available memory info");
            mem_status.ullAvailPhys as usize
        }
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

pub fn get_current_rss_bytes() -> usize {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2
                        && let Ok(kb) = parts[1].parse::<usize>()
                    {
                        return kb * 1024;
                    }
                }
            }
        }
        0
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let pid = std::process::id();
        if let Ok(output) = Command::new("ps")
            .args(["-o", "rss=", "-p", &pid.to_string()])
            .output()
        {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(kb) = s.trim().parse::<usize>() {
                    return kb * 1024;
                }
            }
        }
        0
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let pid = std::process::id();
        if let Ok(output) = Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-Command",
                &format!("(Get-Process -Id {pid}).WorkingSet64"),
            ])
            .output()
        {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(bytes) = s.trim().parse::<usize>() {
                    return bytes;
                }
            }
        }
        0
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
        target_os = "android"
    )))]
    {
        0
    }
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
            if self
                .used_bytes
                .compare_exchange_weak(
                    current,
                    current + bytes,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
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
        let config = MemoryConfig::new(128, 0, 0);
        assert_eq!(config.table_generation_threads, 1);
        assert_eq!(config.search_threads, 1);
    }

    #[test]
    fn test_with_budget_and_setters() {
        let mut config = MemoryConfig::with_budget(128, 0);
        assert_eq!(config.budget_mb(), 128);
        assert_eq!(config.table_generation_threads, 1);
        assert_eq!(config.search_threads, 1);

        config.set_budget_mb(384);
        config.set_table_generation_threads(3);
        config.set_search_threads(5);

        assert_eq!(config.budget_mb(), 384);
        assert_eq!(config.table_generation_threads, 3);
        assert_eq!(config.search_threads, 5);
    }

    #[test]
    fn test_for_desktop_reasonable_defaults() {
        let config = MemoryConfig::for_desktop();
        assert!(config.total_budget_bytes > 0);
        assert!(config.table_generation_threads >= 1);
        assert!(config.search_threads >= 1);
    }

    #[test]
    fn test_available_system_info_non_zero() {
        assert!(MemoryConfig::available_cpus() >= 1);
        assert!(MemoryConfig::available_memory_mb() > 0);
        assert!(get_available_system_memory_bytes() > 0);
        assert!(get_available_memory_mb() > 0);
    }

    #[test]
    fn test_current_rss_bytes_callable() {
        let _ = get_current_rss_bytes();
    }

    #[test]
    fn test_memory_tracker_allocate_and_deallocate() {
        let tracker = MemoryTracker::new(16);
        assert_eq!(tracker.budget_mb(), 16);
        assert_eq!(tracker.used_bytes(), 0);
        assert!(tracker.can_allocate(1024));

        tracker.allocate(1024);
        assert_eq!(tracker.used_bytes(), 1024);
        assert!(tracker.remaining_bytes() < 16 * 1024 * 1024);

        tracker.deallocate(1024);
        assert_eq!(tracker.used_bytes(), 0);
    }

    #[test]
    fn test_memory_tracker_try_allocate_limits() {
        let tracker = MemoryTracker::new(1);
        let mb = 1024 * 1024;

        assert!(tracker.try_allocate(mb / 2));
        assert!(tracker.try_allocate(mb / 2));
        assert!(!tracker.try_allocate(1));
    }

    #[test]
    fn test_memory_tracker_from_config_and_metrics() {
        let config = MemoryConfig::new(10, 2, 2);
        let tracker = MemoryTracker::from_config(&config);
        assert_eq!(tracker.budget_mb(), 10);
        assert_eq!(tracker.used_mb(), 0);
        assert!((tracker.usage_percentage() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_memory_tracker_warning_threshold() {
        let tracker = MemoryTracker::new(10);
        let bytes_for_79_percent = (10 * 1024 * 1024 * 79) / 100;
        let bytes_for_80_percent = (10 * 1024 * 1024 * 80) / 100;

        tracker.allocate(bytes_for_79_percent);
        assert!(!tracker.is_at_warning_threshold());

        tracker.deallocate(bytes_for_79_percent);
        tracker.allocate(bytes_for_80_percent);
        assert!(tracker.is_at_warning_threshold());
    }

    #[test]
    fn test_memory_tracker_zero_budget_percentage() {
        let config = MemoryConfig {
            total_budget_bytes: 0,
            table_generation_threads: 1,
            search_threads: 1,
        };
        let tracker = MemoryTracker::from_config(&config);
        assert_eq!(tracker.usage_percentage(), 100.0);
        assert!(!tracker.can_allocate(1));
    }
}
