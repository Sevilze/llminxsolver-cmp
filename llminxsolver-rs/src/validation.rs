use crate::minx::{NUM_CORNERS, NUM_EDGES};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    InvalidCornerPermutation(String),
    InvalidEdgePermutation(String),
    InvalidCornerOrientation(String),
    InvalidEdgeOrientation(String),
    PermutationParity(String),
    InvalidStateSize(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidCornerPermutation(msg) => {
                write!(f, "Invalid corner permutation: {}", msg)
            }
            ValidationError::InvalidEdgePermutation(msg) => {
                write!(f, "Invalid edge permutation: {}", msg)
            }
            ValidationError::InvalidCornerOrientation(msg) => {
                write!(f, "Invalid corner orientation: {}", msg)
            }
            ValidationError::InvalidEdgeOrientation(msg) => {
                write!(f, "Invalid edge orientation: {}", msg)
            }
            ValidationError::PermutationParity(msg) => {
                write!(f, "Permutation parity error: {}", msg)
            }
            ValidationError::InvalidStateSize(msg) => {
                write!(f, "Invalid state size: {}", msg)
            }
        }
    }
}

impl Error for ValidationError {}

pub struct MegaminxState {
    pub corner_positions: Vec<u8>,
    pub corner_orientations: Vec<u8>,
    pub edge_positions: Vec<u8>,
    pub edge_orientations: Vec<u8>,
}

const CORNER_ORIENTATION_MODULUS: u32 = 3;
const EDGE_ORIENTATION_MODULUS: u32 = 2;
const MAX_CORNER_ORIENTATION: u8 = 2;
const MAX_EDGE_ORIENTATION: u8 = 1;

fn count_inversions(perm: &[u8]) -> usize {
    let mut inversions = 0;
    for i in 0..perm.len() {
        for j in (i + 1)..perm.len() {
            if perm[i] > perm[j] {
                inversions += 1;
            }
        }
    }
    inversions
}

fn is_valid_permutation(positions: &[u8], size: usize) -> Result<(), String> {
    if positions.len() != size {
        return Err(format!(
            "Expected {} positions, got {}",
            size,
            positions.len()
        ));
    }

    let mut seen = vec![false; size];
    for (i, &pos) in positions.iter().enumerate() {
        if pos as usize >= size {
            return Err(format!("Position {} is out of range (0-{})", pos, size - 1));
        }
        if seen[pos as usize] {
            return Err(format!("Duplicate position {} at index {}", pos, i));
        }
        seen[pos as usize] = true;
    }

    Ok(())
}

fn validate_orientation_values(
    orientations: &[u8],
    max_value: u8,
    piece_type: &str,
) -> Result<(), String> {
    for (i, &o) in orientations.iter().enumerate() {
        if o > max_value {
            return Err(format!(
                "{} {} orientation {} is out of range (0-{})",
                piece_type, i, o, max_value
            ));
        }
    }
    Ok(())
}

fn validate_orientation_sum(
    orientations: &[u8],
    modulus: u32,
    piece_type: &str,
) -> Result<(), String> {
    let sum: u32 = orientations.iter().map(|&o| o as u32).sum();
    if !sum.is_multiple_of(modulus) {
        return Err(format!(
            "Sum of {} orientations ({}) is not divisible by {}.",
            piece_type, sum, modulus
        ));
    }
    Ok(())
}

fn validate_permutation_parity(
    corner_positions: &[u8],
    edge_positions: &[u8],
) -> Result<(), String> {
    let corner_inversions = count_inversions(corner_positions);
    let edge_inversions = count_inversions(edge_positions);

    let corner_parity = corner_inversions % 2;
    let edge_parity = edge_inversions % 2;

    if corner_parity != 0 {
        return Err(format!(
            "Corner permutation has odd parity ({} inversions).",
            corner_inversions
        ));
    }

    if edge_parity != 0 {
        return Err(format!(
            "Edge permutation has odd parity ({} inversions).",
            edge_inversions
        ));
    }

    Ok(())
}

pub fn validate_last_layer_state(state: &MegaminxState) -> Result<(), ValidationError> {
    const LL_CORNERS: usize = 5;
    const LL_EDGES: usize = 5;

    if state.corner_positions.len() < LL_CORNERS {
        return Err(ValidationError::InvalidStateSize(format!(
            "Need at least {} corner positions for last layer, got {}",
            LL_CORNERS,
            state.corner_positions.len()
        )));
    }
    if state.edge_positions.len() < LL_EDGES {
        return Err(ValidationError::InvalidStateSize(format!(
            "Need at least {} edge positions for last layer, got {}",
            LL_EDGES,
            state.edge_positions.len()
        )));
    }

    let ll_corner_positions: Vec<u8> = state.corner_positions[..LL_CORNERS].to_vec();
    let ll_edge_positions: Vec<u8> = state.edge_positions[..LL_EDGES].to_vec();

    is_valid_permutation(&ll_corner_positions, LL_CORNERS)
        .map_err(ValidationError::InvalidCornerPermutation)?;

    is_valid_permutation(&ll_edge_positions, LL_EDGES)
        .map_err(ValidationError::InvalidEdgePermutation)?;

    if state.corner_orientations.len() >= LL_CORNERS {
        let ll_corner_orientations = &state.corner_orientations[..LL_CORNERS];

        validate_orientation_values(ll_corner_orientations, MAX_CORNER_ORIENTATION, "Corner")
            .map_err(ValidationError::InvalidCornerOrientation)?;

        validate_orientation_sum(ll_corner_orientations, CORNER_ORIENTATION_MODULUS, "corner")
            .map_err(ValidationError::InvalidCornerOrientation)?;
    }

    if state.edge_orientations.len() >= LL_EDGES {
        let ll_edge_orientations = &state.edge_orientations[..LL_EDGES];

        validate_orientation_values(ll_edge_orientations, MAX_EDGE_ORIENTATION, "Edge")
            .map_err(ValidationError::InvalidEdgeOrientation)?;

        validate_orientation_sum(ll_edge_orientations, EDGE_ORIENTATION_MODULUS, "edge")
            .map_err(ValidationError::InvalidEdgeOrientation)?;
    }

    validate_permutation_parity(&ll_corner_positions, &ll_edge_positions)
        .map_err(ValidationError::PermutationParity)?;

    Ok(())
}

pub fn validate_full_state(state: &MegaminxState) -> Result<(), ValidationError> {
    if state.corner_positions.len() != NUM_CORNERS {
        return Err(ValidationError::InvalidStateSize(format!(
            "Expected {} corner positions, got {}",
            NUM_CORNERS,
            state.corner_positions.len()
        )));
    }
    if state.edge_positions.len() != NUM_EDGES {
        return Err(ValidationError::InvalidStateSize(format!(
            "Expected {} edge positions, got {}",
            NUM_EDGES,
            state.edge_positions.len()
        )));
    }
    if state.corner_orientations.len() != NUM_CORNERS {
        return Err(ValidationError::InvalidStateSize(format!(
            "Expected {} corner orientations, got {}",
            NUM_CORNERS,
            state.corner_orientations.len()
        )));
    }
    if state.edge_orientations.len() != NUM_EDGES {
        return Err(ValidationError::InvalidStateSize(format!(
            "Expected {} edge orientations, got {}",
            NUM_EDGES,
            state.edge_orientations.len()
        )));
    }

    is_valid_permutation(&state.corner_positions, NUM_CORNERS)
        .map_err(ValidationError::InvalidCornerPermutation)?;

    is_valid_permutation(&state.edge_positions, NUM_EDGES)
        .map_err(ValidationError::InvalidEdgePermutation)?;

    validate_orientation_values(&state.corner_orientations, MAX_CORNER_ORIENTATION, "Corner")
        .map_err(ValidationError::InvalidCornerOrientation)?;

    validate_orientation_sum(
        &state.corner_orientations,
        CORNER_ORIENTATION_MODULUS,
        "corner",
    )
    .map_err(ValidationError::InvalidCornerOrientation)?;

    validate_orientation_values(&state.edge_orientations, MAX_EDGE_ORIENTATION, "Edge")
        .map_err(ValidationError::InvalidEdgeOrientation)?;

    validate_orientation_sum(&state.edge_orientations, EDGE_ORIENTATION_MODULUS, "edge")
        .map_err(ValidationError::InvalidEdgeOrientation)?;

    validate_permutation_parity(&state.corner_positions, &state.edge_positions)
        .map_err(ValidationError::PermutationParity)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_solved_ll_state() -> MegaminxState {
        MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        }
    }

    fn create_solved_full_state() -> MegaminxState {
        MegaminxState {
            corner_positions: (0..NUM_CORNERS as u8).collect(),
            corner_orientations: vec![0; NUM_CORNERS],
            edge_positions: (0..NUM_EDGES as u8).collect(),
            edge_orientations: vec![0; NUM_EDGES],
        }
    }

    #[test]
    fn test_valid_solved_state() {
        let state = create_solved_ll_state();
        assert!(validate_last_layer_state(&state).is_ok());
    }

    #[test]
    fn test_valid_full_solved_state() {
        let state = create_solved_full_state();
        assert!(validate_full_state(&state).is_ok());
    }

    #[test]
    fn test_invalid_double_swap_both_odd_parity() {
        let state = MegaminxState {
            corner_positions: vec![1, 0, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![1, 0, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(result, Err(ValidationError::PermutationParity(_))));
    }

    #[test]
    fn test_valid_three_cycle_permutation() {
        let state = MegaminxState {
            corner_positions: vec![1, 2, 0, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![1, 2, 0, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        assert!(validate_last_layer_state(&state).is_ok());
    }

    #[test]
    fn test_invalid_duplicate_corner() {
        let state = MegaminxState {
            corner_positions: vec![0, 0, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidCornerPermutation(_))
        ));
    }

    #[test]
    fn test_invalid_duplicate_edge() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 0, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidEdgePermutation(_))
        ));
    }

    #[test]
    fn test_invalid_orientation_sum_corners() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![1, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidCornerOrientation(_))
        ));
    }

    #[test]
    fn test_valid_twisted_corners_sum_divisible_by_three() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![1, 2, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        assert!(validate_last_layer_state(&state).is_ok());
    }

    #[test]
    fn test_valid_all_corners_twisted_clockwise() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![1, 1, 1, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        assert!(validate_last_layer_state(&state).is_ok());
    }

    #[test]
    fn test_invalid_parity_corner_swap_only() {
        // Single corner swap with no edge change - odd corner parity
        let state = MegaminxState {
            corner_positions: vec![1, 0, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(result, Err(ValidationError::PermutationParity(_))));
    }

    #[test]
    fn test_invalid_parity_edge_swap_only() {
        // Single edge swap with no corner change - odd edge parity
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![1, 0, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(result, Err(ValidationError::PermutationParity(_))));
    }

    #[test]
    fn test_invalid_corner_orientation_value_exceeds_max() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![3, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidCornerOrientation(_))
        ));
    }

    #[test]
    fn test_invalid_edge_orientation_value_exceeds_max() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![2, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidEdgeOrientation(_))
        ));
    }

    #[test]
    fn test_invalid_edge_orientation_sum_odd() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![1, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidEdgeOrientation(_))
        ));
    }

    #[test]
    fn test_valid_flipped_edge_pair() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![1, 1, 0, 0, 0],
        };
        assert!(validate_last_layer_state(&state).is_ok());
    }

    #[test]
    fn test_valid_four_flipped_edges() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![1, 1, 1, 1, 0],
        };
        assert!(validate_last_layer_state(&state).is_ok());
    }

    #[test]
    fn test_invalid_position_out_of_range() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 5],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidCornerPermutation(_))
        ));
    }

    #[test]
    fn test_invalid_insufficient_corners() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3],
            corner_orientations: vec![0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(result, Err(ValidationError::InvalidStateSize(_))));
    }

    #[test]
    fn test_invalid_insufficient_edges() {
        let state = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3],
            edge_orientations: vec![0, 0, 0, 0],
        };
        let result = validate_last_layer_state(&state);
        assert!(matches!(result, Err(ValidationError::InvalidStateSize(_))));
    }

    #[test]
    fn test_valid_complex_state_with_orientation_and_permutation() {
        let state = MegaminxState {
            corner_positions: vec![2, 0, 1, 3, 4],
            corner_orientations: vec![1, 1, 1, 0, 0],
            edge_positions: vec![2, 0, 1, 3, 4],
            edge_orientations: vec![1, 1, 0, 0, 0],
        };
        assert!(validate_last_layer_state(&state).is_ok());
    }

    #[test]
    fn test_full_state_invalid_corner_count() {
        let mut state = create_solved_full_state();
        state.corner_positions.pop();
        let result = validate_full_state(&state);
        assert!(matches!(result, Err(ValidationError::InvalidStateSize(_))));
    }

    #[test]
    fn test_full_state_invalid_edge_count() {
        let mut state = create_solved_full_state();
        state.edge_positions.pop();
        let result = validate_full_state(&state);
        assert!(matches!(result, Err(ValidationError::InvalidStateSize(_))));
    }

    #[test]
    fn test_full_state_valid_with_double_swap() {
        let mut state = create_solved_full_state();
        state.corner_positions.swap(0, 1);
        state.corner_positions.swap(2, 3);
        state.edge_positions.swap(0, 1);
        state.edge_positions.swap(2, 3);
        assert!(validate_full_state(&state).is_ok());
    }
}
