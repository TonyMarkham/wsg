use crate::{
    AppError, AppResult,
    commands::{FileDiffResult, FileStatus, GitStatusResult},
};
use error_location::ErrorLocation;
use std::panic::Location;
use std::path::Path;
use tauri_plugin_dialog::DialogExt;

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
    let output = std::process::Command::new("wsl")
        .args(["--list", "--quiet"])
        .output()
        .map_err(|e| AppError::Io {
            source: e,
            location: ErrorLocation::from(Location::caller()),
        })?;

    let bytes = output.stdout;
    let words: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();
    let raw = String::from_utf16_lossy(&words);

    let distros = raw
        .split('\n')
        .map(|s| s.trim().trim_start_matches('\u{FEFF}').to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(distros)
}

#[tauri::command]
pub async fn pick_folder(
    app: tauri::AppHandle,
    start_path: Option<String>,
) -> AppResult<Option<String>> {
    let mut builder = app.dialog().file();
    if let Some(path) = start_path {
        builder = builder.set_directory(path);
    }
    let result = builder.blocking_pick_folder();
    Ok(result.map(|p| p.to_string()))
}

#[tauri::command]
pub fn validate_git_repo(distro: String, repo_path: String) -> AppResult<bool> {
    let output = wsl_git(&distro, &repo_path)
        .args(["rev-parse", "--git-dir"])
        .output()
        .map_err(|e| AppError::Io {
            source: e,
            location: ErrorLocation::from(Location::caller()),
        })?;

    Ok(output.status.success())
}

#[tauri::command]
pub fn get_git_status(distro: String, repo_path: String) -> AppResult<GitStatusResult> {
    let branch_output = wsl_git(&distro, &repo_path)
        .args(["branch", "--show-current"])
        .output()
        .map_err(|e| AppError::Io {
            source: e,
            location: ErrorLocation::from(Location::caller()),
        })?;

    let branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    let status_output = wsl_git(&distro, &repo_path)
        .args(["status", "--porcelain=v1"])
        .output()
        .map_err(|e| AppError::Io {
            source: e,
            location: ErrorLocation::from(Location::caller()),
        })?;

    let mut staged = Vec::new();
    let mut unstaged = Vec::new();

    for line in String::from_utf8_lossy(&status_output.stdout).lines() {
        if line.len() < 3 {
            continue;
        }
        let x = line.chars().next().unwrap_or(' ');
        let y = line.chars().nth(1).unwrap_or(' ');
        let path = line[3..].to_string();

        if x != ' ' && x != '?' {
            staged.push(FileStatus {
                path: path.clone(),
                status_code: x.to_string(),
            });
        }
        if y != ' ' && y != '?' {
            unstaged.push(FileStatus {
                path: path.clone(),
                status_code: y.to_string(),
            });
        }
    }

    Ok(GitStatusResult {
        branch,
        staged,
        unstaged,
    })
}

#[tauri::command]
pub fn get_file_diff(
    distro: String,
    repo_path: String,
    file_path: String,
    staged: bool,
) -> AppResult<FileDiffResult> {
    let head_output = wsl_git(&distro, &repo_path)
        .args(["show", &format!("HEAD:{file_path}")])
        .output()
        .map_err(|e| AppError::Io {
            source: e,
            location: ErrorLocation::from(Location::caller()),
        })?;

    let original = if head_output.status.success() {
        String::from_utf8_lossy(&head_output.stdout).into_owned()
    } else {
        String::new()
    };

    let modified = if staged {
        let index_output = wsl_git(&distro, &repo_path)
            .args(["show", &format!(":{file_path}")])
            .output()
            .map_err(|e| AppError::Io {
                source: e,
                location: ErrorLocation::from(Location::caller()),
            })?;
        String::from_utf8_lossy(&index_output.stdout).into_owned()
    } else if distro.is_empty() {
        std::fs::read_to_string(Path::new(&repo_path).join(&file_path)).map_err(|e| {
            AppError::Io {
                source: e,
                location: ErrorLocation::from(Location::caller()),
            }
        })?
    } else {
        let cat_output = std::process::Command::new("wsl")
            .args(["-d", &distro, "cat", &format!("{repo_path}/{file_path}")])
            .output()
            .map_err(|e| AppError::Io {
                source: e,
                location: ErrorLocation::from(Location::caller()),
            })?;
        String::from_utf8_lossy(&cat_output.stdout).into_owned()
    };

    Ok(FileDiffResult { original, modified })
}

#[tauri::command]
pub fn stage_file(distro: String, repo_path: String, file_path: String) -> AppResult<()> {
    wsl_git(&distro, &repo_path)
        .args(["add", "--", &file_path])
        .output()
        .map_err(|e| AppError::Io {
            source: e,
            location: ErrorLocation::from(Location::caller()),
        })?;

    Ok(())
}

#[tauri::command]
pub fn unstage_file(distro: String, repo_path: String, file_path: String) -> AppResult<()> {
    wsl_git(&distro, &repo_path)
        .args(["restore", "--staged", "--", &file_path])
        .output()
        .map_err(|e| AppError::Io {
            source: e,
            location: ErrorLocation::from(Location::caller()),
        })?;

    Ok(())
}

#[tauri::command]
pub fn commit_changes(
    distro: String,
    repo_path: String,
    subject: String,
    description: String,
    amend: bool,
) -> AppResult<()> {
    let message = if description.is_empty() {
        subject
    } else {
        format!("{subject}\n\n{description}")
    };

    let mut cmd = wsl_git(&distro, &repo_path);
    cmd.args(["commit", "-m", &message]);
    if amend {
        cmd.arg("--amend");
    }
    cmd.output().map_err(|e| AppError::Io {
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })?;

    Ok(())
}
