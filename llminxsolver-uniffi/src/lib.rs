use llminxsolver_rs::{LLMinx, MemoryConfig, ParallelSolver, Solver, StatusEvent, StatusEventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

uniffi::include_scaffolding!("llminxsolver");

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub primary: String,
    pub on_primary: String,
    pub primary_container: String,
    pub on_primary_container: String,
    pub secondary: String,
    pub on_secondary: String,
    pub secondary_container: String,
    pub on_secondary_container: String,
    pub tertiary: String,
    pub on_tertiary: String,
    pub tertiary_container: String,
    pub on_tertiary_container: String,
    pub error: String,
    pub on_error: String,
    pub error_container: String,
    pub on_error_container: String,
    pub background: String,
    pub on_background: String,
    pub surface: String,
    pub on_surface: String,
    pub surface_variant: String,
    pub on_surface_variant: String,
    pub outline: String,
    pub outline_variant: String,
    pub inverse_surface: String,
    pub inverse_on_surface: String,
    pub inverse_primary: String,
    pub surface_tint: String,
    pub surface_dim: String,
    pub surface_bright: String,
    pub surface_container_lowest: String,
    pub surface_container_low: String,
    pub surface_container: String,
    pub surface_container_high: String,
    pub surface_container_highest: String,
}

impl From<llminxsolver_rs::ThemeColors> for ThemeColors {
    fn from(colors: llminxsolver_rs::ThemeColors) -> Self {
        Self {
            primary: colors.primary,
            on_primary: colors.on_primary,
            primary_container: colors.primary_container,
            on_primary_container: colors.on_primary_container,
            secondary: colors.secondary,
            on_secondary: colors.on_secondary,
            secondary_container: colors.secondary_container,
            on_secondary_container: colors.on_secondary_container,
            tertiary: colors.tertiary,
            on_tertiary: colors.on_tertiary,
            tertiary_container: colors.tertiary_container,
            on_tertiary_container: colors.on_tertiary_container,
            error: colors.error,
            on_error: colors.on_error,
            error_container: colors.error_container,
            on_error_container: colors.on_error_container,
            background: colors.background,
            on_background: colors.on_background,
            surface: colors.surface,
            on_surface: colors.on_surface,
            surface_variant: colors.surface_variant,
            on_surface_variant: colors.on_surface_variant,
            outline: colors.outline,
            outline_variant: colors.outline_variant,
            inverse_surface: colors.inverse_surface,
            inverse_on_surface: colors.inverse_on_surface,
            inverse_primary: colors.inverse_primary,
            surface_tint: colors.surface_tint,
            surface_dim: colors.surface_dim,
            surface_bright: colors.surface_bright,
            surface_container_lowest: colors.surface_container_lowest,
            surface_container_low: colors.surface_container_low,
            surface_container: colors.surface_container,
            surface_container_high: colors.surface_container_high,
            surface_container_highest: colors.surface_container_highest,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SchemeType {
    #[default]
    TonalSpot,
    Content,
    Expressive,
    Fidelity,
    FruitSalad,
    Monochrome,
    Neutral,
    Rainbow,
    Vibrant,
}

impl From<SchemeType> for llminxsolver_rs::SchemeType {
    fn from(scheme_type: SchemeType) -> Self {
        match scheme_type {
            SchemeType::TonalSpot => llminxsolver_rs::SchemeType::TonalSpot,
            SchemeType::Content => llminxsolver_rs::SchemeType::Content,
            SchemeType::Expressive => llminxsolver_rs::SchemeType::Expressive,
            SchemeType::Fidelity => llminxsolver_rs::SchemeType::Fidelity,
            SchemeType::FruitSalad => llminxsolver_rs::SchemeType::FruitSalad,
            SchemeType::Monochrome => llminxsolver_rs::SchemeType::Monochrome,
            SchemeType::Neutral => llminxsolver_rs::SchemeType::Neutral,
            SchemeType::Rainbow => llminxsolver_rs::SchemeType::Rainbow,
            SchemeType::Vibrant => llminxsolver_rs::SchemeType::Vibrant,
        }
    }
}

pub fn generate_theme_from_image(
    image_path: String,
    dark_theme: bool,
    scheme_type: SchemeType,
) -> Option<ThemeColors> {
    llminxsolver_rs::generate_theme_from_image(&image_path, dark_theme, scheme_type.into())
        .map(Into::into)
}

pub fn generate_theme_from_wallpaper(
    dark_theme: bool,
    scheme_type: SchemeType,
) -> Option<ThemeColors> {
    llminxsolver_rs::generate_theme_from_wallpaper(dark_theme, scheme_type.into()).map(Into::into)
}

pub fn detect_wallpaper_path() -> Option<String> {
    llminxsolver_rs::detect_wallpaper_path()
}

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
    pub limit_search_depth: bool,
    pub max_search_depth: u32,
    pub pruning_depth: u8,
    pub ignore_corner_positions: bool,
    pub ignore_edge_positions: bool,
    pub ignore_corner_orientations: bool,
    pub ignore_edge_orientations: bool,
    pub parallel_config: Option<ParallelConfig>,
}

#[derive(Debug, Clone)]
pub struct ModePruningDepth {
    pub mode: SearchMode,
    pub depth: u8,
}

#[derive(Debug, Clone)]
pub struct ParallelSolverConfig {
    pub search_modes: Vec<SearchMode>,
    pub metric: Metric,
    pub limit_search_depth: bool,
    pub max_search_depth: u32,
    pub pruning_depth: u8,
    pub mode_pruning_depths: Vec<ModePruningDepth>,
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
    pub search_mode: Option<String>,
    pub current_depth: u32,
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

pub fn get_default_pruning_depth() -> u8 {
    llminxsolver_rs::DEFAULT_PRUNING_DEPTH
}

pub fn get_min_pruning_depth() -> u8 {
    llminxsolver_rs::MIN_PRUNING_DEPTH
}

pub fn get_max_pruning_depth() -> u8 {
    llminxsolver_rs::MAX_PRUNING_DEPTH
}

pub fn validate_megaminx_state(state: MegaminxState) -> Option<String> {
    let rs_state = llminxsolver_rs::MegaminxState {
        corner_positions: state.corner_positions,
        corner_orientations: state.corner_orientations,
        edge_positions: state.edge_positions,
        edge_orientations: state.edge_orientations,
    };

    match llminxsolver_rs::validate_last_layer_state(&rs_state) {
        Ok(()) => None,
        Err(e) => Some(e.to_string()),
    }
}

#[derive(Debug, Clone)]
pub struct ScoredSolutionExport {
    pub mcc: f64,
    pub move_count: u32,
    pub algorithm: String,
}

impl From<ScoredSolutionExport> for llminxsolver_rs::ScoredSolutionExport {
    fn from(export: ScoredSolutionExport) -> Self {
        Self {
            mcc: export.mcc,
            move_count: export.move_count,
            algorithm: export.algorithm,
        }
    }
}

pub fn export_scored_xlsx(
    output_path: String,
    solutions: Vec<ScoredSolutionExport>,
    image_png_bytes: Option<Vec<u8>>,
    image_size: u32,
) -> Option<String> {
    let rs_solutions: Vec<llminxsolver_rs::ScoredSolutionExport> =
        solutions.into_iter().map(Into::into).collect();

    llminxsolver_rs::export_scored_xlsx(
        &output_path,
        &rs_solutions,
        image_png_bytes.as_deref(),
        image_size,
    )
    .err()
}

pub fn export_raw_xlsx(
    output_path: String,
    algorithms: Vec<String>,
    image_png_bytes: Option<Vec<u8>>,
    image_size: u32,
) -> Option<String> {
    llminxsolver_rs::export_raw_xlsx(
        &output_path,
        &algorithms,
        image_png_bytes.as_deref(),
        image_size,
    )
    .err()
}

pub fn export_raw_xlsx_from_file(
    output_path: String,
    solutions_file_path: String,
    image_png_bytes: Option<Vec<u8>>,
    image_size: u32,
) -> Option<String> {
    llminxsolver_rs::export_raw_xlsx_from_file(
        &output_path,
        &solutions_file_path,
        image_png_bytes.as_deref(),
        image_size,
    )
    .err()
}

pub struct TempFile {
    inner: std::sync::Mutex<llminxsolver_rs::TempFile>,
}

impl Default for TempFile {
    fn default() -> Self {
        Self::new()
    }
}

impl TempFile {
    pub fn new() -> Self {
        let file =
            llminxsolver_rs::TempFile::new().expect("Failed to create raw solutions temp file");
        Self {
            inner: std::sync::Mutex::new(file),
        }
    }

    pub fn append(&self, solution: String) -> Option<String> {
        let mut file = match self.inner.lock() {
            Ok(f) => f,
            Err(e) => return Some(e.to_string()),
        };
        file.append(&solution).err()
    }

    pub fn get_path(&self) -> String {
        let file = self.inner.lock().unwrap();
        file.get_path().to_string_lossy().to_string()
    }

    pub fn count(&self) -> u64 {
        let file = self.inner.lock().unwrap();
        file.count() as u64
    }

    pub fn flush_file(&self) {
        let file = self.inner.lock().unwrap();
        file.flush_file();
    }

    pub fn delete_file(&self) {
        let mut file = self.inner.lock().unwrap();
        file.delete_file();
    }

    pub fn read_page(&self, offset: u64, limit: u64) -> Vec<String> {
        self.flush_file();
        let file = self.inner.lock().unwrap();
        file.read_page(offset as usize, limit as usize)
            .unwrap_or_default()
    }
}

pub fn cleanup_stale_temp_files() {
    llminxsolver_rs::cleanup_stale_temp_files();
}
