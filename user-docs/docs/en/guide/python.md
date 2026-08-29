# Python

Qbit manages a local `./venv` and the project's `requirements.txt`.

```bash
qbit py init
qbit py add requests
qbit py add "django==5.2"
qbit py remove requests
```

`qbit py init` creates `requirements.txt` when missing, finds an available Python interpreter, and creates `venv` when needed. If Python is unavailable, use `qbit install python` first.

`add` and `remove` operate through pip inside the managed virtualenv, then replace `requirements.txt` with the output of `pip freeze`.

Virtualenv interpreter paths:

- Windows: `venv\Scripts\python.exe`
- macOS/Linux: `venv/bin/python`

::: warning
Because dependency mutations sync the complete `pip freeze` output, account for that behavior if your project manually curates `requirements.txt`.
:::
