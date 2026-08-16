# Repository Guidelines

## Project Structure & Module Organization

- Core Rust sources live in `src/`. `main.rs`, `cli.rs`, and `config.rs` define the primary CLI surface and configuration handling.
- Language- and tool-specific helpers live in `src/developers/`, `src/tools/`, `src/utils/`, and OS integration in `src/os/`.
- `src/os/upgrade.rs` is the `qbit upgrade` command's entry point (downloads, checksum-verifies, and installs the latest release for the current OS). It delegates asset selection and checksum logic to `src/os/update/platform.rs` and `src/os/update/checksum.rs` — do not duplicate that logic in `upgrade.rs`.
- `src/os/update/` is the periodic (max once per 24h) update-availability check, separate from `upgrade.rs`'s actual install logic: `mod.rs` (facade — `check_if_due()`), `cache.rs` (24h cache, atomic writes), `github.rs` (injectable GitHub client for tests), `platform.rs` (OS/asset matching), `checksum.rs` (SHA-256 verification).
- The development-only binary `src/bin/qbit-dev.rs` is for local tooling and experiments.
- `assets/` holds CLI assets and installer icons (`icon.svg`, `icon.ico`, `icon.icns`, `icon.png`). `dev-sandbox/` contains a lightweight Node/JS sandbox driven by `qbit.yml` for trying workflows.
- `packaging/` holds native installer configuration: `packaging/windows/QbitCli.wxs` (WiX MSI source), `packaging/macos/distribution.xml` (PKG distribution), `packaging/linux/debian/control.template` (DEB control file template).
- `scripts/package/` holds the build scripts that turn a compiled binary into an installer: `build-linux-deb.sh`, `build-macos.sh`, `build-windows.ps1`. These are called by both `ci.yml` (throwaway test builds) and `release.yml` (real, published builds) — GitHub Actions orchestrates, it does not contain packaging logic itself.
- `scripts/install/` holds legacy bootstrap-only install/uninstall scripts. These are **not** part of the official release or upgrade path — the native installers (DEB/PKG/MSI) and `qbit upgrade` are canonical. Do not add new functionality here without checking whether it belongs in `packaging/`/`scripts/package/` instead.
- GitHub Actions workflows are under `.github/workflows/` (`ci.yml` runs on every PR and never creates a release or tag; `release.yml` runs only on `v*` tag pushes and publishes real installers).

## Naming Conventions (Packaging)

- Rust package name and crate/repo identity: `qbit-cli`.
- Installed binary name: `qbit` (`qbit.exe` on Windows) — set via `Cargo.toml`'s `[[bin]] name = "qbit"`. Never rename the `[package]` name to match; only the `[[bin]]` target changes.
- Release asset naming: `qbit-cli-<version>-windows-<arch>.msi`, `qbit-cli-<version>-macos-<arch>.pkg`, `qbit-cli_<version>_<arch>.deb`. Asset names never match the installed command name — this is intentional.
- No `qbit-cli` alias is installed alongside `qbit`. If you're tempted to add one for backward compatibility, get explicit sign-off and a removal plan first.

## Build, Test, and Development Commands

- Build debug: `cargo build`
- Build release binary: `cargo build --release`
- Run CLI locally: `cargo run -- --help` or `cargo run -- <subcommand> ...`
- Run dev binary: `cargo run --bin qbit-dev -- --help`
- Run tests (unit and integration): `cargo test`
- Lint (must pass with zero warnings before merge): `cargo clippy --all-targets --all-features -- -D warnings`
- Format: `cargo fmt` (CI enforces `cargo fmt -- --check`)
- Build a local test package (see `packaging/README.md` for full details):
  - Linux: `./scripts/package/build-linux-deb.sh <version> <arch> target/release/qbit dist`
  - macOS: `./scripts/package/build-macos.sh <version> <arch> target/release/qbit dist`
  - Windows: `.\scripts\package\build-windows.ps1 -Version <version> -Arch <arch> -BinaryPath target\release\qbit.exe -OutDir dist`

## Coding Style & Naming Conventions

- Use Rust 4-space indentation and standard `rustfmt` formatting: `cargo fmt` before committing.
- Prefer `snake_case` for modules, functions, and local variables; `PascalCase` for types and enums; `SCREAMING_SNAKE_CASE` for constants.
- Keep module files focused (for example, CLI parsing in `cli.rs`, config loading in `config.rs`) and avoid cross-layer coupling.
- Add documentation comments (`///`) for public functions and types that are part of the CLI behavior.

## Testing Guidelines

- Place unit tests in the same file under `#[cfg(test)] mod tests` near the code they cover.
- Use descriptive test names (for example, `parses_basic_qbit_yaml`, `adds_python_dependency_to_requirements`).
- New features should include tests for both the happy path and primary error cases; run `cargo test` before opening a PR.

## Commit & Pull Request Guidelines

- Use short, imperative commit messages (for example, `Add python init workflow`, `Refine os install error handling`).
- Each PR should have a clear description, mention affected commands (for example, `qbit py init`), and link related issues.
- Include notes on testing performed (`cargo test`, manual CLI scenarios) and, when relevant, example commands in the PR body.

