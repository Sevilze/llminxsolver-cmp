use crate::dedicated_solver::{Metric, ParallelConfig, SearchMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortingType {
    SetPriority,
    OrientationOf,
    OrientationAt,
    PermutationOf,
    PermutationAt,
}

impl From<SortingType> for llminxsolver_rs::batch_solver::SortCriterion {
    fn from(st: SortingType) -> Self {
        match st {
            SortingType::SetPriority => {
                llminxsolver_rs::batch_solver::SortCriterion::SetPriority { pieces: vec![] }
            }
            SortingType::OrientationOf => {
                llminxsolver_rs::batch_solver::SortCriterion::OrientationOf { pieces: vec![] }
            }
            SortingType::OrientationAt => {
                llminxsolver_rs::batch_solver::SortCriterion::OrientationAt { pieces: vec![] }
            }
            SortingType::PermutationOf => {
                llminxsolver_rs::batch_solver::SortCriterion::PermutationOf { pieces: vec![] }
            }
            SortingType::PermutationAt => {
                llminxsolver_rs::batch_solver::SortCriterion::PermutationAt { pieces: vec![] }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SortingCriterion {
    pub sorting_type: SortingType,
    pub pieces: String,
}

impl SortingCriterion {
    pub(crate) fn to_rs(&self) -> llminxsolver_rs::batch_solver::SortCriterion {
        let pieces: Vec<String> = self.pieces.split_whitespace().map(String::from).collect();
        match self.sorting_type {
            SortingType::SetPriority => {
                llminxsolver_rs::batch_solver::SortCriterion::SetPriority { pieces }
            }
            SortingType::OrientationOf => {
                llminxsolver_rs::batch_solver::SortCriterion::OrientationOf { pieces }
            }
            SortingType::OrientationAt => {
                llminxsolver_rs::batch_solver::SortCriterion::OrientationAt { pieces }
            }
            SortingType::PermutationOf => {
                llminxsolver_rs::batch_solver::SortCriterion::PermutationOf { pieces }
            }
            SortingType::PermutationAt => {
                llminxsolver_rs::batch_solver::SortCriterion::PermutationAt { pieces }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatchSolverConfig {
    pub scramble: String,
    pub equivalences: String,
    pub pre_adjust: String,
    pub post_adjust: String,
    pub sorting_criteria: Vec<SortingCriterion>,
    pub search_mode: SearchMode,
    pub metric: Metric,
    pub pruning_depth: u8,
    pub search_depth: u32,
    pub stop_after_first: bool,
    pub parallel_config: ParallelConfig,
    pub ignore_corner_permutation: bool,
    pub ignore_edge_permutation: bool,
    pub ignore_corner_orientation: bool,
    pub ignore_edge_orientation: bool,
}

#[derive(Debug, Clone)]
pub struct GeneratedBatchState {
    pub case_number: u32,
    pub setup_moves: String,
    pub corner_positions: Vec<u8>,
    pub corner_orientations: Vec<u8>,
    pub edge_positions: Vec<u8>,
    pub edge_orientations: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BatchCaseResult {
    pub case_number: u32,
    pub setup_moves: String,
    pub solutions: Vec<String>,
    pub best_solution: Option<String>,
    pub solve_time: f64,
}

impl From<llminxsolver_rs::batch_solver::BatchCaseResult> for BatchCaseResult {
    fn from(r: llminxsolver_rs::batch_solver::BatchCaseResult) -> Self {
        Self {
            case_number: r.case_number as u32,
            setup_moves: r.setup_moves,
            solutions: r.solutions,
            best_solution: r.best_solution,
            solve_time: r.solve_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatchSolveResults {
    pub total_cases: u32,
    pub solved_cases: u32,
    pub failed_cases: Vec<u32>,
    pub case_results: Vec<BatchCaseResult>,
    pub total_time: f64,
    pub average_time_per_case: f64,
}

impl From<llminxsolver_rs::batch_solver::BatchResults> for BatchSolveResults {
    fn from(r: llminxsolver_rs::batch_solver::BatchResults) -> Self {
        Self {
            total_cases: r.total_cases as u32,
            solved_cases: r.solved_cases as u32,
            failed_cases: r.failed_cases.into_iter().map(|x| x as u32).collect(),
            case_results: r.case_results.into_iter().map(Into::into).collect(),
            total_time: r.total_time,
            average_time_per_case: r.average_time_per_case,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum BatchSolverError {
    #[error("Parse error: {error_message}")]
    ParseError { error_message: String },
    #[error("Invalid move: {error_message}")]
    InvalidMove { error_message: String },
    #[error("Invalid piece: {error_message}")]
    InvalidPiece { error_message: String },
    #[error("Invalid scramble: {error_message}")]
    InvalidScramble { error_message: String },
    #[error("Invalid adjust: {error_message}")]
    InvalidAdjust { error_message: String },
    #[error("Invalid equivalence: {error_message}")]
    InvalidEquivalence { error_message: String },
}

impl From<llminxsolver_rs::batch_solver::BatchError> for BatchSolverError {
    fn from(e: llminxsolver_rs::batch_solver::BatchError) -> Self {
        match e {
            llminxsolver_rs::batch_solver::BatchError::ParseError(msg) => {
                BatchSolverError::ParseError { error_message: msg }
            }
            llminxsolver_rs::batch_solver::BatchError::InvalidMove(msg) => {
                BatchSolverError::InvalidMove { error_message: msg }
            }
            llminxsolver_rs::batch_solver::BatchError::InvalidPiece(msg) => {
                BatchSolverError::InvalidPiece { error_message: msg }
            }
            llminxsolver_rs::batch_solver::BatchError::InvalidScramble(msg) => {
                BatchSolverError::InvalidScramble { error_message: msg }
            }
            llminxsolver_rs::batch_solver::BatchError::InvalidAdjust(msg) => {
                BatchSolverError::InvalidAdjust { error_message: msg }
            }
            llminxsolver_rs::batch_solver::BatchError::InvalidEquivalence(msg) => {
                BatchSolverError::InvalidEquivalence { error_message: msg }
            }
        }
    }
}
