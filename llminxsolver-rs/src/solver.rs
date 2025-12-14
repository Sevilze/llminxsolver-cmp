use crate::minx::{LLMinx, Move, NUM_CORNERS, NUM_EDGES};
use crate::pruner::Pruner;
use crate::search_mode::{Metric, SearchMode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEventType {
    StartSearch,
    StartDepth,
    EndDepth,
    StartBuildingTable,
    EndBuildingTable,
    Message,
    FinishSearch,
    SolutionFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEvent {
    pub event_type: StatusEventType,
    pub message: String,
    pub progress: f64,
}

impl StatusEvent {
    pub fn new(event_type: StatusEventType, message: &str, progress: f64) -> Self {
        Self {
            event_type,
            message: message.to_string(),
            progress,
        }
    }
}

pub type StatusCallback = Box<dyn Fn(StatusEvent) + Send + Sync>;

pub struct Solver {
    search_mode: SearchMode,
    metric: Metric,
    max_depth: usize,
    limit_depth: bool,
    start: LLMinx,
    ignore_corner_positions: bool,
    ignore_edge_positions: bool,
    ignore_corner_orientations: bool,
    ignore_edge_orientations: bool,
    interrupted: Arc<AtomicBool>,
    status_callback: Option<StatusCallback>,
    pruners: Vec<Box<dyn Pruner>>,
    tables: Vec<Vec<u8>>,
    moves: Vec<Move>,
    first_moves: Vec<Move>,
    next_siblings: Vec<Vec<Option<Move>>>,
    last_search_mode: Option<SearchMode>,
    last_metric: Option<Metric>,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Self {
        Self::with_config(SearchMode::RU, 12)
    }

    pub fn with_config(search_mode: SearchMode, max_depth: usize) -> Self {
        Self {
            search_mode,
            metric: Metric::Fifth,
            max_depth,
            limit_depth: false,
            start: LLMinx::new(),
            ignore_corner_positions: false,
            ignore_edge_positions: false,
            ignore_corner_orientations: false,
            ignore_edge_orientations: false,
            interrupted: Arc::new(AtomicBool::new(false)),
            status_callback: None,
            pruners: Vec::new(),
            tables: Vec::new(),
            moves: Vec::new(),
            first_moves: Vec::new(),
            next_siblings: Vec::new(),
            last_search_mode: None,
            last_metric: None,
        }
    }

    pub fn search_mode(&self) -> SearchMode {
        self.search_mode
    }

    pub fn set_search_mode(&mut self, mode: SearchMode) {
        self.search_mode = mode;
    }

    pub fn metric(&self) -> Metric {
        self.metric
    }

    pub fn set_metric(&mut self, metric: Metric) {
        self.metric = metric;
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }

    pub fn limit_depth(&self) -> bool {
        self.limit_depth
    }

    pub fn set_limit_depth(&mut self, limit: bool) {
        self.limit_depth = limit;
    }

    pub fn start(&self) -> &LLMinx {
        &self.start
    }

    pub fn set_start(&mut self, start: LLMinx) {
        self.start = start;
    }

    pub fn set_ignore_corner_positions(&mut self, ignore: bool) {
        self.ignore_corner_positions = ignore;
    }

    pub fn set_ignore_edge_positions(&mut self, ignore: bool) {
        self.ignore_edge_positions = ignore;
    }

    pub fn set_ignore_corner_orientations(&mut self, ignore: bool) {
        self.ignore_corner_orientations = ignore;
    }

    pub fn set_ignore_edge_orientations(&mut self, ignore: bool) {
        self.ignore_edge_orientations = ignore;
    }

    pub fn set_status_callback<F>(&mut self, callback: F)
    where
        F: Fn(StatusEvent) + Send + Sync + 'static,
    {
        self.status_callback = Some(Box::new(callback));
    }

    pub fn interrupt_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.interrupted)
    }

    pub fn interrupt(&self) {
        self.interrupted.store(true, Ordering::SeqCst);
    }

    fn fire_event(&self, event: StatusEvent) {
        if let Some(ref callback) = self.status_callback {
            callback(event);
        }
    }

    fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::SeqCst)
    }

    pub fn solve(&mut self) -> Vec<String> {
        let start_time = std::time::Instant::now();
        self.interrupted.store(false, Ordering::SeqCst);
        let mut solutions = Vec::new();

        if self.search_mode != self.last_search_mode.unwrap_or(self.search_mode)
            || self.metric != self.last_metric.unwrap_or(self.metric)
            || self.tables.is_empty()
        {
            self.build_moves_table();
            self.build_pruning_tables();

            if !self.is_interrupted() {
                self.last_search_mode = Some(self.search_mode);
                self.last_metric = Some(self.metric);
            } else {
                self.last_search_mode = None;
                self.last_metric = None;
            }
        }

        const IGNORE_CORNER_5: [bool; NUM_CORNERS] = [
            true, true, true, true, true, false, false, false, false, false, false, false, false,
            false,
        ];
        const IGNORE_EDGE_5: [bool; NUM_EDGES] = [
            true, true, true, true, true, false, false, false, false, false, false, false, false,
            false, false, false, false, false,
        ];

        if self.ignore_corner_positions {
            self.start.set_ignore_corner_positions(IGNORE_CORNER_5);
        }
        if self.ignore_edge_positions {
            self.start.set_ignore_edge_positions(IGNORE_EDGE_5);
        }
        if self.ignore_corner_orientations {
            self.start.set_ignore_corner_orientations(IGNORE_CORNER_5);
        }
        if self.ignore_edge_orientations {
            self.start.set_ignore_edge_orientations(IGNORE_EDGE_5);
        }

        let mut goal = LLMinx::new();
        if self.ignore_corner_positions {
            goal.set_ignore_corner_positions(IGNORE_CORNER_5);
        }
        if self.ignore_edge_positions {
            goal.set_ignore_edge_positions(IGNORE_EDGE_5);
        }
        if self.ignore_corner_orientations {
            goal.set_ignore_corner_orientations(IGNORE_CORNER_5);
        }
        if self.ignore_edge_orientations {
            goal.set_ignore_edge_orientations(IGNORE_EDGE_5);
        }

        let used_pruners = self.filter_pruning_tables();

        if !self.is_interrupted() {
            self.fire_event(StatusEvent::new(
                StatusEventType::StartSearch,
                "Searching...",
                0.0,
            ));

            let max_search_depth = if self.limit_depth {
                self.max_depth
            } else {
                127
            };

            for depth in 1..=max_search_depth {
                if self.is_interrupted() {
                    break;
                }

                self.fire_event(StatusEvent::new(
                    StatusEventType::StartDepth,
                    &format!("Searching depth {}...", depth),
                    0.0,
                ));

                let mut minx = self.start.clone();
                let mut stop = false;

                while !stop && !self.is_interrupted() {
                    let levels_left = depth.saturating_sub(minx.depth());

                    if minx.state_equals(&goal) {
                        if levels_left == 0 && Self::check_optimal(&minx) {
                            let msg = format!(
                                "{} ({},{})",
                                minx.get_generating_moves(),
                                minx.get_htm_length(),
                                minx.get_qtm_length()
                            );
                            self.fire_event(StatusEvent::new(
                                StatusEventType::SolutionFound,
                                &msg,
                                0.0,
                            ));
                            solutions.push(msg);
                        }
                        stop = self.back_track(&mut minx);
                    } else if levels_left > 0 {
                        let mut pruned = false;
                        for &(table_idx, pruner) in &used_pruners {
                            let coord = pruner.get_coordinate(&minx);
                            if self.tables[table_idx][coord] as usize > levels_left {
                                pruned = true;
                                break;
                            }
                        }

                        if !pruned {
                            stop = self.next_node(&mut minx, depth);
                        } else {
                            stop = self.back_track(&mut minx);
                        }
                    } else {
                        stop = self.next_node(&mut minx, depth);
                    }
                }

                self.fire_event(StatusEvent::new(
                    StatusEventType::EndDepth,
                    &format!("Finished depth {}", depth),
                    1.0,
                ));
            }
        }

        let elapsed = start_time.elapsed();
        let was_interrupted = self.is_interrupted();
        self.interrupted.store(false, Ordering::SeqCst);

        let msg = if was_interrupted {
            format!("Search interrupted after {} seconds.", elapsed.as_secs())
        } else {
            format!("Search completed in {} seconds.", elapsed.as_secs())
        };

        self.fire_event(StatusEvent::new(StatusEventType::FinishSearch, &msg, 1.0));

        solutions
    }

    fn build_moves_table(&mut self) {
        let possible_moves = self.search_mode.possible_moves();

        self.moves = if self.metric == Metric::Face {
            possible_moves
        } else {
            possible_moves
                .into_iter()
                .filter(|m| (*m as u8) % 4 < 2)
                .collect()
        };

        let max_move_id = Move::B2i as usize + 2;
        self.first_moves = vec![self.moves[0]; max_move_id];
        self.next_siblings = vec![vec![None; Move::B2i as usize + 1]; max_move_id];

        self.first_moves[0] = self.moves[0];
        for i in 0..(self.moves.len() - 1) {
            self.next_siblings[0][self.moves[i] as usize] = Some(self.moves[i + 1]);
        }

        for &last_move in &self.moves.clone() {
            let last_move_index = last_move as usize + 1;

            let mut first_valid = 0;
            while first_valid < self.moves.len()
                && !self.is_move_allowed(last_move, self.moves[first_valid])
            {
                first_valid += 1;
            }
            if first_valid < self.moves.len() {
                self.first_moves[last_move_index] = self.moves[first_valid];
            }

            for i in 0..(self.moves.len() - 1) {
                let current = self.moves[i];
                if !self.is_move_allowed(last_move, current) {
                    continue;
                }

                let mut next_idx = i + 1;
                while next_idx < self.moves.len()
                    && !self.is_move_allowed(last_move, self.moves[next_idx])
                {
                    next_idx += 1;
                }

                if next_idx < self.moves.len() {
                    self.next_siblings[last_move_index][current as usize] =
                        Some(self.moves[next_idx]);
                }
            }
        }
    }

    fn is_move_allowed(&self, previous: Move, current: Move) -> bool {
        if self.metric == Metric::Fifth {
            previous.inverse() != current
        } else {
            previous.face() != current.face()
        }
    }

    fn build_pruning_tables(&mut self) {
        self.pruners = self.search_mode.create_pruners();
        self.tables = Vec::with_capacity(self.pruners.len());

        for pruner in &self.pruners {
            if self.is_interrupted() {
                break;
            }

            self.fire_event(StatusEvent::new(
                StatusEventType::Message,
                &format!("Initializing pruning table {}...", pruner.name()),
                0.0,
            ));

            if pruner.is_precomputed(self.metric) {
                self.fire_event(StatusEvent::new(
                    StatusEventType::Message,
                    "Reading pruning table from disk...",
                    0.0,
                ));
                if let Some(table) = pruner.load_table(self.metric) {
                    self.tables.push(table);
                } else {
                    self.tables.push(self.build_pruning_table(pruner.as_ref()));
                }
            } else {
                self.fire_event(StatusEvent::new(
                    StatusEventType::StartBuildingTable,
                    &format!("Building pruning table {}...", pruner.name()),
                    0.0,
                ));

                let table = self.build_pruning_table(pruner.as_ref());

                if !self.is_interrupted() {
                    self.fire_event(StatusEvent::new(
                        StatusEventType::Message,
                        "Writing table to disk...",
                        0.0,
                    ));
                    pruner.save_table(&table, self.metric);
                }

                self.fire_event(StatusEvent::new(
                    StatusEventType::EndBuildingTable,
                    &format!("Finished building {}...", pruner.name()),
                    1.0,
                ));

                self.tables.push(table);
            }
        }
    }

    fn build_pruning_table(&self, pruner: &dyn Pruner) -> Vec<u8> {
        let table_size = pruner.table_size();
        let mut table = vec![u8::MAX; table_size];

        let mut minx = LLMinx::new();
        let coord = pruner.get_coordinate(&minx);
        table[coord] = 0;

        let mut nodes = 1usize;
        let mut prev_depth_count = 1usize;
        let mut depth: u8 = 0;

        while prev_depth_count > 0 && !self.is_interrupted() {
            self.fire_event(StatusEvent::new(
                StatusEventType::Message,
                &format!("Depth {}: {}", depth, prev_depth_count),
                nodes as f64 / table_size as f64,
            ));

            let forward_search = prev_depth_count < table_size - nodes;
            prev_depth_count = 0;
            let next_depth = depth + 1;

            if forward_search {
                for i in 0..table_size {
                    if self.is_interrupted() {
                        break;
                    }

                    if table[i] == depth {
                        pruner.set_minx(i, &mut minx);

                        for &m in &self.moves {
                            minx.apply_move(m);
                            let new_coord = pruner.get_coordinate(&minx);
                            if table[new_coord] == u8::MAX {
                                table[new_coord] = next_depth;
                                nodes += 1;
                                prev_depth_count += 1;
                            }
                            minx.undo_move();
                        }
                    }
                }
            } else {
                for i in 0..table_size {
                    if self.is_interrupted() {
                        break;
                    }

                    if table[i] == u8::MAX {
                        pruner.set_minx(i, &mut minx);

                        for &m in &self.moves {
                            minx.apply_move(m);
                            let new_coord = pruner.get_coordinate(&minx);
                            if table[new_coord] == depth {
                                table[i] = next_depth;
                                nodes += 1;
                                prev_depth_count += 1;
                                minx.undo_move();
                                break;
                            }
                            minx.undo_move();
                        }
                    }
                }
            }

            depth += 1;
        }

        table
    }

    fn filter_pruning_tables(&self) -> Vec<(usize, &dyn Pruner)> {
        self.pruners
            .iter()
            .enumerate()
            .filter(|(_, pruner)| {
                let dominated = (pruner.uses_corner_permutation() && self.ignore_corner_positions)
                    || (pruner.uses_edge_permutation() && self.ignore_edge_positions)
                    || (pruner.uses_corner_orientation() && self.ignore_corner_orientations)
                    || (pruner.uses_edge_orientation() && self.ignore_edge_orientations);
                !dominated
            })
            .map(|(i, pruner)| (i, pruner.as_ref()))
            .collect()
    }

    fn check_optimal(minx: &LLMinx) -> bool {
        let moves = minx.moves();
        for i in 1..moves.len() {
            if i < moves.len() - 1 && moves[i - 1] == moves[i] && moves[i] == moves[i + 1] {
                return false;
            }

            if i < moves.len() - 1 && moves[i + 1] == moves[i - 1] {
                let face_i = moves[i].face();
                let face_prev = moves[i - 1].face();

                let l_face = Move::L.face();
                let r_face = Move::R.face();
                let f_face = Move::F.face();
                let b_face = Move::B.face();

                if (face_i == l_face && face_prev == r_face)
                    || (face_i == r_face && face_prev == l_face)
                    || (face_i == b_face && face_prev == f_face)
                    || (face_i == f_face && face_prev == b_face)
                {
                    return false;
                }
            }
        }
        true
    }

    fn next_node(&self, minx: &mut LLMinx, target_depth: usize) -> bool {
        if minx.depth() < target_depth {
            let last_move_index = minx.last_move().map(|m| m as usize + 1).unwrap_or(0);
            let first = self.first_moves[last_move_index];
            minx.apply_move(first);
            false
        } else {
            self.back_track(minx)
        }
    }

    fn back_track(&self, minx: &mut LLMinx) -> bool {
        if minx.depth() == 0 {
            return true;
        }

        let sibling = minx.undo_move();
        let Some(sibling) = sibling else {
            return true;
        };

        let last_move = minx.last_move();
        let last_move_index = last_move.map(|m| m as usize + 1).unwrap_or(0);

        let mut next_sibling = self.next_siblings[last_move_index][sibling as usize];

        while last_move.is_some() && next_sibling.is_none() {
            let Some(s) = minx.undo_move() else {
                return true;
            };
            let lm = minx.last_move();
            let lm_index = lm.map(|m| m as usize + 1).unwrap_or(0);
            next_sibling = self.next_siblings[lm_index][s as usize];

            if lm.is_none() && next_sibling.is_none() {
                return true;
            }
        }

        if let Some(ns) = next_sibling {
            minx.apply_move(ns);
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_creation() {
        let solver = Solver::new();
        assert_eq!(solver.search_mode(), SearchMode::RU);
        assert_eq!(solver.metric(), Metric::Fifth);
        assert_eq!(solver.max_depth(), 12);
    }

    #[test]
    fn test_move_table_building() {
        let mut solver = Solver::new();
        solver.build_moves_table();
        assert!(!solver.moves.is_empty());
        assert!(!solver.first_moves.is_empty());
    }

    #[test]
    fn test_llminx_moves() {
        let mut minx = LLMinx::new();
        let goal = LLMinx::new();

        minx.apply_move(Move::R);
        assert!(!minx.state_equals(&goal));

        minx.apply_move(Move::Ri);
        assert!(minx.state_equals(&goal));
    }

    #[test]
    fn test_undo_move() {
        let mut minx = LLMinx::new();
        let goal = LLMinx::new();

        minx.apply_move(Move::R);
        minx.apply_move(Move::U);
        assert_eq!(minx.depth(), 2);

        minx.undo_move();
        assert_eq!(minx.depth(), 1);

        minx.undo_move();
        assert_eq!(minx.depth(), 0);
        assert!(minx.state_equals(&goal));
    }

    #[test]
    fn test_move_sequences() {
        let mut minx = LLMinx::new();
        let goal = LLMinx::new();

        for m in &[Move::R, Move::U, Move::Ri, Move::Ui] {
            minx.apply_move(*m);
        }

        assert_eq!(minx.depth(), 4);
        assert!(!minx.state_equals(&goal));
    }
}
