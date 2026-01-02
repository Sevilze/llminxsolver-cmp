use crate::memory_config::{MemoryConfig, MemoryTracker};
use crate::minx::{LLMinx, Move, NUM_CORNERS, NUM_EDGES};
use crate::pruner::Pruner;
use crate::search_mode::{Metric, SearchMode};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

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
    MemoryWarning,
    MemoryExceeded,
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

pub type StatusCallback = Arc<dyn Fn(StatusEvent) + Send + Sync>;

struct SearchContext<'a> {
    tables: &'a [Arc<Vec<u8>>],
    pruners: &'a [&'a dyn Pruner],
    first_moves: &'a [Move],
    next_siblings: &'a [Vec<Option<Move>>],
    interrupted: &'a Arc<AtomicBool>,
    solution_tx: &'a crossbeam_channel::Sender<String>,
    status_tx: &'a crossbeam_channel::Sender<StatusEvent>,
}

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
    memory_config: MemoryConfig,
    pruners: Vec<Box<dyn Pruner>>,
    tables: Vec<Arc<Vec<u8>>>,
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
        Self::with_parallel_config(search_mode, max_depth, MemoryConfig::default())
    }

    pub fn with_parallel_config(
        search_mode: SearchMode,
        max_depth: usize,
        memory_config: MemoryConfig,
    ) -> Self {
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
            memory_config,
            pruners: Vec::new(),
            tables: Vec::new(),
            moves: Vec::new(),
            first_moves: Vec::new(),
            next_siblings: Vec::new(),
            last_search_mode: None,
            last_metric: None,
        }
    }

    pub fn memory_config(&self) -> &MemoryConfig {
        &self.memory_config
    }

    pub fn set_memory_config(&mut self, config: MemoryConfig) {
        self.memory_config = config;
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
        self.status_callback = Some(Arc::new(callback));
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
        let num_threads = self.memory_config.search_threads;
        let start_time = std::time::Instant::now();
        self.interrupted.store(false, Ordering::SeqCst);

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
            false, false, false, false,
        ];
        const IGNORE_EDGE_5: [bool; NUM_EDGES] = [
            true, true, true, true, true, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
        ];

        let mut start = self.start.clone();
        if self.ignore_corner_positions {
            start.set_ignore_corner_positions(IGNORE_CORNER_5);
        }
        if self.ignore_edge_positions {
            start.set_ignore_edge_positions(IGNORE_EDGE_5);
        }
        if self.ignore_corner_orientations {
            start.set_ignore_corner_orientations(IGNORE_CORNER_5);
        }
        if self.ignore_edge_orientations {
            start.set_ignore_edge_orientations(IGNORE_EDGE_5);
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

        if self.is_interrupted() {
            return Vec::new();
        }

        self.fire_event(StatusEvent::new(
            StatusEventType::StartSearch,
            &format!("Searching with {} threads...", num_threads),
            0.0,
        ));

        let max_search_depth = if self.limit_depth {
            self.max_depth
        } else {
            127
        };

        let (solution_tx, solution_rx) = crossbeam_channel::unbounded::<String>();
        let (status_tx, status_rx) = crossbeam_channel::unbounded::<StatusEvent>();

        let status_callback_clone = self.status_callback.clone();
        let status_thread = std::thread::spawn(move || {
            for event in status_rx.iter() {
                if let Some(ref callback) = status_callback_clone {
                    callback(event);
                }
            }
        });

        let tables: Vec<Arc<Vec<u8>>> = used_pruners.iter().map(|(t, _)| Arc::clone(t)).collect();
        let pruner_indices: Vec<usize> = self
            .pruners
            .iter()
            .enumerate()
            .filter(|(_, pruner)| {
                let dominated = (pruner.uses_corner_permutation() && self.ignore_corner_positions)
                    || (pruner.uses_edge_permutation() && self.ignore_edge_positions)
                    || (pruner.uses_corner_orientation() && self.ignore_corner_orientations)
                    || (pruner.uses_edge_orientation() && self.ignore_edge_orientations);
                !dominated
            })
            .map(|(i, _)| i)
            .collect();

        let moves = self.moves.clone();
        let first_moves = self.first_moves.clone();
        let next_siblings = self.next_siblings.clone();
        let interrupted = Arc::clone(&self.interrupted);

        let search_mode = self.search_mode;

        for depth in 1..=max_search_depth {
            if interrupted.load(Ordering::SeqCst) {
                break;
            }

            let depth_start_time = std::time::Instant::now();

            self.fire_event(StatusEvent::new(
                StatusEventType::StartDepth,
                &format!("Searching depth {} ({} threads)...", depth, num_threads),
                0.0,
            ));

            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .unwrap();

            let moves_clone = moves.clone();
            let first_moves_clone = first_moves.clone();
            let next_siblings_clone = next_siblings.clone();
            let tables_clone = tables.clone();
            let pruner_indices_clone = pruner_indices.clone();
            let start_clone = start.clone();
            let goal_clone = goal.clone();
            let interrupted_clone = Arc::clone(&interrupted);
            let solution_tx_clone = solution_tx.clone();
            let search_mode_clone = search_mode;

            let status_tx_clone = status_tx.clone();

            pool.install(|| {
                moves_clone.par_iter().for_each(|&first_move| {
                    if interrupted_clone.load(Ordering::Relaxed) {
                        return;
                    }

                    let mut minx = start_clone.clone();
                    minx.apply_move(first_move);

                    let all_pruners = search_mode_clone.create_pruners();
                    let local_pruners: Vec<&dyn Pruner> = pruner_indices_clone
                        .iter()
                        .filter_map(|&i| all_pruners.get(i).map(|p| p.as_ref()))
                        .collect();

                    let ctx = SearchContext {
                        tables: &tables_clone,
                        pruners: &local_pruners,
                        first_moves: &first_moves_clone,
                        next_siblings: &next_siblings_clone,
                        interrupted: &interrupted_clone,
                        solution_tx: &solution_tx_clone,
                        status_tx: &status_tx_clone,
                    };

                    Self::search_branch(&mut minx, &goal_clone, depth, &ctx);
                });
            });
            let depth_elapsed = depth_start_time.elapsed().as_secs_f64();

            self.fire_event(StatusEvent::new(
                StatusEventType::EndDepth,
                &format!("Finished depth {} in {:.1}s", depth, depth_elapsed),
                1.0,
            ));
        }

        drop(solution_tx);
        drop(status_tx);

        // Wait for the status thread to finish processing all events
        let _ = status_thread.join();

        let solutions: Vec<String> = solution_rx.iter().collect();

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

    fn search_branch(minx: &mut LLMinx, goal: &LLMinx, target_depth: usize, ctx: &SearchContext) {
        let mut stop = false;

        while !stop && !ctx.interrupted.load(Ordering::Relaxed) {
            let levels_left = target_depth.saturating_sub(minx.depth());

            if minx.state_equals(goal) {
                if levels_left == 0 && Self::check_optimal(minx) {
                    let msg = format!(
                        "{} ({},{})",
                        minx.get_generating_moves(),
                        minx.get_ftm_length(),
                        minx.get_fftm_length()
                    );
                    let _ = ctx.solution_tx.send(msg.clone());
                    let _ = ctx.status_tx.send(StatusEvent::new(
                        StatusEventType::SolutionFound,
                        &msg,
                        0.0,
                    ));
                }
                stop = Self::back_track(minx, ctx.next_siblings);
            } else if levels_left > 0 {
                let mut pruned = false;
                for (table_idx, pruner) in ctx.pruners.iter().enumerate() {
                    if let Some(table) = ctx.tables.get(table_idx) {
                        let coord = pruner.get_coordinate(minx);
                        if coord < table.len() && table[coord] as usize > levels_left {
                            pruned = true;
                            break;
                        }
                    }
                }

                if !pruned {
                    stop = Self::next_node(minx, target_depth, ctx.first_moves, ctx.next_siblings);
                } else {
                    stop = Self::back_track(minx, ctx.next_siblings);
                }
            } else {
                stop = Self::next_node(minx, target_depth, ctx.first_moves, ctx.next_siblings);
            }
        }
    }

    fn next_node(
        minx: &mut LLMinx,
        target_depth: usize,
        first_moves: &[Move],
        next_siblings: &[Vec<Option<Move>>],
    ) -> bool {
        if minx.depth() < target_depth {
            let last_move_index = minx.last_move().map(|m| m as usize + 1).unwrap_or(0);
            if last_move_index < first_moves.len() {
                let first = first_moves[last_move_index];
                minx.apply_move(first);
                false
            } else {
                true
            }
        } else {
            Self::back_track(minx, next_siblings)
        }
    }

    fn back_track(minx: &mut LLMinx, next_siblings: &[Vec<Option<Move>>]) -> bool {
        if minx.depth() <= 1 {
            return true;
        }

        let sibling = minx.undo_move();
        let Some(sibling) = sibling else {
            return true;
        };

        let last_move = minx.last_move();
        let last_move_index = last_move.map(|m| m as usize + 1).unwrap_or(0);

        if last_move_index >= next_siblings.len() {
            return true;
        }

        let sibling_idx = sibling as usize;
        let mut next_sibling = if sibling_idx < next_siblings[last_move_index].len() {
            next_siblings[last_move_index][sibling_idx]
        } else {
            None
        };

        while last_move.is_some() && next_sibling.is_none() && minx.depth() > 1 {
            let Some(s) = minx.undo_move() else {
                return true;
            };
            let lm = minx.last_move();
            let lm_index = lm.map(|m| m as usize + 1).unwrap_or(0);
            if lm_index < next_siblings.len() && (s as usize) < next_siblings[lm_index].len() {
                next_sibling = next_siblings[lm_index][s as usize];
            } else {
                next_sibling = None;
            }

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

        let max_move_id = Move::D2i as usize + 2;
        self.first_moves = vec![self.moves[0]; max_move_id];
        self.next_siblings = vec![vec![None; Move::D2i as usize + 1]; max_move_id];

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
        let memory_tracker = MemoryTracker::from_config(&self.memory_config);

        let total_estimated: usize = self.pruners.iter().map(|p| p.table_size()).sum();
        self.fire_event(StatusEvent::new(
            StatusEventType::Message,
            &format!(
                "Estimated memory: {} MB (budget: {} MB)",
                total_estimated / (1024 * 1024),
                self.memory_config.budget_mb()
            ),
            0.0,
        ));

        if total_estimated > self.memory_config.total_budget_bytes {
            self.fire_event(StatusEvent::new(
                StatusEventType::MemoryWarning,
                &format!(
                    "Warning: Estimated {} MB exceeds {} MB budget",
                    total_estimated / (1024 * 1024),
                    self.memory_config.budget_mb()
                ),
                0.0,
            ));
        }

        for (idx, pruner) in self.pruners.iter().enumerate() {
            if self.is_interrupted() {
                break;
            }

            let table_size_bytes = pruner.table_size();
            let progress = idx as f64 / self.pruners.len() as f64;

            if !memory_tracker.can_allocate(table_size_bytes) {
                self.fire_event(StatusEvent::new(
                    StatusEventType::MemoryExceeded,
                    &format!(
                        "Cannot allocate {} MB for {} (used: {} MB, budget: {} MB). Stopping table generation.",
                        table_size_bytes / (1024 * 1024),
                        pruner.name(),
                        memory_tracker.used_mb(),
                        memory_tracker.budget_mb()
                    ),
                    progress,
                ));
                self.interrupt();
                break;
            }

            self.fire_event(StatusEvent::new(
                StatusEventType::Message,
                &format!(
                    "Initializing pruning table {}... ({} MB, {} MB remaining)",
                    pruner.name(),
                    table_size_bytes / (1024 * 1024),
                    memory_tracker.remaining_mb()
                ),
                progress,
            ));

            if pruner.is_precomputed(self.metric) {
                self.fire_event(StatusEvent::new(
                    StatusEventType::Message,
                    "Reading pruning table from disk...",
                    progress,
                ));
                if let Some(table) = pruner.load_table(self.metric) {
                    memory_tracker.allocate(table.len());
                    self.tables.push(Arc::new(table));
                } else {
                    let table = self.build_pruning_table(pruner.as_ref());
                    memory_tracker.allocate(table.len());
                    self.tables.push(Arc::new(table));
                }
            } else {
                self.fire_event(StatusEvent::new(
                    StatusEventType::StartBuildingTable,
                    &format!("Building pruning table {}...", pruner.name()),
                    progress,
                ));

                let table = self.build_pruning_table(pruner.as_ref());
                memory_tracker.allocate(table.len());

                if memory_tracker.is_at_warning_threshold() {
                    self.fire_event(StatusEvent::new(
                        StatusEventType::MemoryWarning,
                        &format!(
                            "Memory usage at {:.1}% of budget",
                            memory_tracker.usage_percentage()
                        ),
                        progress,
                    ));
                }

                if !self.is_interrupted() {
                    self.fire_event(StatusEvent::new(
                        StatusEventType::Message,
                        "Writing table to disk...",
                        progress,
                    ));
                    pruner.save_table(&table, self.metric);
                }

                self.fire_event(StatusEvent::new(
                    StatusEventType::EndBuildingTable,
                    &format!("Finished building {}...", pruner.name()),
                    progress,
                ));

                self.tables.push(Arc::new(table));
            }
        }
    }

    fn build_pruning_table(&self, pruner: &dyn Pruner) -> Vec<u8> {
        let num_threads = self.memory_config.table_generation_threads;

        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .map(|pool| pool.install(|| self.build_pruning_table_internal(pruner)))
            .unwrap_or_else(|_| self.build_pruning_table_internal(pruner))
    }

    fn build_pruning_table_internal(&self, pruner: &dyn Pruner) -> Vec<u8> {
        let table_size = pruner.table_size();
        let table: Vec<AtomicU8> = (0..table_size).map(|_| AtomicU8::new(u8::MAX)).collect();

        let minx = LLMinx::new();
        let coord = pruner.get_coordinate(&minx);
        table[coord].store(0, Ordering::Relaxed);

        let mut total_nodes = 1usize;
        let mut prev_depth_count = 1usize;
        let mut depth: u8 = 0;

        while prev_depth_count > 0 && !self.is_interrupted() {
            self.fire_event(StatusEvent::new(
                StatusEventType::Message,
                &format!("Depth {}: {}", depth, prev_depth_count),
                total_nodes as f64 / table_size as f64,
            ));

            let forward_search = prev_depth_count < table_size - total_nodes;
            let next_depth = depth + 1;

            let new_count = if forward_search {
                let interrupted = &self.interrupted;
                let moves = &self.moves;

                (0..table_size)
                    .into_par_iter()
                    .fold(
                        || (LLMinx::new(), 0usize),
                        |(mut local_minx, mut count), i| {
                            if interrupted.load(Ordering::Relaxed) {
                                return (local_minx, count);
                            }

                            if table[i].load(Ordering::Relaxed) == depth {
                                pruner.set_minx(i, &mut local_minx);

                                for &m in moves {
                                    local_minx.apply_move(m);
                                    let new_coord = pruner.get_coordinate(&local_minx);
                                    if table[new_coord]
                                        .compare_exchange(
                                            u8::MAX,
                                            next_depth,
                                            Ordering::Relaxed,
                                            Ordering::Relaxed,
                                        )
                                        .is_ok()
                                    {
                                        count += 1;
                                    }
                                    local_minx.undo_move();
                                }
                            }
                            (local_minx, count)
                        },
                    )
                    .map(|(_, count)| count)
                    .sum::<usize>()
            } else {
                let interrupted = &self.interrupted;
                let moves = &self.moves;

                (0..table_size)
                    .into_par_iter()
                    .fold(
                        || (LLMinx::new(), 0usize),
                        |(mut local_minx, mut count), i| {
                            if interrupted.load(Ordering::Relaxed) {
                                return (local_minx, count);
                            }

                            if table[i].load(Ordering::Relaxed) == u8::MAX {
                                pruner.set_minx(i, &mut local_minx);

                                for &m in moves {
                                    local_minx.apply_move(m);
                                    let new_coord = pruner.get_coordinate(&local_minx);
                                    if table[new_coord].load(Ordering::Relaxed) == depth {
                                        table[i].store(next_depth, Ordering::Relaxed);
                                        count += 1;
                                        break;
                                    }
                                    local_minx.undo_move();
                                }
                            }
                            (local_minx, count)
                        },
                    )
                    .map(|(_, count)| count)
                    .sum::<usize>()
            };

            total_nodes += new_count;
            prev_depth_count = new_count;
            depth += 1;
        }

        table
            .into_iter()
            .map(|a| a.load(Ordering::Relaxed))
            .collect()
    }

    fn filter_pruning_tables(&self) -> Vec<(Arc<Vec<u8>>, &dyn Pruner)> {
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
            .map(|(i, pruner)| (Arc::clone(&self.tables[i]), pruner.as_ref()))
            .collect()
    }

    pub fn get_tables(&self) -> &[Arc<Vec<u8>>] {
        &self.tables
    }

    pub fn get_pruners(&self) -> &[Box<dyn Pruner>] {
        &self.pruners
    }

    pub fn get_moves(&self) -> &[Move] {
        &self.moves
    }

    pub fn get_first_moves(&self) -> &[Move] {
        &self.first_moves
    }

    pub fn get_next_siblings(&self) -> &[Vec<Option<Move>>] {
        &self.next_siblings
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
                let bl_face = Move::bL.face();

                if (face_i == l_face && face_prev == r_face)
                    || (face_i == r_face && face_prev == l_face)
                    || (face_i == bl_face && face_prev == f_face)
                    || (face_i == f_face && face_prev == bl_face)
                {
                    return false;
                }
            }
        }
        true
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

    #[test]
    fn test_br_moves_inverse() {
        let mut minx = LLMinx::new();
        let goal = LLMinx::new();

        minx.apply_move(Move::bR);
        assert!(!minx.state_equals(&goal));

        minx.apply_move(Move::bRi);
        assert!(minx.state_equals(&goal));
    }

    #[test]
    fn test_br2_moves_inverse() {
        let mut minx = LLMinx::new();
        let goal = LLMinx::new();

        minx.apply_move(Move::bR2);
        assert!(!minx.state_equals(&goal));

        minx.apply_move(Move::bR2i);
        assert!(minx.state_equals(&goal));
    }
}
