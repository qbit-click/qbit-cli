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
        throw "Could not resolve LocalApplicationData for user install."
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

function Add-ToPath([string]$entry, [System.EnvironmentVariableTarget]$target) {
    $current = [Environment]::GetEnvironmentVariable("Path", $target)
    $segments = @()
    if ($current) {
        $segments = @($current -split ";" | Where-Object { $_ -and $_.Trim() -ne "" })
    }

    $normalizedEntry = $entry.TrimEnd("\")
    $alreadyPresent = $segments | Where-Object { $_.TrimEnd("\") -ieq $normalizedEntry }
    if ($alreadyPresent) {
        Write-Host "PATH already contains: $entry"
        return
    }

    $newValue = if ($current -and $current.Trim() -ne "") {
        "$current;$entry"
    } else {
        $entry
    }
    [Environment]::SetEnvironmentVariable("Path", $newValue, $target)
    Write-Host "PATH updated for $target. Reopen your terminal to use 'qbit'."
}

if (-not $Destination) {
    $Destination = Get-DefaultDestination -installScope $Scope
}

Ensure-AdministratorIfMachineScope -installScope $Scope

$scriptRoot = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$binaryPath = Join-Path $scriptRoot "qbit-cli.exe"
if (-not (Test-Path -LiteralPath $binaryPath)) {
    throw "qbit-cli.exe was not found next to install.ps1. Extract the release archive and run the script from that folder."
}

$binDir = Join-Path $Destination "bin"
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
Copy-Item -LiteralPath $binaryPath -Destination (Join-Path $binDir "qbit.exe") -Force

$target = if ($Scope -eq "Machine") { [System.EnvironmentVariableTarget]::Machine } else { [System.EnvironmentVariableTarget]::User }
Add-ToPath -entry $binDir -target $target

if (-not (($env:Path -split ";" | Where-Object { $_.TrimEnd("\") -ieq $binDir.TrimEnd("\") }))) {
    $env:Path = if ($env:Path) { "$env:Path;$binDir" } else { $binDir }
}

Write-Host "Installed qbit to $binDir\qbit.exe"
