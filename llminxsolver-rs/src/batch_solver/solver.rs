use super::equivalence::EquivalenceHandler;
use super::types::{BatchCaseResult, BatchResults, GeneratedState};
use crate::memory_config::{MemoryConfig, get_current_rss_bytes};
use crate::minx::{LLMinx, Move};
use crate::pruner::Pruner;
use crate::search_mode::{Metric, SearchMode};
use crate::solver::{
    IGNORE_CORNER_5, IGNORE_EDGE_5, Solver, StatusCallback, StatusEvent, StatusEventType,
};
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub type CaseSolvedCallback = Arc<dyn Fn(BatchCaseResult) + Send + Sync>;

const UPPER_BOUND: f64 = 0.90;
const MIN_CONCURRENT_CASES: usize = 1;
const PER_CASE_BASE_BYTES: usize = 4 * 1024;
const PER_THREAD_STACK_BYTES: usize = 2 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct BatchSolverConfig {
    pub search_mode: SearchMode,
    pub metric: Metric,
    pub pruning_depth: u8,
    pub max_search_depth: usize,
    pub stop_after_first: bool,
    pub memory_config: MemoryConfig,
    pub ignore_corner_permutation: bool,
    pub ignore_edge_permutation: bool,
    pub ignore_corner_orientation: bool,
    pub ignore_edge_orientation: bool,
}

impl Default for BatchSolverConfig {
    fn default() -> Self {
        Self {
            search_mode: SearchMode::RU,
            metric: Metric::Fifth,
            pruning_depth: 6,
            max_search_depth: 12,
            stop_after_first: false,
            memory_config: MemoryConfig::default(),
            ignore_corner_permutation: false,
            ignore_edge_permutation: false,
            ignore_corner_orientation: false,
            ignore_edge_orientation: false,
        }
    }
}

struct BatchCase {
    case_number: usize,
    setup_moves: String,
    start: LLMinx,
    goal: LLMinx,
    solved: AtomicBool,
}

unsafe impl Sync for BatchCase {}

struct SearchContext<'a> {
    tables: &'a [Arc<Vec<u8>>],
    pruners: &'a [&'a dyn Pruner],
    first_moves: &'a [Move],
    next_siblings: &'a [Vec<Option<Move>>],
    interrupted: &'a Arc<AtomicBool>,
    solution_tx: &'a crossbeam_channel::Sender<(usize, String)>,
    status_tx: &'a crossbeam_channel::Sender<StatusEvent>,
    case_number: usize,
    case_solved: &'a AtomicBool,
    stop_after_first: bool,
}

pub fn solve_batch_states(
    states: Vec<GeneratedState>,
    config: &BatchSolverConfig,
    equivalence: Option<&Arc<EquivalenceHandler>>,
    interrupt: Arc<AtomicBool>,
    status_callback: Option<StatusCallback>,
    case_solved_callback: Option<CaseSolvedCallback>,
) -> BatchResults {
    let start_time = std::time::Instant::now();
    interrupt.store(false, Ordering::SeqCst);

    let total_cases = states.len();
    if total_cases == 0 {
        fire_event(
            &status_callback,
            StatusEvent::new(StatusEventType::FinishSearch, "No cases to solve.", 1.0),
        );
        return BatchResults::new(0);
    }

    fire_event(
        &status_callback,
        StatusEvent::new(
            StatusEventType::StartSearch,
            &format!(
                "Preparing batch solve for {} cases (memory budget: {} MB)...",
                total_cases,
                config.memory_config.budget_mb()
            ),
            0.0,
        ),
    );

    let mut master_solver = Solver::with_parallel_config(
        config.search_mode,
        config.max_search_depth,
        config.memory_config,
    );
    master_solver.set_metric(config.metric);
    master_solver.set_pruning_depth(config.pruning_depth);
    master_solver.set_limit_search_depth(true);
    master_solver.set_ignore_corner_positions(config.ignore_corner_permutation);
    master_solver.set_ignore_edge_positions(config.ignore_edge_permutation);
    master_solver.set_ignore_corner_orientations(config.ignore_corner_orientation);
    master_solver.set_ignore_edge_orientations(config.ignore_edge_orientation);

    if let Some(ref cb) = status_callback {
        let cb_clone = Arc::clone(cb);
        master_solver.set_status_callback(move |event| {
            cb_clone(event);
        });
    }

    master_solver.set_start(states[0].state.clone());
    master_solver.prepare_tables();

    if interrupt.load(Ordering::SeqCst) {
        return BatchResults::new(total_cases);
    }

    let cases = build_cases(&states, config, equivalence);

    let used_pruners = filter_pruning_tables(&master_solver, config);
    let table_memory_bytes: usize = used_pruners.iter().map(|(t, _)| t.len()).sum();
    let tables: Vec<Arc<Vec<u8>>> = used_pruners.iter().map(|(t, _)| Arc::clone(t)).collect();
    let pruner_indices: Vec<usize> = master_solver
        .get_pruners()
        .iter()
        .enumerate()
        .filter(|(_, pruner)| {
            let dominated = (pruner.uses_corner_permutation() && config.ignore_corner_permutation)
                || (pruner.uses_edge_permutation() && config.ignore_edge_permutation)
                || (pruner.uses_corner_orientation() && config.ignore_corner_orientation)
                || (pruner.uses_edge_orientation() && config.ignore_edge_orientation);
            !dominated
        })
        .map(|(i, _)| i)
        .collect();

    let moves = master_solver.get_moves().to_vec();
    let first_moves = master_solver.get_first_moves().to_vec();
    let next_siblings = master_solver.get_next_siblings().to_vec();
    let num_threads = config.memory_config.search_threads;
    let search_mode = config.search_mode;

    let max_concurrent = calculate_max_concurrent(
        config,
        table_memory_bytes,
        num_threads,
        moves.len(),
        total_cases,
    );

    fire_event(
        &status_callback,
        StatusEvent::new(
            StatusEventType::Message,
            &format!(
                "Starting batch search: {} cases, {} concurrent (tables: {} MB)",
                cases.len(),
                max_concurrent,
                table_memory_bytes / (1024 * 1024),
            ),
            0.05,
        ),
    );

    let (solution_tx, solution_rx) = crossbeam_channel::unbounded::<(usize, String)>();

    let case_solutions: Arc<Mutex<std::collections::HashMap<usize, Vec<String>>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));
    let case_solutions_for_thread = Arc::clone(&case_solutions);
    let case_solved_cb_clone = case_solved_callback.clone();
    let status_callback_for_sol = status_callback.clone();
    let start_time_for_sol = start_time;

    let cases_for_sol: Arc<Vec<(usize, String)>> = Arc::new(
        cases
            .iter()
            .map(|c| (c.case_number, c.setup_moves.clone()))
            .collect(),
    );

    let notified_cases: Arc<Mutex<std::collections::HashSet<usize>>> =
        Arc::new(Mutex::new(std::collections::HashSet::new()));
    let notified_for_thread = Arc::clone(&notified_cases);

    let solution_thread = std::thread::spawn(move || {
        for (case_number, solution) in solution_rx.iter() {
            let elapsed = start_time_for_sol.elapsed().as_secs_f64();

            {
                let mut sols = case_solutions_for_thread.lock().unwrap();
                sols.entry(case_number).or_default().push(solution.clone());
            }

            fire_event(
                &status_callback_for_sol,
                StatusEvent::new(
                    StatusEventType::SolutionFound,
                    &format!("Case {}: {}", case_number, solution),
                    0.0,
                ),
            );

            if let Some(ref cb) = case_solved_cb_clone {
                let sols = case_solutions_for_thread.lock().unwrap();
                let solutions = sols.get(&case_number).cloned().unwrap_or_default();
                let setup_moves = cases_for_sol
                    .iter()
                    .find(|(cn, _)| *cn == case_number)
                    .map(|(_, s)| s.clone())
                    .unwrap_or_default();
                drop(sols);

                let mut result = BatchCaseResult::new(case_number, setup_moves);
                result.best_solution = solutions.first().cloned();
                result.solutions = solutions;
                result.solve_time = elapsed;

                notified_for_thread.lock().unwrap().insert(case_number);
                cb(result);
            }
        }
    });

    let (status_tx, status_rx) = crossbeam_channel::unbounded::<StatusEvent>();
    let status_callback_clone = status_callback.clone();
    let status_thread = std::thread::spawn(move || {
        for event in status_rx.iter() {
            if let Some(ref callback) = status_callback_clone {
                callback(event);
            }
        }
    });

    let max_search_depth = config.max_search_depth;
    let mut current_max_concurrent = max_concurrent;
    let mut results = BatchResults::new(total_cases);

    for depth in 1..=max_search_depth {
        if interrupt.load(Ordering::SeqCst) {
            break;
        }

        if config.stop_after_first && cases.iter().all(|c| c.solved.load(Ordering::Relaxed)) {
            break;
        }

        let active_case_indices: Vec<usize> = cases
            .iter()
            .enumerate()
            .filter(|(_, c)| !config.stop_after_first || !c.solved.load(Ordering::Relaxed))
            .map(|(i, _)| i)
            .collect();

        if active_case_indices.is_empty() {
            break;
        }

        let num_batchs = active_case_indices.len().div_ceil(current_max_concurrent);

        fire_event(
            &status_callback,
            StatusEvent::with_context(
                StatusEventType::StartDepth,
                &format!(
                    "Depth {}: {} active cases, {} concurrent ({} batchs)",
                    depth,
                    active_case_indices.len(),
                    current_max_concurrent,
                    num_batchs,
                ),
                0.0,
                None,
                depth as u32,
            ),
        );

        let rss_before = get_current_rss_bytes();
        let depth_start_time = std::time::Instant::now();

        for (batch_idx, batch_chunk) in active_case_indices
            .chunks(current_max_concurrent)
            .enumerate()
        {
            if interrupt.load(Ordering::SeqCst) {
                break;
            }

            let batch_active: Vec<usize> = batch_chunk
                .iter()
                .copied()
                .filter(|&i| !config.stop_after_first || !cases[i].solved.load(Ordering::Relaxed))
                .collect();

            if batch_active.is_empty() {
                continue;
            }

            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .unwrap();

            let moves_clone = moves.clone();
            let first_moves_clone = first_moves.clone();
            let next_siblings_clone = next_siblings.clone();
            let tables_clone = tables.clone();
            let pruner_indices_clone = pruner_indices.clone();
            let interrupted_clone = Arc::clone(&interrupt);
            let solution_tx_clone = solution_tx.clone();
            let status_tx_clone = status_tx.clone();
            let stop_after_first = config.stop_after_first;

            let total_work = batch_active.len() * moves_clone.len();
            let completed_work = Arc::new(AtomicUsize::new(0));
            let batch_start_time = Arc::new(std::time::Instant::now());

            let _ = status_tx_clone.send(StatusEvent::with_context(
                StatusEventType::Message,
                &format!(
                    "Depth {} - batch {}/{} ({} cases)",
                    depth,
                    batch_idx + 1,
                    num_batchs,
                    batch_active.len(),
                ),
                0.0,
                None,
                depth as u32,
            ));

            pool.install(|| {
                batch_active.par_iter().for_each(|&case_idx| {
                    let case = &cases[case_idx];

                    for &first_move in &moves_clone {
                        if interrupted_clone.load(Ordering::Relaxed) {
                            return;
                        }
                        if stop_after_first && case.solved.load(Ordering::Relaxed) {
                            completed_work.fetch_add(1, Ordering::Relaxed);
                            continue;
                        }

                        let mut minx = case.start.clone();
                        minx.apply_move(first_move);

                        let all_pruners = search_mode.create_pruners();
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
                            case_number: case.case_number,
                            case_solved: &case.solved,
                            stop_after_first,
                        };

                        search_branch(&mut minx, &case.goal, depth, &ctx);

                        let done = completed_work.fetch_add(1, Ordering::Relaxed) + 1;
                        let progress = done as f64 / total_work as f64;
                        let elapsed = batch_start_time.elapsed().as_secs_f64();

                        if done.is_multiple_of(50) || done == total_work {
                            let etr_str = if progress > 0.005 && elapsed > 0.5 {
                                let total_estimated = elapsed / progress;
                                let remaining = total_estimated - elapsed;
                                if remaining < 60.0 {
                                    format!("ETR: {:.1}s", remaining)
                                } else if remaining < 3600.0 {
                                    format!("ETR: {:.1}m", remaining / 60.0)
                                } else {
                                    format!("ETR: {:.1}h", remaining / 3600.0)
                                }
                            } else {
                                "ETR: --".to_string()
                            };

                            let _ = status_tx_clone.send(StatusEvent::with_context(
                                StatusEventType::Message,
                                &format!(
                                    "Depth {} - batch {}/{}... ({})",
                                    depth,
                                    batch_idx + 1,
                                    num_batchs,
                                    etr_str
                                ),
                                progress,
                                None,
                                depth as u32,
                            ));
                        }
                    }
                });
            });
        }

        let depth_elapsed = depth_start_time.elapsed().as_secs_f64();

        fire_event(
            &status_callback,
            StatusEvent::with_context(
                StatusEventType::EndDepth,
                &format!("Finished depth {} in {:.1}s", depth, depth_elapsed),
                1.0,
                None,
                depth as u32,
            ),
        );

        let rss_after = get_current_rss_bytes();
        if rss_before > 0 && rss_after > 0 {
            let budget_bytes =
                (config.memory_config.total_budget_bytes as f64 * UPPER_BOUND) as usize;

            if rss_after > budget_bytes && current_max_concurrent > MIN_CONCURRENT_CASES {
                let ratio = budget_bytes as f64 / rss_after as f64;
                let adjusted =
                    ((current_max_concurrent as f64 * ratio) as usize).max(MIN_CONCURRENT_CASES);
                if adjusted != current_max_concurrent {
                    fire_event(
                        &status_callback,
                        StatusEvent::new(
                            StatusEventType::MemoryWarning,
                            &format!(
                                "Reducing concurrency {} -> {} (RSS: {} MB, budget: {} MB)",
                                current_max_concurrent,
                                adjusted,
                                rss_after / (1024 * 1024),
                                budget_bytes / (1024 * 1024),
                            ),
                            0.0,
                        ),
                    );
                    current_max_concurrent = adjusted;
                }
            } else if rss_after < budget_bytes / 2 && current_max_concurrent < total_cases {
                let headroom = budget_bytes as f64 / rss_after.max(1) as f64;
                let adjusted = ((current_max_concurrent as f64 * headroom * 0.8) as usize)
                    .min(total_cases)
                    .max(current_max_concurrent);
                if adjusted > current_max_concurrent {
                    fire_event(
                        &status_callback,
                        StatusEvent::new(
                            StatusEventType::Message,
                            &format!(
                                "Increasing concurrency {} -> {} (RSS: {} MB, budget: {} MB)",
                                current_max_concurrent,
                                adjusted,
                                rss_after / (1024 * 1024),
                                budget_bytes / (1024 * 1024),
                            ),
                            0.0,
                        ),
                    );
                    current_max_concurrent = adjusted;
                }
            }
        }
    }

    drop(solution_tx);
    drop(status_tx);
    let _ = solution_thread.join();
    let _ = status_thread.join();

    let elapsed = start_time.elapsed().as_secs_f64();
    let already_notified = notified_cases.lock().unwrap().clone();
    let final_solutions = case_solutions.lock().unwrap();

    for case in &cases {
        if already_notified.contains(&case.case_number) {
            let solutions = final_solutions
                .get(&case.case_number)
                .cloned()
                .unwrap_or_default();
            let mut result = BatchCaseResult::new(case.case_number, case.setup_moves.clone());
            result.best_solution = solutions.first().cloned();
            result.solutions = solutions;
            result.solve_time = elapsed;
            results.add_result(result);
            continue;
        }

        let solutions = final_solutions
            .get(&case.case_number)
            .cloned()
            .unwrap_or_default();
        let mut result = BatchCaseResult::new(case.case_number, case.setup_moves.clone());
        result.best_solution = solutions.first().cloned();
        result.solutions = solutions;
        result.solve_time = elapsed;

        if let Some(ref callback) = case_solved_callback {
            callback(result.clone());
        }

        results.add_result(result);
    }

    results.total_time = elapsed;
    results.average_time_per_case = if results.case_results.is_empty() {
        0.0
    } else {
        elapsed / results.case_results.len() as f64
    };

    fire_event(
        &status_callback,
        StatusEvent::new(
            StatusEventType::FinishSearch,
            &format!(
                "Batch solve complete. Solved {}/{} cases in {:.2}s",
                results.solved_cases, total_cases, elapsed
            ),
            1.0,
        ),
    );

    results
}

fn calculate_max_concurrent(
    config: &BatchSolverConfig,
    table_memory_bytes: usize,
    num_threads: usize,
    num_moves: usize,
    total_cases: usize,
) -> usize {
    let budget_bytes = (config.memory_config.total_budget_bytes as f64 * UPPER_BOUND) as usize;
    let search_budget = budget_bytes.saturating_sub(table_memory_bytes);

    let pruner_instance_bytes: usize = config
        .search_mode
        .create_pruners()
        .iter()
        .map(|_| 256)
        .sum::<usize>();

    let per_case_bytes = PER_CASE_BASE_BYTES + (pruner_instance_bytes * num_moves.min(num_threads));

    let thread_overhead = PER_THREAD_STACK_BYTES * num_threads;
    let effective_budget = search_budget.saturating_sub(thread_overhead);

    let max_concurrent = if per_case_bytes > 0 {
        (effective_budget / per_case_bytes).max(MIN_CONCURRENT_CASES)
    } else {
        total_cases
    };

    max_concurrent.min(total_cases).max(MIN_CONCURRENT_CASES)
}

fn build_cases(
    states: &[GeneratedState],
    config: &BatchSolverConfig,
    equivalence: Option<&Arc<EquivalenceHandler>>,
) -> Vec<BatchCase> {
    states
        .iter()
        .map(|generated| {
            let mut start = generated.state.clone();
            start.clear_moves();

            if let Some(equiv) = equivalence {
                equiv.apply_to_state(&mut start);
            }
            if config.ignore_corner_permutation {
                start.set_ignore_corner_positions(IGNORE_CORNER_5);
            }
            if config.ignore_edge_permutation {
                start.set_ignore_edge_positions(IGNORE_EDGE_5);
            }
            if config.ignore_corner_orientation {
                start.set_ignore_corner_orientations(IGNORE_CORNER_5);
            }
            if config.ignore_edge_orientation {
                start.set_ignore_edge_orientations(IGNORE_EDGE_5);
            }

            let mut goal = LLMinx::new();
            if config.ignore_corner_permutation {
                goal.set_ignore_corner_positions(IGNORE_CORNER_5);
            }
            if config.ignore_edge_permutation {
                goal.set_ignore_edge_positions(IGNORE_EDGE_5);
            }
            if config.ignore_corner_orientation {
                goal.set_ignore_corner_orientations(IGNORE_CORNER_5);
            }
            if config.ignore_edge_orientation {
                goal.set_ignore_edge_orientations(IGNORE_EDGE_5);
            }

            BatchCase {
                case_number: generated.case_number,
                setup_moves: generated.setup_moves.clone(),
                start,
                goal,
                solved: AtomicBool::new(false),
            }
        })
        .collect()
}

fn filter_pruning_tables<'a>(
    solver: &'a Solver,
    config: &BatchSolverConfig,
) -> Vec<(Arc<Vec<u8>>, &'a dyn Pruner)> {
    solver
        .get_pruners()
        .iter()
        .enumerate()
        .filter(|(_, pruner)| {
            let dominated = (pruner.uses_corner_permutation() && config.ignore_corner_permutation)
                || (pruner.uses_edge_permutation() && config.ignore_edge_permutation)
                || (pruner.uses_corner_orientation() && config.ignore_corner_orientation)
                || (pruner.uses_edge_orientation() && config.ignore_edge_orientation);
            !dominated
        })
        .filter_map(|(i, pruner)| {
            solver
                .get_tables()
                .get(i)
                .map(|t| (Arc::clone(t), pruner.as_ref()))
        })
        .collect()
}

/// Forward IDA* search identical to `Solver::search_branch`.
fn search_branch(minx: &mut LLMinx, goal: &LLMinx, target_depth: usize, ctx: &SearchContext) {
    let mut stop = false;

    while !stop && !ctx.interrupted.load(Ordering::Relaxed) {
        if ctx.stop_after_first && ctx.case_solved.load(Ordering::Relaxed) {
            return;
        }

        let levels_left = target_depth.saturating_sub(minx.depth());

        if minx.state_equals(goal) {
            if levels_left == 0 && Solver::check_optimal(minx) {
                let msg = format!(
                    "{} ({},{})",
                    minx.get_generating_moves(),
                    minx.get_ftm_length(),
                    minx.get_fftm_length()
                );
                let _ = ctx.solution_tx.send((ctx.case_number, msg.clone()));
                let _ = ctx.status_tx.send(StatusEvent::new(
                    StatusEventType::SolutionFound,
                    &format!("Case {}: {}", ctx.case_number, msg),
                    0.0,
                ));
                ctx.case_solved.store(true, Ordering::SeqCst);
            }
            stop = Solver::back_track(minx, ctx.next_siblings);
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
                stop = Solver::next_node(minx, target_depth, ctx.first_moves, ctx.next_siblings);
            } else {
                stop = Solver::back_track(minx, ctx.next_siblings);
            }
        } else {
            stop = Solver::next_node(minx, target_depth, ctx.first_moves, ctx.next_siblings);
        }
    }
}

fn fire_event(callback: &Option<StatusCallback>, event: StatusEvent) {
    if let Some(cb) = callback {
        cb(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_batch() {
        let config = BatchSolverConfig::default();
        let interrupt = Arc::new(AtomicBool::new(false));
        let results = solve_batch_states(vec![], &config, None, interrupt, None, None);
        assert_eq!(results.total_cases, 0);
    }
}
