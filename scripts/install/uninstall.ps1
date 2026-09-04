<#
  LEGACY / BOOTSTRAP UNINSTALLER — NOT PART OF THE OFFICIAL RELEASE PATH.
  See install.ps1 header. Real end users uninstall via Windows
  "Installed Apps", which invokes the MSI's own uninstall.
#>

param(
    [string]$Destination = (Join-Path $env:LOCALAPPDATA "Programs\Qbit CLI")
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Remove-FromPath([string]$entry, [System.EnvironmentVariableTarget]$target) {
    $current = [Environment]::GetEnvironmentVariable("Path", $target)
    if (-not $current) { return }

    $normalizedEntry = $entry.TrimEnd("\")
    $segments = @($current -split ";" | Where-Object { $_ -and $_.Trim() -ne "" })
    $filtered = @($segments | Where-Object { $_.TrimEnd("\") -ine $normalizedEntry })

    if ($filtered.Count -eq $segments.Count) {
        Write-Host "PATH entry not found (already removed): $entry"
        return
    }

    [Environment]::SetEnvironmentVariable("Path", ($filtered -join ";"), $target)
    Write-Host "Removed PATH entry: $entry"
}

$binDir = Join-Path $Destination "bin"

# Exactly ONE assignment to $binaryPath. A previous version of this
# script assigned this twice in a row (once to qbit.exe, then
# immediately overwritten with qbit-cli.exe), which silently left
# qbit.exe on disk after "successful" uninstall. Do not reintroduce
# a second assignment to this variable.
$binaryPath = Join-Path $binDir "qbit.exe"

if (Test-Path -LiteralPath $binaryPath) {
    Remove-Item -LiteralPath $binaryPath -Force
    Write-Host "Removed: $binaryPath"
} else {
    Write-Host "Already removed (idempotent): $binaryPath"
}

if (Test-Path -LiteralPath $binDir) {
    $remaining = Get-ChildItem -LiteralPath $binDir -Force
    if ($remaining.Count -eq 0) {
        Remove-Item -LiteralPath $binDir -Force
    }
}

if (Test-Path -LiteralPath $Destination) {
    $remainingRoot = Get-ChildItem -LiteralPath $Destination -Force
    if ($remainingRoot.Count -eq 0) {
        Remove-Item -LiteralPath $Destination -Force
    }
}

Remove-FromPath -entry $binDir -target ([System.EnvironmentVariableTarget]::User)

Write-Host "qbit uninstall complete."
