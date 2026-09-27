param(
    [Parameter(Mandatory = $true)]
    [string]$GamesFolder,

    [switch]$Remove
)

$ErrorActionPreference = "Stop"

try {
    if ($Remove) {
        Remove-MpPreference -ExclusionPath $GamesFolder
    } else {
        Add-MpPreference -ExclusionPath $GamesFolder
    }

    exit 0
} catch {
    Write-Error $_.Exception.Message
    exit 1
}
