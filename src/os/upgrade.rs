use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use tar::Archive;
use zip::ZipArchive;

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

    let expected_asset_name = platform_asset_name();
    let asset = find_release_asset(&release, expected_asset_name)?;
    println!("Downloading asset: {}", asset.name);

    let temp = TempDirGuard::new()?;
    let archive_path = temp.path().join(&asset.name);
    download_to_file(&asset.browser_download_url, &archive_path)?;
    extract_archive(&archive_path, temp.path())?;
    run_platform_installer(temp.path())?;

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

fn platform_asset_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "qbit-windows-setup.zip"
    }
    #[cfg(target_os = "macos")]
    {
        "qbit-macos-setup.tar.gz"
    }
    #[cfg(target_os = "linux")]
    {
        "qbit-linux-setup.tar.gz"
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "qbit-linux-setup.tar.gz"
    }
}

fn find_release_asset<'a>(
    release: &'a GithubRelease,
    expected_name: &str,
) -> Result<&'a GithubAsset> {
    release
        .assets
        .iter()
        .find(|asset| asset.name == expected_name)
        .ok_or_else(|| {
            let available = if release.assets.is_empty() {
                "<no assets>".to_string()
            } else {
                release
                    .assets
                    .iter()
                    .map(|asset| asset.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            anyhow::anyhow!(
                "Release asset `{expected_name}` was not found. Available assets: {available}"
            )
        })
}

fn download_to_file(url: &str, destination: &Path) -> Result<()> {
    let client = Client::builder()
        .build()
        .context("building HTTP client for release download")?;

    let mut response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "qbit-cli-upgrader")
        .send()
        .with_context(|| format!("downloading release archive from {url}"))?
        .error_for_status()
        .with_context(|| format!("failed to download release archive from {url}"))?;

    let mut file = File::create(destination)
        .with_context(|| format!("creating archive file {}", destination.display()))?;

    io::copy(&mut response, &mut file)
        .with_context(|| format!("writing archive to {}", destination.display()))?;
    file.flush()
        .with_context(|| format!("flushing archive {}", destination.display()))?;
    Ok(())
}

fn extract_archive(archive_path: &Path, destination: &Path) -> Result<()> {
    let file_name = archive_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();

    if file_name.ends_with(".zip") {
        return extract_zip(archive_path, destination);
    }
    if file_name.ends_with(".tar.gz") {
        return extract_tar_gz(archive_path, destination);
    }

    bail!(
        "Unsupported release archive format: {}",
        archive_path.display()
    );
}

fn extract_tar_gz(archive_path: &Path, destination: &Path) -> Result<()> {
    let file = File::open(archive_path)
        .with_context(|| format!("opening archive {}", archive_path.display()))?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);
    archive
        .unpack(destination)
        .with_context(|| format!("extracting tar.gz archive into {}", destination.display()))?;
    Ok(())
}

fn extract_zip(archive_path: &Path, destination: &Path) -> Result<()> {
    let file = File::open(archive_path)
        .with_context(|| format!("opening archive {}", archive_path.display()))?;
    let mut zip = ZipArchive::new(file).context("opening zip archive")?;

    for idx in 0..zip.len() {
        let mut entry = zip
            .by_index(idx)
            .with_context(|| format!("reading zip entry #{idx}"))?;
        let Some(rel_path) = entry.enclosed_name().map(|p| p.to_path_buf()) else {
            continue;
        };

        let out_path = destination.join(rel_path);
        if entry.is_dir() {
            fs::create_dir_all(&out_path)
                .with_context(|| format!("creating directory {}", out_path.display()))?;
            continue;
        }

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating directory {}", parent.display()))?;
        }
        let mut out_file = File::create(&out_path)
            .with_context(|| format!("creating extracted file {}", out_path.display()))?;
        io::copy(&mut entry, &mut out_file)
            .with_context(|| format!("extracting file {}", out_path.display()))?;
    }

    Ok(())
}

fn run_platform_installer(extracted_dir: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let script = extracted_dir.join("install.ps1");
        if !script.exists() {
            bail!(
                "Windows installer not found after extraction: {}",
                script.display()
            );
        }

        let shell = if command_exists("pwsh") {
            "pwsh"
        } else if command_exists("powershell") {
            "powershell"
        } else {
            bail!("Neither `pwsh` nor `powershell` is available in PATH.");
        };

        let status = Command::new(shell)
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg(&script)
            .current_dir(extracted_dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .with_context(|| format!("running installer {}", script.display()))?;

        if !status.success() {
            bail!(
                "Installer failed (exit code {}): {}",
                status.code().unwrap_or(1),
                script.display()
            );
        }
    }

    #[cfg(target_os = "macos")]
    {
        let script = extracted_dir.join("install_macos.sh");
        if !script.exists() {
            bail!(
                "macOS installer not found after extraction: {}",
                script.display()
            );
        }

        let status = Command::new("sh")
            .arg(&script)
            .current_dir(extracted_dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .with_context(|| format!("running installer {}", script.display()))?;

        if !status.success() {
            bail!(
                "Installer failed (exit code {}): {}",
                status.code().unwrap_or(1),
                script.display()
            );
        }
    }

    #[cfg(target_os = "linux")]
    {
        let script = extracted_dir.join("install.sh");
        if !script.exists() {
            bail!(
                "Linux installer not found after extraction: {}",
                script.display()
            );
        }

        let status = Command::new("sh")
            .arg(&script)
            .current_dir(extracted_dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .with_context(|| format!("running installer {}", script.display()))?;

        if !status.success() {
            bail!(
                "Installer failed (exit code {}): {}",
                status.code().unwrap_or(1),
                script.display()
            );
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        bail!("qbit upgrade is not supported on this operating system.");
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn command_exists(binary: &str) -> bool {
    Command::new(binary)
        .arg("-Command")
        .arg("$PSVersionTable.PSVersion.ToString()")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
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

    #[test]
    fn find_release_asset_matches_expected_name() {
        let release = GithubRelease {
            tag_name: "v1.0.0".to_string(),
            assets: vec![
                GithubAsset {
                    name: "qbit-linux-setup.tar.gz".to_string(),
                    browser_download_url: "https://example.test/linux".to_string(),
                },
                GithubAsset {
                    name: "qbit-windows-setup.zip".to_string(),
                    browser_download_url: "https://example.test/windows".to_string(),
                },
            ],
        };

        let found = find_release_asset(&release, "qbit-windows-setup.zip").expect("asset");
        assert_eq!(found.browser_download_url, "https://example.test/windows");
    }
}
