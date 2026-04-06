namespace Wsg.Models;

public record TreeNode(
    string Name,
    string FullPath,
    bool IsDirectory,
    int Depth,
    bool IsExpanded,
    string? StatusCode  // null for directories
);
