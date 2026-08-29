# Troubleshooting

## `qbit` is not found after installation

Open a new terminal and run `qbit --help`. Windows per-user installation adds `%LOCALAPPDATA%\Qbit\bin` to User PATH. macOS/Linux installers create `/usr/local/bin/qbit`.

## No system package manager is detected

Install a supported manager or explicitly select one:

```bash
QBIT_PACKAGE_MANAGER=brew qbit install node
```

If the selected executable is unavailable, Qbit fails instead of silently switching managers.

## The wrong JavaScript manager is selected

Check the project lockfile and `QBIT_JS_PM`:

```bash
QBIT_JS_PM=npm qbit js run build
```

A detected lockfile whose manager executable is missing is treated as an error.

## Python is unavailable

```bash
qbit install python
qbit py init
```

Make sure the interpreter is visible in PATH after installation.

## Dart is unavailable

Qbit requires `dart --version` to succeed. Install the Dart SDK and fix PATH before retrying `qbit dart init`.

## `qbit run` cannot find a script

Run the command from the project root, verify `qbit.yml`, `qbit.yaml`, or `qbit.toml` exists, and check the script name under `scripts`.

## `qbit upgrade` cannot find a release asset

Official releases must contain the exact platform asset name: `qbit-windows-setup.zip`, `qbit-macos-setup.tar.gz`, or `qbit-linux-setup.tar.gz`.

When reporting a problem, include the OS, command, `qbit --help` output, and complete error text. Do not include credentials or secrets.
