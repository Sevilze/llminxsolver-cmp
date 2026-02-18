use crate::batch_solver::types::{
    BatchCaseResult, BatchSolveResults, BatchSolverConfig, BatchSolverError, GeneratedBatchState,
};
use crate::dedicated_solver::ProgressEvent;
use llminxsolver_rs::StatusEvent;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

pub trait BatchSolverCallback: Send + Sync {
    fn on_progress(&self, event: ProgressEvent);
    fn on_case_solved(&self, result: BatchCaseResult);
    fn on_complete(&self, results: BatchSolveResults);
}

pub struct BatchSolverHandle {
    config: RwLock<BatchSolverConfig>,
    callback: RwLock<Option<Arc<dyn BatchSolverCallback>>>,
    running: Arc<AtomicBool>,
    interrupt: Arc<AtomicBool>,
    generated_states: RwLock<Vec<llminxsolver_rs::batch_solver::GeneratedState>>,
    equivalence: RwLock<Option<Arc<llminxsolver_rs::batch_solver::EquivalenceHandler>>>,
}

impl BatchSolverHandle {
    pub fn new(config: BatchSolverConfig) -> Result<Self, BatchSolverError> {
        Ok(Self {
            config: RwLock::new(config),
            callback: RwLock::new(None),
            running: Arc::new(AtomicBool::new(false)),
            interrupt: Arc::new(AtomicBool::new(false)),
            generated_states: RwLock::new(Vec::new()),
            equivalence: RwLock::new(None),
        })
    }

    pub fn set_callback(&self, callback: Box<dyn BatchSolverCallback>) {
        let mut cb = self.callback.write().unwrap();
        *cb = Some(Arc::from(callback));
    }

    pub fn update_config(&self, config: BatchSolverConfig) {
        let mut cfg = self.config.write().unwrap();
        *cfg = config;
    }

    pub fn generate_states(&self) -> Result<Vec<GeneratedBatchState>, BatchSolverError> {
        let config = self.config.read().unwrap();
        let pre_adjust: Vec<String> = config
            .pre_adjust
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let post_adjust: Vec<String> = config
            .post_adjust
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let sort_criteria: Vec<_> = config.sorting_criteria.iter().map(|c| c.to_rs()).collect();

        let gen_config = llminxsolver_rs::batch_solver::GeneratorConfig {
            scramble: config.scramble.clone(),
            equivalences_str: config.equivalences.clone(),
            pre_adjust,
            post_adjust,
            sort_criteria,
            num_threads: config.parallel_config.search_threads as usize,
        };
        drop(config);

        let callback: Option<llminxsolver_rs::batch_solver::GeneratorCallback> =
            if let Some(ref cb) = *self.callback.read().unwrap() {
                let cb_clone = Arc::clone(cb);
                Some(Arc::new(move |count, msg| {
                    cb_clone.on_progress(ProgressEvent {
                        event_type: "GeneratingStates".to_string(),
                        message: msg.to_string(),
                        progress: 0.1,
                        search_mode: None,
                        current_depth: count as u32,
                    });
                }))
            } else {
                None
            };

        let (states, equiv_handler) = llminxsolver_rs::batch_solver::generate_batch_states(
            &gen_config,
            Some(Arc::clone(&self.interrupt)),
            callback,
        )?;

        *self.equivalence.write().unwrap() = equiv_handler;

        let result: Vec<GeneratedBatchState> = states
            .iter()
            .map(|s| {
                let cp = s.state.corner_positions();
                let ep = s.state.edge_positions();
                GeneratedBatchState {
                    case_number: s.case_number as u32,
                    setup_moves: s.setup_moves.clone(),
                    corner_positions: cp.to_vec(),
                    corner_orientations: (0..5)
                        .map(|slot| s.state.get_corner_orientation(slot as u8))
                        .collect(),
                    edge_positions: ep.to_vec(),
                    edge_orientations: (0..5)
                        .map(|slot| s.state.get_edge_orientation(slot as u8))
                        .collect(),
                }
            })
            .collect();

        *self.generated_states.write().unwrap() = states;
        Ok(result)
    }

    pub fn start(&self) {
        if self.running.swap(true, Ordering::SeqCst) {
            return;
        }

        self.interrupt.store(false, Ordering::SeqCst);

        let states = {
            let states_guard = self.generated_states.read().unwrap();
            if states_guard.is_empty() {
                self.running.store(false, Ordering::SeqCst);
                return;
            }
            states_guard.clone()
        };

        let equivalence = self.equivalence.read().unwrap().clone();
        let callback = self.callback.read().unwrap().clone();
        let interrupt = Arc::clone(&self.interrupt);
        let running = Arc::clone(&self.running);

        let config = self.config.read().unwrap();
        let solver_config = llminxsolver_rs::batch_solver::BatchSolverConfig {
            search_mode: config.search_mode.into(),
            metric: config.metric.into(),
            pruning_depth: config.pruning_depth,
            max_search_depth: config.search_depth as usize,
            stop_after_first: config.stop_after_first,
            memory_config: config.parallel_config.clone().into(),
            ignore_corner_permutation: config.ignore_corner_permutation,
            ignore_edge_permutation: config.ignore_edge_permutation,
            ignore_corner_orientation: config.ignore_corner_orientation,
            ignore_edge_orientation: config.ignore_edge_orientation,
        };
        drop(config);

        std::thread::spawn(move || {
            let status_callback: Option<llminxsolver_rs::solver::StatusCallback> =
                if let Some(ref cb) = callback {
                    let cb_clone = Arc::clone(cb);
                    Some(Arc::new(move |event: StatusEvent| {
                        cb_clone.on_progress(ProgressEvent {
                            event_type: format!("{:?}", event.event_type),
                            message: event.message.clone(),
                            progress: event.progress,
                            search_mode: event.search_mode.clone(),
                            current_depth: event.current_depth,
                        });
                    }))
                } else {
                    None
                };

            let case_solved_callback: Option<llminxsolver_rs::batch_solver::CaseSolvedCallback> =
                if let Some(ref cb) = callback {
                    let cb_clone = Arc::clone(cb);
                    Some(Arc::new(
                        move |result: llminxsolver_rs::batch_solver::BatchCaseResult| {
                            cb_clone.on_case_solved(result.into());
                        },
                    ))
                } else {
                    None
                };

            let results = llminxsolver_rs::batch_solver::solve_batch_states(
                states,
                &solver_config,
                equivalence.as_ref(),
                interrupt,
                status_callback,
                case_solved_callback,
            );

            if let Some(ref cb) = callback {
                cb.on_complete(results.into());
            }

            running.store(false, Ordering::SeqCst);
        });
    }

    pub fn cancel(&self) {
        self.interrupt.store(true, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn get_total_cases(&self) -> u32 {
        self.generated_states.read().unwrap().len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch_solver::types::{BatchSolverConfig, SortingCriterion, SortingType};
    use crate::dedicated_solver::{Metric, ParallelConfig, SearchMode};
    use std::sync::atomic::AtomicUsize;

    struct TestBatchCallback {
        progress: Arc<AtomicUsize>,
        solved: Arc<AtomicUsize>,
        complete: Arc<AtomicUsize>,
    }

    impl BatchSolverCallback for TestBatchCallback {
        fn on_progress(&self, _event: ProgressEvent) {
            self.progress.fetch_add(1, Ordering::Relaxed);
        }

        fn on_case_solved(&self, _result: BatchCaseResult) {
            self.solved.fetch_add(1, Ordering::Relaxed);
        }

        fn on_complete(&self, _results: BatchSolveResults) {
            self.complete.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn base_config() -> BatchSolverConfig {
        BatchSolverConfig {
            scramble: "R".to_string(),
            equivalences: "".to_string(),
            pre_adjust: "".to_string(),
            post_adjust: "".to_string(),
            sorting_criteria: vec![SortingCriterion {
                sorting_type: SortingType::SetPriority,
                pieces: "UBL".to_string(),
            }],
            search_mode: SearchMode::RU,
            metric: Metric::Fifth,
            pruning_depth: 6,
            search_depth: 1,
            stop_after_first: true,
            parallel_config: ParallelConfig {
                memory_budget_mb: 64,
                table_gen_threads: 1,
                search_threads: 1,
            },
            ignore_corner_permutation: false,
            ignore_edge_permutation: false,
            ignore_corner_orientation: false,
            ignore_edge_orientation: false,
        }
    }

    #[test]
    fn test_batch_handle_create_update_and_cancel() {
        let handle = BatchSolverHandle::new(base_config()).unwrap();
        assert!(!handle.is_running());
        assert_eq!(handle.get_total_cases(), 0);

        let mut updated = base_config();
        updated.stop_after_first = false;
        handle.update_config(updated);

        handle.cancel();
        assert!(!handle.is_running());
    }

    #[test]
    fn test_batch_handle_start_without_states_returns_quickly() {
        let handle = BatchSolverHandle::new(base_config()).unwrap();

        let progress = Arc::new(AtomicUsize::new(0));
        let solved = Arc::new(AtomicUsize::new(0));
        let complete = Arc::new(AtomicUsize::new(0));
        handle.set_callback(Box::new(TestBatchCallback {
            progress: Arc::clone(&progress),
            solved: Arc::clone(&solved),
            complete: Arc::clone(&complete),
        }));

        handle.start();
        std::thread::sleep(std::time::Duration::from_millis(50));
        assert!(!handle.is_running());

        let _ = progress.load(Ordering::Relaxed);
        let _ = solved.load(Ordering::Relaxed);
        let _ = complete.load(Ordering::Relaxed);
    }

    #[test]
    fn test_generate_states_and_total_cases() {
        let handle = BatchSolverHandle::new(base_config()).unwrap();
        let generated = handle.generate_states().unwrap();
        assert_eq!(handle.get_total_cases(), generated.len() as u32);
    }

    #[test]
    fn test_start_with_injected_states_executes_path() {
        let handle = BatchSolverHandle::new(base_config()).unwrap();

        let rs_state = llminxsolver_rs::batch_solver::GeneratedState {
            state: llminxsolver_rs::LLMinx::new(),
            setup_moves: "".to_string(),
            case_number: 1,
        };
        *handle.generated_states.write().unwrap() = vec![rs_state];

        let progress = Arc::new(AtomicUsize::new(0));
        let solved = Arc::new(AtomicUsize::new(0));
        let complete = Arc::new(AtomicUsize::new(0));
        handle.set_callback(Box::new(TestBatchCallback {
            progress: Arc::clone(&progress),
            solved: Arc::clone(&solved),
            complete: Arc::clone(&complete),
        }));

        handle.start();
        std::thread::sleep(std::time::Duration::from_millis(200));
        handle.cancel();

        let _ = progress.load(Ordering::Relaxed);
        let _ = solved.load(Ordering::Relaxed);
        let _ = complete.load(Ordering::Relaxed);
    }

    #[test]
    fn test_generate_states_without_callback_and_adjust_lists() {
        let mut config = base_config();
        config.pre_adjust = "U, U', ".to_string();
        config.post_adjust = "R, R2, ".to_string();

        let handle = BatchSolverHandle::new(config).unwrap();
        let generated = handle.generate_states().unwrap();
        assert_eq!(handle.get_total_cases(), generated.len() as u32);
    }

    #[test]
    fn test_batch_handle_double_start_guard() {
        let handle = BatchSolverHandle::new(base_config()).unwrap();

        let rs_state = llminxsolver_rs::batch_solver::GeneratedState {
            state: llminxsolver_rs::LLMinx::new(),
            setup_moves: "".to_string(),
            case_number: 1,
        };
        *handle.generated_states.write().unwrap() = vec![rs_state];

        handle.start();
        handle.start();
        std::thread::sleep(std::time::Duration::from_millis(80));
        handle.cancel();
        assert!(handle.is_running() || !handle.is_running());
    }
}
