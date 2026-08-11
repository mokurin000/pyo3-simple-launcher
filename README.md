# pyo3-simple-launcher

Alternative `Simple Launcher` in pure rust with `PyO3`, for CPython 3.9+ and Windows.

> [!NOTE]
> This launcher setups `PYTHONUTF8=1` at runtime for modern environments.

## Windows

You should specify `console` as the third argument to `generate.py` for console apps.

The `AttachConsole(-1)` would result a hassle on `cmd.exe` and `powershell.exe`,
see [fresh#2965](https://github.com/sinelaw/fresh/issues/2965) for more details.

## Build environment variables

All variables are **optional** — each one has a sensible default baked into the
binary at compile time via `option_env!`. Set them before running
`cargo build --release`.

### `PSL_PYTHON_DLL_PATH`

Controls which Python DLL is loaded at startup on Windows.

The path is passed to `LoadLibraryW`, so it must resolve from the directory of
the application (or be an absolute path).
