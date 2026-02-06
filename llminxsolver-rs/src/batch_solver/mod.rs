//! Batched solving functionality for megaminx
//!
//! This module provides batch solving capabilities for megaminx.
//! It allows defining multiple equivalent states using scramble syntax and solving them in a batched manner.

pub mod adjust;
pub mod equivalence;
pub mod parser;
pub mod sorting;
pub mod types;

pub use adjust::AdjustHandler;
pub use equivalence::EquivalenceHandler;
pub use parser::ScrambleParser;
pub use sorting::CaseSorter;
pub use types::{
    BatchCaseResult, BatchConfig, BatchError, BatchResults, CaseModifiers, EquivalenceSet,
    GeneratedState, NormalizedState, OrientationGroup, ParsedScramble, PieceMap, ScrambleSegment,
    SortCriterion,
};
