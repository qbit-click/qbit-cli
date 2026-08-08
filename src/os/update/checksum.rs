//! SHA-256 checksum verification for downloaded installers. Shared by
//! `upgrade.rs`. A missing or mismatched checksum must always stop
//! the upgrade — this module has no "skip verification" mode.

use std::fs::File;
use std::io;
use std::path::Path;

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};

/// Parses a `sha256sum`-style checksum file's contents ("<hex>  <filename>")
/// and returns just the lowercase hex digest, validated as a real
/// 64-character SHA-256 hex string.
pub fn parse_checksum_text(text: &str) -> Result<String> {
    let hex = text
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow::anyhow!("checksum file was empty"))?
        .to_lowercase();

    if hex.len() != 64 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("checksum file did not contain a valid 64-character SHA-256 hex digest: `{hex}`");
    }

    Ok(hex)
}

/// Hashes the file at `file_path` and compares against
/// `expected_hex`. Fails loudly on any mismatch.
pub fn verify_file(file_path: &Path, expected_hex: &str) -> Result<()> {
    let mut file = File::open(file_path)
        .with_context(|| format!("opening {} for hashing", file_path.display()))?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)
        .with_context(|| format!("hashing {}", file_path.display()))?;
    let actual_hex = format!("{:x}", hasher.finalize());

    if actual_hex != expected_hex {
        bail!(
            "Checksum mismatch for {}.\n  expected: {expected_hex}\n  actual:   {actual_hex}\n\
             Refusing to install a file that does not match its published checksum.",
            file_path.display()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn parse_checksum_text_extracts_hex_from_sha256sum_format() {
        let hex = "a".repeat(64);
        let line = format!("{hex}  qbit-cli_1.2.3_amd64.deb\n");
        assert_eq!(parse_checksum_text(&line).unwrap(), hex);
    }

    #[test]
    fn parse_checksum_text_rejects_empty_input() {
        assert!(parse_checksum_text("").is_err());
    }

    #[test]
    fn parse_checksum_text_rejects_wrong_length() {
        assert!(parse_checksum_text("abc123  file.deb").is_err());
    }

    #[test]
    fn parse_checksum_text_rejects_non_hex() {
        let bad = "g".repeat(64);
        assert!(parse_checksum_text(&format!("{bad}  file.deb")).is_err());
    }

    #[test]
    fn verify_file_accepts_matching_hash() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.bin");
        std::fs::write(&path, b"hello world").unwrap();

        let mut hasher = Sha256::new();
        hasher.update(b"hello world");
        let expected = format!("{:x}", hasher.finalize());

        verify_file(&path, &expected).unwrap();
    }

    #[test]
    fn verify_file_rejects_mismatched_hash() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.bin");
        std::fs::write(&path, b"hello world").unwrap();

        let err = verify_file(&path, &"0".repeat(64)).unwrap_err();
        assert!(err.to_string().contains("Checksum mismatch"));
    }
}
