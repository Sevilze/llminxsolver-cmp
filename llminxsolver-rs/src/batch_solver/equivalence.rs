//! Equivalence handling for batch solving
//!
//! Handles:
//! - Equivalence sets: Pieces that are treated as equivalent (permutation doesn't matter)
//! - Orientation groups: Custom number of unique orientations for pieces

use super::types::{BatchError, EquivalenceSet, NormalizedState, OrientationGroup, PieceMap};
use crate::minx::{LLMinx, NUM_CORNERS, NUM_EDGES};
use std::collections::{HashMap, HashSet};

/// Handler for equivalences and orientation groups
pub struct EquivalenceHandler {
    equivalences: Vec<EquivalenceSet>,
    orientation_groups: Vec<OrientationGroup>,
    piece_map: PieceMap,
    /// Maps piece indices to their equivalence class representative
    corner_equivalence_map: HashMap<usize, usize>,
    edge_equivalence_map: HashMap<usize, usize>,
    /// Number of unique orientations for each piece
    corner_orientation_counts: Vec<u8>,
    edge_orientation_counts: Vec<u8>,
}

impl EquivalenceHandler {
    /// Create a new equivalence handler
    pub fn new(
        equivalences: Vec<EquivalenceSet>,
        orientation_groups: Vec<OrientationGroup>,
        piece_map: PieceMap,
    ) -> Result<Self, BatchError> {
        let mut handler = Self {
            equivalences,
            orientation_groups,
            piece_map,
            corner_equivalence_map: HashMap::new(),
            edge_equivalence_map: HashMap::new(),
            corner_orientation_counts: vec![3; NUM_CORNERS],
            edge_orientation_counts: vec![2; NUM_EDGES],
        };

        handler.build_equivalence_maps()?;
        handler.apply_orientation_groups()?;

        Ok(handler)
    }

    /// Get the corner equivalence map (for debugging)
    pub fn corner_map(&self) -> &HashMap<usize, usize> {
        &self.corner_equivalence_map
    }

    /// Get the edge equivalence map (for debugging)
    pub fn edge_map(&self) -> &HashMap<usize, usize> {
        &self.edge_equivalence_map
    }

    /// Build equivalence maps from equivalence sets
    fn build_equivalence_maps(&mut self) -> Result<(), BatchError> {
        for equiv in &self.equivalences {
            let mut piece_indices = Vec::new();

            for piece_name in &equiv.pieces {
                if let Some(idx) = self.piece_map.get_corner(piece_name) {
                    piece_indices.push((true, idx));
                } else if let Some(idx) = self.piece_map.get_edge(piece_name) {
                    piece_indices.push((false, idx));
                } else {
                    return Err(BatchError::InvalidPiece(format!(
                        "Unknown piece in equivalence set: {}",
                        piece_name
                    )));
                }
            }

            // All pieces in an equivalence set must be of the same type
            if piece_indices.len() > 1 {
                let first_is_corner = piece_indices[0].0;
                for (is_corner, idx) in &piece_indices {
                    if *is_corner != first_is_corner {
                        return Err(BatchError::InvalidEquivalence(
                            "Equivalence set contains mixed piece types (corners and edges)"
                                .to_string(),
                        ));
                    }

                    // Map all pieces to the first piece in the set
                    let representative = piece_indices[0].1;
                    if *is_corner {
                        self.corner_equivalence_map.insert(*idx, representative);
                    } else {
                        self.edge_equivalence_map.insert(*idx, representative);
                    }
                }
            }
        }

        Ok(())
    }

    /// Apply orientation groups
    fn apply_orientation_groups(&mut self) -> Result<(), BatchError> {
        for group in &self.orientation_groups {
            for piece_name in &group.pieces {
                if let Some(idx) = self.piece_map.get_corner(piece_name) {
                    if 3 % group.num_orientations != 0 {
                        return Err(BatchError::InvalidEquivalence(format!(
                            "Cannot set {} orientations for corner {} (must divide 3)",
                            group.num_orientations, piece_name
                        )));
                    }
                    self.corner_orientation_counts[idx] = group.num_orientations;
                } else if let Some(idx) = self.piece_map.get_edge(piece_name) {
                    if 2 % group.num_orientations != 0 {
                        return Err(BatchError::InvalidEquivalence(format!(
                            "Cannot set {} orientations for edge {} (must divide 2)",
                            group.num_orientations, piece_name
                        )));
                    }
                    self.edge_orientation_counts[idx] = group.num_orientations;
                } else {
                    return Err(BatchError::InvalidPiece(format!(
                        "Unknown piece in orientation group: {}",
                        piece_name
                    )));
                }
            }
        }

        Ok(())
    }

    /// Check if two states are equivalent under the defined equivalences
    pub fn are_equivalent(&self, state1: &LLMinx, state2: &LLMinx) -> bool {
        self.normalize(state1) == self.normalize(state2)
    }

    /// Normalize a state for comparison
    /// Returns a canonical representation that accounts for equivalences
    pub fn normalize(&self, state: &LLMinx) -> NormalizedState {
        let mut corner_sig = state.corner_positions().to_vec();
        let mut edge_sig = state.edge_positions().to_vec();
        let mut corner_ori: Vec<u8> = (0..NUM_CORNERS as u8)
            .map(|i| state.get_corner_orientation(i))
            .collect();
        let mut edge_ori: Vec<u8> = (0..NUM_EDGES as u8)
            .map(|i| state.get_edge_orientation(i))
            .collect();

        // Apply equivalence mapping to corner positions
        for pos in &mut corner_sig {
            if let Some(&representative) = self.corner_equivalence_map.get(&(*pos as usize)) {
                *pos = representative as u8;
            }
        }

        // Apply equivalence mapping to edge positions
        for pos in &mut edge_sig {
            if let Some(&representative) = self.edge_equivalence_map.get(&(*pos as usize)) {
                *pos = representative as u8;
            }
        }

        // Apply orientation group mappings
        for (i, ori) in corner_ori.iter_mut().enumerate() {
            let count = self.corner_orientation_counts[i];
            if count < 3 {
                // Normalize orientation to reduced set
                *ori %= count;
            }
        }

        for (i, ori) in edge_ori.iter_mut().enumerate() {
            let count = self.edge_orientation_counts[i];
            if count < 2 {
                // Normalize orientation to reduced set
                *ori %= count;
            }
        }

        NormalizedState {
            corner_signature: corner_sig,
            edge_signature: edge_sig,
            corner_orientation: corner_ori,
            edge_orientation: edge_ori,
        }
    }

    /// Apply equivalence ignores to an LLMinx state
    pub fn apply_to_state(&self, state: &mut LLMinx) {
        // Set ignore flags for equivalent piece positions
        let mut ignore_corners = [false; NUM_CORNERS];
        let mut ignore_edges = [false; NUM_EDGES];

        // For each equivalence set, ignore all non-representative pieces
        for (piece_idx, representative) in &self.corner_equivalence_map {
            if piece_idx != representative {
                ignore_corners[*piece_idx] = true;
            }
        }

        for (piece_idx, representative) in &self.edge_equivalence_map {
            if piece_idx != representative {
                ignore_edges[*piece_idx] = true;
            }
        }

        state.set_ignore_corner_positions(ignore_corners);
        state.set_ignore_edge_positions(ignore_edges);

        // For orientation groups with 1 orientation, ignore orientations
        let mut ignore_corner_ori = [false; NUM_CORNERS];
        let mut ignore_edge_ori = [false; NUM_EDGES];

        for (i, ori_count) in self.corner_orientation_counts.iter().enumerate() {
            if *ori_count == 1 {
                ignore_corner_ori[i] = true;
            }
        }

        for (i, ori_count) in self.edge_orientation_counts.iter().enumerate() {
            if *ori_count == 1 {
                ignore_edge_ori[i] = true;
            }
        }

        state.set_ignore_corner_orientations(ignore_corner_ori);
        state.set_ignore_edge_orientations(ignore_edge_ori);
    }

    /// Get unique states by removing equivalent duplicates
    pub fn deduplicate(&self, states: &mut Vec<super::types::GeneratedState>) {
        let mut seen = HashSet::new();
        states.retain(|state| {
            let normalized = self.normalize(&state.state);
            seen.insert(normalized)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> EquivalenceHandler {
        let piece_map = PieceMap::default_megaminx();

        let equivalences = vec![EquivalenceSet {
            pieces: vec!["UC1".to_string(), "UC2".to_string()],
        }];

        let orientation_groups = vec![OrientationGroup {
            num_orientations: 1,
            pieces: vec!["UC3".to_string()],
        }];

        EquivalenceHandler::new(equivalences, orientation_groups, piece_map).unwrap()
    }

    #[test]
    fn test_equivalence_handler_creation() {
        let handler = create_test_handler();
        assert!(!handler.corner_equivalence_map.is_empty());
    }

    #[test]
    fn test_normalize_state() {
        let handler = create_test_handler();
        let state = LLMinx::new();
        let normalized = handler.normalize(&state);

        assert_eq!(normalized.corner_signature.len(), NUM_CORNERS);
        assert_eq!(normalized.edge_signature.len(), NUM_EDGES);
    }

    #[test]
    fn test_equivalence_same_state() {
        let handler = create_test_handler();
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        assert!(handler.are_equivalent(&state1, &state2));
    }

    #[test]
    fn test_invalid_orientation_group() {
        let piece_map = PieceMap::default_megaminx();
        let equivalences = vec![];
        let orientation_groups = vec![OrientationGroup {
            num_orientations: 2,
            pieces: vec!["UC1".to_string()],
        }];

        let result = EquivalenceHandler::new(equivalences, orientation_groups, piece_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_mixed_piece_types_in_equivalence() {
        let piece_map = PieceMap::default_megaminx();
        let equivalences = vec![EquivalenceSet {
            pieces: vec!["UC1".to_string(), "UE1".to_string()],
        }];

        let result = EquivalenceHandler::new(equivalences, vec![], piece_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_u_move_equivalent_with_full_ll() {
        use crate::minx::Move;

        // Create handler with all U layer corners and edges equivalent
        let piece_map = PieceMap::default_megaminx();
        let equivalences = vec![
            EquivalenceSet {
                pieces: vec![
                    "UC1".to_string(),
                    "UC2".to_string(),
                    "UC3".to_string(),
                    "UC4".to_string(),
                    "UC5".to_string(),
                ],
            },
            EquivalenceSet {
                pieces: vec![
                    "UE1".to_string(),
                    "UE2".to_string(),
                    "UE3".to_string(),
                    "UE4".to_string(),
                    "UE5".to_string(),
                ],
            },
        ];

        let handler = EquivalenceHandler::new(equivalences, vec![], piece_map).unwrap();
        let solved = LLMinx::new();
        let norm_solved = handler.normalize(&solved);

        let mut after_u = LLMinx::new();
        after_u.apply_move(Move::U);
        let norm_u = handler.normalize(&after_u);
        assert_eq!(norm_solved, norm_u,);
    }
}
