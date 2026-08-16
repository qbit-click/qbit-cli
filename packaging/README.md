# Packaging

This directory contains the source configuration for qbit's native installers. Actual build logic lives in `scripts/package/`; GitHub Actions (`ci.yml`, `release.yml`) only orchestrates calls to these scripts — it contains no packaging logic itself.

## Layout

```
packaging/
├── windows/QbitCli.wxs          WiX v5 source for the MSI
├── macos/distribution.xml       productbuild distribution definition
└── linux/debian/control.template  DEB control file template

scripts/package/
├── build-windows.ps1
├── build-macos.sh
└── build-linux-deb.sh
```

## Naming convention

| Concept | Value |
|---|---|
| Installed command | `qbit` (`qbit.exe` on Windows) |
| Rust package / crate name | `qbit-cli` |
| Release asset prefix | `qbit-cli-<version>-...` |

Asset filenames never match the installed command name — this is intentional. See `AGENTS.md` for the full rationale.

## Build locally

All three scripts take the same shape of arguments: version, architecture, path to the built binary, output directory.

### Linux (.deb)

Requires `dpkg-deb` (present by default on Debian/Ubuntu).

```bash
cargo build --release
chmod +x scripts/package/build-linux-deb.sh
./scripts/package/build-linux-deb.sh 1.0.0 amd64 target/release/qbit dist
```

Verify:
```bash
sudo dpkg -i dist/qbit-cli_1.0.0_amd64.deb
qbit --help
sudo dpkg -r qbit-cli
```

### macOS (.pkg)

Requires `pkgbuild`/`productbuild` (part of Xcode Command Line Tools).

```bash
cargo build --release
chmod +x scripts/package/build-macos.sh
./scripts/package/build-macos.sh 1.0.0 arm64 target/release/qbit dist
```

Verify:
```bash
sudo installer -pkg dist/qbit-cli-1.0.0-macos-arm64.pkg -target /
qbit --help
```

Optional signing/notarization (unsigned by default — clearly logged either way):
```bash
export MACOS_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)"
export MACOS_INSTALLER_IDENTITY="Developer ID Installer: Your Name (TEAMID)"
export MACOS_NOTARY_PROFILE="your-notarytool-profile"
```

### Windows (.msi)

Requires WiX v5 (pinned deliberately — v6+ requires accepting the Open Source Maintenance Fee EULA):
```powershell
dotnet tool install --global wix --version 5.0.2
```

```powershell
cargo build --release
.\scripts\package\build-windows.ps1 -Version 1.0.0 -Arch x64 -BinaryPath target\release\qbit.exe -OutDir dist
```

Verify:
```powershell
msiexec /i dist\qbit-cli-1.0.0-windows-x64.msi /qn
qbit --help
```

Requires `assets/icon.ico` to exist — the build fails loudly if it's missing or not a valid `.ico`.

Optional signing (unsigned by default):
```powershell
$env:WINDOWS_CERT_PFX = "<base64-encoded .pfx>"
$env:WINDOWS_CERT_PASSWORD = "..."
```

## Support matrix

Currently: Windows (MSI), macOS (PKG), Linux (DEB). RPM is not built unless explicitly added to the support matrix — do not add `packaging/linux/rpm/` speculatively.

## CI vs. release

- `ci.yml` builds and end-to-end tests (`install → qbit --help → uninstall`) a throwaway package on every PR, using a fixed test version (`0.0.1`), on native runners per OS. It never creates a Release or a tag.
- `release.yml` only runs on a `v*` tag push. It builds real, versioned installers, generates `.sha256` checksums for each, and publishes them as GitHub Release assets. No `.zip`/`.tar.gz` setup archives are produced.
