# Requires Pester (Install-Module Pester -Force) to run:
#   Invoke-Pester tests/e2e/uninstall.Tests.ps1

Describe "uninstall.ps1 (legacy bootstrap installer)" {

    BeforeEach {
        $script:TestDest = Join-Path $env:TEMP "qbit-uninstall-test-$(Get-Random)"
        $script:BinDir = Join-Path $TestDest "bin"
        New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
        Set-Content -LiteralPath (Join-Path $BinDir "qbit.exe") -Value "fake binary"

        $repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
        $script:UninstallScript = Join-Path $repoRoot "scripts\install\uninstall.ps1"
    }

    AfterEach {
        if (Test-Path -LiteralPath $TestDest) {
            Remove-Item -LiteralPath $TestDest -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    It "removes qbit.exe (regression test for the $binaryPath overwrite bug)" {
        & $UninstallScript -Destination $TestDest
        Test-Path (Join-Path $BinDir "qbit.exe") | Should -BeFalse
    }

    It "removes the empty bin directory" {
        & $UninstallScript -Destination $TestDest
        Test-Path $BinDir | Should -BeFalse
    }

    It "removes the empty installation root directory" {
        & $UninstallScript -Destination $TestDest
        Test-Path $TestDest | Should -BeFalse
    }

    It "does not error when run a second time (idempotent)" {
        & $UninstallScript -Destination $TestDest
        { & $UninstallScript -Destination $TestDest } | Should -Not -Throw
    }

    It "leaves nothing behind after two consecutive runs" {
        & $UninstallScript -Destination $TestDest
        & $UninstallScript -Destination $TestDest
        Test-Path (Join-Path $BinDir "qbit.exe") | Should -BeFalse
        Test-Path $BinDir | Should -BeFalse
        Test-Path $TestDest | Should -BeFalse
    }

    It "removes the bin directory from User PATH if present" {
        $originalPath = [Environment]::GetEnvironmentVariable("Path", "User")
        try {
            $withEntry = if ($originalPath) { "$originalPath;$BinDir" } else { $BinDir }
            [Environment]::SetEnvironmentVariable("Path", $withEntry, "User")

            & $UninstallScript -Destination $TestDest

            $afterPath = [Environment]::GetEnvironmentVariable("Path", "User")
            $segments = @($afterPath -split ";" | Where-Object { $_ -and $_.Trim() -ne "" })
            $stillPresent = $segments | Where-Object { $_.TrimEnd("\") -ieq $BinDir.TrimEnd("\") }
            $stillPresent | Should -BeNullOrEmpty
        } finally {
            [Environment]::SetEnvironmentVariable("Path", $originalPath, "User")
        }
    }
}
