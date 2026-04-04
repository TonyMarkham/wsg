use error_location::ErrorLocation;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("App error: {message} {location}")]
    Generic {
        message: String,
        location: ErrorLocation,
    },
}

pub type AppResult<T> = StdResult<T, AppError>;