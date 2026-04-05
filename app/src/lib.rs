mod commands;
pub mod error;

// ---------------------------------------------------------------------------------------------- //

pub use error::{AppError, AppResult};

// ---------------------------------------------------------------------------------------------- //

use tauri::{generate_context, generate_handler};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(generate_handler![
            commands::git::get_wsl_distros,
            commands::git::pick_folder,
            commands::git::validate_git_repo,
            commands::git::get_git_status,
            commands::git::get_file_diff,
            commands::git::stage_file,
            commands::git::unstage_file,
            commands::git::commit_changes,
        ])
        .run(generate_context!())
        .expect("error while running wsg");
}
