use error_location::ErrorLocation;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowsApiError {
    #[error("Windows API error: {message} {location}")]
    Generic {
        message: String,
        location: ErrorLocation,
    },
}

pub type WindowsApiResult<T> = StdResult<T, WindowsApiError>;