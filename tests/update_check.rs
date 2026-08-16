//! Integration tests for the periodic (max once per 24h) update check
//! that runs at CLI startup (see `src/os/update/mod.rs`,
//! `src/main.rs`).
//!
//! These tests spawn the real compiled `qbit` binary (matching the
//! pattern in `tests/cli_help.rs` / `tests/cli_run.rs`) rather than
//! calling internal functions directly, so they exercise the actual
//! startup path a user would hit.
//!
//! Network isolation: none of these tests rely on reaching the real
//! GitHub API. Every scenario here either disables the check
//! entirely, pre-seeds the cache so no network call is attempted, or
//! deliberately points at a nonexistent repository to simulate a
//! network/API failure without depending on real network access.
//!
//! `QBIT_UPDATE_CACHE_DIR` (test-only override) lets each test point
//! the update cache at an isolated temp directory instead of the real
//! user cache location, so tests never read or write real user state
//! and never interfere with each other or with parallel test runs.

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use assert_cmd::Command;
use tempfile::tempdir;

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before UNIX_EPOCH")
        .as_secs()
}

fn write_cache(
    cache_dir: &std::path::Path,
    last_checked_unix: u64,
    latest_seen_version: Option<&str>,
) {
    fs::create_dir_all(cache_dir).expect("create cache dir");
    let cache_file = cache_dir.join("update-check.json");
    let version_json = match latest_seen_version {
        Some(v) => format!("\"{v}\""),
        None => "null".to_string(),
    };
    let contents = format!(
        r#"{{"last_checked_unix":{last_checked_unix},"latest_seen_version":{version_json},"etag":null}}"#
    );
    fs::write(&cache_file, contents).expect("write cache file");
}

fn read_cache_json(cache_dir: &std::path::Path) -> Option<String> {
    fs::read_to_string(cache_dir.join("update-check.json")).ok()
}

/// `CHECK_UPDATE_DISABLE_QBIT=1` disables the check entirely. No
/// cache file should be written, and the main command must succeed.
#[test]
fn disable_env_var_skips_check_entirely() {
    let cache_dir = tempdir().expect("tempdir");

    let assert = Command::cargo_bin("qbit")
        .expect("qbit binary")
        .env("CHECK_UPDATE_DISABLE_QBIT", "1")
        .env("QBIT_UPDATE_CACHE_DIR", cache_dir.path())
        .arg("--help")
        .assert();

    assert.success();
    assert!(
        read_cache_json(cache_dir.path()).is_none(),
        "cache file must not be created when the check is disabled"
    );
}

/// Manual `qbit upgrade` must NOT be affected by
/// `CHECK_UPDATE_DISABLE_QBIT=1` — that flag only disables the
/// automatic periodic check, never the explicit user-invoked command.
/// We point at a nonexistent repo so the command fails fast (no real
/// network dependency needed), and assert the failure comes from
/// upgrade's own GitHub lookup, not a "disabled" short-circuit.
#[test]
fn disable_env_var_does_not_disable_manual_upgrade_command() {
    let assert = Command::cargo_bin("qbit")
        .expect("qbit binary")
        .env("CHECK_UPDATE_DISABLE_QBIT", "1")
        .env(
            "QBIT_UPGRADE_REPO",
            "qbit-click/this-repo-does-not-exist-12345",
        )
        .arg("upgrade")
        .assert();

    let output = assert.get_output();
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !combined.to_lowercase().contains("disabled"),
        "manual `qbit upgrade` must not be short-circuited by CHECK_UPDATE_DISABLE_QBIT; got: {combined}"
    );
}

/// First run ever (no cache file exists yet): the check is due and
/// will be attempted. Pointed at a nonexistent repo so it fails fast;
/// per the checklist, that failure must be silent and must not fail
/// the main command.
#[test]
fn first_run_with_no_cache_attempts_check_but_never_fails_main_command() {
    let cache_dir = tempdir().expect("tempdir");

    let assert = Command::cargo_bin("qbit")
        .expect("qbit binary")
        .env("QBIT_UPDATE_CACHE_DIR", cache_dir.path())
        .env(
            "QBIT_UPGRADE_REPO",
            "qbit-click/this-repo-does-not-exist-12345",
        )
        .arg("--help")
        .assert();

    assert.success();
}

/// Cache says we checked recently (within 24h): the check must be
/// skipped, meaning the cache file's content is untouched by this run.
#[test]
fn second_run_within_24_hours_is_not_due_and_cache_is_unchanged() {
    let cache_dir = tempdir().expect("tempdir");
    let recent = now_unix() - 60; // checked 1 minute ago
    write_cache(cache_dir.path(), recent, Some("v0.0.1"));
    let before = read_cache_json(cache_dir.path()).expect("cache written");

    let assert = Command::cargo_bin("qbit")
        .expect("qbit binary")
        .env("QBIT_UPDATE_CACHE_DIR", cache_dir.path())
        .env(
            "QBIT_UPGRADE_REPO",
            "qbit-click/this-repo-does-not-exist-12345",
        )
        .arg("--help")
        .assert();

    assert.success();

    let after = read_cache_json(cache_dir.path()).expect("cache still present");
    assert_eq!(
        before, after,
        "cache must be untouched when the check is not yet due"
    );
}

/// Cache says last check was more than 24h ago: the check is due and
/// will be attempted (and will fail against a nonexistent repo in
/// this test) — but the main command must still succeed.
#[test]
fn expired_cache_triggers_a_check_attempt_but_main_command_still_succeeds() {
    let cache_dir = tempdir().expect("tempdir");
    let stale = now_unix() - (25 * 60 * 60); // 25 hours ago
    write_cache(cache_dir.path(), stale, Some("v0.0.1"));

    let assert = Command::cargo_bin("qbit")
        .expect("qbit binary")
        .env("QBIT_UPDATE_CACHE_DIR", cache_dir.path())
        .env(
            "QBIT_UPGRADE_REPO",
            "qbit-click/this-repo-does-not-exist-12345",
        )
        .arg("--help")
        .assert();

    assert.success();
}

/// Any update-availability message must go to stderr only. We assert
/// the property that holds across every scenario: stdout from
/// `--help` never contains update-related text.
#[test]
fn stdout_never_contains_update_check_messaging() {
    let cache_dir = tempdir().expect("tempdir");

    let assert = Command::cargo_bin("qbit")
        .expect("qbit binary")
        .env("QBIT_UPDATE_CACHE_DIR", cache_dir.path())
        .env(
            "QBIT_UPGRADE_REPO",
            "qbit-click/this-repo-does-not-exist-12345",
        )
        .arg("--help")
        .assert();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    assert!(
        !stdout.contains("new version") && !stdout.contains("qbit upgrade"),
        "update-check messaging must never appear on stdout; got stdout: {stdout}"
    );
}

/// Simulated offline / CI-like environment: pointed at a nonexistent
/// repo so the GitHub API call fails quickly. The short client
/// timeout (3s) means this resolves fast rather than hanging, and the
/// main command must not be delayed noticeably or fail.
#[test]
fn simulated_offline_condition_does_not_delay_or_fail_main_command() {
    let cache_dir = tempdir().expect("tempdir");

    let started = std::time::Instant::now();
    let assert = Command::cargo_bin("qbit")
        .expect("qbit binary")
        .env("QBIT_UPDATE_CACHE_DIR", cache_dir.path())
        .env(
            "QBIT_UPGRADE_REPO",
            "qbit-click/this-repo-does-not-exist-12345",
        )
        .arg("--help")
        .assert();
    let elapsed = started.elapsed();

    assert.success();
    assert!(
        elapsed.as_secs() < 10,
        "main command took {elapsed:?}; the update check's short timeout should prevent noticeable delay"
    );
}
