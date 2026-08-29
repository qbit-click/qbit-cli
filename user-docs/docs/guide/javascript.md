# JavaScript

Qbit می‌تواند یک پروژه JavaScript حداقلی بسازد و packageها یا scriptهای `package.json` را با package manager موجود اجرا کند.

## مقداردهی اولیه

```bash
qbit js init
```

در صورت نبودن فایل‌ها، Qbit این موارد را ایجاد می‌کند:

- `package.json`
- `src/index.js`
- `qbit.yml` به‌عنوان template اولیه config

این command dependency installation را خودکار اجرا نمی‌کند.

## انتخاب package manager

ترتیب تشخیص Qbit:

1. مقدار `QBIT_JS_PM` در صورت تنظیم؛
2. lockfile پروژه؛
3. اولین manager موجود در PATH با اولویت Bun → pnpm → Yarn → npm.

lockfileهای نسخه stable فعلی:

- `bun.lockb` → Bun
- `pnpm-lock.yaml` → pnpm
- `yarn.lock` → Yarn
- `package-lock.json` → npm

برای override:

```bash
QBIT_JS_PM=pnpm qbit js add react
```

در PowerShell:

```powershell
$env:QBIT_JS_PM = "pnpm"
qbit js add react
```

## افزودن و حذف package

```bash
qbit js add react
qbit js remove react
```

Qbit syntax مناسب manager انتخاب‌شده را می‌سازد و همان executable را اجرا می‌کند.

## اجرای script

```bash
qbit js run build
qbit js run test -- --watch
```

argumentهای بعد از `--` به script package manager منتقل می‌شوند.

::: warning
اگر lockfile مربوط به یک manager وجود داشته باشد ولی executable همان manager در PATH نباشد، Qbit به‌جای fallback پنهان، خطای واضح می‌دهد. manager را نصب کنید یا `QBIT_JS_PM` را به یک manager نصب‌شده تغییر دهید.
:::
