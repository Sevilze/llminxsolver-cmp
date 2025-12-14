mod commands;

use commands::SolverHandle;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(SolverHandle::default())
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::solve,
            commands::cancel_solve
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
