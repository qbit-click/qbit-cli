# Qbit configuration

Qbit reads project configuration from the current directory. It checks `qbit.yml`, `qbit.yaml`, then `qbit.toml`, and uses the first file found.

## YAML

```yaml
scripts:
  dev: "npm run dev"
  verify:
    - "cargo test"
    - "cargo build --release"

install:
  node: "OpenJS.NodeJS"
  python:
    version: "3.12"
    identifiers:
      winget: "Python.Python.3.12"
      brew: "python@3.12"
```

## TOML

```toml
[scripts]
dev = "npm run dev"
verify = ["cargo test", "cargo build --release"]

[install]
node = "OpenJS.NodeJS"

[install.python]
version = "3.12"

[install.python.identifiers]
winget = "Python.Python.3.12"
brew = "python@3.12"
```

`scripts` values can be a single command or an ordered command list. `install` entries can be one shared identifier or a detailed version plus per-manager identifier map.

## Environment variables

| Variable | Purpose |
| --- | --- |
| `QBIT_PACKAGE_MANAGER` | Force the native manager used by `qbit install` |
| `QBIT_JS_PM` | Force the JavaScript package manager |
| `QBIT_UPGRADE_REPO` | Override the upgrade repository for testing/custom distribution |
