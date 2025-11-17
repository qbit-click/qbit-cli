# Qbit CLI – Unified Dev Environment & Package Automation

Qbit is a cross-platform developer command line that turns repetitive environment setup into a single command. Install system tools, bootstrap Python/JavaScript/Dart projects, and run your own scripts from one `qbit` binary that works on Windows, macOS, and Linux.

## Why Qbit?

- **Install anything with one command**
  ```bash
  qbit install java
  qbit install chrome:127.0.0.0
  ```
  Qbit detects the native package manager (`winget`, `brew`, `apt`, `choco`, or `scoop`), maps the logical name to the correct package ID, and installs the exact version you request.

- **Project bootstrapping**
  ```bash
  qbit py init
  qbit js init
  qbit dart init
  ```
  Scaffold virtual environments, `requirements.txt`, `package.json`, entry files, and other boilerplate instantly.

- **Language-aware dependency management**
  ```bash
  qbit py add pandas
  qbit js add react
  ```
  Python packages are installed inside the managed venv, frozen back into `requirements.txt`, and JavaScript packages are added through whichever manager (npm/pnpm/yarn/bun/bun) is detected.

- **Script automation with `qbit run`**
  Define workflows inside `qbit.yml`/`qbit.toml` and execute them anywhere:
  ```bash
  qbit run dev
  qbit js run start
  ```
  Front-end builds, backend migrations, or multi-step CI recipes all become reusable commands.

## Power of `qbit.yml`

Qbit looks at `qbit.yml` (or `qbit.toml`) in your project root. Running `qbit js init` generates a starter file like this:

```yaml
scripts:
  dev: "npm run dev"
  build-all:
    - "qbit js run build"
    - "cargo build --release"

install:
  postgres:
    version: "15"
    identifiers:
      apt: "postgresql"
      winget: "PostgreSQL.PostgreSQL"
      default: "postgresql"
  redis:
    version: "7.2"
    identifiers:
      apt: "redis-server"
      winget: "Redis.Redis-CLI"
```

- `qbit run build-all` executes the commands sequentially.
- `qbit install postgres` installs version 15 and automatically chooses the correct package ID for each platform.
- Inline overrides are supported: `qbit install chrome:127.0.0.0`.

## Installers & PATH integration

Every release ships with platform-specific setup archives:

| Platform | Asset | Inside the archive |
|----------|-------|--------------------|
| Windows  | `qbit-windows-setup.zip` | `qbit-cli.exe`, `install.ps1` (adds to `Program Files\Qbit` and updates PATH), icon |
| macOS    | `qbit-macos-setup.tar.gz` | `qbit-cli`, `install_macos.sh` (drops a `.app` bundle + symlink), icon |
| Linux    | `qbit-linux-setup.tar.gz` | `qbit-cli`, `install.sh` (copies to `/opt/qbit` + `/usr/local/bin/qbit` symlink), optional GPG signature |

Usage example on Linux/macOS:
```bash
tar -xzf qbit-linux-setup.tar.gz
sudo ./install.sh
qbit --help
```
On Windows, extract the archive, right-click `install.ps1` → *Run with PowerShell* (or execute from an elevated terminal). The script copies the binary and appends the destination to the system PATH, so the `qbit` command is available globally.

## Supported Commands

- `qbit install <name[:version]>` – Install operating-system dependencies via native package managers.
- `qbit run <script>` – Execute custom workflows defined in configuration.
- `qbit py <init|add|remove>` – Python virtualenv management with automatic `requirements.txt` updates.
- `qbit js <init|add|remove|run>` – JavaScript project scaffolding, npm/yarn/pnpm/bun integration, and script execution.
- `qbit dart ...` – Dart scaffolding (extensible for Flutter or server projects).

Use `qbit --help` or `qbit <command> --help` for details.

## Build from Source

```bash
git clone https://github.com/<your-org>/qbit-cli.git
cd qbit-cli
cargo build --release
./target/release/qbit --help
```

Rust 1.76+ is recommended. The repository also includes `cargo dev` for sandbox testing inside `dev-sandbox/`.

## Contributing

Issues and pull requests are welcome. Before submitting a PR:
1. Run `cargo fmt && cargo clippy && cargo test`.
2. Test key workflows (`qbit py init`, `qbit install ...`, `qbit run ...`) in the dev sandbox.
3. Describe the motivation and behavior changes clearly.

## License

Distributed under the terms of the MIT License. See [LICENSE](LICENSE) for details.
