# Testing Guide

## Run the suite

- Run all tests:
  - `cargo test`
- Run integration tests only:
  - `cargo test --test cli_help --test cli_run`
- Run snapshot tests only:
  - `cargo test snapshot_`

## Test categories in this repo

- Unit tests (inside modules):
  - `src/config.rs`: YAML/TOML parsing for scripts/install entries.
  - `src/utils/shell.rs`: platform shell selection and empty-command errors.
  - `src/os/install.rs`: target parsing, command planning, and dry-run behavior.
  - `src/developers/js.rs`: scaffold generation and package-manager command logic.
  - `src/developers/py.rs`, `src/utils/python.rs`: path and command-splitting helpers.
- Integration tests (`tests/`):
  - `tests/cli_help.rs`: CLI help smoke test.
  - `tests/cli_run.rs`: `qbit run` success and failure paths.
- Property-based tests:
  - `src/os/install.rs`: `parse_target_spec` robustness over random inputs.
- Snapshot tests:
  - `snapshots/js_qbit_yml_template.snap`
  - `snapshots/js_npm_run_args.snap`

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
