use crate::StatusCallback;
use crate::memory_config::MemoryConfig;
use crate::minx::LLMinx;
use crate::search_mode::{Metric, SearchMode};
use crate::solver::{Solver, StatusEvent, StatusEventType};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ParallelSolver {
    modes: Vec<SearchMode>,
    metric: Metric,
    max_depth: usize,
    limit_depth: bool,
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
            max_depth: 12,
            limit_depth: false,
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

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }

    pub fn limit_depth(&self) -> bool {
        self.limit_depth
    }

    pub fn set_limit_depth(&mut self, limit: bool) {
        self.limit_depth = limit;
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

        let modes = self.modes.clone();
        let metric = self.metric;
        let max_depth = self.max_depth;
        let limit_depth = self.limit_depth;
        let memory_config = self.memory_config;
        let ignore_corner_positions = self.ignore_corner_positions;
        let ignore_edge_positions = self.ignore_edge_positions;
        let ignore_corner_orientations = self.ignore_corner_orientations;
        let ignore_edge_orientations = self.ignore_edge_orientations;
        let interrupted = Arc::clone(&self.interrupted);
        let parent_callback = self.status_callback.clone();

        let threads_per_mode = (memory_config.search_threads / modes.len()).max(1);

        let mode_config = MemoryConfig::new(
            memory_config.budget_mb() / modes.len().max(1),
            memory_config.table_generation_threads,
            threads_per_mode,
        );

        std::thread::scope(|s| {
            let handles: Vec<_> = modes
                .iter()
                .map(|&mode| {
                    let start_clone = start.clone();
                    let interrupted_clone = Arc::clone(&interrupted);
                    let mode_config_clone = mode_config;
                    let callback_clone = parent_callback.clone();

                    s.spawn(move || {
                        if interrupted_clone.load(Ordering::Relaxed) {
                            return;
                        }

                        let mut solver =
                            Solver::with_parallel_config(mode, max_depth, mode_config_clone);
                        solver.set_metric(metric);
                        solver.set_limit_depth(limit_depth);
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
        let mut solver = Solver::with_parallel_config(mode, self.max_depth, self.memory_config);
        solver.set_metric(self.metric);
        solver.set_limit_depth(self.limit_depth);
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

    #[test]
    fn test_parallel_solver_creation() {
        let solver = ParallelSolver::new(vec![SearchMode::RU, SearchMode::RUF]);
        assert_eq!(solver.modes().len(), 2);
    }

    #[test]
    fn test_add_remove_modes() {
        let mut solver = ParallelSolver::new(vec![SearchMode::RU]);
        solver.add_mode(SearchMode::RUF);
        assert_eq!(solver.modes().len(), 2);

        solver.remove_mode(SearchMode::RU);
        assert_eq!(solver.modes().len(), 1);
        assert_eq!(solver.modes()[0], SearchMode::RUF);

        solver.remove_mode(SearchMode::RUF);
        assert_eq!(solver.modes().len(), 1);
    }
}
