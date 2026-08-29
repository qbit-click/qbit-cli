# Download and install

Qbit CLI ships platform-specific setup archives through GitHub Releases. These links always resolve to the **latest release**.

## Direct downloads

| Platform | File |
| --- | --- |
| Windows | [qbit-windows-setup.zip](https://github.com/qbit-click/qbit-cli/releases/latest/download/qbit-windows-setup.zip) |
| macOS | [qbit-macos-setup.tar.gz](https://github.com/qbit-click/qbit-cli/releases/latest/download/qbit-macos-setup.tar.gz) |
| Linux | [qbit-linux-setup.tar.gz](https://github.com/qbit-click/qbit-cli/releases/latest/download/qbit-linux-setup.tar.gz) |

[Browse all releases](https://github.com/qbit-click/qbit-cli/releases)

## Windows

Extract the ZIP, open PowerShell in the extracted directory, then run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\install.ps1
```

The default is a per-user install and does not require elevation. Qbit is installed at:

```text
%LOCALAPPDATA%\Qbit\bin\qbit.exe
```

For a machine-wide install, use an elevated PowerShell:

```powershell
.\install.ps1 -Scope Machine
```

The default machine path is `%ProgramFiles%\Qbit\bin\qbit.exe`. Open a new terminal after PATH changes.

## macOS

```bash
tar -xzf qbit-macos-setup.tar.gz
cd <extracted-directory>
sudo ./install_macos.sh
qbit --help
```

The current installer places the binary under `/Applications/QbitCLI/Contents/MacOS/qbit` and creates `/usr/local/bin/qbit`.

## Linux

```bash
tar -xzf qbit-linux-setup.tar.gz
cd <extracted-directory>
sudo ./install.sh
qbit --help
```

The default installer stores Qbit at `/opt/qbit/qbit` and creates `/usr/local/bin/qbit`.

## Uninstall

Each archive includes the matching uninstall script: `uninstall.ps1`, `uninstall_macos.sh`, or `uninstall.sh`.

::: warning
Extract the complete setup archive and run the installer from that directory; the installer expects the release binary beside the script.
:::
