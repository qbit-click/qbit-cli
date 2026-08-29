# Getting started

Qbit CLI is a cross-platform command line for system dependency installation, Python/JavaScript/Dart project management, and repeatable project workflows.

## Verify the installation

Open a new terminal and run:

```bash
qbit --help
```

Current top-level commands:

```text
qbit install <target[:version]> [--yes] [--dry-run]
qbit run <script>
qbit py <init|add|remove>
qbit js <init|add|remove|run>
qbit dart <init|add|remove>
qbit upgrade
```

## Quick examples

```bash
qbit install python
qbit install postgres:15 --dry-run
qbit py init
qbit py add requests
qbit js init
qbit js add react
qbit js run build -- --mode production
qbit dart init
qbit run dev
qbit upgrade
```

Qbit detects a supported system package manager for `install`, manages a local Python virtualenv and `requirements.txt`, selects Bun/pnpm/Yarn/npm for JavaScript, invokes `dart pub`, and executes scripts from `qbit.yml`, `qbit.yaml`, or `qbit.toml`.

::: tip
Use `--dry-run` before a system installation to inspect the resolved package-manager command without executing it.
:::
