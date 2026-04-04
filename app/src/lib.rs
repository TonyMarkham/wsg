mod commands;
pub mod error;

// ---------------------------------------------------------------------------------------------- //

pub use error::{AppError, AppResult};

// ---------------------------------------------------------------------------------------------- //

use tauri::{
    generate_handler, generate_context
};

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![])
        .run(generate_context!())
        .expect("error while running wsg");
}
