use crate::StatusCallback;
use crate::memory_config::MemoryConfig;
use crate::minx::LLMinx;
use crate::pruner::{DEFAULT_PRUNING_DEPTH, MAX_PRUNING_DEPTH, MIN_PRUNING_DEPTH};
use crate::search_mode::{Metric, SearchMode};
use crate::solver::{Solver, StatusEvent, StatusEventType};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ParallelSolver {
    modes: Vec<SearchMode>,
    metric: Metric,
    max_search_depth: usize,
    limit_search_depth: bool,
    pruning_depth: u8,
    mode_pruning_depths: std::collections::HashMap<SearchMode, u8>,
    memory_config: MemoryConfig,
    ignore_corner_positions: bool,
    ignore_edge_positions: bool,
    ignore_corner_orientations: bool,
    ignore_edge_orientations: bool,
    interrupted: Arc<AtomicBool>,
    status_callback: Option<StatusCallback>,
}

impl Default for ParallelSolver {
    fn default() -> Self {
        Self::new(vec![SearchMode::RU])
    }
}

impl ParallelSolver {
    pub fn new(modes: Vec<SearchMode>) -> Self {
        Self::with_config(modes, MemoryConfig::default())
    }

    pub fn with_config(modes: Vec<SearchMode>, memory_config: MemoryConfig) -> Self {
        let modes = if modes.is_empty() {
            vec![SearchMode::RU]
        } else {
            modes
        };

        Self {
            modes,
            metric: Metric::Fifth,
            max_search_depth: 12,
            limit_search_depth: false,
            pruning_depth: DEFAULT_PRUNING_DEPTH,
            mode_pruning_depths: std::collections::HashMap::new(),
            memory_config,
            ignore_corner_positions: false,
            ignore_edge_positions: false,
            ignore_corner_orientations: false,
            ignore_edge_orientations: false,
            interrupted: Arc::new(AtomicBool::new(false)),
            status_callback: None,
        }
    }

    pub fn modes(&self) -> &[SearchMode] {
        &self.modes
    }

    pub fn set_modes(&mut self, modes: Vec<SearchMode>) {
        self.modes = if modes.is_empty() {
            vec![SearchMode::RU]
        } else {
            modes
        };
    }

    pub fn add_mode(&mut self, mode: SearchMode) {
        if !self.modes.contains(&mode) {
            self.modes.push(mode);
        }
    }

    pub fn remove_mode(&mut self, mode: SearchMode) {
        self.modes.retain(|m| *m != mode);
        if self.modes.is_empty() {
            self.modes.push(SearchMode::RU);
        }
    }

    pub fn metric(&self) -> Metric {
        self.metric
    }

    pub fn set_metric(&mut self, metric: Metric) {
        self.metric = metric;
    }

    pub fn max_search_depth(&self) -> usize {
        self.max_search_depth
    }

    pub fn set_max_search_depth(&mut self, depth: usize) {
        self.max_search_depth = depth;
    }

    pub fn limit_search_depth(&self) -> bool {
        self.limit_search_depth
    }

    pub fn set_limit_search_depth(&mut self, limit: bool) {
        self.limit_search_depth = limit;
    }

    pub fn pruning_depth(&self) -> u8 {
        self.pruning_depth
    }

    pub fn set_pruning_depth(&mut self, depth: u8) {
        self.pruning_depth = depth.clamp(MIN_PRUNING_DEPTH, MAX_PRUNING_DEPTH);
    }

    pub fn set_mode_pruning_depth(&mut self, mode: SearchMode, depth: u8) {
        let clamped = depth.clamp(MIN_PRUNING_DEPTH, MAX_PRUNING_DEPTH);
        self.mode_pruning_depths.insert(mode, clamped);
    }

    fn get_pruning_depth_for_mode(&self, mode: SearchMode) -> u8 {
        *self
            .mode_pruning_depths
            .get(&mode)
            .unwrap_or(&self.pruning_depth)
    }

    pub fn memory_config(&self) -> &MemoryConfig {
        &self.memory_config
    }

    pub fn set_memory_config(&mut self, config: MemoryConfig) {
        self.memory_config = config;
    }

    pub fn set_ignore_corner_positions(&mut self, ignore: bool) {
        self.ignore_corner_positions = ignore;
    }

    pub fn set_ignore_edge_positions(&mut self, ignore: bool) {
        self.ignore_edge_positions = ignore;
    }

    pub fn set_ignore_corner_orientations(&mut self, ignore: bool) {
        self.ignore_corner_orientations = ignore;
    }

    pub fn set_ignore_edge_orientations(&mut self, ignore: bool) {
        self.ignore_edge_orientations = ignore;
    }

    pub fn set_status_callback<F>(&mut self, callback: F)
    where
        F: Fn(StatusEvent) + Send + Sync + 'static,
    {
        self.status_callback = Some(Arc::new(callback));
    }

    pub fn interrupt_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.interrupted)
    }

    pub fn interrupt(&self) {
        self.interrupted.store(true, Ordering::SeqCst);
    }

    fn fire_event(&self, event: StatusEvent) {
        if let Some(ref callback) = self.status_callback {
            callback(event);
        }
    }

    fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::SeqCst)
    }

    pub fn solve(&mut self, start: LLMinx) -> Vec<String> {
        let start_time = std::time::Instant::now();
        self.interrupted.store(false, Ordering::SeqCst);

        if self.modes.len() == 1 {
            return self.solve_single_mode(start, self.modes[0]);
        }

        self.fire_event(StatusEvent::new(
            StatusEventType::StartSearch,
            "Searching...",
            0.0,
        ));

        let modes_with_depths: Vec<_> = self
            .modes
            .iter()
            .map(|&mode| (mode, self.get_pruning_depth_for_mode(mode)))
            .collect();

        let metric = self.metric;
        let max_search_depth = self.max_search_depth;
        let limit_search_depth = self.limit_search_depth;
        let memory_config = self.memory_config;
        let ignore_corner_positions = self.ignore_corner_positions;
        let ignore_edge_positions = self.ignore_edge_positions;
        let ignore_corner_orientations = self.ignore_corner_orientations;
        let ignore_edge_orientations = self.ignore_edge_orientations;
        let interrupted = Arc::clone(&self.interrupted);
        let parent_callback = self.status_callback.clone();

        let threads_per_mode = (memory_config.search_threads / self.modes.len()).max(1);

        let mode_config = MemoryConfig::new(
            memory_config.budget_mb() / self.modes.len().max(1),
            memory_config.table_generation_threads,
            threads_per_mode,
        );

        std::thread::scope(|s| {
            let handles: Vec<_> = modes_with_depths
                .into_iter()
                .map(|(mode, pruning_depth)| {
                    let start_clone = start.clone();
                    let interrupted_clone = Arc::clone(&interrupted);
                    let mode_config_clone = mode_config;
                    let callback_clone = parent_callback.clone();

                    s.spawn(move || {
                        if interrupted_clone.load(Ordering::Relaxed) {
                            return;
                        }

                        let mut solver =
                            Solver::with_parallel_config(mode, max_search_depth, mode_config_clone);
                        solver.set_metric(metric);
                        solver.set_limit_search_depth(limit_search_depth);
                        solver.set_pruning_depth(pruning_depth);
                        solver.set_start(start_clone);
                        solver.set_ignore_corner_positions(ignore_corner_positions);
                        solver.set_ignore_edge_positions(ignore_edge_positions);
                        solver.set_ignore_corner_orientations(ignore_corner_orientations);
                        solver.set_ignore_edge_orientations(ignore_edge_orientations);

                        let solver_interrupt = solver.interrupt_handle();
                        let interrupt_watcher = Arc::clone(&interrupted_clone);
                        std::thread::spawn(move || {
                            while !interrupt_watcher.load(Ordering::Relaxed) {
                                std::thread::sleep(std::time::Duration::from_millis(50));
                            }
                            solver_interrupt.store(true, Ordering::SeqCst);
                        });

                        if let Some(ref cb) = callback_clone {
                            let cb_ref = Arc::clone(cb);
                            let mode_name = format!("{:?}", mode);
                            solver.set_status_callback(move |event| {
                                let event_with_mode = StatusEvent::with_context(
                                    event.event_type,
                                    &event.message,
                                    event.progress,
                                    Some(mode_name.clone()),
                                    event.current_depth,
                                );
                                cb_ref(event_with_mode);
                            });
                        }

                        solver.solve();
                    })
                })
                .collect();

            for handle in handles {
                let _ = handle.join();
            }
        });

        let elapsed = start_time.elapsed();
        let was_interrupted = self.is_interrupted();
        self.interrupted.store(false, Ordering::SeqCst);

        let msg = if was_interrupted {
            format!("Interrupted after {}s", elapsed.as_secs())
        } else {
            format!("Completed in {}s", elapsed.as_secs())
        };

        self.fire_event(StatusEvent::new(StatusEventType::FinishSearch, &msg, 1.0));

        Vec::new()
    }

    fn solve_single_mode(&mut self, start: LLMinx, mode: SearchMode) -> Vec<String> {
        let mut solver =
            Solver::with_parallel_config(mode, self.max_search_depth, self.memory_config);
        solver.set_metric(self.metric);
        solver.set_limit_search_depth(self.limit_search_depth);
        solver.set_pruning_depth(self.get_pruning_depth_for_mode(mode));
        solver.set_start(start);
        solver.set_ignore_corner_positions(self.ignore_corner_positions);
        solver.set_ignore_edge_positions(self.ignore_edge_positions);
        solver.set_ignore_corner_orientations(self.ignore_corner_orientations);
        solver.set_ignore_edge_orientations(self.ignore_edge_orientations);

        if let Some(ref callback) = self.status_callback {
            let callback_clone: Arc<dyn Fn(StatusEvent) + Send + Sync> = Arc::from(unsafe {
                std::mem::transmute::<
                    &(dyn Fn(StatusEvent) + Send + Sync),
                    &'static (dyn Fn(StatusEvent) + Send + Sync),
                >(callback.as_ref())
            });
            solver.set_status_callback(move |event| callback_clone(event));
        }

        let solver_interrupt = solver.interrupt_handle();
        let interrupt_watcher = Arc::clone(&self.interrupted);
        std::thread::spawn(move || {
            while !interrupt_watcher.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            solver_interrupt.store(true, Ordering::SeqCst);
        });

        solver.solve()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn lock() -> &'static Mutex<()> {
        static LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn test_parallel_solver_creation() {
        let _guard = lock().lock().unwrap();
        let solver = ParallelSolver::new(vec![SearchMode::RU, SearchMode::RUF]);
        assert_eq!(solver.modes().len(), 2);
    }

    #[test]
    fn test_add_remove_modes() {
        let _guard = lock().lock().unwrap();
        let mut solver = ParallelSolver::new(vec![SearchMode::RU]);
        solver.add_mode(SearchMode::RUF);
        assert_eq!(solver.modes().len(), 2);

        solver.remove_mode(SearchMode::RU);
        assert_eq!(solver.modes().len(), 1);
        assert_eq!(solver.modes()[0], SearchMode::RUF);

        solver.remove_mode(SearchMode::RUF);
        assert_eq!(solver.modes().len(), 1);
    }

    #[test]
    fn test_default_mode_when_empty_modes_provided() {
        let _guard = lock().lock().unwrap();
        let mut solver = ParallelSolver::new(vec![]);
        assert_eq!(solver.modes(), &[SearchMode::RU]);

        solver.set_modes(vec![]);
        assert_eq!(solver.modes(), &[SearchMode::RU]);
    }

    #[test]
    fn test_metric_depth_and_memory_setters() {
        let _guard = lock().lock().unwrap();
        let mut solver = ParallelSolver::new(vec![SearchMode::RU]);

        solver.set_metric(Metric::Face);
        solver.set_max_search_depth(7);
        solver.set_limit_search_depth(true);
        solver.set_pruning_depth(255);
        solver.set_mode_pruning_depth(SearchMode::RU, 1);

        let cfg = MemoryConfig::new(128, 2, 2);
        solver.set_memory_config(cfg);

        assert_eq!(solver.metric(), Metric::Face);
        assert_eq!(solver.max_search_depth(), 7);
        assert!(solver.limit_search_depth());
        assert_eq!(solver.pruning_depth(), MAX_PRUNING_DEPTH);
        assert_eq!(solver.memory_config().budget_mb(), 128);
    }

    #[test]
    fn test_interrupt_handle_and_interrupt() {
        let _guard = lock().lock().unwrap();
        let solver = ParallelSolver::new(vec![SearchMode::RU]);
        let handle = solver.interrupt_handle();
        assert!(!handle.load(Ordering::SeqCst));
        solver.interrupt();
        assert!(handle.load(Ordering::SeqCst));
    }

    #[test]
    fn test_single_mode_solve_returns_vec() {
        let _guard = lock().lock().unwrap();
        let mut solver = ParallelSolver::new(vec![SearchMode::RU]);
        solver.set_max_search_depth(0);
        solver.set_limit_search_depth(true);
        solver.set_memory_config(MemoryConfig::new(0, 1, 1));
        let out = solver.solve(LLMinx::new());
        assert!(out.is_empty());
    }

    #[test]
    fn test_multi_mode_solve_and_callback_path() {
        let _guard = lock().lock().unwrap();
        let mut solver = ParallelSolver::new(vec![SearchMode::RU, SearchMode::RUF]);
        solver.set_max_search_depth(0);
        solver.set_limit_search_depth(true);
        solver.set_memory_config(MemoryConfig::new(128, 1, 2));
        solver.set_mode_pruning_depth(SearchMode::RU, 8);
        solver.set_mode_pruning_depth(SearchMode::RUF, 9);

        let interrupt = solver.interrupt_handle();

        let event_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let event_count_clone = Arc::clone(&event_count);
        solver.set_status_callback(move |_event| {
            event_count_clone.fetch_add(1, Ordering::Relaxed);
            interrupt.store(true, Ordering::SeqCst);
        });

        let out = solver.solve(LLMinx::new());
        assert!(out.is_empty());
        assert!(event_count.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn test_default_constructor_and_set_modes_non_empty() {
        let _guard = lock().lock().unwrap();
        let mut solver = ParallelSolver::default();
        assert_eq!(solver.modes(), &[SearchMode::RU]);

        solver.set_modes(vec![SearchMode::RUF]);
        assert_eq!(solver.modes(), &[SearchMode::RUF]);
    }

    #[test]
    fn test_single_mode_callback_branch_runs() {
        let _guard = lock().lock().unwrap();
        let mut solver = ParallelSolver::new(vec![SearchMode::RU]);
        solver.set_max_search_depth(0);
        solver.set_limit_search_depth(true);
        solver.set_memory_config(MemoryConfig::new(0, 1, 1));

        let seen = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let seen_clone = Arc::clone(&seen);
        solver.set_status_callback(move |_event| {
            seen_clone.fetch_add(1, Ordering::Relaxed);
        });

        let out = solver.solve(LLMinx::new());
        assert!(out.is_empty());
        assert!(seen.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn test_multi_mode_non_interrupted_completion_path() {
        let _guard = lock().lock().unwrap();
        let mut solver = ParallelSolver::new(vec![SearchMode::RU, SearchMode::RUF]);
        solver.set_max_search_depth(0);
        solver.set_limit_search_depth(true);
        solver.set_memory_config(MemoryConfig::new(0, 1, 2));
        solver.set_mode_pruning_depth(SearchMode::RU, 8);
        solver.set_mode_pruning_depth(SearchMode::RUF, 9);
        solver.set_ignore_corner_positions(true);
        solver.set_ignore_edge_positions(true);
        solver.set_ignore_corner_orientations(true);
        solver.set_ignore_edge_orientations(true);

        let events = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let events_clone = Arc::clone(&events);
        solver.set_status_callback(move |_event| {
            events_clone.fetch_add(1, Ordering::Relaxed);
        });

        let out = solver.solve(LLMinx::new());
        assert!(out.is_empty());
        assert!(events.load(Ordering::Relaxed) > 0);
        assert!(!solver.interrupt_handle().load(Ordering::SeqCst));
    }
}
