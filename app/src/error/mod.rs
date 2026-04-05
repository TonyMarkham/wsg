use error_location::ErrorLocation;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Git command failed: {message} {location}")]
    GitCommand {
        message: String,
        location: ErrorLocation,
    },

    #[error("WSL error: {message} {location}")]
    Wsl {
        message: String,
        location: ErrorLocation,
    },

    #[error("IO error: {source} {location}")]
    Io {
        #[source]
        source: std::io::Error,
        location: ErrorLocation,
    },

    #[error("{message} {location}")]
    Generic {
        message: String,
        location: ErrorLocation,
    },
}

// Required for Tauri commands — serializes error message to the frontend
impl serde::Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = StdResult<T, AppError>;
