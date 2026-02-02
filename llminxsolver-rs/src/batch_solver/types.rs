//! Core types for batch solving functionality

use crate::minx::LLMinx;
use std::collections::HashMap;
use std::fmt;

/// Error type for batch solver operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchError {
    ParseError(String),
    InvalidMove(String),
    InvalidPiece(String),
    InvalidScramble(String),
    InvalidAdjust(String),
    InvalidEquivalence(String),
}

impl fmt::Display for BatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatchError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            BatchError::InvalidMove(msg) => write!(f, "Invalid move: {}", msg),
            BatchError::InvalidPiece(msg) => write!(f, "Invalid piece: {}", msg),
            BatchError::InvalidScramble(msg) => write!(f, "Invalid scramble: {}", msg),
            BatchError::InvalidAdjust(msg) => write!(f, "Invalid adjust: {}", msg),
            BatchError::InvalidEquivalence(msg) => write!(f, "Invalid equivalence: {}", msg),
        }
    }
}

impl std::error::Error for BatchError {}

/// Represents a parsed scramble segment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrambleSegment {
    /// Plain moves to apply directly
    Plain(String),
    /// Series of alternative algorithms [alg1, alg2, ...]
    Series(Vec<String>),
    /// BFS generators <gen1, gen2, ...>
    Generators(Vec<String>),
}

impl ScrambleSegment {
    /// Returns true if this segment contains no operations
    pub fn is_empty(&self) -> bool {
        match self {
            ScrambleSegment::Plain(s) => s.trim().is_empty(),
            ScrambleSegment::Series(v) => v.is_empty() || v.iter().all(|s| s.trim().is_empty()),
            ScrambleSegment::Generators(v) => v.is_empty() || v.iter().all(|s| s.trim().is_empty()),
        }
    }
}

/// Case selection modifiers for selective solving
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CaseModifiers {
    /// Specific case numbers to solve (1-indexed)
    pub specific_cases: Vec<usize>,
    /// Ranges of cases to solve (inclusive, 1-indexed)
    pub ranges: Vec<(usize, usize)>,
    /// Start from this case number to the end
    pub start_from: Option<usize>,
}

impl CaseModifiers {
    /// Check if a given case number should be solved
    pub fn should_solve(&self, case_number: usize) -> bool {
        // If no modifiers specified, solve all cases
        if self.is_empty() {
            return true;
        }

        // Check specific cases
        if self.specific_cases.contains(&case_number) {
            return true;
        }

        // Check ranges
        for (start, end) in &self.ranges {
            if case_number >= *start && case_number <= *end {
                return true;
            }
        }

        // Check start_from
        if let Some(start) = self.start_from
            && case_number >= start
        {
            return true;
        }

        false
    }

    /// Returns true if no modifiers are specified
    pub fn is_empty(&self) -> bool {
        self.specific_cases.is_empty() && self.ranges.is_empty() && self.start_from.is_none()
    }
}

/// Parsed scramble with segments and optional modifiers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedScramble {
    pub segments: Vec<ScrambleSegment>,
    pub modifiers: CaseModifiers,
}

impl ParsedScramble {
    /// Returns true if the scramble has no segments
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() || self.segments.iter().all(|s| s.is_empty())
    }
}

/// Equivalence set definition (pieces that are treated as equivalent)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquivalenceSet {
    /// Names of pieces in this equivalence set
    pub pieces: Vec<String>,
}

impl EquivalenceSet {
    /// Check if a piece is in this equivalence set
    pub fn contains(&self, piece: &str) -> bool {
        self.pieces.iter().any(|p| p == piece)
    }
}

/// Orientation group definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrientationGroup {
    /// Number of unique orientations for pieces in this group
    pub num_orientations: u8,
    /// Names of pieces in this group
    pub pieces: Vec<String>,
}

/// Case sorting criteria
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortCriterion {
    /// Set custom priority for pieces
    SetPriority { pieces: Vec<String> },
    /// Sort by orientation of pieces at specified locations
    OrientationAt { pieces: Vec<String> },
    /// Sort by orientation of pieces themselves (where they belong)
    OrientationOf { pieces: Vec<String> },
    /// Sort by permutation of pieces at specified locations
    PermutationAt { pieces: Vec<String> },
    /// Sort by permutation of pieces themselves (where they belong)
    PermutationOf { pieces: Vec<String> },
}

impl SortCriterion {
    /// Get the piece names associated with this criterion
    pub fn pieces(&self) -> &[String] {
        match self {
            SortCriterion::SetPriority { pieces }
            | SortCriterion::OrientationAt { pieces }
            | SortCriterion::OrientationOf { pieces }
            | SortCriterion::PermutationAt { pieces }
            | SortCriterion::PermutationOf { pieces } => pieces,
        }
    }
}

/// Batch solving configuration
#[derive(Debug, Clone, Default)]
pub struct BatchConfig {
    /// Scramble syntax string
    pub scramble: String,
    /// Pre-adjust move strings (e.g., ["U", "U'", "U2"])
    pub pre_adjust: Vec<String>,
    /// Post-adjust move strings for ending symmetry
    pub post_adjust: Vec<String>,
    /// Equivalence sets for pieces
    pub equivalences: Vec<EquivalenceSet>,
    /// Orientation groups for pieces
    pub orientation_groups: Vec<OrientationGroup>,
    /// Case sorting criteria
    pub sort_criteria: Vec<SortCriterion>,
    /// Stop searching after finding the first solution for each case
    pub stop_after_first: bool,
}

impl BatchConfig {
    /// Create a new batch config with the given scramble
    pub fn new(scramble: impl Into<String>) -> Self {
        Self {
            scramble: scramble.into(),
            ..Default::default()
        }
    }

    /// Add pre-adjust moves
    pub fn with_pre_adjust(mut self, moves: Vec<String>) -> Self {
        self.pre_adjust = moves;
        self
    }

    /// Add post-adjust moves
    pub fn with_post_adjust(mut self, moves: Vec<String>) -> Self {
        self.post_adjust = moves;
        self
    }

    /// Add equivalence sets
    pub fn with_equivalences(mut self, equivalences: Vec<EquivalenceSet>) -> Self {
        self.equivalences = equivalences;
        self
    }

    /// Add orientation groups
    pub fn with_orientation_groups(mut self, groups: Vec<OrientationGroup>) -> Self {
        self.orientation_groups = groups;
        self
    }

    /// Add sort criteria
    pub fn with_sort_criteria(mut self, criteria: Vec<SortCriterion>) -> Self {
        self.sort_criteria = criteria;
        self
    }

    /// Set stop_after_first flag
    pub fn with_stop_after_first(mut self, stop: bool) -> Self {
        self.stop_after_first = stop;
        self
    }
}

/// A generated state with its setup moves
pub struct GeneratedState {
    /// The megaminx state
    pub state: LLMinx,
    /// The setup moves to reach this state
    pub setup_moves: String,
    /// Case number after sorting
    pub case_number: usize,
}

impl std::fmt::Debug for GeneratedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeneratedState")
            .field("setup_moves", &self.setup_moves)
            .field("case_number", &self.case_number)
            .finish()
    }
}

impl Clone for GeneratedState {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            setup_moves: self.setup_moves.clone(),
            case_number: self.case_number,
        }
    }
}

impl GeneratedState {
    /// Create a new generated state
    pub fn new(state: LLMinx, setup_moves: String) -> Self {
        Self {
            state,
            setup_moves,
            case_number: 0,
        }
    }
}

/// Result for a single case in batch solving
#[derive(Debug, Clone)]
pub struct BatchCaseResult {
    /// Case number (1-indexed)
    pub case_number: usize,
    /// Setup moves for this case
    pub setup_moves: String,
    /// All solutions found for this case
    pub solutions: Vec<String>,
    /// Best solution (first one, or shortest if sorted)
    pub best_solution: Option<String>,
    /// Time taken to solve this case (in seconds)
    pub solve_time: f64,
}

impl BatchCaseResult {
    /// Create a new case result
    pub fn new(case_number: usize, setup_moves: String) -> Self {
        Self {
            case_number,
            setup_moves,
            solutions: Vec::new(),
            best_solution: None,
            solve_time: 0.0,
        }
    }

    /// Returns true if at least one solution was found
    pub fn is_solved(&self) -> bool {
        !self.solutions.is_empty()
    }
}

/// Complete batch solving results
#[derive(Debug, Clone)]
pub struct BatchResults {
    /// Total number of cases
    pub total_cases: usize,
    /// Number of cases with at least one solution
    pub solved_cases: usize,
    /// Number of cases with no solutions
    pub failed_cases: Vec<usize>,
    /// Individual case results
    pub case_results: Vec<BatchCaseResult>,
    /// Total time for all cases (in seconds)
    pub total_time: f64,
    /// Average time per case (in seconds)
    pub average_time_per_case: f64,
}

impl BatchResults {
    /// Create new empty results
    pub fn new(total_cases: usize) -> Self {
        Self {
            total_cases,
            solved_cases: 0,
            failed_cases: Vec::new(),
            case_results: Vec::new(),
            total_time: 0.0,
            average_time_per_case: 0.0,
        }
    }

    /// Add a case result and update statistics
    pub fn add_result(&mut self, result: BatchCaseResult) {
        if result.is_solved() {
            self.solved_cases += 1;
        } else {
            self.failed_cases.push(result.case_number);
        }
        self.total_time += result.solve_time;
        self.case_results.push(result);
        self.average_time_per_case = self.total_time / self.total_cases as f64;
    }

    /// Get the success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_cases == 0 {
            0.0
        } else {
            (self.solved_cases as f64 / self.total_cases as f64) * 100.0
        }
    }
}

/// Piece map for looking up piece names to indices
#[derive(Debug, Clone, Default)]
pub struct PieceMap {
    corner_map: HashMap<String, usize>,
    edge_map: HashMap<String, usize>,
}

impl PieceMap {
    /// Create a new empty piece map
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a corner piece mapping
    pub fn add_corner(&mut self, name: String, index: usize) {
        self.corner_map.insert(name, index);
    }

    /// Add an edge piece mapping
    pub fn add_edge(&mut self, name: String, index: usize) {
        self.edge_map.insert(name, index);
    }

    /// Get corner index by name
    pub fn get_corner(&self, name: &str) -> Option<usize> {
        self.corner_map.get(name).copied()
    }

    /// Get edge index by name
    pub fn get_edge(&self, name: &str) -> Option<usize> {
        self.edge_map.get(name).copied()
    }

    /// Check if a piece name is valid (either corner or edge)
    pub fn contains(&self, name: &str) -> bool {
        self.corner_map.contains_key(name) || self.edge_map.contains_key(name)
    }

    /// Create default piece map for megaminx
    pub fn default_megaminx() -> Self {
        let mut map = Self::new();

        // Add corner pieces (UC1-UC5, RC1, RC5, FC5, FC1, FC2, LC1, LC2, etc.)
        let corner_names = [
            "UC1", "UC2", "UC3", "UC4", "UC5", "RC1", "RC5", "FC5", "FC1", "FC2", "LC1", "LC2",
            "BLC1", "BLC2", "BRC1", "DC1", "DC2",
        ];
        for (i, name) in corner_names.iter().enumerate() {
            map.add_corner(name.to_string(), i);
        }

        // Add edge pieces (UE1-UE5, RE2-RE4, FE2-FE5, LE3-LE5, etc.)
        let edge_names = [
            "UE1", "UE2", "UE3", "UE4", "UE5", "RE2", "RE3", "RE4", "FE2", "FE3", "FE4", "FE5",
            "LE3", "LE4", "LE5", "BLE3", "BLE4", "BLE5", "BRE3", "BRE4", "DE1", "DE2", "DE3",
        ];
        for (i, name) in edge_names.iter().enumerate() {
            map.add_edge(name.to_string(), i);
        }

        map
    }
}

/// Normalized state representation for equivalence comparison
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NormalizedState {
    /// Corner signature (piece indices at each position)
    pub corner_signature: Vec<u8>,
    /// Edge signature (piece indices at each position)
    pub edge_signature: Vec<u8>,
    /// Corner orientation signature
    pub corner_orientation: Vec<u8>,
    /// Edge orientation signature
    pub edge_orientation: Vec<u8>,
}

impl NormalizedState {
    /// Create from an LLMinx state
    pub fn from_minx(minx: &LLMinx) -> Self {
        let corner_signature = minx.corner_positions().to_vec();
        let edge_signature = minx.edge_positions().to_vec();

        let corner_orientation: Vec<u8> = (0..super::super::minx::NUM_CORNERS as u8)
            .map(|i| minx.get_corner_orientation(i))
            .collect();

        let edge_orientation: Vec<u8> = (0..super::super::minx::NUM_EDGES as u8)
            .map(|i| minx.get_edge_orientation(i))
            .collect();

        Self {
            corner_signature,
            edge_signature,
            corner_orientation,
            edge_orientation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_modifiers() {
        let modifiers = CaseModifiers {
            specific_cases: vec![1, 3, 5],
            ranges: vec![(10, 15)],
            start_from: Some(20),
        };

        assert!(modifiers.should_solve(1));
        assert!(modifiers.should_solve(3));
        assert!(modifiers.should_solve(12));
        assert!(modifiers.should_solve(25));
        assert!(!modifiers.should_solve(2));
        assert!(!modifiers.should_solve(8));
    }

    #[test]
    fn test_empty_modifiers_solve_all() {
        let modifiers = CaseModifiers::default();
        assert!(modifiers.should_solve(1));
        assert!(modifiers.should_solve(100));
    }

    #[test]
    fn test_batch_config_builder() {
        let config = BatchConfig::new("R U R'")
            .with_pre_adjust(vec!["U".to_string()])
            .with_post_adjust(vec!["U".to_string()]);

        assert_eq!(config.scramble, "R U R'");
        assert_eq!(config.pre_adjust.len(), 1);
        assert_eq!(config.post_adjust.len(), 1);
    }

    #[test]
    fn test_piece_map() {
        let map = PieceMap::default_megaminx();
        assert!(map.get_corner("UC1").is_some());
        assert!(map.get_edge("UE1").is_some());
        assert!(!map.contains("INVALID"));
    }
}
