using Wsg.Models;

namespace Wsg.Services;

public class AppState
{
    public List<RepoContext> Repos { get; } = [];
    public RepoContext? ActiveRepo { get; private set; }
    public bool ShowAddRepoModal { get; set; }
    public event Action? OnChange;

    public void AddRepo(RepoContext repo) { Repos.Add(repo); SetActiveRepo(repo); }
    public void SetActiveRepo(RepoContext repo) { ActiveRepo = repo; NotifyChange(); }
    public void NotifyChange() => OnChange?.Invoke();
}

public class RepoContext
{
    public required string WslDistro { get; init; }  // e.g. "Ubuntu", "Ubuntu-22.04", or "" for Windows
    public required string WslPath { get; init; }    // Linux path e.g. /home/user/repos/myrepo
    public bool IsWindows => string.IsNullOrEmpty(WslDistro);
    public string Name => WslPath.Split('/').Last();
    public string? Branch { get; set; }
    public string? SelectedFile { get; set; }
    public bool SelectedFileIsStaged { get; set; }
    public string CommitSubject { get; set; } = "";
    public string CommitDescription { get; set; } = "";
    public GitStatusResult? LastStatus { get; set; }
}
