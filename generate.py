import sys
from pathlib import Path
from string import digits, ascii_letters

import regex

TARGET_DIRECTORY = Path(__file__).parent / "src" / "bin"


def split_entrypoint(entry_point: str) -> tuple[str, str]:
    assert ":" in entry_point, "':' was not found in EntryPoint!"
    return entry_point.split(":", maxsplit=1)


def validate_unicode_xid(xid: str):
    assert regex.fullmatch(r"\p{XID_Start}\p{XID_Continue}*", xid), (
        f"Invalid identifier: {xid}"
    )


def validate_ascii_identifier(identifier: str):
    valid_characters = digits + ascii_letters + "_"

    assert identifier, "identifier must not be empty!"
    assert identifier[0] not in digits, "identifier must not starts with digits!"
    assert all(map(lambda ch: ch in valid_characters, identifier)), (
        "identifier contains invalid character!"
    )


def main():
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <BIN_NAME> <ENTRY_POINT>")
        sys.exit(1)

    _, bin_name, entry_point = sys.argv
    module_path, func_name = split_entrypoint(entry_point)

    validate_unicode_xid(bin_name)
    validate_unicode_xid(func_name)

    for module in module_path.split("."):
        validate_ascii_identifier(module)

    TARGET_DIRECTORY.mkdir(exist_ok=True)
    output_file = TARGET_DIRECTORY / f"{bin_name}.rs"

    output_file.write_text(
        f"""fn main() -> Result<(), Box<dyn std::error::Error>> {{
    pyo3_simple_launcher::main("{module_path}", "{func_name}")
}}""",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
