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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting_type_into_rs() {
        let variants = [
            SortingType::SetPriority,
            SortingType::OrientationOf,
            SortingType::OrientationAt,
            SortingType::PermutationOf,
            SortingType::PermutationAt,
        ];

        for variant in variants {
            let rs: llminxsolver_rs::batch_solver::SortCriterion = variant.into();
            assert!(matches!(
                rs,
                llminxsolver_rs::batch_solver::SortCriterion::SetPriority { .. }
                    | llminxsolver_rs::batch_solver::SortCriterion::OrientationOf { .. }
                    | llminxsolver_rs::batch_solver::SortCriterion::OrientationAt { .. }
                    | llminxsolver_rs::batch_solver::SortCriterion::PermutationOf { .. }
                    | llminxsolver_rs::batch_solver::SortCriterion::PermutationAt { .. }
            ));
        }
    }

    #[test]
    fn test_sorting_criterion_to_rs_parses_pieces() {
        let criterion = SortingCriterion {
            sorting_type: SortingType::PermutationAt,
            pieces: "UBL UFR".to_string(),
        };

        let rs = criterion.to_rs();
        match rs {
            llminxsolver_rs::batch_solver::SortCriterion::PermutationAt { pieces } => {
                assert_eq!(pieces, vec!["UBL".to_string(), "UFR".to_string()]);
            }
            _ => panic!("Unexpected criterion type"),
        }
    }

    #[test]
    fn test_batch_result_conversions() {
        let mut rs_results = llminxsolver_rs::batch_solver::BatchResults::new(2);
        let mut rs_case = llminxsolver_rs::batch_solver::BatchCaseResult::new(1, "R U".to_string());
        rs_case.solutions = vec!["R U R'".to_string()];
        rs_case.best_solution = Some("R U R'".to_string());
        rs_case.solve_time = 0.2;
        rs_results.add_result(rs_case);

        let out: BatchSolveResults = rs_results.into();
        assert_eq!(out.total_cases, 2);
        assert_eq!(out.solved_cases, 1);
        assert_eq!(out.case_results.len(), 1);
        assert_eq!(out.case_results[0].case_number, 1);
        assert_eq!(out.case_results[0].best_solution.as_deref(), Some("R U R'"));
    }

    #[test]
    fn test_batch_error_mapping() {
        let parse: BatchSolverError =
            llminxsolver_rs::batch_solver::BatchError::ParseError("x".into()).into();
        let invalid_move: BatchSolverError =
            llminxsolver_rs::batch_solver::BatchError::InvalidMove("m".into()).into();
        let invalid_piece: BatchSolverError =
            llminxsolver_rs::batch_solver::BatchError::InvalidPiece("p".into()).into();
        let invalid_scramble: BatchSolverError =
            llminxsolver_rs::batch_solver::BatchError::InvalidScramble("s".into()).into();
        let invalid_adjust: BatchSolverError =
            llminxsolver_rs::batch_solver::BatchError::InvalidAdjust("a".into()).into();
        let invalid_equivalence: BatchSolverError =
            llminxsolver_rs::batch_solver::BatchError::InvalidEquivalence("e".into()).into();

        assert!(matches!(parse, BatchSolverError::ParseError { .. }));
        assert!(matches!(invalid_move, BatchSolverError::InvalidMove { .. }));
        assert!(matches!(
            invalid_piece,
            BatchSolverError::InvalidPiece { .. }
        ));
        assert!(matches!(
            invalid_scramble,
            BatchSolverError::InvalidScramble { .. }
        ));
        assert!(matches!(
            invalid_adjust,
            BatchSolverError::InvalidAdjust { .. }
        ));
        assert!(matches!(
            invalid_equivalence,
            BatchSolverError::InvalidEquivalence { .. }
        ));
    }

    #[test]
    fn test_batch_config_and_generated_state_structs() {
        let cfg = BatchSolverConfig {
            scramble: "R U".to_string(),
            equivalences: "".to_string(),
            pre_adjust: "U".to_string(),
            post_adjust: "U'".to_string(),
            sorting_criteria: vec![SortingCriterion {
                sorting_type: SortingType::SetPriority,
                pieces: "UBL".to_string(),
            }],
            search_mode: SearchMode::RU,
            metric: Metric::Face,
            pruning_depth: 6,
            search_depth: 8,
            stop_after_first: true,
            parallel_config: ParallelConfig {
                memory_budget_mb: 64,
                table_gen_threads: 1,
                search_threads: 2,
            },
            ignore_corner_permutation: true,
            ignore_edge_permutation: false,
            ignore_corner_orientation: true,
            ignore_edge_orientation: false,
        };
        assert!(cfg.stop_after_first);
        assert_eq!(cfg.search_depth, 8);

        let generated = GeneratedBatchState {
            case_number: 4,
            setup_moves: "R U".to_string(),
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 1, 2, 0, 1],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 1, 0, 1, 0],
        };
        assert_eq!(generated.case_number, 4);
        assert_eq!(generated.corner_positions.len(), 5);
    }

    #[test]
    fn test_sorting_criterion_to_rs_all_variants() {
        let cases = [
            (SortingType::SetPriority, "A B"),
            (SortingType::OrientationOf, "A B"),
            (SortingType::OrientationAt, "A B"),
            (SortingType::PermutationOf, "A B"),
            (SortingType::PermutationAt, "A B"),
        ];

        for (sorting_type, pieces) in cases {
            let criterion = SortingCriterion {
                sorting_type,
                pieces: pieces.to_string(),
            };
            let rs = criterion.to_rs();
            match rs {
                llminxsolver_rs::batch_solver::SortCriterion::SetPriority { pieces }
                | llminxsolver_rs::batch_solver::SortCriterion::OrientationOf { pieces }
                | llminxsolver_rs::batch_solver::SortCriterion::OrientationAt { pieces }
                | llminxsolver_rs::batch_solver::SortCriterion::PermutationOf { pieces }
                | llminxsolver_rs::batch_solver::SortCriterion::PermutationAt { pieces } => {
                    assert_eq!(pieces, vec!["A".to_string(), "B".to_string()]);
                }
            }
        }
    }
}
