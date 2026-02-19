use super::types::{GeneratedState, PieceMap, SortCriterion};
use crate::minx::{LLMinx, NUM_CORNERS, NUM_EDGES};
use std::cmp::Ordering;

pub struct CaseSorter {
    criteria: Vec<SortCriterion>,
    piece_map: PieceMap,
}

impl CaseSorter {
    pub fn new(criteria: Vec<SortCriterion>, piece_map: PieceMap) -> Self {
        Self {
            criteria,
            piece_map,
        }
    }

    pub fn sort(&self, states: &mut [GeneratedState]) {
        states.sort_by(|a, b| self.compare_states(a, b));

        for (i, state) in states.iter_mut().enumerate() {
            state.case_number = i + 1;
        }
    }

    fn compare_states(&self, a: &GeneratedState, b: &GeneratedState) -> Ordering {
        for criterion in &self.criteria {
            let comparison = self.compare_by_criterion(a, b, criterion);
            if comparison != Ordering::Equal {
                return comparison;
            }
        }

        a.setup_moves.cmp(&b.setup_moves)
    }

    fn compare_by_criterion(
        &self,
        a: &GeneratedState,
        b: &GeneratedState,
        criterion: &SortCriterion,
    ) -> Ordering {
        match criterion {
            SortCriterion::SetPriority { pieces } => {
                self.compare_set_priority(&a.state, &b.state, pieces)
            }
            SortCriterion::OrientationAt { pieces } => {
                self.compare_orientation_at(&a.state, &b.state, pieces)
            }
            SortCriterion::OrientationOf { pieces } => {
                self.compare_orientation_of(&a.state, &b.state, pieces)
            }
            SortCriterion::PermutationAt { pieces } => {
                self.compare_permutation_at(&a.state, &b.state, pieces)
            }
            SortCriterion::PermutationOf { pieces } => {
                self.compare_permutation_of(&a.state, &b.state, pieces)
            }
        }
    }

    fn compare_set_priority(&self, a: &LLMinx, b: &LLMinx, pieces: &[String]) -> Ordering {
        let mut priority_map = std::collections::HashMap::new();
        for (i, piece) in pieces.iter().enumerate() {
            if let Some(idx) = self.piece_map.get_corner(piece) {
                priority_map.insert((true, idx), i as i32);
            } else if let Some(idx) = self.piece_map.get_edge(piece) {
                priority_map.insert((false, idx), i as i32);
            }
        }

        let unlisted_priority = pieces.len() as i32;

        for i in 0..NUM_CORNERS {
            let piece_a = a.corner_positions()[i] as usize;
            let piece_b = b.corner_positions()[i] as usize;

            let priority_a = priority_map
                .get(&(true, piece_a))
                .copied()
                .unwrap_or(unlisted_priority);
            let priority_b = priority_map
                .get(&(true, piece_b))
                .copied()
                .unwrap_or(unlisted_priority);

            match priority_a.cmp(&priority_b) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        for i in 0..NUM_EDGES {
            let piece_a = a.edge_positions()[i] as usize;
            let piece_b = b.edge_positions()[i] as usize;

            let priority_a = priority_map
                .get(&(false, piece_a))
                .copied()
                .unwrap_or(unlisted_priority);
            let priority_b = priority_map
                .get(&(false, piece_b))
                .copied()
                .unwrap_or(unlisted_priority);

            match priority_a.cmp(&priority_b) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        Ordering::Equal
    }

    fn compare_orientation_at(&self, a: &LLMinx, b: &LLMinx, pieces: &[String]) -> Ordering {
        let oriented_count_a = self.count_oriented_at(a, pieces);
        let oriented_count_b = self.count_oriented_at(b, pieces);

        match oriented_count_b.cmp(&oriented_count_a) {
            Ordering::Equal => {}
            other => return other,
        }

        for piece_name in pieces {
            if let Some(idx) = self.piece_map.get_corner(piece_name) {
                let ori_a = a.get_corner_orientation(idx as u8);
                let ori_b = b.get_corner_orientation(idx as u8);
                match ori_a.cmp(&ori_b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            } else if let Some(idx) = self.piece_map.get_edge(piece_name) {
                let ori_a = a.get_edge_orientation(idx as u8);
                let ori_b = b.get_edge_orientation(idx as u8);
                match ori_a.cmp(&ori_b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
        }

        Ordering::Equal
    }

    fn compare_orientation_of(&self, a: &LLMinx, b: &LLMinx, pieces: &[String]) -> Ordering {
        for piece_name in pieces {
            if let Some(piece_idx) = self.piece_map.get_corner(piece_name) {
                let location_a = self.find_corner_by_piece(a, piece_idx);
                let location_b = self.find_corner_by_piece(b, piece_idx);

                if let (Some(loc_a), Some(loc_b)) = (location_a, location_b) {
                    let ori_a = a.get_corner_orientation(loc_a);
                    let ori_b = b.get_corner_orientation(loc_b);
                    match ori_a.cmp(&ori_b) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
            } else if let Some(piece_idx) = self.piece_map.get_edge(piece_name) {
                let location_a = self.find_edge_by_piece(a, piece_idx);
                let location_b = self.find_edge_by_piece(b, piece_idx);

                if let (Some(loc_a), Some(loc_b)) = (location_a, location_b) {
                    let ori_a = a.get_edge_orientation(loc_a);
                    let ori_b = b.get_edge_orientation(loc_b);
                    match ori_a.cmp(&ori_b) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
            }
        }

        Ordering::Equal
    }

    fn compare_permutation_at(&self, a: &LLMinx, b: &LLMinx, pieces: &[String]) -> Ordering {
        for piece_name in pieces {
            if let Some(idx) = self.piece_map.get_corner(piece_name) {
                let piece_at_a = a.corner_positions()[idx];
                let piece_at_b = b.corner_positions()[idx];

                match piece_at_a.cmp(&piece_at_b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            } else if let Some(idx) = self.piece_map.get_edge(piece_name) {
                let piece_at_a = a.edge_positions()[idx];
                let piece_at_b = b.edge_positions()[idx];

                match piece_at_a.cmp(&piece_at_b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
        }

        Ordering::Equal
    }

    fn compare_permutation_of(&self, a: &LLMinx, b: &LLMinx, pieces: &[String]) -> Ordering {
        for piece_name in pieces {
            if let Some(piece_idx) = self.piece_map.get_corner(piece_name) {
                let pos_a = self.find_corner_by_piece(a, piece_idx);
                let pos_b = self.find_corner_by_piece(b, piece_idx);

                match pos_a.cmp(&pos_b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            } else if let Some(piece_idx) = self.piece_map.get_edge(piece_name) {
                let pos_a = self.find_edge_by_piece(a, piece_idx);
                let pos_b = self.find_edge_by_piece(b, piece_idx);

                match pos_a.cmp(&pos_b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
        }

        Ordering::Equal
    }

    fn count_oriented_at(&self, state: &LLMinx, pieces: &[String]) -> usize {
        let mut count = 0;

        for piece_name in pieces {
            if let Some(idx) = self.piece_map.get_corner(piece_name) {
                if state.get_corner_orientation(idx as u8) == 0 {
                    count += 1;
                }
            } else if let Some(idx) = self.piece_map.get_edge(piece_name)
                && state.get_edge_orientation(idx as u8) == 0
            {
                count += 1;
            }
        }

        count
    }

    fn find_corner_by_piece(&self, state: &LLMinx, piece_idx: usize) -> Option<u8> {
        for (i, &piece) in state.corner_positions().iter().enumerate() {
            if piece as usize == piece_idx {
                return Some(i as u8);
            }
        }
        None
    }

    fn find_edge_by_piece(&self, state: &LLMinx, piece_idx: usize) -> Option<u8> {
        for (i, &piece) in state.edge_positions().iter().enumerate() {
            if piece as usize == piece_idx {
                return Some(i as u8);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::minx::Move;

    fn create_test_sorter() -> CaseSorter {
        let criteria = vec![SortCriterion::OrientationAt {
            pieces: vec!["UC1".to_string(), "UC2".to_string()],
        }];
        CaseSorter::new(criteria, PieceMap::default_megaminx())
    }

    #[test]
    fn test_sort_states() {
        let sorter = create_test_sorter();
        let mut states = vec![
            GeneratedState::new(LLMinx::new(), "R".to_string()),
            GeneratedState::new(LLMinx::new(), "U".to_string()),
        ];

        sorter.sort(&mut states);

        assert_eq!(states[0].case_number, 1);
        assert_eq!(states[1].case_number, 2);
    }

    #[test]
    fn test_compare_orientation_at() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        let result = sorter.compare_orientation_at(&state1, &state2, &["UC1".to_string()]);

        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_count_oriented() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state = LLMinx::new();

        let pieces = vec!["UC1".to_string(), "UC2".to_string(), "UC3".to_string()];
        let count = sorter.count_oriented_at(&state, &pieces);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_compare_set_priority() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        let pieces = vec!["UC1".to_string(), "UC2".to_string()];
        let result = sorter.compare_set_priority(&state1, &state2, &pieces);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_compare_orientation_of() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        let pieces = vec!["UC1".to_string()];
        let result = sorter.compare_orientation_of(&state1, &state2, &pieces);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_compare_permutation_at() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        let pieces = vec!["UC1".to_string()];
        let result = sorter.compare_permutation_at(&state1, &state2, &pieces);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_compare_permutation_of() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        let pieces = vec!["UC1".to_string()];
        let result = sorter.compare_permutation_of(&state1, &state2, &pieces);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_find_corner_by_piece() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state = LLMinx::new();

        let pos = sorter.find_corner_by_piece(&state, 0);
        assert_eq!(pos, Some(0));

        let pos = sorter.find_corner_by_piece(&state, 100);
        assert_eq!(pos, None);
    }

    #[test]
    fn test_find_edge_by_piece() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state = LLMinx::new();

        let pos = sorter.find_edge_by_piece(&state, 0);
        assert_eq!(pos, Some(0));

        let pos = sorter.find_edge_by_piece(&state, 100);
        assert_eq!(pos, None);
    }

    #[test]
    fn test_count_oriented_edges() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state = LLMinx::new();

        let pieces = vec!["UE1".to_string(), "UE2".to_string()];
        let count = sorter.count_oriented_at(&state, &pieces);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_sort_with_multiple_criteria() {
        let criteria = vec![
            SortCriterion::OrientationAt {
                pieces: vec!["UC1".to_string()],
            },
            SortCriterion::PermutationAt {
                pieces: vec!["UC1".to_string()],
            },
        ];
        let sorter = CaseSorter::new(criteria, PieceMap::default_megaminx());

        let mut states = vec![
            GeneratedState::new(LLMinx::new(), "A".to_string()),
            GeneratedState::new(LLMinx::new(), "B".to_string()),
        ];

        sorter.sort(&mut states);
        assert_eq!(states[0].case_number, 1);
    }

    #[test]
    fn test_compare_with_move_applied() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());

        let state1 = LLMinx::new();
        let mut state2 = LLMinx::new();
        state2.apply_move(Move::R);

        let pieces = vec!["UC1".to_string()];
        let result = sorter.compare_permutation_at(&state1, &state2, &pieces);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_compare_edges_orientation_of() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        let pieces = vec!["UE1".to_string()];
        let result = sorter.compare_orientation_of(&state1, &state2, &pieces);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_compare_edges_permutation_of() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        let pieces = vec!["UE1".to_string()];
        let result = sorter.compare_permutation_of(&state1, &state2, &pieces);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_compare_edges_permutation_at() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let state1 = LLMinx::new();
        let state2 = LLMinx::new();

        let pieces = vec!["UE1".to_string()];
        let result = sorter.compare_permutation_at(&state1, &state2, &pieces);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_sort_by_set_priority() {
        let criteria = vec![SortCriterion::SetPriority {
            pieces: vec!["UC1".to_string(), "UC2".to_string()],
        }];
        let sorter = CaseSorter::new(criteria, PieceMap::default_megaminx());

        let mut states = vec![
            GeneratedState::new(LLMinx::new(), "B".to_string()),
            GeneratedState::new(LLMinx::new(), "A".to_string()),
        ];

        sorter.sort(&mut states);
        assert_eq!(states.len(), 2);
    }

    #[test]
    fn test_sort_by_orientation_of() {
        let criteria = vec![SortCriterion::OrientationOf {
            pieces: vec!["UC1".to_string()],
        }];
        let sorter = CaseSorter::new(criteria, PieceMap::default_megaminx());

        let mut states = vec![GeneratedState::new(LLMinx::new(), "A".to_string())];

        sorter.sort(&mut states);
        assert_eq!(states[0].case_number, 1);
    }

    #[test]
    fn test_sort_by_permutation_of() {
        let criteria = vec![SortCriterion::PermutationOf {
            pieces: vec!["UC1".to_string()],
        }];
        let sorter = CaseSorter::new(criteria, PieceMap::default_megaminx());

        let mut states = vec![GeneratedState::new(LLMinx::new(), "A".to_string())];

        sorter.sort(&mut states);
        assert_eq!(states[0].case_number, 1);
    }

    #[test]
    fn test_compare_states_falls_back_to_setup_moves() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let a = GeneratedState::new(LLMinx::new(), "A".to_string());
        let b = GeneratedState::new(LLMinx::new(), "B".to_string());
        assert_eq!(sorter.compare_states(&a, &b), Ordering::Less);
    }

    #[test]
    fn test_compare_by_criterion_all_arms_with_edge_piece() {
        let piece_map = PieceMap::default_megaminx();
        let sorter = CaseSorter::new(vec![], piece_map);

        let mut a_state = LLMinx::new();
        let mut b_state = LLMinx::new();
        a_state.apply_move(Move::U);
        b_state.apply_move(Move::R);

        let a = GeneratedState::new(a_state, "A".to_string());
        let b = GeneratedState::new(b_state, "B".to_string());
        let pieces = vec!["UE1".to_string()];

        let criteria = vec![
            SortCriterion::OrientationOf {
                pieces: pieces.clone(),
            },
            SortCriterion::PermutationAt {
                pieces: pieces.clone(),
            },
            SortCriterion::PermutationOf {
                pieces: pieces.clone(),
            },
        ];

        for criterion in criteria {
            let _ = sorter.compare_by_criterion(&a, &b, &criterion);
        }
    }

    #[test]
    fn test_compare_orientation_at_with_edge_branch() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let mut a = LLMinx::new();
        let mut b = LLMinx::new();
        a.apply_move(Move::U);
        b.apply_move(Move::R);

        let pieces = vec!["UE1".to_string()];
        let _ = sorter.compare_orientation_at(&a, &b, &pieces);
    }

    #[test]
    fn test_compare_states_returns_on_first_non_equal_criterion() {
        let sorter = CaseSorter::new(
            vec![SortCriterion::PermutationOf {
                pieces: vec!["UC1".to_string()],
            }],
            PieceMap::default_megaminx(),
        );

        let a = GeneratedState::new(LLMinx::new(), "Z".to_string());
        let mut changed = LLMinx::new();
        changed.apply_move(Move::R);
        let b = GeneratedState::new(changed, "A".to_string());

        assert_ne!(sorter.compare_states(&a, &b), Ordering::Equal);
    }

    #[test]
    fn test_orientation_and_permutation_comparators_non_equal_paths() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let mut a = LLMinx::new();
        let mut b = LLMinx::new();
        a.apply_move(Move::R);
        b.apply_move(Move::U);

        let _ = sorter.compare_orientation_of(&a, &b, &["UC1".to_string()]);
        let _ = sorter.compare_orientation_of(&a, &b, &["UE1".to_string()]);
        let _ = sorter.compare_permutation_of(&a, &b, &["UC1".to_string()]);
    }

    #[test]
    fn test_comparators_return_non_equal_for_different_states() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let mut a = LLMinx::new();
        let mut b = LLMinx::new();

        a.set_corner_orientation(0, 1);
        assert_ne!(
            sorter.compare_orientation_at(&a, &b, &["UC1".to_string()]),
            Ordering::Equal
        );

        b.corner_positions_mut().swap(0, 1);
        assert_ne!(
            sorter.compare_permutation_at(&a, &b, &["UC1".to_string()]),
            Ordering::Equal
        );
    }

    #[test]
    fn test_set_priority_and_unknown_piece_paths() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());
        let mut a = LLMinx::new();
        let b = LLMinx::new();
        a.corner_positions_mut().swap(0, 1);

        let pieces = vec!["UC2".to_string(), "NOT_A_PIECE".to_string()];
        let cmp = sorter.compare_set_priority(&a, &b, &pieces);
        assert!(matches!(
            cmp,
            Ordering::Less | Ordering::Greater | Ordering::Equal
        ));

        let unknown_cmp = sorter.compare_orientation_of(&a, &b, &["UNKNOWN".to_string()]);
        assert_eq!(unknown_cmp, Ordering::Equal);
    }

    #[test]
    fn test_compare_set_priority_edge_and_state_return_paths() {
        let sorter = CaseSorter::new(
            vec![SortCriterion::SetPriority {
                pieces: vec!["UE2".to_string(), "UE1".to_string()],
            }],
            PieceMap::default_megaminx(),
        );

        let mut a_state = LLMinx::new();
        let b_state = LLMinx::new();
        a_state.edge_positions_mut().swap(0, 1);

        let a = GeneratedState::new(a_state.clone(), "A".to_string());
        let b = GeneratedState::new(b_state.clone(), "B".to_string());

        assert_ne!(
            sorter.compare_set_priority(
                &a_state,
                &b_state,
                &["UE2".to_string(), "UE1".to_string()]
            ),
            Ordering::Equal
        );
        assert_ne!(sorter.compare_states(&a, &b), Ordering::Equal);
    }

    #[test]
    fn test_compare_orientation_at_corner_and_edge_non_equal_returns() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());

        let mut a_corner = LLMinx::new();
        let b_corner = LLMinx::new();
        a_corner.set_corner_orientation(0, 1);
        assert_ne!(
            sorter.compare_orientation_at(&a_corner, &b_corner, &["UC1".to_string()]),
            Ordering::Equal
        );

        let mut a_edge = LLMinx::new();
        let b_edge = LLMinx::new();
        a_edge.set_edge_orientation(0, 1);
        assert_ne!(
            sorter.compare_orientation_at(&a_edge, &b_edge, &["UE1".to_string()]),
            Ordering::Equal
        );
    }

    #[test]
    fn test_compare_orientation_of_corner_and_edge_non_equal_returns() {
        let sorter = CaseSorter::new(vec![], PieceMap::default_megaminx());

        let mut a_corner = LLMinx::new();
        let b_corner = LLMinx::new();
        a_corner.set_corner_orientation(0, 1);
        assert_ne!(
            sorter.compare_orientation_of(&a_corner, &b_corner, &["UC1".to_string()]),
            Ordering::Equal
        );

        let mut a_edge = LLMinx::new();
        let b_edge = LLMinx::new();
        a_edge.set_edge_orientation(0, 1);
        assert_ne!(
            sorter.compare_orientation_of(&a_edge, &b_edge, &["UE1".to_string()]),
            Ordering::Equal
        );
    }
}
