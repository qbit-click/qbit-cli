# ارتقا Qbit

برای بررسی و نصب آخرین release منتشرشده:

```bash
qbit upgrade
```

Qbit نسخه فعلی باینری را با latest release repository رسمی `qbit-hub/qbit-cli` مقایسه می‌کند. اگر نسخه جدیدتری وجود داشته باشد، asset مخصوص سیستم‌عامل را دانلود، extract و installer همان platform را اجرا می‌کند.

assetهای مورد انتظار:

- Windows: `qbit-windows-setup.zip`
- macOS: `qbit-macos-setup.tar.gz`
- Linux: `qbit-linux-setup.tar.gz`

اگر نسخه embedded باینری جدید یا برابر latest release باشد، Qbit بدون تغییر پایان می‌یابد.

::: warning محدودیت release فعلی
release `v0.1.3` با package version داخلی `0.1.0` build شده است. به همین دلیل `qbit upgrade` روی این release ممکن است دوباره `v0.1.3` را جدیدتر تشخیص دهد. اگر این رفتار را دیدید، از [دانلود مستقیم آخرین release](/guide/download-install) استفاده کنید.
:::

## repository سفارشی

متغیر `QBIT_UPGRADE_REPO` برای سناریوهای تست/توزیع سفارشی وجود دارد:

```bash
QBIT_UPGRADE_REPO=owner/repository qbit upgrade
```

کاربر عادی نباید این متغیر را تنظیم کند.

::: warning
ارتقا به GitHub و دسترسی شبکه نیاز دارد و installer platform ممکن است بسته به نوع نصب به مجوزهای سیستم نیاز داشته باشد.
:::
