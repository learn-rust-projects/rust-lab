use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

use tera::{Context, Tera};

use crate::strategy::prelude::*;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CliOpts {}

impl Strategy for CliOpts {
    fn execute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        let project_name = std::env::current_dir()
            .map_err(|e| MvpError::Custom(e.to_string()))?
            .file_name()
            .ok_or_else(|| MvpError::Custom("Failed to get project name".into()))?
            .to_string_lossy()
            .into_owned();
        context.insert("project_name", &project_name);

        let src_path = Path::new("src");
        if !src_path.exists() {
            return Err(MvpError::Custom("src directory not found".into()));
        }

        fs::create_dir_all(src_path.join("cli"))?;

        for template_name in ["mod.rs", "sub.rs", "preludes.rs"] {
            let src_template = format!("cli/{}", template_name);
            let rendered = tera.render(&src_template, context)?;
            let dest_path = src_path.join(&src_template);
            File::create(&dest_path)?.write_all(rendered.as_bytes())?;
        }

        let lib_path = src_path.join("lib.rs");
        let mut file = File::options()
            .create(true)
            .read(true)
            .append(true)
            .open(&lib_path)?;
        writeln!(file, "\nmod cli;")?;
        writeln!(file, "pub use cli::*;")?;

        let deps = [
            ("anyhow", None),
            ("clap", Some("derive")),
            ("enum_dispatch", None),
            ("tokio", Some("rt-multi-thread,macros")),
        ];
        for (name, features) in deps {
            let mut args = vec!["add", name];
            if let Some(f) = features {
                args.push("--features");
                args.push(f);
            }
            let status = Command::new("cargo").args(&args).status()?;
            if !status.success() {
                return Err(MvpError::Custom(format!("cargo add {} failed", name)));
            }
        }

        Ok(())
    }
}
