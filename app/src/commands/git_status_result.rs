use crate::commands::FileStatus;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GitStatusResult {
    pub branch: String,
    pub unstaged: Vec<FileStatus>,
    pub staged: Vec<FileStatus>,
}
