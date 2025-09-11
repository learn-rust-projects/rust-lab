use std::{fs, path::Path};

fn main() {
    let root = Path::new("templates");

    let template_files = glob::glob("templates/**/*")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|entry| entry.is_file());

    let mut templates = String::new();
    let mut map_entries = Vec::new();

    for file in template_files {
        let content = fs::read_to_string(&file).expect("Failed to read template file");

        // 模板相对路径，保留子目录
        let relative_path = file.strip_prefix(root).unwrap().to_str().unwrap();

        // 生成 Rust 常量名，替换非字母数字为下划线，大写
        let const_name = relative_path
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_")
            .to_uppercase();

        // 写入 Rust 常量
        templates.push_str(&format!(
            "pub const {}: &str = r#\"{}\"#;\n",
            const_name, content
        ));

        // 写入映射表，保持相对路径
        map_entries.push(format!("    (\"{}\", {}),", relative_path, const_name));
    }

    // 生成映射静态变量
    templates.push_str("\npub static TEMPLATE_MAP: &[(&str, &str)] = &[\n");
    for entry in map_entries {
        templates.push_str(&entry);
        templates.push('\n');
    }
    templates.push_str("];\n");

    // 输出到 src/templates.rs
    let out_path = Path::new("src/templates.rs");
    fs::write(out_path, templates).expect("Failed to write templates.rs");

    println!("cargo:rerun-if-changed=templates/");
}
