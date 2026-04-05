use crate::{
    AppResult,
    commands::{FileDiffResult, GitStatusResult},
};

/// Builds a `wsl [-d <distro>] git -C <repo_path>` command base.
/// Empty distro = Windows repo, uses git directly instead of wsl.
fn wsl_git(distro: &str, repo_path: &str) -> std::process::Command {
    if distro.is_empty() {
        let mut cmd = std::process::Command::new("git");
        cmd.args(["-C", repo_path]);
        cmd
    } else {
        let mut cmd = std::process::Command::new("wsl");
        cmd.args(["-d", distro, "git", "-C", repo_path]);
        cmd
    }
}

#[tauri::command]
pub fn get_wsl_distros() -> AppResult<Vec<String>> {
    // wsl --list --quiet → collect stdout bytes → decode UTF-16 LE
    // wsl.exe on Windows always outputs UTF-16 LE with BOM regardless of system locale —
    // do NOT use String::from_utf8, it will produce garbage.
    // Use String::from_utf16_lossy after reinterpreting the byte slice as a &[u16]:
    //   let bytes = output.stdout;
    //   let words: Vec<u16> = bytes.chunks_exact(2).map(|b| u16::from_le_bytes([b[0], b[1]])).collect();
    //   let raw = String::from_utf16_lossy(&words);
    // Then split on '\n', trim each line, strip BOM (U+FEFF), filter empty lines.
    todo!()
}

#[tauri::command]
pub async fn pick_folder(
    app: tauri::AppHandle,
    start_path: Option<String>,
) -> AppResult<Option<String>> {
    // Use blocking_pick_folder (safe on async commands — runs off the main thread):
    //   use tauri_plugin_dialog::DialogExt;
    //   let mut builder = app.dialog().file();
    //   if let Some(path) = start_path { builder = builder.set_directory(path); }
    //   let result = builder.blocking_pick_folder();
    //   Ok(result.map(|p| p.to_string()))
    // Returns None if user cancels
    todo!()
}

#[tauri::command]
pub fn validate_git_repo(distro: String, repo_path: String) -> AppResult<bool> {
    // wsl_git rev-parse --git-dir
    // Ok(true) if exit 0, Ok(false) if non-zero, Err(AppError::Io) on IO failure
    todo!()
}

#[tauri::command]
pub fn get_git_status(distro: String, repo_path: String) -> AppResult<GitStatusResult> {
    // wsl_git branch --show-current → branch name
    // wsl_git status --porcelain=v1 → parse XY lines
    // X (col 0) = index/staged, Y (col 1) = worktree/unstaged, ' ' = unmodified
    todo!()
}

#[tauri::command]
pub fn get_file_diff(
    distro: String,
    repo_path: String,
    file_path: String,
    staged: bool,
) -> AppResult<FileDiffResult> {
    // Returns original and modified file content as separate strings for StandaloneDiffEditor.
    // Simplest correct approach for all cases:
    //   original = wsl_git(distro, repo_path) show HEAD:<file>
    //              (returns empty string for new/untracked files — treat non-zero exit as "")
    //   staged=true  → modified = wsl_git(distro, repo_path) show :<file>   (index content)
    //   staged=false → modified = std::fs::read_to_string for Windows repos (distro.is_empty()),
    //                             or Command::new("wsl").args(["-d", distro, "cat", &format!("{repo_path}/{file_path}")])
    //                             for WSL repos (worktree content)
    // Wrap each in String::from_utf8_lossy(&output.stdout).into_owned()
    todo!()
}

#[tauri::command]
pub fn stage_file(distro: String, repo_path: String, file_path: String) -> AppResult<()> {
    // wsl_git add -- <file>
    todo!()
}

#[tauri::command]
pub fn unstage_file(distro: String, repo_path: String, file_path: String) -> AppResult<()> {
    // wsl_git restore --staged -- <file>
    todo!()
}

#[tauri::command]
pub fn commit_changes(
    distro: String,
    repo_path: String,
    subject: String,
    description: String,
    amend: bool,
) -> AppResult<()> {
    // message = if description.is_empty() { subject } else { "{subject}\n\n{description}" }
    // wsl_git commit -m <message> [--amend]
    todo!()
}
