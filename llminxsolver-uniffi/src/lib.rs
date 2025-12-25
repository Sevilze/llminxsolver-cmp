use llminxsolver_rs::{LLMinx, MemoryConfig, ParallelSolver, Solver, StatusEvent, StatusEventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

uniffi::include_scaffolding!("llminxsolver");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchMode {
    RU,
    RUF,
    RUL,
    RUFL,
    RUFLbL,
    RUbL,
    RUbR,
    RUD,
}

impl From<SearchMode> for llminxsolver_rs::SearchMode {
    fn from(mode: SearchMode) -> Self {
        match mode {
            SearchMode::RU => llminxsolver_rs::SearchMode::RU,
            SearchMode::RUF => llminxsolver_rs::SearchMode::RUF,
            SearchMode::RUL => llminxsolver_rs::SearchMode::RUL,
            SearchMode::RUFL => llminxsolver_rs::SearchMode::RUFL,
            SearchMode::RUFLbL => llminxsolver_rs::SearchMode::RUFLbL,
            SearchMode::RUbL => llminxsolver_rs::SearchMode::RUbL,
            SearchMode::RUbR => llminxsolver_rs::SearchMode::RUbR,
            SearchMode::RUD => llminxsolver_rs::SearchMode::RUD,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    Face,
    Fifth,
}

impl From<Metric> for llminxsolver_rs::Metric {
    fn from(metric: Metric) -> Self {
        match metric {
            Metric::Face => llminxsolver_rs::Metric::Face,
            Metric::Fifth => llminxsolver_rs::Metric::Fifth,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParallelConfig {
    pub memory_budget_mb: u32,
    pub table_gen_threads: u32,
    pub search_threads: u32,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        let config = MemoryConfig::default();
        Self {
            memory_budget_mb: config.budget_mb() as u32,
            table_gen_threads: config.table_generation_threads as u32,
            search_threads: config.search_threads as u32,
        }
    }
}

impl From<ParallelConfig> for MemoryConfig {
    fn from(config: ParallelConfig) -> Self {
        MemoryConfig::new(
            config.memory_budget_mb as usize,
            config.table_gen_threads as usize,
            config.search_threads as usize,
        )
    }
}

#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub search_mode: SearchMode,
    pub metric: Metric,
    pub limit_depth: bool,
    pub max_depth: u32,
    pub ignore_corner_positions: bool,
    pub ignore_edge_positions: bool,
    pub ignore_corner_orientations: bool,
    pub ignore_edge_orientations: bool,
    pub parallel_config: Option<ParallelConfig>,
}

#[derive(Debug, Clone)]
pub struct ParallelSolverConfig {
    pub search_modes: Vec<SearchMode>,
    pub metric: Metric,
    pub limit_depth: bool,
    pub max_depth: u32,
    pub ignore_corner_positions: bool,
    pub ignore_edge_positions: bool,
    pub ignore_corner_orientations: bool,
    pub ignore_edge_orientations: bool,
    pub parallel_config: ParallelConfig,
}

#[derive(Debug, Clone)]
pub struct MegaminxState {
    pub corner_positions: Vec<u8>,
    pub corner_orientations: Vec<u8>,
    pub edge_positions: Vec<u8>,
    pub edge_orientations: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ProgressEvent {
    pub event_type: String,
    pub message: String,
    pub progress: f64,
}

pub trait SolverCallback: Send + Sync {
    fn on_progress(&self, event: ProgressEvent);
    fn on_solution_found(&self, solution: String);
    fn on_complete(&self);
}

fn build_llminx(state: &MegaminxState) -> LLMinx {
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
            let max_depth = if config.limit_depth {
                config.max_depth as usize
            } else {
                50
            };

            let start_state = build_llminx(&state);

            let memory_config = config
                .parallel_config
                .map(|pc| pc.into())
                .unwrap_or_default();

            let mut solver = Solver::with_parallel_config(search_mode, max_depth, memory_config);
            solver.set_metric(metric);
            solver.set_limit_depth(config.limit_depth);
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
            let max_depth = if config.limit_depth {
                config.max_depth as usize
            } else {
                50
            };

            let start_state = build_llminx(&state);
            let memory_config: MemoryConfig = config.parallel_config.into();

            let mut parallel_solver = ParallelSolver::with_config(modes, memory_config);
            parallel_solver.set_metric(metric);
            parallel_solver.set_max_depth(max_depth);
            parallel_solver.set_limit_depth(config.limit_depth);
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
                parallel_solver.set_status_callback(move |event: StatusEvent| match event.event_type {
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
                        });
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

pub fn get_move_count(algorithm: String, metric: String) -> u32 {
    llminxsolver_rs::get_move_count(&algorithm, &metric)
}

pub fn calculate_mcc(sequence: String) -> f64 {
    llminxsolver_rs::calculate_mcc(&sequence)
}

pub fn set_data_directory(path: String) {
    llminxsolver_rs::set_data_directory(&path);
}

pub fn get_available_cpus() -> u32 {
    MemoryConfig::available_cpus() as u32
}

pub fn get_available_memory_mb() -> u32 {
    MemoryConfig::available_memory_mb() as u32
}
