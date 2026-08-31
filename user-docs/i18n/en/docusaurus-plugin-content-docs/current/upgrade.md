# Upgrade Qbit

Check the official GitHub repository for a newer release and install it with:

```bash
qbit upgrade
```

Qbit compares its build version with the latest `qbit-hub/qbit-cli` release. When a newer version exists, it downloads the platform archive, extracts it, and runs that platform's installer.

Expected assets:

- Windows: `qbit-windows-setup.zip`
- macOS: `qbit-macos-setup.tar.gz`
- Linux: `qbit-linux-setup.tar.gz`

If the binary's embedded version is already current, no installation is performed.

::: warning Current-release limitation
Release `v0.1.3` was built with an embedded Cargo package version of `0.1.0`. As a result, `qbit upgrade` on that release can consider `v0.1.3` newer again. If you encounter this behavior, use the [direct latest-release download](/guide/download-install).
:::

`QBIT_UPGRADE_REPO=owner/repository` can override the repository for testing/custom distribution. Normal users should leave it unset.

::: warning
Upgrade requires network access to GitHub. The platform installer can require operating-system privileges depending on the installation scope.
:::
