# نصب ابزارهای سیستم

دستور `qbit install` package manager سیستم‌عامل را تشخیص می‌دهد، شناسه مناسب package را resolve می‌کند و command نصب را اجرا می‌کند.

## استفاده پایه

```bash
qbit install python
qbit install java
qbit install postgres:15
```

فرمت target:

```text
<name>[:version]
```

## مشاهده command بدون اجرا

```bash
qbit install postgres:15 --dry-run
```

`--dry-run` command نهایی installer را چاپ می‌کند اما آن را اجرا نمی‌کند.

## نصب non-interactive

```bash
qbit install python --yes
```

Qbit در package managerهایی که پشتیبانی می‌کنند flag مناسب non-interactive را اضافه می‌کند.

## package managerهای پشتیبانی‌شده

- Linux: `apt-get`، `dnf`، `pacman`، `zypper`
- macOS: `brew`
- Windows: `winget`، `choco`، `scoop`

برای اجبار یک manager مشخص:

```bash
QBIT_PACKAGE_MANAGER=winget qbit install python
```

در PowerShell:

```powershell
$env:QBIT_PACKAGE_MANAGER = "winget"
qbit install python
```

## mappingهای سفارشی

اگر نام logical ابزار با package ID سیستم‌ها متفاوت است، آن را در `qbit.yml` یا `qbit.toml` تعریف کنید:

```yaml
install:
  postgres:
    version: "15"
    identifiers:
      apt: "postgresql"
      winget: "PostgreSQL.PostgreSQL"
```

سپس:

```bash
qbit install postgres
```

::: warning
version pinning به package manager بستگی دارد. برای managerهایی که pin مستقیم قابل اتکا ندارند، Qbit خطای قابل اقدام می‌دهد و نصب مبهم انجام نمی‌دهد.
:::
