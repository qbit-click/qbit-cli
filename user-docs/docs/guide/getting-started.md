# شروع کار

Qbit CLI یک command-line tool چندسکویی برای نصب dependencyهای سیستم، bootstrap و مدیریت پروژه‌های Python/JavaScript/Dart و اجرای workflowهای تعریف‌شده در پروژه است.

## بعد از نصب

ابتدا در یک terminal جدید بررسی کنید که Qbit در PATH قرار گرفته است:

```bash
qbit --help
```

commandهای اصلی نسخه فعلی:

```text
qbit install <target[:version]> [--yes] [--dry-run]
qbit run <script>
qbit py <init|add|remove>
qbit js <init|add|remove|run>
qbit dart <init|add|remove>
qbit upgrade
```

## چند مثال سریع

```bash
qbit install python
qbit install postgres:15 --dry-run
qbit py init
qbit py add requests
qbit js init
qbit js add react
qbit js run build -- --mode production
qbit dart init
qbit run dev
qbit upgrade
```

## Qbit چه چیزی را مدیریت می‌کند؟

- package manager سیستم‌عامل را برای `qbit install` تشخیص می‌دهد؛
- virtualenv و `requirements.txt` پروژه Python را مدیریت می‌کند؛
- برای JavaScript بین Bun، pnpm، Yarn و npm انتخاب می‌کند؛
- `dart pub` را برای پروژه Dart اجرا می‌کند؛
- commandهای `scripts` در `qbit.yml`، `qbit.yaml` یا `qbit.toml` را به ترتیب اجرا می‌کند.

::: tip
قبل از اجرای نصب سیستم، می‌توانید با `--dry-run` command دقیق package manager را بدون اجرا ببینید.
:::
