//! Platform detection and installer-asset naming for the update
//! check. Deliberately has no knowledge of *how* to invoke an
//! installer (that's `upgrade.rs`'s job) — this module only answers
//! "what asset name should we be looking for on this platform and
//! architecture."

use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Platform {
    Windows,
    MacOs,
    Linux,
}

/// Architecture, normalized to the exact tokens used in release asset
/// filenames. This is intentionally a distinct type from Rust's raw
/// `std::env::consts::ARCH` string, since the asset-naming convention
/// doesn't always match Rust's arch names 1:1 (see `from_rust_arch`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Arch {
    /// Windows: "x64"
    WindowsX64,
    /// Windows: "arm64"
    WindowsArm64,
    /// macOS: "x86_64"
    MacOsX86_64,
    /// macOS: "arm64"
    MacOsArm64,
    /// Linux: "amd64"
    LinuxAmd64,
    /// Linux: "arm64"
    LinuxArm64,
}

impl Arch {
    /// Maps the current platform plus Rust's `std::env::consts::ARCH`
    /// to the exact architecture token used in release asset names.
    ///
    /// Mapping (per the packaging naming convention):
    /// ```text
    /// Windows x86_64  -> x64
    /// Windows aarch64 -> arm64
    /// macOS   x86_64  -> x86_64
    /// macOS   aarch64 -> arm64
    /// Linux   x86_64  -> amd64
    /// Linux   aarch64 -> arm64
    /// ```
    pub fn detect(platform: Platform) -> Result<Arch> {
        let rust_arch = std::env::consts::ARCH;
        match (platform, rust_arch) {
            (Platform::Windows, "x86_64") => Ok(Arch::WindowsX64),
            (Platform::Windows, "aarch64") => Ok(Arch::WindowsArm64),
            (Platform::MacOs, "x86_64") => Ok(Arch::MacOsX86_64),
            (Platform::MacOs, "aarch64") => Ok(Arch::MacOsArm64),
            (Platform::Linux, "x86_64") => Ok(Arch::LinuxAmd64),
            (Platform::Linux, "aarch64") => Ok(Arch::LinuxArm64),
            (_, other) => bail!(
                "Unsupported architecture `{other}` for this platform. qbit upgrade does not have a known installer asset naming convention for it."
            ),
        }
    }

    /// The exact token used in release asset filenames for this
    /// architecture, matching the naming convention table above.
    pub fn asset_token(self) -> &'static str {
        match self {
            Arch::WindowsX64 => "x64",
            Arch::WindowsArm64 => "arm64",
            Arch::MacOsX86_64 => "x86_64",
            Arch::MacOsArm64 => "arm64",
            Arch::LinuxAmd64 => "amd64",
            Arch::LinuxArm64 => "arm64",
        }
    }
}

impl Platform {
    pub fn current() -> Result<Platform> {
        #[cfg(target_os = "windows")]
        {
            Ok(Platform::Windows)
        }
        #[cfg(target_os = "macos")]
        {
            Ok(Platform::MacOs)
        }
        #[cfg(target_os = "linux")]
        {
            Ok(Platform::Linux)
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            bail!("qbit update checks are not supported on this operating system.");
        }
    }

    pub fn installer_extension(self) -> &'static str {
        match self {
            Platform::Windows => ".msi",
            Platform::MacOs => ".pkg",
            Platform::Linux => ".deb",
        }
    }

    /// Returns (prefix, os_marker) used to identify this platform's
    /// installer asset among a release's assets. The architecture
    /// token is matched separately (see `select_asset`) so the two
    /// concerns — which OS, which CPU architecture — stay independent
    /// and both must match, rather than architecture being ignored.
    fn os_pattern(self) -> (&'static str, &'static str) {
        match self {
            Platform::Windows => ("qbit-cli-", "-windows-"),
            Platform::MacOs => ("qbit-cli-", "-macos-"),
            Platform::Linux => ("qbit-cli", "_"),
        }
    }
}

/// Finds the asset name matching BOTH the current platform AND the
/// current architecture among a list of asset names.
///
/// This is deliberately architecture-aware: a release containing
/// both `qbit-cli-1.2.3-windows-x64.msi` and
/// `qbit-cli-1.2.3-windows-arm64.msi` must resolve to exactly the one
/// matching the running machine's architecture — never ambiguous
/// between architectures, and never accidentally installing the
/// wrong one.
///
/// An ambiguous match (more than one asset fits OS+arch) is treated
/// as a hard failure — silently picking one would be unsafe.
pub fn select_asset<'a>(platform: Platform, asset_names: &[&'a str]) -> Result<&'a str> {
    let arch = Arch::detect(platform)?;
    let (prefix, os_marker) = platform.os_pattern();
    let ext = platform.installer_extension();
    let arch_token = arch.asset_token();

    let matches: Vec<&str> = asset_names
        .iter()
        .filter(|name| {
            name.starts_with(prefix)
                && name.contains(os_marker)
                && name.ends_with(ext)
                && !name.ends_with(".sha256")
                && asset_contains_arch_token(name, os_marker, arch_token)
        })
        .copied()
        .collect();

    match matches.as_slice() {
        [] => bail!(
            "No installer asset found for this platform/architecture (prefix `{prefix}`, marker `{os_marker}`, arch `{arch_token}`, extension `{ext}`). Available: {}",
            if asset_names.is_empty() {
                "<none>".to_string()
            } else {
                asset_names.join(", ")
            }
        ),
        [single] => Ok(single),
        multiple => bail!(
            "Ambiguous installer asset selection for this platform/architecture: {} candidates matched ({}). Refusing to guess.",
            multiple.len(),
            multiple.join(", ")
        ),
    }
}

/// Checks that the architecture token appears in the asset name at
/// the position implied by the naming convention: immediately after
/// the OS marker for Windows/macOS (`-windows-x64.msi`,
/// `-macos-arm64.pkg`), or as the final underscore-delimited segment
/// before the extension for Linux (`qbit-cli_1.2.3_amd64.deb`).
///
/// A plain substring `.contains(arch_token)` would be unsafe here:
/// since "arm64" is used by both Windows and macOS, and Linux asset
/// names could in principle contain a version number that coincides
/// with an arch token, this checks structural position, not just
/// presence anywhere in the string.
///
/// This uses `rsplit_once` (split from the right) rather than
/// `split(...).nth(1)`. For Windows/macOS the marker is a distinctive
/// multi-character string (`-windows-`, `-macos-`) that only appears
/// once, so either approach would work. But Linux's marker is a bare
/// `_`, which also appears inside the version number
/// (`qbit-cli_1.2.3_amd64.deb` has two underscores) — `split("_")
/// .nth(1)` incorrectly returns the version segment ("1.2.3") instead
/// of the architecture segment ("amd64.deb"). Splitting from the
/// right correctly isolates the final segment regardless of how many
/// underscores appear earlier in the name.
fn asset_contains_arch_token(name: &str, os_marker: &str, arch_token: &str) -> bool {
    match name.rsplit_once(os_marker) {
        Some((_, after_marker)) => {
            // after_marker looks like "x64.msi", "arm64.pkg", or
            // "amd64.deb" — the arch token must be the leading
            // segment, immediately followed by the extension's dot.
            after_marker.starts_with(arch_token)
                && after_marker[arch_token.len()..].starts_with('.')
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_asset_finds_unique_linux_amd64_match_among_multiple_arches() {
        let assets = vec![
            "qbit-cli_1.2.3_amd64.deb",
            "qbit-cli_1.2.3_arm64.deb",
            "qbit-cli_1.2.3_amd64.deb.sha256",
        ];
        // This test only asserts the correct behavior when running on
        // amd64; on arm64 CI runners the equivalent arm64 test below
        // covers the same logic. Detect and skip if arch doesn't match.
        if std::env::consts::ARCH != "x86_64" {
            return;
        }
        let found = select_asset(Platform::Linux, &assets).unwrap();
        assert_eq!(found, "qbit-cli_1.2.3_amd64.deb");
    }

    #[test]
    fn select_asset_finds_unique_linux_arm64_match_among_multiple_arches() {
        if std::env::consts::ARCH != "aarch64" {
            return;
        }
        let assets = vec!["qbit-cli_1.2.3_amd64.deb", "qbit-cli_1.2.3_arm64.deb"];
        let found = select_asset(Platform::Linux, &assets).unwrap();
        assert_eq!(found, "qbit-cli_1.2.3_arm64.deb");
    }

    #[test]
    fn select_asset_finds_unique_windows_x64_match_among_multiple_arches() {
        if std::env::consts::ARCH != "x86_64" {
            return;
        }
        let assets = vec![
            "qbit-cli-1.2.3-windows-x64.msi",
            "qbit-cli-1.2.3-windows-arm64.msi",
        ];
        let found = select_asset(Platform::Windows, &assets).unwrap();
        assert_eq!(found, "qbit-cli-1.2.3-windows-x64.msi");
    }

    #[test]
    fn select_asset_finds_unique_windows_arm64_match_among_multiple_arches() {
        if std::env::consts::ARCH != "aarch64" {
            return;
        }
        let assets = vec![
            "qbit-cli-1.2.3-windows-x64.msi",
            "qbit-cli-1.2.3-windows-arm64.msi",
        ];
        let found = select_asset(Platform::Windows, &assets).unwrap();
        assert_eq!(found, "qbit-cli-1.2.3-windows-arm64.msi");
    }

    #[test]
    fn select_asset_finds_unique_macos_x86_64_match_among_multiple_arches() {
        if std::env::consts::ARCH != "x86_64" {
            return;
        }
        let assets = vec![
            "qbit-cli-1.2.3-macos-x86_64.pkg",
            "qbit-cli-1.2.3-macos-arm64.pkg",
        ];
        let found = select_asset(Platform::MacOs, &assets).unwrap();
        assert_eq!(found, "qbit-cli-1.2.3-macos-x86_64.pkg");
    }

    #[test]
    fn select_asset_finds_unique_macos_arm64_match_among_multiple_arches() {
        if std::env::consts::ARCH != "aarch64" {
            return;
        }
        let assets = vec![
            "qbit-cli-1.2.3-macos-x86_64.pkg",
            "qbit-cli-1.2.3-macos-arm64.pkg",
        ];
        let found = select_asset(Platform::MacOs, &assets).unwrap();
        assert_eq!(found, "qbit-cli-1.2.3-macos-arm64.pkg");
    }

    #[test]
    fn select_asset_errors_when_arch_missing_from_release() {
        // Simulates a release that only shipped one architecture,
        // which is not the one this (hypothetical) machine needs.
        // We can't force std::env::consts::ARCH in a test, so this
        // uses an asset list crafted to never match ANY of our known
        // arch tokens for Linux, proving the "no match" path fires
        // rather than silently picking something wrong.
        let assets = vec!["qbit-cli_1.2.3_riscv64.deb"];
        let err = select_asset(Platform::Linux, &assets).unwrap_err();
        assert!(err.to_string().contains("No installer asset found"));
    }

    #[test]
    fn select_asset_does_not_select_wrong_architecture() {
        // Regression test for the exact scenario in the review doc:
        // on this machine's actual architecture, the asset for a
        // DIFFERENT architecture must never be selected, even if it's
        // the only asset present (this must fail closed, not fall
        // back to installing the wrong arch).
        let wrong_arch_only = if std::env::consts::ARCH == "x86_64" {
            vec!["qbit-cli-1.2.3-windows-arm64.msi"]
        } else {
            vec!["qbit-cli-1.2.3-windows-x64.msi"]
        };
        let err = select_asset(Platform::Windows, &wrong_arch_only).unwrap_err();
        assert!(err.to_string().contains("No installer asset found"));
    }

    #[test]
    fn select_asset_ignores_sha256_files() {
        let assets = vec![
            "qbit-cli_1.2.3_amd64.deb",
            "qbit-cli_1.2.3_amd64.deb.sha256",
        ];
        if std::env::consts::ARCH != "x86_64" {
            return;
        }
        let found = select_asset(Platform::Linux, &assets).unwrap();
        assert_eq!(found, "qbit-cli_1.2.3_amd64.deb");
    }

    #[test]
    fn arch_detect_maps_windows_x86_64_to_x64_token() {
        // Directly test the mapping table without depending on the
        // actual host architecture, by constructing Arch values
        // directly rather than through detect().
        assert_eq!(Arch::WindowsX64.asset_token(), "x64");
        assert_eq!(Arch::WindowsArm64.asset_token(), "arm64");
        assert_eq!(Arch::MacOsX86_64.asset_token(), "x86_64");
        assert_eq!(Arch::MacOsArm64.asset_token(), "arm64");
        assert_eq!(Arch::LinuxAmd64.asset_token(), "amd64");
        assert_eq!(Arch::LinuxArm64.asset_token(), "arm64");
    }
}
