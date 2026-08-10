# pyo3-simple-launcher

Alternative `Simple Launcher` in pure rust with `PyO3`, for Windows CPython 3.9+.

> [!NOTE]
> This launcher setups `PYTHONUTF8=1` at runtime for modern environments.

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

Controls which Python DLL is loaded at startup on Windows (see `src/windows.rs`).
The path is passed to `LoadLibraryW`, so it must resolve from the directory of
the application (or be an absolute path).


### `PYO3_PYTHON`

Not read by this crate directly, but required by the `pyo3` build script
(the crate uses `pyo3` with the `abi3` feature). It tells pyo3 which Python
interpreter to use at build time. Use any interpreter whose version matches the
`abi3` target.

- **Example (PowerShell):**

  ```powershell
  $env:PYO3_PYTHON = "C:\Python\python-embed-amd64\python.exe"
  ```
