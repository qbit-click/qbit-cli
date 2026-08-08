//! Facade for the periodic (max once per 24h) update-availability
//! check. This module is deliberately separate from `upgrade.rs`:
//! this one only *checks and reports*; `upgrade.rs` is what actually
//! downloads and installs.
//!
//! Consumers (`main.rs`) should only call [`check_if_due`]. Everything
//! else here is internal plumbing.

pub mod cache;
pub mod checksum;
pub mod github;
pub mod platform;

use semver::Version;

use cache::UpdateCache;
use github::{CheckOutcome, GithubClient, RealGithubClient};

const DEFAULT_REPOSITORY: &str = "qbit-click/qbit-cli";
const DISABLE_ENV_VAR: &str = "CHECK_UPDATE_DISABLE_QBIT";

/// Result of a check attempt, for the caller to decide what (if
/// anything) to print. All variants are non-fatal by design — this
/// function should never be treated as something that can "fail" the
/// CLI's main dispatch.
pub enum CheckResult {
    /// Not due yet (checked within the last 24h) — nothing done.
    NotDue,
    /// Disabled via env var — nothing done.
    Disabled,
    /// Checked successfully; a newer version is available.
    UpdateAvailable { latest: String },
    /// Checked successfully; already up to date.
    UpToDate,
    /// The check itself failed (network error, parse error, rate
    /// limit, etc). Contains a message for stderr-only display. This
    /// must NEVER propagate as a hard error to the caller — it is
    /// always something to silently continue past, optionally logging
    /// to stderr.
    CheckFailed(String),
}

/// The single entry point `main.rs` should call before normal command
/// dispatch. This function:
/// - returns immediately if disabled via env var
/// - returns immediately if not yet due (checked cache says so)
/// - otherwise performs a short-timeout GitHub check and updates the
///   cache
///
/// This function does not print anything itself — callers decide
/// whether/how to surface the result (per the requirement that any
/// message goes to stderr only, never stdout).
pub fn check_if_due() -> CheckResult {
    if is_disabled() {
        return CheckResult::Disabled;
    }

    let cache_path = cache::default_cache_path();
    let existing = UpdateCache::load(&cache_path);
    let now = cache::now_unix();

    if !existing.is_due(now) {
        return CheckResult::NotDue;
    }

    let client = match RealGithubClient::new() {
        Ok(c) => c,
        Err(e) => return CheckResult::CheckFailed(e.to_string()),
    };

    check_with_client(&client, &existing, &cache_path, now)
}

/// Core logic factored out from [`check_if_due`] so it can be
/// exercised in tests with a fake `GithubClient`, without touching the
/// real cache file location or the network.
pub fn check_with_client(
    client: &dyn GithubClient,
    existing: &UpdateCache,
    cache_path: &std::path::Path,
    now: u64,
) -> CheckResult {
    let repository = repository_override();

    let outcome = match client.get_latest_release(&repository, existing.etag.as_deref()) {
        Ok(outcome) => outcome,
        Err(e) => return CheckResult::CheckFailed(e.to_string()),
    };

    match outcome {
        CheckOutcome::NotModified => {
            // Nothing changed since last check, but we did successfully
            // check — refresh the timestamp so we don't check again for
            // another 24h, while keeping the same cached version/etag.
            let refreshed = UpdateCache {
                last_checked_unix: now,
                ..existing.clone()
            };
            let _ = refreshed.save(cache_path); // best-effort; ignore failure

            match &existing.latest_seen_version {
                Some(v) if is_newer(v, current_version_str()) => {
                    CheckResult::UpdateAvailable { latest: v.clone() }
                }
                _ => CheckResult::UpToDate,
            }
        }
        CheckOutcome::Fresh { release, etag } => {
            let refreshed = UpdateCache {
                last_checked_unix: now,
                latest_seen_version: Some(release.tag_name.clone()),
                etag,
            };
            let _ = refreshed.save(cache_path); // best-effort; ignore failure

            if is_newer(&release.tag_name, current_version_str()) {
                CheckResult::UpdateAvailable {
                    latest: release.tag_name,
                }
            } else {
                CheckResult::UpToDate
            }
        }
    }
}

fn is_disabled() -> bool {
    std::env::var(DISABLE_ENV_VAR)
        .map(|v| v.trim() == "1")
        .unwrap_or(false)
}

fn repository_override() -> String {
    std::env::var("QBIT_UPGRADE_REPO")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_REPOSITORY.to_string())
}

fn current_version_str() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Compares a possibly-"v"-prefixed tag against the current build
/// version. Any parse failure is treated as "not newer" (fail safe:
/// we'd rather silently skip a malformed comparison than falsely
/// claim an update is available).
fn is_newer(candidate_tag: &str, current: &str) -> bool {
    let normalize = |s: &str| s.trim().strip_prefix('v').unwrap_or(s.trim()).to_string();

    let candidate = match Version::parse(&normalize(candidate_tag)) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let current = match Version::parse(&normalize(current)) {
        Ok(v) => v,
        Err(_) => return false,
    };

    candidate > current
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::update::github::test_support::FakeGithubClient;
    use tempfile::tempdir;

    #[test]
    fn is_newer_detects_greater_version() {
        assert!(is_newer("v2.0.0", "1.0.0"));
        assert!(is_newer("2.0.0", "1.9.9"));
    }

    #[test]
    fn is_newer_false_for_equal_or_lower() {
        assert!(!is_newer("1.0.0", "1.0.0"));
        assert!(!is_newer("0.9.0", "1.0.0"));
    }

    #[test]
    fn is_newer_false_on_unparseable_input() {
        assert!(!is_newer("not-a-version", "1.0.0"));
    }

    #[test]
    fn disabled_env_var_short_circuits() {
        // SAFETY: this test sets/removes only the specific env var
        // under test, and does not run concurrently with other tests
        // that read or write the same variable.
        unsafe {
            std::env::set_var(DISABLE_ENV_VAR, "1");
        }
        let result = check_if_due();
        unsafe {
            std::env::remove_var(DISABLE_ENV_VAR);
        }
        assert!(matches!(result, CheckResult::Disabled));
    }

    #[test]
    fn fresh_outcome_newer_version_reports_update_available() {
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("cache.json");
        let existing = UpdateCache::default();

        let client = FakeGithubClient::returning(Ok(CheckOutcome::Fresh {
            release: github::ReleaseSummary {
                tag_name: "v999.0.0".to_string(),
            },
            etag: Some("\"e1\"".to_string()),
        }));

        let result = check_with_client(&client, &existing, &cache_path, 1_000_000);
        match result {
            CheckResult::UpdateAvailable { latest } => assert_eq!(latest, "v999.0.0"),
            _ => panic!("expected UpdateAvailable"),
        }

        let saved = UpdateCache::load(&cache_path);
        assert_eq!(saved.last_checked_unix, 1_000_000);
        assert_eq!(saved.latest_seen_version.as_deref(), Some("v999.0.0"));
    }

    #[test]
    fn not_modified_outcome_refreshes_timestamp_without_changing_version() {
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("cache.json");
        let existing = UpdateCache {
            last_checked_unix: 500_000,
            latest_seen_version: Some("v0.0.1".to_string()),
            etag: Some("\"cached\"".to_string()),
        };

        let client = FakeGithubClient::returning(Ok(CheckOutcome::NotModified));
        let result = check_with_client(&client, &existing, &cache_path, 1_000_000);
        assert!(matches!(result, CheckResult::UpToDate));

        let saved = UpdateCache::load(&cache_path);
        assert_eq!(saved.last_checked_unix, 1_000_000);
        assert_eq!(saved.latest_seen_version.as_deref(), Some("v0.0.1"));
    }

    #[test]
    fn network_error_is_reported_as_check_failed_not_panic() {
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("cache.json");
        let existing = UpdateCache::default();

        let client = FakeGithubClient::returning(Err(anyhow::anyhow!("simulated network error")));
        let result = check_with_client(&client, &existing, &cache_path, 1_000_000);
        match result {
            CheckResult::CheckFailed(msg) => assert!(msg.contains("simulated network error")),
            _ => panic!("expected CheckFailed"),
        }
    }
}
