# Dart

Qbit creates a minimal Dart console scaffold and uses `dart pub` for dependencies.

```bash
qbit dart init
qbit dart add dio
qbit dart add dio riverpod
qbit dart remove dio
```

When `pubspec.yaml` is missing, `qbit dart init` creates `pubspec.yaml` and `bin/main.dart`, then runs `dart pub get`. The Dart SDK must be available in PATH and `dart --version` must succeed.

`add` and `remove` require an existing `pubspec.yaml` and accept one or more package names.

::: info
The current generated console scaffold uses a Dart SDK constraint of `>=3.0.0 <4.0.0`.
:::
