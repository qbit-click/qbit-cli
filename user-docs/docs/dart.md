# Dart

Qbit برای پروژه‌های Dart یک scaffold کنسولی حداقلی می‌سازد و `dart pub` را برای dependencyها اجرا می‌کند.

## مقداردهی اولیه

```bash
qbit dart init
```

اگر `pubspec.yaml` وجود نداشته باشد، Qbit این ساختار را ایجاد می‌کند:

```text
pubspec.yaml
bin/main.dart
```

سپس `dart pub get` اجرا می‌شود. Dart SDK باید در PATH موجود باشد و `dart --version` کار کند.

## افزودن package

```bash
qbit dart add dio
qbit dart add dio riverpod
```

این دستور معادل اجرای `dart pub add` برای packageهای داده‌شده است.

## حذف package

```bash
qbit dart remove dio
qbit dart remove dio riverpod
```

برای `add/remove` باید `pubspec.yaml` در directory فعلی وجود داشته باشد.

::: info
scaffold نسخه فعلی برای یک Dart console application است و SDK constraint پیش‌فرض آن `>=3.0.0 <4.0.0` است.
:::
