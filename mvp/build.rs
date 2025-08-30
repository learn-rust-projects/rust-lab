use std::{fs, path::Path};

fn main() {
    // Find all .html files in the templates directory
    let template_files = glob::glob("templates/**/*")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|entry| entry.is_file());

    // Collect template constants
    let mut templates = String::new();

    for file in template_files {
        let content = fs::read_to_string(&file).expect("Failed to read template file");
        let file_name = file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_")
            .to_uppercase();

        templates.push_str(&format!(
            "pub const {}: &str = r#\"{}\"#;\n",
            file_name, content
        ));
    }

    // Write the generated Rust file
    let out_path = Path::new("src/templates.rs");
    fs::write(out_path, templates).expect("Failed to write templates.rs");

    println!("cargo:rerun-if-changed=templates/");
}
