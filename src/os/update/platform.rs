//! Platform detection and installer-asset naming for the update
//! check. Deliberately has no knowledge of *how* to invoke an
//! installer (that's `upgrade.rs`'s job) — this module only answers
//! "what asset name should we be looking for on this platform."

use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOs,
    Linux,
}

impl Platform {
    pub fn current() -> Result<Platform> {
        #[cfg(target_os = "windows")]
        {
            return Ok(Platform::Windows);
        }
        #[cfg(target_os = "macos")]
        {
            return Ok(Platform::MacOs);
        }
        #[cfg(target_os = "linux")]
        {
            return Ok(Platform::Linux);
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

    /// Returns (prefix, marker) used to identify this platform's
    /// installer asset among a release's assets, without relying on
    /// exact filename matches (which embed version/arch we don't
    /// know ahead of time).
    pub fn asset_pattern(self) -> (&'static str, &'static str) {
        match self {
            Platform::Windows => ("qbit-cli-", "-windows-"),
            Platform::MacOs => ("qbit-cli-", "-macos-"),
            Platform::Linux => ("qbit-cli", "_"),
        }
    }
}

/// Finds the asset name (not the whole release struct — this module
/// doesn't know about GithubAsset/GithubRelease types, keeping it
/// decoupled from the github.rs and upgrade.rs internals) matching
/// the current platform among a list of asset names.
///
/// An ambiguous match (more than one asset fits the pattern) is
/// treated as a hard failure — silently picking one would be unsafe.
pub fn select_asset<'a>(platform: Platform, asset_names: &[&'a str]) -> Result<&'a str> {
    let (prefix, marker) = platform.asset_pattern();
    let ext = platform.installer_extension();

    let matches: Vec<&str> = asset_names
        .iter()
        .filter(|name| {
            name.starts_with(prefix)
                && name.contains(marker)
                && name.ends_with(ext)
                && !name.ends_with(".sha256")
        })
        .copied()
        .collect();

    match matches.as_slice() {
        [] => bail!(
            "No installer asset found for this platform (prefix `{prefix}`, marker `{marker}`, extension `{ext}`). Available: {}",
            if asset_names.is_empty() {
                "<none>".to_string()
            } else {
                asset_names.join(", ")
            }
        ),
        [single] => Ok(single),
        multiple => bail!(
            "Ambiguous installer asset selection for this platform: {} candidates matched ({}). Refusing to guess.",
            multiple.len(),
            multiple.join(", ")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_asset_finds_unique_linux_match() {
        let assets = vec![
            "qbit-cli_1.2.3_amd64.deb",
            "qbit-cli_1.2.3_amd64.deb.sha256",
            "qbit-cli-1.2.3-windows-x64.msi",
            "qbit-cli-1.2.3-macos-arm64.pkg",
        ];
        let found = select_asset(Platform::Linux, &assets).unwrap();
        assert_eq!(found, "qbit-cli_1.2.3_amd64.deb");
    }

    #[test]
    fn select_asset_finds_unique_windows_match() {
        let assets = vec![
            "qbit-cli_1.2.3_amd64.deb",
            "qbit-cli-1.2.3-windows-x64.msi",
            "qbit-cli-1.2.3-windows-x64.msi.sha256",
        ];
        let found = select_asset(Platform::Windows, &assets).unwrap();
        assert_eq!(found, "qbit-cli-1.2.3-windows-x64.msi");
    }

    #[test]
    fn select_asset_errors_when_none_match() {
        let assets = vec!["some-other-tool-1.0.0.tar.gz"];
        let err = select_asset(Platform::Linux, &assets).unwrap_err();
        assert!(err.to_string().contains("No installer asset found"));
    }

    #[test]
    fn select_asset_errors_on_ambiguous_match() {
        // Two Linux .deb candidates for different arches — this
        // module has no arch info, so both would match Linux's
        // pattern and it must fail closed rather than pick one.
        let assets = vec!["qbit-cli_1.2.3_amd64.deb", "qbit-cli_1.2.3_arm64.deb"];
        let err = select_asset(Platform::Linux, &assets).unwrap_err();
        assert!(err.to_string().contains("Ambiguous"));
    }

    #[test]
    fn select_asset_ignores_sha256_files() {
        let assets = vec![
            "qbit-cli_1.2.3_amd64.deb",
            "qbit-cli_1.2.3_amd64.deb.sha256",
        ];
        let found = select_asset(Platform::Linux, &assets).unwrap();
        assert_eq!(found, "qbit-cli_1.2.3_amd64.deb");
    }
}
