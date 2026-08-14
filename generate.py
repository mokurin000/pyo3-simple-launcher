import sys
from pathlib import Path

import regex

TARGET_DIRECTORY = Path(__file__).parent / "src" / "bin"


def split_entrypoint(entry_point: str) -> tuple[str, str]:
    assert ":" in entry_point, "':' was not found in EntryPoint!"
    return entry_point.split(":", maxsplit=1)


def validate_unicode_xid(xid: str):
    assert regex.fullmatch(r"\p{XID_Continue}+", xid), f"Invalid identifier: {xid}"


def validate_unicode_fnname(xid: str):
    assert regex.fullmatch(r"[\p{XID_Start}_]\p{XID_Continue}*", xid), (
        f"Invalid identifier: {xid}"
    )


def main():
    if len(sys.argv) not in [3, 4]:
        print(f"Usage: {sys.argv[0]} <BIN_NAME> <ENTRY_POINT> [CONSOLE|WINDOWS]")
        sys.exit(1)

    _, bin_name, entry_point = sys.argv[:3]

    if len(sys.argv) > 3:
        subsystem = sys.argv[3].lower()
        if subsystem not in ["console", "windows"]:
            print(f"Unsupported subsystem: {subsystem}")
            sys.exit(1)
    else:
        subsystem = "windows"

    module_path, func_name = split_entrypoint(entry_point)

    validate_unicode_xid(bin_name)

    for module in module_path.split("."):
        validate_unicode_xid(module)

    validate_unicode_fnname(func_name)

    TARGET_DIRECTORY.mkdir(exist_ok=True)
    output_file = TARGET_DIRECTORY / f"{bin_name}.rs"

    output_file.write_text(
        (
            '#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]'
            if subsystem == "windows"
            else ""
        )
        + f"""
fn main() -> Result<(), Box<dyn std::error::Error>> {{
    pyo3_simple_launcher::main("{module_path}", "{func_name}")
}}
""",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
