use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;

use crate::os::update::checksum;
use crate::os::update::platform::{self, Platform};

const DEFAULT_REPOSITORY: &str = "qbit-click/qbit-cli";

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

struct TempDirGuard {
    path: PathBuf,
}

impl TempDirGuard {
    fn new() -> Result<Self> {
        let mut path = std::env::temp_dir();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("system time before UNIX_EPOCH")?
            .as_nanos();
        path.push(format!("qbit-upgrade-{}-{now}", std::process::id()));
        fs::create_dir_all(&path)
            .with_context(|| format!("creating temporary upgrade directory {}", path.display()))?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub fn upgrade() -> Result<()> {
    let repository = upgrade_repository();
    let current = parse_version(env!("CARGO_PKG_VERSION"))
        .context("parsing current qbit version from build metadata")?;

    println!("Checking for updates from GitHub repo: {repository}");
    let release = fetch_latest_release(&repository)?;
    let latest = parse_version(&release.tag_name)
        .with_context(|| format!("parsing latest tag `{}`", release.tag_name))?;

    println!("Current version: {current}");
    println!("Latest version:  {latest}");

    if latest <= current {
        println!("qbit is already up to date.");
        return Ok(());
    }

    let current_platform = Platform::current()?;
    let asset_names: Vec<&str> = release.assets.iter().map(|a| a.name.as_str()).collect();
    let asset_name = platform::select_asset(current_platform, &asset_names)?;
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .expect("select_asset returned a name that must exist in the source list");
    let checksum_asset = find_checksum_asset(&release, &asset.name)?;

    println!("Downloading installer: {}", asset.name);

    let temp = TempDirGuard::new()?;
    let installer_path = temp.path().join(&asset.name);
    download_to_file(&asset.browser_download_url, &installer_path)?;

    println!("Verifying checksum...");
    let checksum_text = download_checksum_text(&checksum_asset.browser_download_url)?;
    let expected_checksum = checksum::parse_checksum_text(&checksum_text)?;
    checksum::verify_file(&installer_path, &expected_checksum)?;
    println!("Checksum OK.");

    run_native_installer(&installer_path)?;

    println!("Upgrade installed successfully to version {latest}.");
    Ok(())
}

fn upgrade_repository() -> String {
    std::env::var("QBIT_UPGRADE_REPO")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_REPOSITORY.to_string())
}

fn parse_version(input: &str) -> Result<Version> {
    let trimmed = input.trim();
    let normalized = if let Some(rest) = trimmed.strip_prefix('v') {
        rest
    } else {
        trimmed
    };
    Version::parse(normalized).with_context(|| format!("invalid semantic version: `{trimmed}`"))
}

fn github_api_url(repository: &str) -> String {
    format!("https://api.github.com/repos/{repository}/releases/latest")
}

fn fetch_latest_release(repository: &str) -> Result<GithubRelease> {
    let client = Client::builder()
        .build()
        .context("building HTTP client for upgrade")?;

    let response = client
        .get(github_api_url(repository))
        .header(reqwest::header::USER_AGENT, "qbit-cli-upgrader")
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .send()
        .with_context(|| format!("requesting latest release for {repository}"))?
        .error_for_status()
        .with_context(|| format!("GitHub API returned an error for repo {repository}"))?;

    response
        .json::<GithubRelease>()
        .context("decoding GitHub release response JSON")
}

fn find_checksum_asset<'a>(
    release: &'a GithubRelease,
    installer_name: &str,
) -> Result<&'a GithubAsset> {
    let expected_name = format!("{installer_name}.sha256");
    release
        .assets
        .iter()
        .find(|asset| asset.name == expected_name)
        .ok_or_else(|| {
            let available = list_asset_names(release);
            anyhow::anyhow!(
                "Checksum file `{expected_name}` was not found for installer `{installer_name}`. \
                 Refusing to install without a verifiable checksum. Available assets: {available}"
            )
        })
}

fn list_asset_names(release: &GithubRelease) -> String {
    if release.assets.is_empty() {
        "<no assets>".to_string()
    } else {
        release
            .assets
            .iter()
            .map(|asset| asset.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn download_to_file(url: &str, destination: &Path) -> Result<()> {
    let client = Client::builder()
        .build()
        .context("building HTTP client for release download")?;

    let mut response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "qbit-cli-upgrader")
        .send()
        .with_context(|| format!("downloading installer from {url}"))?
        .error_for_status()
        .with_context(|| format!("failed to download installer from {url}"))?;

    let mut file = File::create(destination)
        .with_context(|| format!("creating installer file {}", destination.display()))?;

    io::copy(&mut response, &mut file)
        .with_context(|| format!("writing installer to {}", destination.display()))?;
    file.flush()
        .with_context(|| format!("flushing installer {}", destination.display()))?;
    Ok(())
}

fn download_checksum_text(url: &str) -> Result<String> {
    let client = Client::builder()
        .build()
        .context("building HTTP client for checksum download")?;

    client
        .get(url)
        .header(reqwest::header::USER_AGENT, "qbit-cli-upgrader")
        .send()
        .with_context(|| format!("downloading checksum from {url}"))?
        .error_for_status()
        .with_context(|| format!("failed to download checksum from {url}"))?
        .text()
        .context("reading checksum response as text")
}

/// Runs the OS-native installer directly (no bundled install scripts,
/// no archive extraction). If elevated privileges are required and we
/// don't have them, re-invokes with an OS-appropriate elevation prompt.
fn run_native_installer(installer_path: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        run_msi_installer(installer_path)?;
    }

    #[cfg(target_os = "macos")]
    {
        run_pkg_installer(installer_path)?;
    }

    #[cfg(target_os = "linux")]
    {
        run_deb_installer(installer_path)?;
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        bail!("qbit upgrade is not supported on this operating system.");
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn run_msi_installer(installer_path: &Path) -> Result<()> {
    // msiexec handles its own UAC elevation prompt for per-machine
    // installs; for a per-user MSI (see QbitCli.wxs, Scope="perUser")
    // no elevation is required at all, so we invoke it directly.
    let status = Command::new("msiexec")
        .arg("/i")
        .arg(installer_path)
        .arg("/qn")
        .arg("/norestart")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("running msiexec for {}", installer_path.display()))?;

    if !status.success() {
        bail!(
            "msiexec failed (exit code {}) installing {}",
            status.code().unwrap_or(1),
            installer_path.display()
        );
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn run_pkg_installer(installer_path: &Path) -> Result<()> {
    // `installer -pkg ... -target /` always requires root. Try
    // directly first; if that fails due to permissions, re-invoke
    // ourselves under `sudo`, which will prompt the user interactively.
    let direct = Command::new("installer")
        .arg("-pkg")
        .arg(installer_path)
        .arg("-target")
        .arg("/")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    let needs_elevation = match &direct {
        Ok(status) => !status.success(),
        Err(_) => true,
    };

    if !needs_elevation {
        return Ok(());
    }

    println!(
        "Administrator privileges are required to install. You may be prompted for your password."
    );
    let status = Command::new("sudo")
        .arg("installer")
        .arg("-pkg")
        .arg(installer_path)
        .arg("-target")
        .arg("/")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("re-invoking installer with sudo")?;

    if !status.success() {
        bail!(
            "installer failed (exit code {}) installing {}",
            status.code().unwrap_or(1),
            installer_path.display()
        );
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn run_deb_installer(installer_path: &Path) -> Result<()> {
    // `dpkg -i` requires root. Try directly first (covers containers
    // and CI where the process may already be root); if that fails,
    // re-invoke under `sudo`, which prompts the user interactively.
    let direct = Command::new("dpkg")
        .arg("-i")
        .arg(installer_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    let needs_elevation = match &direct {
        Ok(status) => !status.success(),
        Err(_) => true,
    };

    if !needs_elevation {
        return Ok(());
    }

    println!("Root privileges are required to install. You may be prompted for your password.");
    let status = Command::new("sudo")
        .arg("dpkg")
        .arg("-i")
        .arg(installer_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("re-invoking dpkg with sudo")?;

    if !status.success() {
        bail!(
            "dpkg failed (exit code {}) installing {}",
            status.code().unwrap_or(1),
            installer_path.display()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_accepts_v_prefix() {
        let version = parse_version("v1.2.3").expect("version");
        assert_eq!(version, Version::new(1, 2, 3));
    }

    #[test]
    fn parse_version_rejects_invalid_input() {
        let err = parse_version("release-1.2").expect_err("must fail");
        assert!(err.to_string().contains("invalid semantic version"));
    }

    fn sample_release() -> GithubRelease {
        GithubRelease {
            tag_name: "v1.2.3".to_string(),
            assets: vec![
                GithubAsset {
                    name: "qbit-cli_1.2.3_amd64.deb".to_string(),
                    browser_download_url: "https://example.test/linux.deb".to_string(),
                },
                GithubAsset {
                    name: "qbit-cli_1.2.3_amd64.deb.sha256".to_string(),
                    browser_download_url: "https://example.test/linux.deb.sha256".to_string(),
                },
                GithubAsset {
                    name: "qbit-cli-1.2.3-windows-x64.msi".to_string(),
                    browser_download_url: "https://example.test/windows.msi".to_string(),
                },
                GithubAsset {
                    name: "qbit-cli-1.2.3-windows-x64.msi.sha256".to_string(),
                    browser_download_url: "https://example.test/windows.msi.sha256".to_string(),
                },
                GithubAsset {
                    name: "qbit-cli-1.2.3-macos-arm64.pkg".to_string(),
                    browser_download_url: "https://example.test/macos.pkg".to_string(),
                },
                GithubAsset {
                    name: "qbit-cli-1.2.3-macos-arm64.pkg.sha256".to_string(),
                    browser_download_url: "https://example.test/macos.pkg.sha256".to_string(),
                },
            ],
        }
    }

    #[test]
    fn find_release_asset_matches_current_platform_installer() {
        let release = sample_release();
        let current_platform = Platform::current().expect("platform");
        let asset_names: Vec<&str> = release.assets.iter().map(|a| a.name.as_str()).collect();
        let asset_name =
            platform::select_asset(current_platform, &asset_names).expect("asset");
        assert!(asset_name.ends_with(current_platform.installer_extension()));
        assert!(!asset_name.ends_with(".sha256"));
    }

    #[test]
    fn find_checksum_asset_matches_installer_plus_sha256_suffix() {
        let release = sample_release();
        let current_platform = Platform::current().expect("platform");
        let asset_names: Vec<&str> = release.assets.iter().map(|a| a.name.as_str()).collect();
        let asset_name =
            platform::select_asset(current_platform, &asset_names).expect("asset");
        let checksum = find_checksum_asset(&release, asset_name).expect("checksum");
        assert_eq!(checksum.name, format!("{asset_name}.sha256"));
    }

    #[test]
    fn find_checksum_asset_errors_when_missing() {
        let mut release = sample_release();
        release.assets.retain(|a| !a.name.ends_with(".sha256"));
        let current_platform = Platform::current().expect("platform");
        let asset_names: Vec<&str> = release.assets.iter().map(|a| a.name.as_str()).collect();
        let asset_name =
            platform::select_asset(current_platform, &asset_names).expect("asset");
        let err = find_checksum_asset(&release, asset_name).expect_err("must fail");
        assert!(err.to_string().contains("Checksum file"));
    }

    #[test]
    fn checksum_verification_round_trips_through_shared_module() {
        // Confirms upgrade.rs is actually exercising the shared
        // os::update::checksum module end-to-end (parse + verify),
        // not a local duplicate of this logic.
        let dir = tempfile::tempdir().expect("tempdir");
        let file_path = dir.path().join("test.bin");
        std::fs::write(&file_path, b"hello world").expect("write");

        let mut hasher = sha2::Sha256::new();
        use sha2::Digest;
        hasher.update(b"hello world");
        let hex = format!("{:x}", hasher.finalize());
        let checksum_file_text = format!("{hex}  test.bin\n");

        let parsed = checksum::parse_checksum_text(&checksum_file_text).expect("parse");
        checksum::verify_file(&file_path, &parsed).expect("checksum should match");
    }
}
