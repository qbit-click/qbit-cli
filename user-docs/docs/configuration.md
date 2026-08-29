# پیکربندی Qbit

Qbit config پروژه را فقط از directory فعلی می‌خواند و به ترتیب `qbit.yml`، `qbit.yaml` و `qbit.toml` را بررسی می‌کند. اولین فایل موجود استفاده می‌شود.

## YAML

```yaml
scripts:
  dev: "npm run dev"
  verify:
    - "cargo test"
    - "cargo build --release"

install:
  node: "OpenJS.NodeJS"
  python:
    version: "3.12"
    identifiers:
      winget: "Python.Python.3.12"
      brew: "python@3.12"
```

## TOML

```toml
[scripts]
dev = "npm run dev"
verify = ["cargo test", "cargo build --release"]

[install]
node = "OpenJS.NodeJS"

[install.python]
version = "3.12"

[install.python.identifiers]
winget = "Python.Python.3.12"
brew = "python@3.12"
```

## `scripts`

هر script می‌تواند یک command string یا یک لیست ordered از commandها باشد. با `qbit run <name>` اجرا می‌شود.

## `install`

دو شکل وجود دارد:

### شناسه مشترک

```yaml
install:
  node: "OpenJS.NodeJS"
```

این identifier برای همه managerها استفاده می‌شود.

### شناسه per-manager

```yaml
install:
  postgres:
    version: "15"
    identifiers:
      apt: "postgresql"
      winget: "PostgreSQL.PostgreSQL"
```

lookup نام target و keyهای manager case-insensitive است، ولی مقدار package ID همان casing ثبت‌شده را حفظ می‌کند.

## متغیرهای محیطی

| متغیر | کاربرد |
| --- | --- |
| `QBIT_PACKAGE_MANAGER` | اجبار manager سیستم برای `qbit install` |
| `QBIT_JS_PM` | اجبار JavaScript package manager |
| `QBIT_UPGRADE_REPO` | override repository برای upgrade؛ عمدتاً تست/توزیع سفارشی |
