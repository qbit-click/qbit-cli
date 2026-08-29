# رفع اشکال

## `qbit` بعد از نصب پیدا نمی‌شود

یک terminal جدید باز کنید و دوباره اجرا کنید:

```bash
qbit --help
```

در Windows نصب per-user مسیر `%LOCALAPPDATA%\Qbit\bin` را به User PATH اضافه می‌کند. در macOS/Linux installer پیش‌فرض symlink را در `/usr/local/bin/qbit` می‌سازد.

## package manager سیستم پیدا نمی‌شود

Qbit فقط managerهای پشتیبانی‌شده موجود در PATH را تشخیص می‌دهد. می‌توانید manager را صریحاً انتخاب کنید:

```bash
QBIT_PACKAGE_MANAGER=brew qbit install node
```

اگر executable انتخاب‌شده وجود نداشته باشد، Qbit fail می‌شود و manager دیگری را پنهانی استفاده نمی‌کند.

## JavaScript manager اشتباه انتخاب شده

lockfile و `QBIT_JS_PM` را بررسی کنید. override نمونه:

```bash
QBIT_JS_PM=npm qbit js run build
```

اگر lockfile یک manager وجود دارد ولی خود executable نصب نیست، یا manager را نصب کنید یا override را به manager موجود تغییر دهید.

## Python پیدا نمی‌شود

```bash
qbit install python
qbit py init
```

بعد از نصب مطمئن شوید interpreter در PATH است.

## Dart پیدا نمی‌شود

Qbit نیاز دارد `dart --version` موفق باشد. Dart SDK را نصب و PATH را اصلاح کنید، سپس دوباره `qbit dart init` را اجرا کنید.

## `qbit run` script را پیدا نمی‌کند

- command را از root پروژه اجرا کنید؛
- وجود `qbit.yml`، `qbit.yaml` یا `qbit.toml` را بررسی کنید؛
- نام script زیر `scripts` باید با argument command تطابق داشته باشد.

## `qbit upgrade` asset را پیدا نمی‌کند

upgrade به asset platform با نام دقیق وابسته است. در release رسمی باید یکی از این نام‌ها وجود داشته باشد: `qbit-windows-setup.zip`، `qbit-macos-setup.tar.gz` یا `qbit-linux-setup.tar.gz`.

برای گزارش مشکل، خروجی `qbit --help`، command اجراشده، سیستم‌عامل و متن کامل خطا را ضمیمه کنید؛ secret یا credential ارسال نکنید.
