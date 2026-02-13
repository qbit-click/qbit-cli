<!-- Copilot / agent instructions for qbit-cli -->
# Quick agent guidance — qbit-cli

This project is a small Rust-based CLI for multi-language project workflows (currently focused on Python). The goal of these instructions is to help AI coding agents be immediately productive and avoid guesswork.

**Architecture:**
- **`src/cli.rs`:** CLI surface and dispatcher built with `clap`. Look here to see subcommands and expected UX (e.g. `py init`, `py add`, `py remove`).
- **`src/developers/*`**: Per-language command implementations. Example: `src/developers/py.rs` contains the Python flows (creates `requirements.txt` and a `venv`).
- **`src/utils/python.rs`:** Python discovery logic. It tries `QBIT_PY` env var first, then platform-ordered candidates (Windows: `py -3`, `py`, `python`, `python3`). Use `find_python()` when you need to locate an interpreter.
- **`src/os/*` and `src/tools/*`:** OS-level helpers and tool integrations; many files are currently stubs — prefer changing `developers/*` behavior unless adding cross-cutting OS logic.

**Build & run (what humans use):**
- Build: `cargo build`.
- Run the CLI locally: `cargo run -- --help` or `cargo run -- py init` (pass subcommands after `--`).
- Tests: no tests present currently; add unit tests under `src/` and run with `cargo test`.

**Repository conventions / patterns**
- Single-binary workspace. `main.rs` calls `cli::run()` and most work is in `developers` modules.
- CLI dispatching uses `clap` derive types in `src/cli.rs`. Modify subcommands there when adding new top-level commands.
- Language features live under `src/developers/<lang>.rs`. Prefer implementing flow logic here (business logic) and keep `cli.rs` as the thin dispatcher.
- Use `anyhow::Result` for fallible operations (consistent with existing files).

**Notable behaviours to preserve or follow**
- `py init` will call `developers::py::init()` which:
  - Ensures `requirements.txt` exists (creates a minimal file if not).
  - Uses `utils::python::find_python()` to pick an interpreter.
  - Creates a `venv` using the interpreter (`<interpreter> -m venv venv`).
- `QBIT_PY` env var is respected by `find_python()` — tests or CI that need a specific interpreter should set this.

**When editing or adding features**
- Update `src/cli.rs` first to expose the command signature and help text, then implement the logic in `src/developers/*`.
- Keep OS-specific command logic in `src/utils` or `src/os` if it will be reused by multiple languages.
- Add small, focused unit tests next to changed code. Prefer testing plumbing in `utils` and functional behaviour in `developers`.

**Examples (exact commands you can use)**
- Build and run the help: `cargo run -- --help`
- Run the Python initializer: `cargo run -- py init`
- Emulate a user with a specific Python: `QBIT_PY="python3.11" cargo run -- py init` (PowerShell on Windows: `powershell -Command "$env:QBIT_PY='python3.11'; cargo run -- py init"`)

**Files worth opening first**
- `src/cli.rs` — command surface
- `src/developers/py.rs` — Python workflows
- `src/utils/python.rs` — interpreter discovery
- `Cargo.toml` — deps (`clap`, `anyhow`, `notify`)

If anything in these notes is unclear or you need examples for additional flows (JS, Dart), tell me which area to expand and I will update this file.
