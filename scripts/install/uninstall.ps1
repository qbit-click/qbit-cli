param(
    [ValidateSet("User", "Machine")]
    [string]$Scope = "User",
    [string]$Destination
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Get-DefaultDestination([string]$installScope) {
    if ($installScope -eq "Machine") {
        return (Join-Path $env:ProgramFiles "Qbit")
    }

    $localAppData = [Environment]::GetFolderPath("LocalApplicationData")
    if (-not $localAppData) {
        throw "Could not resolve LocalApplicationData for user uninstall."
    }
    return (Join-Path $localAppData "Qbit")
}

function Ensure-AdministratorIfMachineScope([string]$installScope) {
    if ($installScope -ne "Machine") {
        return
    }

    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    $isAdmin = $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    if (-not $isAdmin) {
        throw "Machine scope requires an elevated PowerShell. Re-run as Administrator or use -Scope User."
    }
}

function Remove-FromPath([string]$entry, [System.EnvironmentVariableTarget]$target) {
    $current = [Environment]::GetEnvironmentVariable("Path", $target)
    if (-not $current) {
        return
    }

    $normalizedEntry = $entry.TrimEnd("\")
    $segments = @($current -split ";" | Where-Object { $_ -and $_.Trim() -ne "" })
    $filtered = @($segments | Where-Object { $_.TrimEnd("\") -ine $normalizedEntry })

    if ($filtered.Count -eq $segments.Count) {
        Write-Host "PATH entry not found: $entry"
        return
    }

    [Environment]::SetEnvironmentVariable("Path", ($filtered -join ";"), $target)
    Write-Host "Removed PATH entry for $target: $entry"
}

if (-not $Destination) {
    $Destination = Get-DefaultDestination -installScope $Scope
}

Ensure-AdministratorIfMachineScope -installScope $Scope

$binDir = Join-Path $Destination "bin"
$binaryPath = Join-Path $binDir "qbit.exe"

if (Test-Path -LiteralPath $binaryPath) {
    Remove-Item -LiteralPath $binaryPath -Force
    Write-Host "Removed: $binaryPath"
} else {
    Write-Host "Binary not found (already removed): $binaryPath"
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

$target = if ($Scope -eq "Machine") { [System.EnvironmentVariableTarget]::Machine } else { [System.EnvironmentVariableTarget]::User }
Remove-FromPath -entry $binDir -target $target

$processSegments = @($env:Path -split ";" | Where-Object { $_ -and $_.Trim() -ne "" })
$env:Path = (@($processSegments | Where-Object { $_.TrimEnd("\") -ine $binDir.TrimEnd("\") }) -join ";")

Write-Host "qbit uninstall complete."
