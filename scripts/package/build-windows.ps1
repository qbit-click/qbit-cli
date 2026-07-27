# Builds a .msi installer from the release binary using WiX v5.
#
# Prerequisites (one-time, on the build machine / CI runner):
#   dotnet tool install --global wix
#
# Usage:
#   .\build-windows.ps1 -Version 1.0.0 -Arch x64 -BinaryPath target\release\qbit.exe -OutDir dist

param(
    [Parameter(Mandatory=$true)][string]$Version,
    [Parameter(Mandatory=$true)][string]$Arch,
    [Parameter(Mandatory=$true)][string]$BinaryPath,
    [string]$OutDir = "dist"
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $BinaryPath)) {
    throw "Binary not found at $BinaryPath. Run 'cargo build --release' first."
}

$scriptDir = $PSScriptRoot
$repoRoot = Resolve-Path (Join-Path $scriptDir "..\..")
$wxsPath = Join-Path $repoRoot "packaging\windows\QbitCli.wxs"
$iconPath = Join-Path $repoRoot "assets\icon.ico"

if (-not (Test-Path -LiteralPath $wxsPath)) {
    throw "WiX source not found at $wxsPath"
}
if (-not (Test-Path -LiteralPath $iconPath)) {
    throw "Icon not found at $iconPath (expected assets\icon.svg)"
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$outFile = Join-Path $OutDir "qbit-cli-$Version-windows-$Arch.msi"
$absBinaryPath = (Resolve-Path $BinaryPath).Path

wix build $wxsPath `
    -d "ProductVersion=$Version" `
    -d "QbitBinaryPath=$absBinaryPath" `
    -d "IconPath=$iconPath" `
    -arch $Arch `
    -out $outFile

if ($LASTEXITCODE -ne 0) {
    throw "wix build failed with exit code $LASTEXITCODE"
}

Write-Host "Built: $outFile"
