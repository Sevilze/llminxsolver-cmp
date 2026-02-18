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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_mode_conversion_all_variants() {
        let all = [
            SearchMode::RU,
            SearchMode::RUF,
            SearchMode::RUL,
            SearchMode::RUFL,
            SearchMode::RUFLbL,
            SearchMode::RUbL,
            SearchMode::RUbR,
            SearchMode::RUD,
        ];

        for mode in all {
            let rs_mode: llminxsolver_rs::SearchMode = mode.into();
            assert!(matches!(
                rs_mode,
                llminxsolver_rs::SearchMode::RU
                    | llminxsolver_rs::SearchMode::RUF
                    | llminxsolver_rs::SearchMode::RUL
                    | llminxsolver_rs::SearchMode::RUFL
                    | llminxsolver_rs::SearchMode::RUFLbL
                    | llminxsolver_rs::SearchMode::RUbL
                    | llminxsolver_rs::SearchMode::RUbR
                    | llminxsolver_rs::SearchMode::RUD
            ));
        }
    }

    #[test]
    fn test_metric_conversion() {
        let face: llminxsolver_rs::Metric = Metric::Face.into();
        let fifth: llminxsolver_rs::Metric = Metric::Fifth.into();
        assert_eq!(face, llminxsolver_rs::Metric::Face);
        assert_eq!(fifth, llminxsolver_rs::Metric::Fifth);
    }

    #[test]
    fn test_parallel_config_default_and_into_memory_config() {
        let default = ParallelConfig::default();
        assert!(default.memory_budget_mb > 0);
        assert!(default.table_gen_threads > 0);
        assert!(default.search_threads > 0);

        let mc: MemoryConfig = default.clone().into();
        assert_eq!(mc.budget_mb() as u32, default.memory_budget_mb);
        assert_eq!(
            mc.table_generation_threads as u32,
            default.table_gen_threads
        );
        assert_eq!(mc.search_threads as u32, default.search_threads);
    }

    #[test]
    fn test_config_and_payload_structs_construct() {
        let solver_cfg = SolverConfig {
            search_mode: SearchMode::RUF,
            metric: Metric::Face,
            limit_search_depth: true,
            max_search_depth: 7,
            pruning_depth: 6,
            ignore_corner_positions: true,
            ignore_edge_positions: false,
            ignore_corner_orientations: true,
            ignore_edge_orientations: false,
            parallel_config: None,
        };
        assert!(solver_cfg.limit_search_depth);
        assert!(solver_cfg.parallel_config.is_none());

        let mode_depth = ModePruningDepth {
            mode: SearchMode::RUD,
            depth: 5,
        };
        assert_eq!(mode_depth.depth, 5);

        let parallel_cfg = ParallelSolverConfig {
            search_modes: vec![SearchMode::RU, SearchMode::RUF],
            metric: Metric::Fifth,
            limit_search_depth: false,
            max_search_depth: 12,
            pruning_depth: 6,
            mode_pruning_depths: vec![mode_depth.clone()],
            ignore_corner_positions: false,
            ignore_edge_positions: false,
            ignore_corner_orientations: false,
            ignore_edge_orientations: false,
            parallel_config: ParallelConfig {
                memory_budget_mb: 64,
                table_gen_threads: 1,
                search_threads: 1,
            },
        };
        assert_eq!(parallel_cfg.mode_pruning_depths.len(), 1);

        let state = MegaminxState {
            corner_positions: vec![0, 1, 2],
            corner_orientations: vec![0, 1, 2],
            edge_positions: vec![0, 1, 2],
            edge_orientations: vec![0, 1, 0],
        };
        assert_eq!(state.corner_positions[1], 1);

        let progress = ProgressEvent {
            event_type: "Message".to_string(),
            message: "hello".to_string(),
            progress: 0.5,
            search_mode: Some("RU".to_string()),
            current_depth: 3,
        };
        assert_eq!(progress.search_mode.as_deref(), Some("RU"));
    }
}
