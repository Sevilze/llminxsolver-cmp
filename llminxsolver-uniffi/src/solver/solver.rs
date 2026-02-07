use crate::solver::types::{MegaminxState, ParallelSolverConfig, ProgressEvent, SolverConfig};
use llminxsolver_rs::{LLMinx, MemoryConfig, ParallelSolver, Solver, StatusEvent, StatusEventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

pub trait SolverCallback: Send + Sync {
    fn on_progress(&self, event: ProgressEvent);
    fn on_solution_found(&self, solution: String);
    fn on_complete(&self);
}

pub(crate) fn build_llminx(state: &MegaminxState) -> LLMinx {
    let mut minx = LLMinx::default();

    for i in 0..5 {
        if i < state.corner_positions.len() {
            minx.corner_positions_mut()[i] = state.corner_positions[i];
        }
        if i < state.edge_positions.len() {
            minx.edge_positions_mut()[i] = state.edge_positions[i];
        }
        if i < state.corner_orientations.len() {
            minx.set_corner_orientation(i as u8, state.corner_orientations[i]);
        }
        if i < state.edge_orientations.len() {
            minx.set_edge_orientation(i as u8, state.edge_orientations[i]);
        }
    }

    minx
}

pub struct SolverHandle {
    config: SolverConfig,
    state: MegaminxState,
    callback: RwLock<Option<Arc<dyn SolverCallback>>>,
    running: Arc<AtomicBool>,
    interrupt: Arc<AtomicBool>,
}

impl SolverHandle {
    pub fn new(config: SolverConfig, state: MegaminxState) -> Self {
        Self {
            config,
            state,
            callback: RwLock::new(None),
            running: Arc::new(AtomicBool::new(false)),
            interrupt: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_callback(&self, callback: Box<dyn SolverCallback>) {
        let mut cb = self.callback.write().unwrap();
        *cb = Some(Arc::from(callback));
    }

    pub fn start(&self) {
        if self.running.swap(true, Ordering::SeqCst) {
            return;
        }

        self.interrupt.store(false, Ordering::SeqCst);

        let config = self.config.clone();
        let state = self.state.clone();
        let callback = self.callback.read().unwrap().clone();
        let interrupt = Arc::clone(&self.interrupt);
        let running = Arc::clone(&self.running);

        std::thread::spawn(move || {
            let search_mode: llminxsolver_rs::SearchMode = config.search_mode.into();
            let metric: llminxsolver_rs::Metric = config.metric.into();
            let max_search_depth = if config.limit_search_depth {
                config.max_search_depth as usize
            } else {
                50
            };

            let start_state = build_llminx(&state);

            let memory_config = config
                .parallel_config
                .map(|pc| pc.into())
                .unwrap_or_default();

            let mut solver =
                Solver::with_parallel_config(search_mode, max_search_depth, memory_config);
            solver.set_metric(metric);
            solver.set_limit_search_depth(config.limit_search_depth);
            solver.set_pruning_depth(config.pruning_depth);
            solver.set_start(start_state);
            solver.set_ignore_corner_positions(config.ignore_corner_positions);
            solver.set_ignore_edge_positions(config.ignore_edge_positions);
            solver.set_ignore_corner_orientations(config.ignore_corner_orientations);
            solver.set_ignore_edge_orientations(config.ignore_edge_orientations);

            let solver_interrupt = solver.interrupt_handle();
            let interrupt_clone = Arc::clone(&interrupt);
            let running_clone = Arc::clone(&running);
            std::thread::spawn(move || {
                while !interrupt_clone.load(Ordering::SeqCst)
                    && running_clone.load(Ordering::SeqCst)
                {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                solver_interrupt.store(true, Ordering::SeqCst);
            });

            if let Some(ref cb) = callback {
                let cb_clone = Arc::clone(cb);
                solver.set_status_callback(move |event: StatusEvent| match event.event_type {
                    StatusEventType::SolutionFound => {
                        cb_clone.on_solution_found(event.message.clone());
                    }
                    StatusEventType::FinishSearch => {
                        cb_clone.on_complete();
                    }
                    _ => {
                        cb_clone.on_progress(ProgressEvent {
                            event_type: format!("{:?}", event.event_type),
                            message: event.message.clone(),
                            progress: event.progress,
                            search_mode: event.search_mode.clone(),
                            current_depth: event.current_depth,
                        });
                    }
                });
            }

            solver.solve();

            running.store(false, Ordering::SeqCst);
        });
    }

    pub fn cancel(&self) {
        self.interrupt.store(true, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

pub struct ParallelSolverHandle {
    config: ParallelSolverConfig,
    state: MegaminxState,
    callback: RwLock<Option<Arc<dyn SolverCallback>>>,
    running: Arc<AtomicBool>,
    interrupt: Arc<AtomicBool>,
}

impl ParallelSolverHandle {
    pub fn new(config: ParallelSolverConfig, state: MegaminxState) -> Self {
        Self {
            config,
            state,
            callback: RwLock::new(None),
            running: Arc::new(AtomicBool::new(false)),
            interrupt: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_callback(&self, callback: Box<dyn SolverCallback>) {
        let mut cb = self.callback.write().unwrap();
        *cb = Some(Arc::from(callback));
    }

    pub fn start(&self) {
        if self.running.swap(true, Ordering::SeqCst) {
            return;
        }

        self.interrupt.store(false, Ordering::SeqCst);

        let config = self.config.clone();
        let state = self.state.clone();
        let callback = self.callback.read().unwrap().clone();
        let interrupt = Arc::clone(&self.interrupt);
        let running = Arc::clone(&self.running);

        std::thread::spawn(move || {
            let modes: Vec<llminxsolver_rs::SearchMode> =
                config.search_modes.iter().map(|&m| m.into()).collect();
            let metric: llminxsolver_rs::Metric = config.metric.into();
            let max_search_depth = if config.limit_search_depth {
                config.max_search_depth as usize
            } else {
                50
            };

            let start_state = build_llminx(&state);
            let memory_config: MemoryConfig = config.parallel_config.into();

            let mut parallel_solver = ParallelSolver::with_config(modes, memory_config);
            parallel_solver.set_metric(metric);
            parallel_solver.set_max_search_depth(max_search_depth);
            parallel_solver.set_limit_search_depth(config.limit_search_depth);
            parallel_solver.set_pruning_depth(config.pruning_depth);
            for mode_depth in config.mode_pruning_depths {
                parallel_solver.set_mode_pruning_depth(mode_depth.mode.into(), mode_depth.depth);
            }
            parallel_solver.set_ignore_corner_positions(config.ignore_corner_positions);
            parallel_solver.set_ignore_edge_positions(config.ignore_edge_positions);
            parallel_solver.set_ignore_corner_orientations(config.ignore_corner_orientations);
            parallel_solver.set_ignore_edge_orientations(config.ignore_edge_orientations);

            let solver_interrupt = parallel_solver.interrupt_handle();
            let interrupt_clone = Arc::clone(&interrupt);
            let running_clone = Arc::clone(&running);
            std::thread::spawn(move || {
                while !interrupt_clone.load(Ordering::SeqCst)
                    && running_clone.load(Ordering::SeqCst)
                {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                solver_interrupt.store(true, Ordering::SeqCst);
            });

            if let Some(ref cb) = callback {
                let cb_clone = Arc::clone(cb);
                parallel_solver.set_status_callback(move |event: StatusEvent| {
                    match event.event_type {
                        StatusEventType::SolutionFound => {
                            cb_clone.on_solution_found(event.message.clone());
                        }
                        StatusEventType::FinishSearch => {
                            cb_clone.on_complete();
                        }
                        _ => {
                            cb_clone.on_progress(ProgressEvent {
                                event_type: format!("{:?}", event.event_type),
                                message: event.message.clone(),
                                progress: event.progress,
                                search_mode: event.search_mode.clone(),
                                current_depth: event.current_depth,
                            });
                        }
                    }
                });
            }

            let _ = parallel_solver.solve(start_state);

            running.store(false, Ordering::SeqCst);
        });
    }

    pub fn cancel(&self) {
        self.interrupt.store(true, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}
