# pyo3-simple-launcher

Alternative `Simple Launcher` in pure rust with `PyO3`, for CPython 3.9+.

> [!NOTE]
> This launcher setups `PYTHONUTF8=1` at runtime for modern environments.
>

## Windows

Executables are using the `WINDOWS` subsystem by default.

For console apps, you may want to allocate a `conhost.exe` on start,
which requires the `CONSOLE` subsystem.

For Windows 11 24H2 and later, you may also refer to the [Console Allocation Policy](https://learn.microsoft.com/en-us/windows/console/console-allocation-policy).

## Build environment variables

All variables are **optional** — each one has a sensible default baked into the
binary at compile time via `option_env!`. Set them before running
`cargo build --release`.

### `PSL_PYTHON_SYS_PATH`

Controls the `sys.path` entries injected into the embedded interpreter (see
`src/lib.rs`).

- **Format:** semicolon-separated (`;`) list of paths.
- **Resolution:** each entry is joined to the **current working directory at
  runtime** and converted to an absolute path. Use `./`-relative entries so the
  launcher works no matter where it is launched from.


### `PSL_PYTHON_DLL_PATH`

Controls which Python DLL is loaded at startup on Windows.

The path is passed to `LoadLibraryW`, so it must resolve from the directory of
the application (or be an absolute path).

