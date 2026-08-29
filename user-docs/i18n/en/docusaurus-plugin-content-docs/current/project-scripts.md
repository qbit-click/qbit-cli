# Project scripts

Qbit executes named workflows from the current project's config. It searches `qbit.yml`, `qbit.yaml`, then `qbit.toml` and uses the first file found.

A script can be one command:

```yaml
scripts:
  dev: "npm run dev"
```

or an ordered list:

```yaml
scripts:
  verify:
    - "cargo fmt -- --check"
    - "cargo test"
    - "cargo build --release"
```

Run it with:

```bash
qbit run verify
```

Commands are executed in order and the workflow stops on failure.

`qbit run <name>` executes Qbit project-config scripts. `qbit js run <script>` executes a `package.json` script through the selected JavaScript package manager.

See [Qbit configuration](/guide/configuration) for the full config shape.
