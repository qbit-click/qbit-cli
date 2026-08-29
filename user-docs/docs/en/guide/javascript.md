# JavaScript

Qbit can scaffold a minimal JavaScript project and manage packages or `package.json` scripts through an available package manager.

```bash
qbit js init
qbit js add react
qbit js remove react
qbit js run build
qbit js run test -- --watch
```

`qbit js init` creates `package.json`, `src/index.js`, and a starter `qbit.yml` when they are missing. It does not automatically install dependencies.

## Package-manager selection

Priority:

1. `QBIT_JS_PM` override;
2. detected project lockfile;
3. first available executable in this order: Bun → pnpm → Yarn → npm.

Stable lockfile mapping:

- `bun.lockb` → Bun
- `pnpm-lock.yaml` → pnpm
- `yarn.lock` → Yarn
- `package-lock.json` → npm

Override example:

```bash
QBIT_JS_PM=pnpm qbit js add react
```

Arguments after `--` in `qbit js run` are forwarded to the package script.

::: warning
If Qbit detects a lockfile but its matching package-manager executable is unavailable, it reports an error instead of silently choosing another manager. Install the manager or set `QBIT_JS_PM` explicitly.
:::
