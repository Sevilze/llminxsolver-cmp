use llminxsolver_rs::MemoryConfig;

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
