//! Pre-adjust and post-adjust handling for batch solving
//!
//! Pre-adjust moves are applied before solving (e.g., U face adjustment).
//! Post-adjust moves define symmetry for ending positions.

use super::equivalence::EquivalenceHandler;
use super::types::{BatchError, GeneratedState, NormalizedState};
use crate::minx::{LLMinx, Move};
use std::collections::HashSet;
use std::sync::Arc;

/// Handler for pre-adjust and post-adjust moves
pub struct AdjustHandler {
    pre_adjust: Vec<Vec<Move>>,
    post_adjust: Vec<Vec<Move>>,
}

impl AdjustHandler {
    /// Create a new adjust handler from move strings
    ///
    /// # Arguments
    /// * `pre_adjust` - List of pre-adjust move strings. Each string can be:
    ///   - A base move like "U" which expands to all powers [U, U', U2, U2']
    ///   - An explicit move like "U'" which stays as-is
    /// * `post_adjust` - List of post-adjust move strings (same format)
    ///
    /// # Errors
    /// Returns `BatchError::InvalidMove` for unrecognized move strings
    pub fn new(pre_adjust: &[String], post_adjust: &[String]) -> Result<Self, BatchError> {
        let pre_adjust_moves = Self::expand_adjust_list(pre_adjust)?;
        let post_adjust_moves = Self::expand_adjust_list(post_adjust)?;

        Ok(Self {
            pre_adjust: pre_adjust_moves,
            post_adjust: post_adjust_moves,
        })
    }

    /// Expand a list of adjust strings, generating all powers for base moves
    fn expand_adjust_list(adjusts: &[String]) -> Result<Vec<Vec<Move>>, BatchError> {
        let mut result = Vec::new();

        for adj in adjusts {
            let trimmed = adj.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check if this is a base move (no modifier) that should be expanded
            if let Some(powers) = Self::get_move_powers(trimmed) {
                for mv in powers {
                    result.push(vec![mv]);
                }
            } else {
                // Parse as explicit move sequence
                let moves = Self::parse_moves(trimmed)?;
                if !moves.is_empty() {
                    result.push(moves);
                }
            }
        }

        Ok(result)
    }

    /// Get all powers of a base move (e.g., "U" -> [U, U', U2, U2'])
    /// Returns None if the input is not a base move
    fn get_move_powers(input: &str) -> Option<Vec<Move>> {
        match input {
            "U" => Some(vec![Move::U, Move::Ui, Move::U2, Move::U2i]),
            "R" => Some(vec![Move::R, Move::Ri, Move::R2, Move::R2i]),
            "L" => Some(vec![Move::L, Move::Li, Move::L2, Move::L2i]),
            "F" => Some(vec![Move::F, Move::Fi, Move::F2, Move::F2i]),
            "D" => Some(vec![Move::D, Move::Di, Move::D2, Move::D2i]),
            "bL" => Some(vec![Move::bL, Move::bLi, Move::bL2, Move::bL2i]),
            "bR" => Some(vec![Move::bR, Move::bRi, Move::bR2, Move::bR2i]),
            _ => None,
        }
    }

    /// Generate all pre-adjust sequences
    /// Returns a list of move sequences that can be applied
    pub fn pre_adjust_sequences(&self) -> &[Vec<Move>] {
        &self.pre_adjust
    }

    /// Generate all post-adjust sequences
    pub fn post_adjust_sequences(&self) -> &[Vec<Move>] {
        &self.post_adjust
    }

    /// Apply pre-adjust to a state
    /// Returns the adjusted state and the moves applied
    pub fn apply_pre_adjust(&self, state: &LLMinx, adjust_seq: &[Move]) -> (LLMinx, String) {
        let mut new_state = state.clone();
        for &mv in adjust_seq {
            new_state.apply_move(mv);
        }

        let moves_str = adjust_seq
            .iter()
            .map(|m| m.to_string().trim())
            .collect::<Vec<_>>()
            .join(" ");

        (new_state, moves_str)
    }

    /// Apply post-adjust to a state
    pub fn apply_post_adjust(&self, state: &LLMinx, adjust_seq: &[Move]) -> (LLMinx, String) {
        // Apply inverse of post-adjust moves
        let mut new_state = state.clone();
        for &mv in adjust_seq.iter().rev() {
            new_state.apply_move(mv.inverse());
        }

        let moves_str = adjust_seq
            .iter()
            .map(|m| m.to_string().trim())
            .collect::<Vec<_>>()
            .join(" ");

        (new_state, moves_str)
    }

    /// Get reduced set of states by removing equivalents under adjust
    /// States that differ only by pre/post adjust are considered equivalent
    pub fn reduce_states(
        &self,
        states: &[GeneratedState],
        equivalence: Option<&Arc<EquivalenceHandler>>,
    ) -> Vec<GeneratedState> {
        let mut result = Vec::new();
        let mut duplicate_states: HashSet<NormalizedState> = HashSet::new();

        // Get all pre-adjust sequences (always including identity/empty sequence)
        let mut pre_sequences: Vec<Vec<Move>> = vec![vec![]];
        pre_sequences.extend(self.pre_adjust.clone());

        // Get all post-adjust sequences (always including identity/empty sequence)
        let mut post_sequences: Vec<Vec<Move>> = vec![vec![]];
        post_sequences.extend(self.post_adjust.clone());

        for state in states {
            let state_key = Self::normalize_state(&state.state, equivalence);

            if !duplicate_states.contains(&state_key) {
                // This state is not a duplicate, keep it
                result.push(state.clone());

                // Mark all adjust variants as duplicates
                for pre_seq in &pre_sequences {
                    for post_seq in &post_sequences {
                        let variant =
                            self.compute_variant_from_moves(&state.setup_moves, pre_seq, post_seq);
                        let variant_key = Self::normalize_state(&variant, equivalence);
                        duplicate_states.insert(variant_key);
                    }
                }
            }
        }

        result
    }

    /// Normalize a state using equivalence handler if configured
    fn normalize_state(
        state: &LLMinx,
        equivalence: Option<&Arc<EquivalenceHandler>>,
    ) -> NormalizedState {
        if let Some(equiv) = equivalence {
            equiv.normalize(state)
        } else {
            NormalizedState::from_minx(state)
        }
    }

    /// Compute a variant state by executing: post_moves + setup_moves + pre_moves from solved
    fn compute_variant_from_moves(
        &self,
        setup_moves: &str,
        pre_seq: &[Move],
        post_seq: &[Move],
    ) -> LLMinx {
        let mut result = LLMinx::new();

        // Apply post-adjust moves first
        for &mv in post_seq {
            result.apply_move(mv);
        }

        // Apply the original setup moves
        if let Ok(moves) = Self::parse_moves(setup_moves) {
            for mv in moves {
                result.apply_move(mv);
            }
        }

        // Apply pre-adjust moves last
        for &mv in pre_seq {
            result.apply_move(mv);
        }

        result
    }

    /// Check if a state is solved under post-adjust
    /// Returns the post-adjust moves that solve the state, if any
    pub fn find_post_adjust_solution(&self, state: &LLMinx, goal: &LLMinx) -> Option<Vec<Move>> {
        // Try each post-adjust sequence
        for post_seq in &self.post_adjust {
            let (adjusted, _) = self.apply_post_adjust(state, post_seq);
            if adjusted.state_equals(goal) {
                return Some(post_seq.clone());
            }
        }

        None
    }

    /// Check if a state is solved considering post-adjust
    pub fn is_solved_with_post_adjust(&self, state: &LLMinx, goal: &LLMinx) -> bool {
        self.find_post_adjust_solution(state, goal).is_some()
    }

    /// Parse a move string into Move enums
    fn parse_moves(input: &str) -> Result<Vec<Move>, BatchError> {
        let move_strs: Vec<&str> = input.split_whitespace().collect();
        let mut moves = Vec::with_capacity(move_strs.len());

        for s in move_strs {
            let mv = Self::parse_single_move(s.trim())?;
            moves.push(mv);
        }

        Ok(moves)
    }

    /// Parse a single move string
    fn parse_single_move(input: &str) -> Result<Move, BatchError> {
        let input = input.trim();

        match input {
            "R" => Ok(Move::R),
            "R'" | "Ri" => Ok(Move::Ri),
            "R2" => Ok(Move::R2),
            "R2'" | "R2i" => Ok(Move::R2i),

            "L" => Ok(Move::L),
            "L'" | "Li" => Ok(Move::Li),
            "L2" => Ok(Move::L2),
            "L2'" | "L2i" => Ok(Move::L2i),

            "U" => Ok(Move::U),
            "U'" | "Ui" => Ok(Move::Ui),
            "U2" => Ok(Move::U2),
            "U2'" | "U2i" => Ok(Move::U2i),

            "F" => Ok(Move::F),
            "F'" | "Fi" => Ok(Move::Fi),
            "F2" => Ok(Move::F2),
            "F2'" | "F2i" => Ok(Move::F2i),

            "bL" => Ok(Move::bL),
            "bL'" | "bLi" => Ok(Move::bLi),
            "bL2" => Ok(Move::bL2),
            "bL2'" | "bL2i" => Ok(Move::bL2i),

            "bR" => Ok(Move::bR),
            "bR'" | "bRi" => Ok(Move::bRi),
            "bR2" => Ok(Move::bR2),
            "bR2'" | "bR2i" => Ok(Move::bR2i),

            "D" => Ok(Move::D),
            "D'" | "Di" => Ok(Move::Di),
            "D2" => Ok(Move::D2),
            "D2'" | "D2i" => Ok(Move::D2i),

            _ => Err(BatchError::InvalidMove(format!(
                "Unrecognized move: '{}'",
                input
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_handler_base_move_expansion() {
        let pre = vec!["U".to_string()];
        let post = vec!["R".to_string()];

        let handler = AdjustHandler::new(&pre, &post).unwrap();
        assert_eq!(handler.pre_adjust_sequences().len(), 4); // U, U', U2, U2'
        assert_eq!(handler.post_adjust_sequences().len(), 4); // R, R', R2, R2'
    }

    #[test]
    fn test_adjust_handler_explicit_moves() {
        // Explicit moves like "U'" should not expand
        let pre = vec!["U'".to_string(), "U2".to_string()];
        let post = vec!["R'".to_string()];

        let handler = AdjustHandler::new(&pre, &post).unwrap();
        assert_eq!(handler.pre_adjust_sequences().len(), 2); // U', U2
        assert_eq!(handler.post_adjust_sequences().len(), 1); // R'
    }

    #[test]
    fn test_apply_pre_adjust() {
        let handler = AdjustHandler::new(&[], &[]).unwrap();
        let state = LLMinx::new();

        let adjusted = handler.apply_pre_adjust(&state, &[Move::R]);
        assert_ne!(adjusted.0.corner_positions(), state.corner_positions());
    }

    #[test]
    fn test_reduce_states() {
        // Create properly constructed states where state matches setup_moves
        let handler = AdjustHandler::new(&[], &[]).unwrap();

        let mut state1_minx = LLMinx::new();
        state1_minx.apply_move(Move::R);
        let state1 = GeneratedState::new(state1_minx.clone(), "R".to_string());

        let mut state2_minx = LLMinx::new();
        state2_minx.apply_move(Move::Ri);
        let state2 = GeneratedState::new(state2_minx.clone(), "R'".to_string());

        let states = vec![state1, state2];
        let reduced = handler.reduce_states(&states, None);

        // R and R' produce different cube states, so both should be kept with no adjust
        assert_eq!(reduced.len(), 2);
    }

    #[test]
    fn test_reduce_states_with_adjust() {
        // Test that adjust moves actually reduce states
        // With U adjust: U, U', U2, U2' all become equivalent starting points
        let pre = vec!["U".to_string()];
        let handler = AdjustHandler::new(&pre, &[]).unwrap();

        // Create two states that differ by just a U move
        let mut state1_minx = LLMinx::new();
        state1_minx.apply_move(Move::R);
        let state1 = GeneratedState::new(state1_minx, "R".to_string());

        let mut state2_minx = LLMinx::new();
        state2_minx.apply_move(Move::R);
        state2_minx.apply_move(Move::U);
        let state2 = GeneratedState::new(state2_minx, "R U".to_string());

        let states = vec![state1, state2];
        let reduced = handler.reduce_states(&states, None);

        // R U should be equivalent to R under U pre-adjust
        assert_eq!(reduced.len(), 1);
    }

    #[test]
    fn test_is_solved_with_post_adjust() {
        // "U" expands to include U, so U-adjusted state should be solvable
        let post = vec!["U".to_string()];
        let handler = AdjustHandler::new(&[], &post).unwrap();

        let mut state = LLMinx::new();
        state.apply_move(Move::U);

        let goal = LLMinx::new();

        assert!(handler.is_solved_with_post_adjust(&state, &goal));
    }

    #[test]
    fn test_invalid_move() {
        let pre = vec!["X".to_string()];
        let result = AdjustHandler::new(&pre, &[]);
        assert!(result.is_err());
    }
}
