//! GitHub Releases API access for the periodic update check.
//!
//! Separate from `src/os/upgrade.rs`'s own GitHub calls: this module
//! is specifically for the lightweight, frequent "is there a newer
//! version" check, and is built to be mockable in tests (no test here
//! should make a real network request).

use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);
const USER_AGENT: &str = "qbit-cli-update-check";

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseSummary {
    pub tag_name: String,
}

/// Result of a check-for-update call against GitHub.
pub enum CheckOutcome {
    /// Got a fresh release body; here's the tag and the ETag to cache
    /// for next time.
    Fresh {
        release: ReleaseSummary,
        etag: Option<String>,
    },
    /// GitHub responded 304 Not Modified (our cached ETag is still
    /// current) — nothing changed since last check.
    NotModified,
}

/// Abstraction over the HTTP call so tests can inject a fake
/// implementation instead of hitting api.github.com.
pub trait GithubClient {
    fn get_latest_release(
        &self,
        repository: &str,
        etag: Option<&str>,
    ) -> Result<CheckOutcome>;
}

/// Real implementation using `reqwest::blocking`, with a short timeout
/// since this call must never noticeably delay CLI startup.
pub struct RealGithubClient {
    client: reqwest::blocking::Client,
}

impl RealGithubClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .context("building HTTP client for update check")?;
        Ok(Self { client })
    }
}

impl GithubClient for RealGithubClient {
    fn get_latest_release(
        &self,
        repository: &str,
        etag: Option<&str>,
    ) -> Result<CheckOutcome> {
        let url = format!("https://api.github.com/repos/{repository}/releases/latest");

        let mut request = self
            .client
            .get(&url)
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .header(reqwest::header::ACCEPT, "application/vnd.github+json");

        if let Some(etag) = etag {
            request = request.header(reqwest::header::IF_NONE_MATCH, etag);
        }

        let response = request
            .send()
            .with_context(|| format!("requesting latest release for {repository}"))?;

        if response.status() == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(CheckOutcome::NotModified);
        }

        // Respect GitHub's rate-limit signalling: if we're rate
        // limited, surface that clearly rather than a generic HTTP
        // error, so the caller can decide to just skip silently.
        if response.status() == reqwest::StatusCode::FORBIDDEN
            || response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS
        {
            anyhow::bail!("GitHub API rate limit reached (status {})", response.status());
        }

        let response = response
            .error_for_status()
            .with_context(|| format!("GitHub API returned an error for repo {repository}"))?;

        let new_etag = response
            .headers()
            .get(reqwest::header::ETAG)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let release: ReleaseSummary = response
            .json()
            .context("decoding GitHub release response JSON")?;

        Ok(CheckOutcome::Fresh {
            release,
            etag: new_etag,
        })
    }
}

#[cfg(test)]
pub mod test_support {
    use super::*;
    use std::cell::RefCell;

    /// A fake client for tests — never touches the network. Returns a
    /// preconfigured outcome or error. Panics if called more than
    /// once, since each test should set up exactly the calls it expects.
    pub struct FakeGithubClient {
        pub outcome: RefCell<Option<Result<CheckOutcome>>>,
        pub last_etag_sent: RefCell<Option<String>>,
    }

    impl FakeGithubClient {
        pub fn returning(outcome: Result<CheckOutcome>) -> Self {
            Self {
                outcome: RefCell::new(Some(outcome)),
                last_etag_sent: RefCell::new(None),
            }
        }
    }

    impl GithubClient for FakeGithubClient {
        fn get_latest_release(
            &self,
            _repository: &str,
            etag: Option<&str>,
        ) -> Result<CheckOutcome> {
            *self.last_etag_sent.borrow_mut() = etag.map(|s| s.to_string());
            self.outcome
                .borrow_mut()
                .take()
                .expect("FakeGithubClient.get_latest_release called more than once")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::FakeGithubClient;
    use super::*;

    #[test]
    fn fresh_outcome_carries_release_and_etag() {
        let client = FakeGithubClient::returning(Ok(CheckOutcome::Fresh {
            release: ReleaseSummary {
                tag_name: "v1.2.3".to_string(),
            },
            etag: Some("\"xyz\"".to_string()),
        }));

        let outcome = client.get_latest_release("qbit-click/qbit-cli", None).unwrap();
        match outcome {
            CheckOutcome::Fresh { release, etag } => {
                assert_eq!(release.tag_name, "v1.2.3");
                assert_eq!(etag.as_deref(), Some("\"xyz\""));
            }
            CheckOutcome::NotModified => panic!("expected Fresh"),
        }
    }

    #[test]
    fn not_modified_outcome_is_passed_through() {
        let client = FakeGithubClient::returning(Ok(CheckOutcome::NotModified));
        let outcome = client
            .get_latest_release("qbit-click/qbit-cli", Some("\"cached-etag\""))
            .unwrap();
        assert!(matches!(outcome, CheckOutcome::NotModified));
    }

    #[test]
    fn etag_is_forwarded_to_client() {
        let client = FakeGithubClient::returning(Ok(CheckOutcome::NotModified));
        let _ = client.get_latest_release("qbit-click/qbit-cli", Some("\"abc\""));
        assert_eq!(
            client.last_etag_sent.borrow().as_deref(),
            Some("\"abc\"")
        );
    }
}
