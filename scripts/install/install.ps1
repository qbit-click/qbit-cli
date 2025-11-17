param(
    [string]$Destination = "$env:ProgramFiles\Qbit"
)

$binary = "qbit-cli.exe"
if (!(Test-Path $binary)) { throw "binary not found" }

New-Item -ItemType Directory -Force -Path $Destination | Out-Null
Copy-Item $binary "$Destination\qbit.exe" -Force

$envPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if ($envPath -notlike "*$Destination*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$envPath;$Destination",
        "Machine"
    )
    Write-Host "PATH updated. Reopen shells to pick up qbit."
} else {
    Write-Host "Destination already on PATH."
}
