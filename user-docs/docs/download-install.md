# دانلود و نصب

Qbit CLI برای Windows، macOS و Linux به‌صورت setup archive در GitHub Releases منتشر می‌شود. لینک‌های زیر همیشه به **آخرین release** اشاره می‌کنند.

## دانلود مستقیم

| سیستم‌عامل | فایل |
| --- | --- |
| Windows | [qbit-windows-setup.zip](https://github.com/qbit-hub/qbit-cli/releases/latest/download/qbit-windows-setup.zip) |
| macOS | [qbit-macos-setup.tar.gz](https://github.com/qbit-hub/qbit-cli/releases/latest/download/qbit-macos-setup.tar.gz) |
| Linux | [qbit-linux-setup.tar.gz](https://github.com/qbit-hub/qbit-cli/releases/latest/download/qbit-linux-setup.tar.gz) |

[مشاهده همه releaseها](https://github.com/qbit-hub/qbit-cli/releases)

## Windows

1. فایل ZIP را دانلود و extract کنید.
2. PowerShell را در پوشه extract‌شده باز کنید.
3. installer را اجرا کنید:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\install.ps1
```

نصب پیش‌فرض **per-user** است و admin نمی‌خواهد. باینری در مسیر زیر قرار می‌گیرد:

```text
%LOCALAPPDATA%\Qbit\bin\qbit.exe
```

برای نصب machine-wide، PowerShell را با Administrator باز کنید و اجرا کنید:

```powershell
.\install.ps1 -Scope Machine
```

در این حالت مسیر پیش‌فرض `%ProgramFiles%\Qbit\bin\qbit.exe` است. بعد از تغییر PATH یک terminal جدید باز کنید.

## macOS

```bash
tar -xzf qbit-macos-setup.tar.gz
cd <extracted-directory>
sudo ./install_macos.sh
qbit --help
```

installer فعلی باینری را زیر `/Applications/QbitCLI/Contents/MacOS/qbit` قرار می‌دهد و symlink سراسری `/usr/local/bin/qbit` را می‌سازد.

## Linux

```bash
tar -xzf qbit-linux-setup.tar.gz
cd <extracted-directory>
sudo ./install.sh
qbit --help
```

installer پیش‌فرض Qbit را در `/opt/qbit/qbit` نصب و `/usr/local/bin/qbit` را به آن symlink می‌کند.

## حذف Qbit

هر archive شامل uninstall script متناظر همان سیستم‌عامل است:

- Windows: `uninstall.ps1`
- macOS: `uninstall_macos.sh`
- Linux: `uninstall.sh`

::: warning
setup archive را کامل extract کنید و installer را از همان پوشه اجرا کنید؛ installer به باینری موجود کنار script نیاز دارد.
:::
