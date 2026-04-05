use serde::{Deserialize, Serialize};

// Two separate full-text strings required by StandaloneDiffEditor (via DiffEditorModel).
// Do NOT return a unified diff — the Monaco diff editor takes original/modified text, not a patch.
#[derive(Serialize, Deserialize)]
pub struct FileDiffResult {
    pub original: String, // file content before changes (from HEAD or index)
    pub modified: String, // file content after changes (from index or working tree)
}
