# اسکریپت‌های پروژه

Qbit می‌تواند workflowهای نام‌گذاری‌شده را از فایل config پروژه اجرا کند.

## فایل‌های پشتیبانی‌شده

Qbit در root فعلی به این ترتیب دنبال config می‌گردد:

1. `qbit.yml`
2. `qbit.yaml`
3. `qbit.toml`

## یک command

```yaml
scripts:
  dev: "npm run dev"
```

اجرا:

```bash
qbit run dev
```

## چند command پشت سر هم

```yaml
scripts:
  verify:
    - "cargo fmt -- --check"
    - "cargo test"
    - "cargo build --release"
```

```bash
qbit run verify
```

commandها به ترتیبی که در config نوشته شده‌اند اجرا می‌شوند. اگر یکی از commandها fail شود، workflow با خطا متوقف می‌شود.

## تفاوت با `qbit js run`

- `qbit run <name>` اسکریپت‌های `qbit.yml/qbit.toml` را اجرا می‌کند.
- `qbit js run <script>` یک script از `package.json` را از طریق JavaScript package manager انتخاب‌شده اجرا می‌کند.

برای schema کامل، [پیکربندی Qbit](/guide/configuration) را ببینید.
