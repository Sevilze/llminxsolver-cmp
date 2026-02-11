//! State generator for batch solving
//!
//! Generates all states from a parsed scramble using:
//! - Plain moves: Applied directly to current states
//! - Series: Cartesian product of current states with series options
//! - Generators: BFS expansion from current states

use super::adjust::AdjustHandler;
use super::equivalence::EquivalenceHandler;
use super::parser::ScrambleParser;
use super::sorting::CaseSorter;
use super::types::{
    BatchError, EquivalenceSet, GeneratedState, NormalizedState, OrientationGroup, ParsedScramble,
    PieceMap, ScrambleSegment, SortCriterion,
};
use crate::minx::{LLMinx, NUM_CORNERS, NUM_EDGES};
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub type GeneratorCallback = Arc<dyn Fn(usize, &str) + Send + Sync>;

/// Configuration for batch state generation
#[derive(Debug, Clone, Default)]
pub struct GeneratorConfig {
    pub scramble: String,
    pub equivalences_str: String,
    pub pre_adjust: Vec<String>,
    pub post_adjust: Vec<String>,
    pub sort_criteria: Vec<SortCriterion>,
}

/// Batch generation pipeline: parsing, equivalences, adjust, and sorting
pub fn generate_batch_states(
    config: &GeneratorConfig,
    interrupt: Option<Arc<AtomicBool>>,
    callback: Option<GeneratorCallback>,
) -> Result<(Vec<GeneratedState>, Option<Arc<EquivalenceHandler>>), BatchError> {
    if let Some(ref int) = interrupt {
        int.store(false, Ordering::SeqCst);
    }

    let parsed = ScrambleParser::parse(&config.scramble)?;
    if parsed.is_empty() {
        return Ok((Vec::new(), None));
    }

    let (equivalences, orientation_groups) =
        ScrambleParser::parse_equivalences(&config.equivalences_str);
    let equiv_sets: Vec<_> = equivalences
        .into_iter()
        .map(|pieces| EquivalenceSet { pieces })
        .collect();
    let orient_groups: Vec<_> = orientation_groups
        .into_iter()
        .map(|(num, pieces)| OrientationGroup {
            num_orientations: num,
            pieces,
        })
        .collect();

    let equiv_handler = if !equiv_sets.is_empty() || !orient_groups.is_empty() {
        let piece_map = PieceMap::default_megaminx();
        EquivalenceHandler::new(equiv_sets, orient_groups, piece_map)
            .ok()
            .map(Arc::new)
    } else {
        None
    };

    let mut generator = StateGenerator::new_solved();

    if let Some(int) = interrupt.clone() {
        generator.set_interrupt(int);
    }

    if let Some(ref equiv) = equiv_handler {
        generator.set_equivalence(Arc::clone(equiv));
    }

    if let Some(cb) = callback {
        generator.set_callback(move |count, msg| {
            cb(count, msg);
        });
    }

    let mut states = generator.generate_filtered(&parsed)?;

    if let Some(ref int) = interrupt
        && int.load(Ordering::SeqCst)
    {
        return Err(BatchError::ParseError("Generation cancelled.".to_string()));
    }

    if let Some(ref equiv) = equiv_handler {
        equiv.deduplicate(&mut states);
    }

    if (!config.pre_adjust.is_empty() || !config.post_adjust.is_empty())
        && let Ok(adjust_handler) = AdjustHandler::new(&config.pre_adjust, &config.post_adjust)
    {
        states = adjust_handler.reduce_states(&states, equiv_handler.as_ref());
    }

    if !config.sort_criteria.is_empty() {
        let piece_map = PieceMap::default_megaminx();
        let sorter = CaseSorter::new(config.sort_criteria.clone(), piece_map);
        sorter.sort(&mut states);
    }

    for (i, state) in states.iter_mut().enumerate() {
        state.case_number = i + 1;
    }

    Ok((states, equiv_handler))
}

/// Generates states from parsed scramble segments
pub struct StateGenerator {
    base_state: LLMinx,
    interrupted: Option<Arc<AtomicBool>>,
    callback: Option<GeneratorCallback>,
    equivalence: Option<Arc<EquivalenceHandler>>,
}

impl StateGenerator {
    /// Create a new state generator with the given base state
    pub fn new(base_state: LLMinx) -> Self {
        Self {
            base_state,
            interrupted: None,
            callback: None,
            equivalence: None,
        }
    }

    /// Create a new state generator with a solved base state
    pub fn new_solved() -> Self {
        Self::new(LLMinx::new())
    }

    /// Set the interrupt flag for cancellation support
    pub fn set_interrupt(&mut self, interrupted: Arc<AtomicBool>) {
        self.interrupted = Some(interrupted);
    }

    /// Set the progress callback
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(usize, &str) + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(callback));
    }

    /// Set the equivalence handler for equivalence-aware BFS deduplication
    pub fn set_equivalence(&mut self, equivalence: Arc<EquivalenceHandler>) {
        self.equivalence = Some(equivalence);
    }

    /// Normalize a state, using equivalence handler if available
    fn normalize_state(&self, state: &LLMinx) -> NormalizedState {
        if let Some(ref equiv) = self.equivalence {
            equiv.normalize(state)
        } else {
            // Default normalization when no equivalence handler
            NormalizedState {
                corner_signature: state.corner_positions().to_vec(),
                edge_signature: state.edge_positions().to_vec(),
                corner_orientation: (0..NUM_CORNERS as u8)
                    .map(|i| state.get_corner_orientation(i))
                    .collect(),
                edge_orientation: (0..NUM_EDGES as u8)
                    .map(|i| state.get_edge_orientation(i))
                    .collect(),
            }
        }
    }

    fn is_interrupted(&self) -> bool {
        self.interrupted
            .as_ref()
            .is_some_and(|i| i.load(Ordering::Relaxed))
    }

    fn fire_callback(&self, count: usize, message: &str) {
        if let Some(ref cb) = self.callback {
            cb(count, message);
        }
    }

    /// Generate all states from a parsed scramble
    ///
    /// The generation process works as follows:
    /// 1. Start with the base state
    /// 2. Process each segment sequentially:
    ///    - Plain: Apply moves to all current states
    ///    - Series: Cartesian product with series options
    ///    - Generators: BFS expansion using generator moves
    ///
    /// # Errors
    /// Returns `BatchError::InvalidMove` for unrecognized move strings
    pub fn generate(&self, parsed: &ParsedScramble) -> Result<Vec<GeneratedState>, BatchError> {
        if parsed.is_empty() {
            return Ok(vec![]);
        }

        let mut states = vec![GeneratedState::new(self.base_state.clone(), String::new())];

        for segment in &parsed.segments {
            if self.is_interrupted() {
                break;
            }
            match segment {
                ScrambleSegment::Plain(moves_str) => {
                    states = self.apply_plain_moves(states, moves_str)?;
                }
                ScrambleSegment::Series(options) => {
                    states = self.apply_series(states, options)?;
                }
                ScrambleSegment::Generators(generators) => {
                    states = self.apply_generators(states, generators)?;
                }
            }
        }

        Ok(states)
    }

    /// Generate all states from a parsed scramble, filtering out solved states
    pub fn generate_filtered(
        &self,
        parsed: &ParsedScramble,
    ) -> Result<Vec<GeneratedState>, BatchError> {
        let mut states = self.generate(parsed)?;
        let solved = LLMinx::new();
        states.retain(|s| !s.state.state_equals(&solved));
        states.retain(|s| !self.is_trivial_state(&s.state, &s.setup_moves));
        Ok(states)
    }

    fn is_trivial_state(&self, _state: &LLMinx, setup_moves: &str) -> bool {
        let trimmed = setup_moves.trim();
        if trimmed.is_empty() {
            return false;
        }

        let moves: Vec<&str> = trimmed.split_whitespace().collect();
        if moves.is_empty() {
            return false;
        }

        let first_move = moves[0];
        moves.iter().all(|&m| m == first_move)
    }

    /// Apply plain moves to all states
    fn apply_plain_moves(
        &self,
        states: Vec<GeneratedState>,
        moves_str: &str,
    ) -> Result<Vec<GeneratedState>, BatchError> {
        let moves = ScrambleParser::parse_moves(moves_str)?;

        let mut result = Vec::with_capacity(states.len());

        for state in states {
            let mut new_state = state.state.clone();
            for &mv in &moves {
                new_state.apply_move(mv);
            }

            let new_moves = if state.setup_moves.is_empty() {
                moves_str.to_string()
            } else {
                format!("{} {}", state.setup_moves, moves_str)
            };

            result.push(GeneratedState::new(new_state, new_moves));
        }

        Ok(result)
    }

    fn apply_series(
        &self,
        states: Vec<GeneratedState>,
        options: &[String],
    ) -> Result<Vec<GeneratedState>, BatchError> {
        use std::collections::HashMap;

        let mut result_map: HashMap<NormalizedState, GeneratedState> = HashMap::new();

        for state in &states {
            let key = self.normalize_state(&state.state);
            result_map.entry(key).or_insert_with(|| state.clone());
        }

        let mut new_map: HashMap<NormalizedState, GeneratedState> = HashMap::new();

        for state in result_map.values() {
            for option in options {
                let moves = ScrambleParser::parse_moves(option)?;
                let mut new_state = state.state.clone();

                for &mv in &moves {
                    new_state.apply_move(mv);
                }

                let new_moves = if state.setup_moves.is_empty() {
                    option.clone()
                } else {
                    format!("{} {}", state.setup_moves, option)
                };

                let key = self.normalize_state(&new_state);
                new_map
                    .entry(key)
                    .or_insert_with(|| GeneratedState::new(new_state, new_moves));
            }
        }

        Ok(new_map.into_values().collect())
    }

    /// Apply BFS expansion using generators
    fn apply_generators(
        &self,
        states: Vec<GeneratedState>,
        generators: &[String],
    ) -> Result<Vec<GeneratedState>, BatchError> {
        // Parse all generator moves
        let mut generator_moves = Vec::new();
        for gen_str in generators {
            let moves = ScrambleParser::parse_moves(gen_str)?;
            generator_moves.push(moves);
        }

        // Start with all input states
        let mut all_states: HashSet<NormalizedState> = HashSet::new();
        let mut result = Vec::new();

        for state in states {
            let normalized = self.normalize_state(&state.state);
            if all_states.insert(normalized) {
                result.push(state);
            }
        }

        // BFS expansion with interrupt support and progress callbacks
        let mut queue = result.clone();
        let mut last_callback_count = 0;

        while !queue.is_empty() {
            if self.is_interrupted() {
                break;
            }

            let current = queue.remove(0);

            // Fire progress callback periodically
            if result.len() - last_callback_count >= 10 {
                self.fire_callback(
                    result.len(),
                    &format!("Generated {} states...", result.len()),
                );
                last_callback_count = result.len();
            }

            // Try each generator
            for gen_moves in &generator_moves {
                if self.is_interrupted() {
                    break;
                }

                let mut new_state = current.state.clone();

                for &mv in gen_moves {
                    new_state.apply_move(mv);
                }

                let normalized = self.normalize_state(&new_state);

                if all_states.insert(normalized) {
                    let gen_str = gen_moves
                        .iter()
                        .map(|m| m.to_string().trim())
                        .collect::<Vec<_>>()
                        .join(" ");

                    let new_moves = if current.setup_moves.is_empty() {
                        gen_str
                    } else {
                        format!("{} {}", current.setup_moves, gen_str)
                    };

                    let gen_result = GeneratedState::new(new_state, new_moves);
                    queue.push(gen_result.clone());
                    result.push(gen_result);
                }
            }
        }

        self.fire_callback(
            result.len(),
            &format!("Generation complete: {} states", result.len()),
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch_solver::types::CaseModifiers;
    use crate::minx::Move;

    #[test]
    fn test_generate_plain() {
        let generator = StateGenerator::new_solved();
        let parsed = ParsedScramble {
            segments: vec![ScrambleSegment::Plain("R U".to_string())],
            modifiers: CaseModifiers::default(),
        };

        let states = generator.generate(&parsed).unwrap();
        assert_eq!(states.len(), 1);
    }

    #[test]
    fn test_generate_series() {
        let generator = StateGenerator::new_solved();
        let parsed = ParsedScramble {
            segments: vec![ScrambleSegment::Series(vec![
                "R".to_string(),
                "U".to_string(),
            ])],
            modifiers: CaseModifiers::default(),
        };

        let states = generator.generate(&parsed).unwrap();
        assert_eq!(states.len(), 2);
    }

    #[test]
    fn test_generate_complex() {
        let generator = StateGenerator::new_solved();
        let parsed = ParsedScramble {
            segments: vec![
                ScrambleSegment::Plain("R".to_string()),
                ScrambleSegment::Series(vec!["U".to_string(), "U'".to_string()]),
            ],
            modifiers: CaseModifiers::default(),
        };

        let states = generator.generate(&parsed).unwrap();
        assert_eq!(states.len(), 2);
    }

    #[test]
    fn test_parse_moves() {
        let moves = ScrambleParser::parse_moves("R U R'").unwrap();
        assert_eq!(moves.len(), 3);
        assert_eq!(moves[0], Move::R);
        assert_eq!(moves[1], Move::U);
        assert_eq!(moves[2], Move::Ri);
    }

    #[test]
    fn test_parse_invalid_move() {
        let result = ScrambleParser::parse_moves("R X U");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_with_equivalence() {
        use crate::batch_solver::equivalence::EquivalenceHandler;
        use crate::batch_solver::types::{EquivalenceSet, PieceMap};

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

        let equiv_handler =
            Arc::new(EquivalenceHandler::new(equivalences, vec![], piece_map).unwrap());

        let mut generator = StateGenerator::new_solved();
        generator.set_equivalence(equiv_handler);

        let parsed = ParsedScramble {
            segments: vec![ScrambleSegment::Generators(vec!["U".to_string()])],
            modifiers: CaseModifiers::default(),
        };

        let states = generator.generate(&parsed).unwrap();
        assert!(states.len() <= 5,);
    }

    #[test]
    fn test_generate_generators_filtered() {
        // Test that generate_filtered removes the solved state
        let generator = StateGenerator::new_solved();
        let parsed = ParsedScramble {
            segments: vec![ScrambleSegment::Generators(vec!["R".to_string()])],
            modifiers: CaseModifiers::default(),
        };

        let all_states = generator.generate(&parsed).unwrap();
        let filtered_states = generator.generate_filtered(&parsed).unwrap();

        assert!(filtered_states.len() < all_states.len() || all_states.is_empty());
    }
}
