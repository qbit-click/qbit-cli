<!-- Copilot / agent instructions for qbit-cli -->
# Quick agent guidance — qbit-cli

This project is a small Rust-based CLI for multi-language project workflows (Python, JavaScript, Dart), OS package installation, and self-updating via native OS installers. The goal of these instructions is to help AI coding agents be immediately productive and avoid guesswork.

**Architecture:**
- **`src/cli.rs`:** CLI surface and dispatcher built with `clap`. Look here to see subcommands and expected UX (e.g. `py init`, `install`, `upgrade`, `run`).
- **`src/developers/*`**: Per-language command implementations. Example: `src/developers/py.rs` contains the Python flows (creates `requirements.txt` and a `venv`).
- **`src/utils/python.rs`:** Python discovery logic. It tries `QBIT_PY` env var first, then platform-ordered candidates (Windows: `py -3`, `py`, `python`, `python3`). Use `find_python()` when you need to locate an interpreter.
- **`src/os/install.rs`, `src/os/package_manager.rs`:** system package installation (`qbit install <name>`) — detects the native package manager (apt/dnf/pacman/zypper/brew/winget/choco/scoop) and installs the requested tool. This is a different concern from installing *qbit itself* — do not confuse the two.
- **`src/os/upgrade.rs`:** the `qbit upgrade` command. Downloads the latest official GitHub release's installer for the current OS, verifies its SHA-256 checksum (via `src/os/update/checksum.rs`), selects the correct asset (via `src/os/update/platform.rs`), and invokes the OS's native installer (`dpkg`, `installer`, `msiexec`) directly — no archive extraction, no bundled install scripts.
- **`src/os/update/`:** the periodic (max once per 24h) "is a newer version available" check, run automatically at CLI startup, separate from `upgrade.rs`'s actual install logic:
  - `mod.rs` — facade; the only function other code should call is `check_if_due()`.
  - `cache.rs` — persists last-checked timestamp/version/ETag; atomic writes; a corrupt or missing cache file is never fatal.
  - `github.rs` — `GithubClient` trait with a real (`reqwest`) and fake (test-only) implementation, so nothing in the test suite hits the real GitHub API.
  - `platform.rs` — maps the current OS to its installer asset naming pattern (`Platform::current()`, `select_asset()`); fails closed on an ambiguous match rather than guessing.
  - `checksum.rs` — SHA-256 parsing/verification, shared by both `upgrade.rs` and the periodic check.

**Binary naming (do not get this wrong):**
- Cargo `[package] name` is `qbit-cli`. The **installed binary** is `qbit` (`qbit.exe` on Windows), set via `[[bin]] name = "qbit"` in `Cargo.toml`. These are intentionally different — never rename the package to `qbit`, and never let a script/test reference `qbit-cli` as the binary to execute.
- Release installer filenames use the `qbit-cli-<version>-...` prefix (e.g. `qbit-cli-1.2.3-windows-x64.msi`) — this is also intentionally different from the `qbit` command name. See `packaging/README.md` for the full convention.
- No `qbit-cli` alias/symlink is installed alongside `qbit`. If a change would add one, that needs explicit sign-off, not silent implementation.

**Build & run (what humans use):**
- Build: `cargo build`.
- Run the CLI locally: `cargo run -- --help` or `cargo run -- py init` (pass subcommands after `--`).
- Tests: `cargo test` (unit tests live next to the code they cover; integration tests live in `tests/`). See `TESTING.md` for the full breakdown by category.
- Lint: `cargo clippy --all-targets --all-features -- -D warnings` — this is enforced in CI with zero tolerance for warnings, including `dead_code`. If you add a struct field or function that isn't yet called anywhere, either wire it into real usage or the build will fail — don't reach for `#[allow(dead_code)]` as a first resort.

**Repository conventions / patterns**
- Single-binary workspace. `main.rs` calls `os::update::check_if_due()` (best-effort, silent on failure, stderr-only messaging) before `cli::run()`, and most work happens in `developers`/`os` modules.
- CLI dispatching uses `clap` derive types in `src/cli.rs`. Modify subcommands there when adding new top-level commands.
- Language features live under `src/developers/<lang>.rs`. Prefer implementing flow logic here (business logic) and keep `cli.rs` as the thin dispatcher.
- Use `anyhow::Result` for fallible operations (consistent with existing files).
- Packaging logic (building `.deb`/`.pkg`/`.msi`) lives in `scripts/package/*.sh`/`*.ps1` and is called identically by `ci.yml` (throwaway test builds on every PR) and `release.yml` (real published builds on `v*` tags). GitHub Actions orchestrates; it should never contain packaging logic inline in YAML.
- `ci.yml` never creates a GitHub Release or a tag. Only a `v*` tag push triggers `release.yml`.

**Notable behaviours to preserve or follow**
- `py init` will call `developers::py::init()` which:
  - Ensures `requirements.txt` exists (creates a minimal file if not).
  - Uses `utils::python::find_python()` to pick an interpreter.
  - Creates a `venv` using the interpreter (`<interpreter> -m venv venv`).
- `QBIT_PY` env var is respected by `find_python()` — tests or CI that need a specific interpreter should set this.
- `qbit upgrade` respects `QBIT_UPGRADE_REPO` (defaults to `qbit-click/qbit-cli`) — tests use this to point at a nonexistent repo for deterministic failure paths without live network dependency.
- The periodic update check respects `CHECK_UPDATE_DISABLE_QBIT=1` to disable itself entirely — this must never affect a manually-invoked `qbit upgrade`, which always runs regardless of this flag.
- The update-check cache honors `QBIT_UPDATE_CACHE_DIR` as a test-only override — not a documented user-facing setting, but useful if you're debugging or writing new tests in this area.

**When editing or adding features**
- Update `src/cli.rs` first to expose the command signature and help text, then implement the logic in `src/developers/*` or `src/os/*`.
- Keep OS-specific command logic in `src/utils` or `src/os` if it will be reused by multiple languages.
- If you're touching upgrade/update logic, prefer extending `src/os/update/*`'s shared modules over adding new logic directly to `src/os/upgrade.rs` — `upgrade.rs` should stay a thin orchestrator that calls into `platform`, `checksum`, and (for the periodic check) `update::mod`.
- Add small, focused unit tests next to changed code. Prefer testing plumbing in `utils`/`os/update` and functional behaviour in `developers`. For anything that spawns the compiled binary, follow the `assert_cmd`-based pattern in `tests/cli_help.rs`, `tests/cli_run.rs`, and `tests/update_check.rs`.

**Examples (exact commands you can use)**
- Build and run the help: `cargo run -- --help`
- Run the Python initializer: `cargo run -- py init`
- Emulate a user with a specific Python: `QBIT_PY="python3.11" cargo run -- py init` (PowerShell on Windows: `powershell -Command "$env:QBIT_PY='python3.11'; cargo run -- py init"`)
- Run just the update-check integration tests: `cargo test --test update_check`

**Files worth opening first**
- `src/cli.rs` — command surface
- `src/os/upgrade.rs` — self-update entry point
- `src/os/update/mod.rs` — periodic update-check facade
- `src/developers/py.rs` — Python workflows
- `src/utils/python.rs` — interpreter discovery
- `Cargo.toml` — deps and the `[[bin]] name = "qbit"` target
- `AGENTS.md` — naming conventions and full directory responsibilities
- `packaging/README.md` — local packaging build/verify commands

If anything in these notes is unclear or you need examples for additional flows (JS, Dart), tell me which area to expand and I will update this file.
