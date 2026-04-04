use error_location::ErrorLocation;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinuxApiError {
    #[error("Linux API error: {message} {location}")]
    Generic {
        message: String,
        location: ErrorLocation,
    },
}

pub type LinuxApiResult<T> = StdResult<T, LinuxApiError>;