using Microsoft.JSInterop;
using Wsg.Models;

namespace Wsg.Services;

// Tauri v2 with withGlobalTauri: true exposes window.__TAURI__.core.invoke.
// Blazor resolves dot-path identifiers natively — no authored JS wrapper needed.
// Tauri v2 auto-converts Rust snake_case parameter names to camelCase on the JS side.
public class GitService(IJSRuntime js)
{
    private Task<T> Invoke<T>(string cmd, object args) =>
        js.InvokeAsync<T>("__TAURI__.core.invoke", cmd, args).AsTask();

    private Task InvokeVoid(string cmd, object args) =>
        js.InvokeVoidAsync("__TAURI__.core.invoke", cmd, args).AsTask();

    // ── Repo discovery ──────────────────────────────────────────────
    public Task<List<string>> GetWslDistrosAsync() =>
        Invoke<List<string>>("get_wsl_distros", new { });

    // `app` is injected by Tauri — only pass startPath.
    public Task<string?> PickFolderAsync(string? startPath) =>
        Invoke<string?>("pick_folder", new { startPath });

    public Task<bool> ValidateGitRepoAsync(string distro, string repoPath) =>
        Invoke<bool>("validate_git_repo", new { distro, repoPath });

    // ── Git operations ──────────────────────────────────────────────
    public Task<GitStatusResult> GetStatusAsync(string distro, string repoPath) =>
        Invoke<GitStatusResult>("get_git_status", new { distro, repoPath });

    public Task<FileDiffResult> GetDiffAsync(string distro, string repoPath, string filePath, bool staged) =>
        Invoke<FileDiffResult>("get_file_diff", new { distro, repoPath, filePath, staged });

    public Task StageFileAsync(string distro, string repoPath, string filePath) =>
        InvokeVoid("stage_file", new { distro, repoPath, filePath });

    public Task UnstageFileAsync(string distro, string repoPath, string filePath) =>
        InvokeVoid("unstage_file", new { distro, repoPath, filePath });

    public Task CommitAsync(string distro, string repoPath, string subject, string description, bool amend) =>
        InvokeVoid("commit_changes", new { distro, repoPath, subject, description, amend });
}
