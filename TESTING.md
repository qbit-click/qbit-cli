# Testing Guide

## Run the suite

- Run all tests:
  - `cargo test`
- Run integration tests only:
  - `cargo test --test cli_help --test cli_run --test update_check`
- Run just the update-check integration tests:
  - `cargo test --test update_check`
- Run snapshot tests only:
  - `cargo test snapshot_`

## Test categories in this repo

- Unit tests (inside modules):
  - `src/config.rs`: YAML/TOML parsing for scripts/install entries.
  - `src/utils/shell.rs`: platform shell selection and empty-command errors.
  - `src/os/install.rs`: target parsing, command planning, and dry-run behavior.
  - `src/developers/js.rs`: scaffold generation and package-manager command logic.
  - `src/developers/py.rs`, `src/utils/python.rs`: path and command-splitting helpers.
  - `src/os/upgrade.rs`: version parsing, asset/checksum lookup (delegates to `os::update::platform`/`os::update::checksum`).
  - `src/os/update/cache.rs`: 24h cache load/save, atomic-write safety, corrupt-file recovery.
  - `src/os/update/github.rs`: GitHub client behavior against a fake/injected client — no real network calls.
  - `src/os/update/platform.rs`: OS/asset-name matching, ambiguous-match rejection.
  - `src/os/update/checksum.rs`: SHA-256 parsing and verification.
  - `src/os/update/mod.rs`: the `check_if_due` facade — disabled flag, not-due skip, fresh/not-modified outcomes, failure handling.
- Integration tests (`tests/`):
  - `tests/cli_help.rs`: CLI help smoke test.
  - `tests/cli_run.rs`: `qbit run` success and failure paths.
  - `tests/update_check.rs`: black-box tests of the 24h auto-update check against the real compiled binary — disable flag, first run, cache-not-due, expired cache, stdout cleanliness, and that manual `qbit upgrade` is unaffected by the disable flag.
- Property-based tests:
  - `src/os/install.rs`: `parse_target_spec` robustness over random inputs.
- Snapshot tests:
  - `snapshots/js_qbit_yml_template.snap`
  - `snapshots/js_npm_run_args.snap`

## Packaging tests

Packaging (DEB/PKG/MSI) is validated by `ci.yml` on every PR, on native runners per OS:
- `scripts/package/build-linux-deb.sh` — builds a test `.deb`, then `dpkg -i`/`qbit --help`/`dpkg -r` on a real Ubuntu runner.
- `scripts/package/build-macos.sh` — builds a test `.pkg`, then `installer -pkg`/`qbit --help` on a real macOS runner.
- `scripts/package/build-windows.ps1` — builds a test `.msi` (requires `dotnet tool install --global wix --version 5.0.2`), then `msiexec /i`/`qbit --help` on a real Windows runner.

CI never creates a GitHub Release or tag — packaging output is discarded at the end of the job. Only pushing a `v*` tag triggers `release.yml`, which publishes real, versioned installers plus `.sha256` checksums.

To build and test a package locally, see `packaging/README.md`.

## Update-check cache testing

`tests/update_check.rs` uses the `QBIT_UPDATE_CACHE_DIR` environment variable (test-only, not a documented user-facing setting) to redirect the update-check cache to an isolated temp directory, so tests never read or write real user state. Tests use `QBIT_UPGRADE_REPO` pointed at a nonexistent repository to force fast, deterministic network-error paths without depending on live GitHub API availability.

## Fake executable strategy (JS tests)

JavaScript command tests do not require real `npm/pnpm/yarn/bun`.

- Tests create a temporary `fakebin/` directory and prepend it to `PATH`.
- Unix:
  - create executable scripts named `npm`, `pnpm`, `yarn`, `bun`.
  - scripts respond to `--version` and log invocations.
- Windows:
  - create `*.cmd` shims (`npm.cmd`, etc.) with the same behavior.
- Tests set `QBIT_FAKE_LOG` to capture invoked args and assert command construction.

All env-var and `current_dir` mutating tests are serialized with `serial_test` to avoid races.
