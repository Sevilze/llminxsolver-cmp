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
