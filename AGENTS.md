# Repository Guidelines

## Project Structure & Module Organization

- Core Rust sources live in `src/`. `main.rs`, `cli.rs`, and `config.rs` define the primary CLI surface and configuration handling.
- Language- and tool-specific helpers live in `src/developers/`, `src/tools/`, `src/utils/`, and OS integration in `src/os/`.
- The development-only binary `src/bin/qbit-dev.rs` is for local tooling and experiments.
- `assets/` holds CLI assets (for example `assets/icon.svg`). `dev-sandbox/` contains a lightweight Node/JS sandbox driven by `qbit.yml` for trying workflows.
- GitHub Actions workflows are under `.github/workflows/`, and helper scripts (if any) belong in `scripts/`.

## Build, Test, and Development Commands

- Build debug: `cargo build`
- Build release binary: `cargo build --release`
- Run CLI locally: `cargo run -- --help` or `cargo run -- <subcommand> ...`
- Run dev binary: `cargo run --bin qbit-dev -- --help`
- Run tests (unit and integration): `cargo test`

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

