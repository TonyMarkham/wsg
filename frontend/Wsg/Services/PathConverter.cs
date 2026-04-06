namespace Wsg.Services;

public static class PathConverter
{
    private const string UncPrefix = @"\\wsl.localhost\";

    /// \\wsl.localhost\Ubuntu\home\user\repos\myrepo → /home/user/repos/myrepo
    public static string UncToLinux(string uncPath)
    {
        var withoutPrefix = uncPath[UncPrefix.Length..];
        var slashIdx = withoutPrefix.IndexOf('\\');
        var linuxPart = slashIdx >= 0 ? withoutPrefix[slashIdx..] : "";
        return linuxPart.Replace('\\', '/');
    }

    /// \\wsl.localhost\Ubuntu\home\user → Ubuntu
    public static string UncToDistro(string uncPath)
    {
        var withoutPrefix = uncPath[UncPrefix.Length..];
        var slashIdx = withoutPrefix.IndexOf('\\');
        return slashIdx >= 0 ? withoutPrefix[..slashIdx] : withoutPrefix;
    }

    /// Ubuntu → \\wsl.localhost\Ubuntu\
    public static string DistroToUncRoot(string distro) =>
        $@"\\wsl.localhost\{distro}\";
}
