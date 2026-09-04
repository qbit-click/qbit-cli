<#
  LEGACY / BOOTSTRAP INSTALLER — NOT PART OF THE OFFICIAL RELEASE PATH.

  This script is not published as part of any GitHub Release and is
  never invoked by `qbit upgrade`. The supported end-user
  install/upgrade/uninstall path is the MSI (see
  packaging/windows/QbitCli.wxs). This script exists only as an
  optional convenience for local development: building qbit.exe
  yourself and wanting it on PATH without building the full MSI.
#>

param(
    [string]$Destination = (Join-Path $env:LOCALAPPDATA "Programs\Qbit CLI")
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

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

    $newValue = if ($current -and $current.Trim() -ne "") { "$current;$entry" } else { $entry }
    [Environment]::SetEnvironmentVariable("Path", $newValue, $target)
    Write-Host "PATH updated for $target. Reopen your terminal to use 'qbit'."
}

$scriptRoot = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }

# Only qbit.exe is ever expected or installed by this script.
# qbit-cli.exe is not a recognized input and is never created.
$binaryPath = Join-Path $scriptRoot "qbit.exe"
if (-not (Test-Path -LiteralPath $binaryPath)) {
    throw "qbit.exe was not found next to install.ps1. Build it first with 'cargo build --release' and copy target\release\qbit.exe next to this script."
}

# Matches the MSI-installed layout: %LOCALAPPDATA%\Programs\Qbit CLI\bin\qbit.exe
$binDir = Join-Path $Destination "bin"
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
Copy-Item -LiteralPath $binaryPath -Destination (Join-Path $binDir "qbit.exe") -Force

Add-ToPath -entry $binDir -target ([System.EnvironmentVariableTarget]::User)

Write-Host "Installed qbit to $binDir\qbit.exe"
