#!/usr/bin/env python3
"""Generate templates.rs from physical template files."""

import os


def to_const_name(path):
    """Convert file path to Rust constant name"""
    name = path.replace("/", "_").replace(".", "_").replace("-", "_").upper()
    if name.endswith("_JSON"):
        name = name[:-5] + "_JSON"
    return name


def escape_rust_raw_string(s):
    """Escape content for Rust raw string - minimal escaping needed for r#"..."# syntax"""
    return s


def process_file(filepath, const_name):
    """Read file and generate Rust const with proper formatting"""
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    # For raw strings, we just need to be careful with #" and "#
    # Replace " with \" inside the raw string content
    escaped = content.replace('\\"', '\\\\"')
    return f'pub const {const_name}: &str = r#"{escaped}"#;'


def main():
    templates_dir = "templates"
    output_file = "src/templates.rs"

    files = [
        ".gitattributes",
        ".github/workflows/ci.yml",
        ".gitignore",
        ".vscode/settings.json",
        ".vscode/tasks.json",
        "LICENSE-APACHE",
        "LICENSE-MIT",
        "LICENSE.md",
        "README.md",
        "SINGLE-LIC.md",
        "cli/mod.rs",
        "cli/preludes.rs",
        "cli/sub.rs",
        "rustfmt.toml",
    ]

    consts = []
    mappings = []

    for f in files:
        src_path = os.path.join(templates_dir, f)
        if os.path.exists(src_path):
            const_name = to_const_name(f)
            const_content = process_file(src_path, const_name)
            consts.append(const_content)
            mappings.append(f'    ("{f}", {const_name})')

    template_map = (
        "pub static TEMPLATE_MAP: &[(&str, &str)] = &[\n" + ",\n".join(mappings) + "\n];"
    )

    output = "\n\n".join(consts) + "\n\n" + template_map + "\n"

    with open(output_file, "w", encoding="utf-8") as f:
        f.write(output)

    print(f"Generated {output_file} with {len(consts)} templates")


if __name__ == "__main__":
    main()
