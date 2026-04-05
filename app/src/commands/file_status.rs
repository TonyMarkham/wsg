use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileStatus {
    pub path: String,
    pub status_code: String,
}
