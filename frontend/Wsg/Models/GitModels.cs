using System.Text.Json.Serialization;

namespace Wsg.Models;

// Rust serde serialises field names in snake_case by default.
// Single-word fields map via case-insensitive matching, but multi-word fields
// need explicit [JsonPropertyName] because underscores don't collapse
// (e.g. "status_code" ≠ "StatusCode" even case-insensitively).
public record FileStatus(
    [property: JsonPropertyName("path")]        string Path,
    [property: JsonPropertyName("status_code")] string StatusCode
);

public record GitStatusResult(
    [property: JsonPropertyName("branch")]   string Branch,
    [property: JsonPropertyName("unstaged")] List<FileStatus> Unstaged,
    [property: JsonPropertyName("staged")]   List<FileStatus> Staged
);

// Original = content before changes; Modified = content after changes.
// Both are full file text strings — StandaloneDiffEditor requires two separate
// TextModel instances, not a unified diff string.
public record FileDiffResult(
    [property: JsonPropertyName("original")] string Original,
    [property: JsonPropertyName("modified")] string Modified
);
