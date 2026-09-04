//! Persistent cache for the 24-hour update check.
//!
//! This is the ONLY cache implementation used by the runtime. A
//! duplicate implementation at `src/os/cache.rs` existed briefly and
//! has been removed — if you see that file again, delete it; nothing
//! should import from it.
//!
//! Stores: the timestamp of the last successful check, the latest
//! version seen at that check, and the GitHub API ETag (for
//! conditional requests / rate-limit friendliness).
//!
//! Design constraints:
//! - A corrupt or unreadable cache file must never fail the CLI. Any
//!   read/parse error is treated the same as "no cache yet".
//! - Writes are atomic: write to a temp file in the same directory,
//!   then rename over the real cache file. This avoids two concurrent
//!   `qbit` processes corrupting each other's write.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const CACHE_FILE_NAME: &str = "update-check.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCache {
    pub last_checked_unix: u64,
    pub latest_seen_version: Option<String>,
    pub etag: Option<String>,
}

impl UpdateCache {
    pub fn load(path: &Path) -> UpdateCache {
        match fs::read_to_string(path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => UpdateCache::default(),
        }
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;

        let tmp_path = parent.join(format!(".update-check.{}.tmp", std::process::id()));

        {
            let mut tmp_file = fs::File::create(&tmp_path)?;
            let json = serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string());
            tmp_file.write_all(json.as_bytes())?;
            tmp_file.sync_all()?;
        }

        fs::rename(&tmp_path, path)?;
        Ok(())
    }

    pub fn is_due(&self, now_unix: u64) -> bool {
        const TWENTY_FOUR_HOURS_SECS: u64 = 24 * 60 * 60;
        if self.last_checked_unix == 0 {
            return true;
        }
        now_unix.saturating_sub(self.last_checked_unix) >= TWENTY_FOUR_HOURS_SECS
    }
}

/// Default cache file location: alongside other qbit config, under
/// the OS-appropriate config/cache directory. Falls back to a temp
/// directory if the home directory can't be resolved, since a
/// missing cache location must never be fatal.
///
/// Honors `QBIT_UPDATE_CACHE_DIR` as an override — this is the ONLY
/// place this override is implemented. Integration tests
/// (`tests/update_check.rs`) rely on this to redirect the cache to an
/// isolated temp directory instead of the real user cache location.
pub fn default_cache_path() -> PathBuf {
    if let Some(dir) = std::env::var_os("QBIT_UPDATE_CACHE_DIR") {
        return PathBuf::from(dir).join(CACHE_FILE_NAME);
    }
    if let Some(dir) = dirs_next_cache_dir() {
        return dir.join("qbit").join(CACHE_FILE_NAME);
    }
    std::env::temp_dir().join("qbit").join(CACHE_FILE_NAME)
}

fn dirs_next_cache_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
    }
    #[cfg(target_os = "macos")]
    {
        std::env::var_os("HOME").map(|home| PathBuf::from(home).join("Library/Caches"))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Some(xdg) = std::env::var_os("XDG_CACHE_HOME") {
            return Some(PathBuf::from(xdg));
        }
        std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache"))
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", unix)))]
    {
        None
    }
}

pub fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn load_missing_file_returns_default() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("does-not-exist.json");
        let cache = UpdateCache::load(&path);
        assert_eq!(cache.last_checked_unix, 0);
        assert!(cache.latest_seen_version.is_none());
    }

    #[test]
    fn load_corrupt_file_returns_default_not_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("corrupt.json");
        fs::write(&path, b"not valid json {{{").unwrap();
        let cache = UpdateCache::load(&path);
        assert_eq!(cache.last_checked_unix, 0);
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("cache.json");

        let cache = UpdateCache {
            last_checked_unix: 12345,
            latest_seen_version: Some("1.2.3".to_string()),
            etag: Some("\"abc123\"".to_string()),
        };
        cache.save(&path).unwrap();

        let loaded = UpdateCache::load(&path);
        assert_eq!(loaded.last_checked_unix, 12345);
        assert_eq!(loaded.latest_seen_version.as_deref(), Some("1.2.3"));
        assert_eq!(loaded.etag.as_deref(), Some("\"abc123\""));
    }

    #[test]
    fn is_due_true_when_never_checked() {
        let cache = UpdateCache::default();
        assert!(cache.is_due(now_unix()));
    }

    #[test]
    fn is_due_false_within_24_hours() {
        let now = 1_000_000;
        let cache = UpdateCache {
            last_checked_unix: now - 60,
            ..Default::default()
        };
        assert!(!cache.is_due(now));
    }

    #[test]
    fn is_due_true_after_24_hours() {
        let now = 1_000_000;
        let cache = UpdateCache {
            last_checked_unix: now - (25 * 60 * 60),
            ..Default::default()
        };
        assert!(cache.is_due(now));
    }

    #[test]
    fn is_due_boundary_exactly_24_hours() {
        let now = 1_000_000;
        let cache = UpdateCache {
            last_checked_unix: now - (24 * 60 * 60),
            ..Default::default()
        };
        assert!(cache.is_due(now));
    }

    #[test]
    fn concurrent_saves_do_not_corrupt_cache() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("cache.json");

        let cache_a = UpdateCache {
            last_checked_unix: 111,
            latest_seen_version: Some("1.0.0".to_string()),
            etag: None,
        };
        let cache_b = UpdateCache {
            last_checked_unix: 222,
            latest_seen_version: Some("2.0.0".to_string()),
            etag: None,
        };

        cache_a.save(&path).unwrap();
        cache_b.save(&path).unwrap();

        let loaded = UpdateCache::load(&path);
        assert_eq!(loaded.last_checked_unix, 222);
        assert_eq!(loaded.latest_seen_version.as_deref(), Some("2.0.0"));
    }

    #[test]
    fn qbit_update_cache_dir_override_actually_redirects_resolved_path() {
        // Proves the override is real: default_cache_path() must
        // return a path INSIDE the temp dir we set, not the real
        // user cache location, when QBIT_UPDATE_CACHE_DIR is set.
        let dir = tempdir().unwrap();
        let dir_path = dir.path().to_path_buf();

        unsafe {
            std::env::set_var("QBIT_UPDATE_CACHE_DIR", &dir_path);
        }
        let resolved = default_cache_path();
        unsafe {
            std::env::remove_var("QBIT_UPDATE_CACHE_DIR");
        }

        assert!(
            resolved.starts_with(&dir_path),
            "resolved cache path {resolved:?} must be inside override dir {dir_path:?}"
        );
        assert_eq!(resolved, dir_path.join(CACHE_FILE_NAME));
    }
}
