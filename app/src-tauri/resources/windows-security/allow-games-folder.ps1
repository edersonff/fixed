param(
    [Parameter(Mandatory = $true)]
    [string]$GamesFolder,

    [switch]$Remove
)

$ErrorActionPreference = "Stop"

$MarkerKey = "HKLM:\SOFTWARE\FIXED"

$MarkerName = "GamesFolderAllowed"

try {
    if ($Remove) {
        $owned = (Get-ItemProperty -Path $MarkerKey -Name $MarkerName -ErrorAction SilentlyContinue).$MarkerName

        if ($owned -eq 1) {
            Remove-MpPreference -ExclusionPath $GamesFolder
            Remove-ItemProperty -Path $MarkerKey -Name $MarkerName -ErrorAction SilentlyContinue
        }

        exit 0
    }

    $alreadyAllowed = @((Get-MpPreference).ExclusionPath) -contains $GamesFolder

    if ($alreadyAllowed) {
        exit 0
    }

    Add-MpPreference -ExclusionPath $GamesFolder

    New-Item -Path $MarkerKey -Force | Out-Null

    New-ItemProperty -Path $MarkerKey -Name $MarkerName -Value 1 -PropertyType DWord -Force | Out-Null

    exit 0
} catch {
    Write-Error $_.Exception.Message
    exit 1
}
