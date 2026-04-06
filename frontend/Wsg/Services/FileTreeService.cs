using Wsg.Models;

namespace Wsg.Services;

public class FileTreeService
{
    /// Builds flat visible node list from FileStatus list.
    /// Groups files by directory segments, sorts folders before files, assigns depth.
    /// All folders start expanded so the initial view mirrors `git status` output.
    public List<TreeNode> BuildTree(List<FileStatus> files)
    {
        var dirs = new HashSet<string>();
        foreach (var f in files)
        {
            var parts = f.Path.Split('/');
            for (int i = 1; i < parts.Length; i++)
                dirs.Add(string.Join('/', parts[..i]));
        }

        var allEntries = dirs
            .Select(d => (Path: d, IsDir: true, StatusCode: (string?)null))
            .Concat(files.Select(f => (Path: f.Path, IsDir: false, StatusCode: (string?)f.StatusCode)))
            .OrderBy(e =>
            {
                var lastSlash = e.Path.LastIndexOf('/');
                var parent = lastSlash >= 0 ? e.Path[..lastSlash] : "";
                var name = lastSlash >= 0 ? e.Path[(lastSlash + 1)..] : e.Path;
                var dirOrder = e.IsDir ? "0" : "1";
                return $"{parent}/{dirOrder}{name}";
            })
            .ToList();

        var result = new List<TreeNode>();
        foreach (var entry in allEntries)
        {
            var depth = entry.Path.Count(c => c == '/');
            var name = entry.Path.Split('/').Last();
            result.Add(new TreeNode(name, entry.Path, entry.IsDir, depth, entry.IsDir, entry.StatusCode));
        }

        return result;
    }

    /// Toggles folder expand/collapse — splices children in/out of flat list.
    public void ToggleFolder(List<TreeNode> visibleNodes, TreeNode folder)
    {
        var idx = visibleNodes.IndexOf(folder);
        if (idx < 0) return;

        if (folder.IsExpanded)
        {
            var prefix = folder.FullPath + "/";
            int removeStart = idx + 1;
            int removeCount = 0;
            while (removeStart + removeCount < visibleNodes.Count &&
                   visibleNodes[removeStart + removeCount].FullPath.StartsWith(prefix))
                removeCount++;
            visibleNodes.RemoveRange(removeStart, removeCount);
            visibleNodes[idx] = folder with { IsExpanded = false };
        }
        else
        {
            visibleNodes[idx] = folder with { IsExpanded = true };
        }
    }
}
