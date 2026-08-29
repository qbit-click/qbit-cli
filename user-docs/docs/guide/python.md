# Python

Qbit یک virtual environment محلی در `./venv` و فایل `requirements.txt` پروژه را مدیریت می‌کند.

## مقداردهی اولیه

```bash
qbit py init
```

اگر `requirements.txt` وجود نداشته باشد ساخته می‌شود. Qbit سپس Python موجود در سیستم را پیدا می‌کند و در صورت نبودن `venv` آن را می‌سازد.

اگر Python در دسترس نباشد، Qbit پیشنهاد می‌دهد ابتدا آن را نصب کنید:

```bash
qbit install python
```

## افزودن dependency

```bash
qbit py add requests
qbit py add "django==5.2"
```

Qbit package را با `pip` داخل virtualenv مدیریت‌شده نصب می‌کند و سپس خروجی `pip freeze` را در `requirements.txt` می‌نویسد.

## حذف dependency

```bash
qbit py remove requests
```

حذف نیز داخل همان virtualenv انجام می‌شود و بعد `requirements.txt` دوباره sync می‌شود.

## مسیر Python در virtualenv

- Windows: `venv\Scripts\python.exe`
- macOS/Linux: `venv/bin/python`

::: warning
`qbit py add/remove` کل خروجی `pip freeze` محیط مدیریت‌شده را در `requirements.txt` sync می‌کند. اگر فایل را دستی curate می‌کنید، قبل از استفاده این رفتار را در workflow پروژه لحاظ کنید.
:::
