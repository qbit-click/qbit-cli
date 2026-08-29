# System packages

`qbit install` detects a supported native package manager, resolves the package identifier, and executes the install command.

```bash
qbit install python
qbit install postgres:15
qbit install postgres:15 --dry-run
qbit install python --yes
```

Target format is `<name>[:version]`. `--dry-run` prints the resolved command without executing it; `--yes` requests non-interactive behavior where supported.

## Supported managers

- Linux: `apt-get`, `dnf`, `pacman`, `zypper`
- macOS: `brew`
- Windows: `winget`, `choco`, `scoop`

Override automatic detection with `QBIT_PACKAGE_MANAGER`:

```bash
QBIT_PACKAGE_MANAGER=winget qbit install python
```

Project configuration can map a logical target to per-manager identifiers:

```yaml
install:
  postgres:
    version: "15"
    identifiers:
      apt: "postgresql"
      winget: "PostgreSQL.PostgreSQL"
```

::: warning
Version pinning is package-manager dependent. Qbit reports an actionable error when direct reliable pinning is not supported instead of silently changing the requested behavior.
:::
