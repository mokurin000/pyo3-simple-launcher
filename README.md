# pyo3-simple-launcher

Alternative `Simple Launcher` in pure rust with `PyO3`, for CPython 3.9+ and Windows.

> [!NOTE]
> This launcher setups `PYTHONUTF8=1` at runtime for modern environments.

## Dependencies

For example, for CPython 3.14, the packaging layout can be:

```text
your-program.exe
python314.dll
python314._pth
Lib/
    site-packages/
    ... files extracted from python-embed.zip
```

Where the python314._pth contains:

```text
Lib/
Lib/python314.zip
import site
```

Executables compiled using this launcher could support Python 3.9+ without any recompilation.

For more details, check:

- [getpath.py](https://github.com/python/cpython/blob/837627dc96dc557e1655690d9f59892725ed85b1/Modules/getpath.py#L466-L482)
- [library](https://github.com/python/cpython/blob/7c072c1fcc3c04535dde873a7709781be2794583/Modules/getpath.c#L754-L770)
  - [DllMain](https://github.com/python/cpython/blob/837627dc96dc557e1655690d9f59892725ed85b1/PC/dl_nt.c#L14-L36), which setups the `PyWin_DLLhModule`.
- [real_executable](https://github.com/python/cpython/blob/837627dc96dc557e1655690d9f59892725ed85b1/Modules/getpath.c#L774-L804)

## Windows

You should specify `console` as the third argument to `generate.py` for console apps.

The `AttachConsole(-1)` would result a hassle on `cmd.exe` and `powershell.exe`,
see [fresh#2965](https://github.com/sinelaw/fresh/issues/2965) for more details.
