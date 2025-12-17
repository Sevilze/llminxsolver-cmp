use llminxsolver_rs::{LLMinx, Metric, SearchMode, Solver, StatusEvent, StatusEventType};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolverConfigDto {
    pub allowed_faces: String,
    pub metric: String,
    pub limit_depth: bool,
    pub max_depth: usize,
    pub ignore_corner_positions: bool,
    pub ignore_edge_positions: bool,
    pub ignore_corner_orientations: bool,
    pub ignore_edge_orientations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MegaminxStateDto {
    pub corner_positions: [u8; 5],
    pub corner_orientations: [u8; 5],
    pub edge_positions: [u8; 5],
    pub edge_orientations: [u8; 5],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEvent {
    pub event_type: String,
    pub message: String,
    pub progress: f64,
}

impl From<StatusEvent> for ProgressEvent {
    fn from(event: StatusEvent) -> Self {
        Self {
            event_type: format!("{:?}", event.event_type),
            message: event.message.clone(),
            progress: event.progress,
        }
    }
}

#[derive(Default)]
pub struct SolverHandle {
    pub interrupt: Arc<Mutex<Option<Arc<AtomicBool>>>>,
}

fn parse_search_mode(allowed_faces: &str) -> SearchMode {
    match allowed_faces {
        "R_U" => SearchMode::RU,
        "R_U_L" => SearchMode::RUL,
        "R_U_F" => SearchMode::RUF,
        "R_U_D" => SearchMode::RUD,
        "R_U_bL" => SearchMode::RUbL,
        "R_U_bR" => SearchMode::RUbR,
        "R_U_L_F" => SearchMode::RUFL,
        "R_U_L_F_bL" => SearchMode::RUFLbL,
        _ => SearchMode::RU,
    }
}

fn parse_metric(metric: &str) -> Metric {
    match metric {
        "FTM" => Metric::Face,
        "FFTM" => Metric::Fifth,
        _ => Metric::Face,
    }
}

fn build_llminx(state: &MegaminxStateDto) -> LLMinx {
    let mut minx = LLMinx::default();

    for i in 0..5 {
        minx.corner_positions_mut()[i] = state.corner_positions[i];
        minx.edge_positions_mut()[i] = state.edge_positions[i];
        minx.set_corner_orientation(i as u8, state.corner_orientations[i]);
        minx.set_edge_orientation(i as u8, state.edge_orientations[i]);
    }

    minx
}

#[tauri::command]
pub async fn solve(
    app: AppHandle,
    solver_handle: State<'_, SolverHandle>,
    config: SolverConfigDto,
    megaminx_state: MegaminxStateDto,
) -> Result<Vec<String>, String> {
    let search_mode = parse_search_mode(&config.allowed_faces);
    let metric = parse_metric(&config.metric);
    let max_depth = if config.limit_depth {
        config.max_depth
    } else {
        50
    };

    let start_state = build_llminx(&megaminx_state);

    let (tx, rx) = std::sync::mpsc::channel::<StatusEvent>();

    let interrupt_flag = Arc::new(AtomicBool::new(false));
    {
        let mut handle = solver_handle.interrupt.lock().map_err(|e| e.to_string())?;
        *handle = Some(Arc::clone(&interrupt_flag));
    }

    let interrupt_clone = Arc::clone(&interrupt_flag);

    let solve_handle = std::thread::spawn(move || {
        let mut solver = Solver::with_config(search_mode, max_depth);
        solver.set_metric(metric);
        solver.set_limit_depth(config.limit_depth);
        solver.set_start(start_state);
        solver.set_ignore_corner_positions(config.ignore_corner_positions);
        solver.set_ignore_edge_positions(config.ignore_edge_positions);
        solver.set_ignore_corner_orientations(config.ignore_corner_orientations);
        solver.set_ignore_edge_orientations(config.ignore_edge_orientations);

        let interrupt_handle = solver.interrupt_handle();
        std::thread::spawn(move || {
            while !interrupt_clone.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            interrupt_handle.store(true, Ordering::SeqCst);
        });

        let tx_clone = tx.clone();
        solver.set_status_callback(move |event: StatusEvent| {
            let _ = tx_clone.send(event);
        });

        solver.solve()
    });

    let app_clone = app.clone();
    std::thread::spawn(move || {
        for event in rx {
            match event.event_type {
                StatusEventType::SolutionFound => {
                    let _ = app_clone.emit("solver:solution", event.message.clone());
                }
                StatusEventType::FinishSearch => {
                    let _ = app_clone.emit("solver:complete", ());
                }
                _ => {
                    let progress_event: ProgressEvent = event.into();
                    let _ = app_clone.emit("solver:progress", progress_event);
                }
            }
        }
    });

    let solutions = solve_handle
        .join()
        .map_err(|_| "Solver thread panicked".to_string())?;

    {
        let mut handle = solver_handle.interrupt.lock().map_err(|e| e.to_string())?;
        *handle = None;
    }

    Ok(solutions)
}

#[tauri::command]
pub fn cancel_solve(solver_handle: State<'_, SolverHandle>) -> Result<(), String> {
    let handle = solver_handle.interrupt.lock().map_err(|e| e.to_string())?;

    if let Some(ref flag) = *handle {
        flag.store(true, Ordering::SeqCst);
    }

    Ok(())
}
