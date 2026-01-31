use std::fs;

use clap::Parser;

use crate::strategy::prelude::*;

pub struct VscodeStrategy;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct VscodeOpts {
    /// 是否添加 tasks.json
    #[arg(
        long,
        help = "Add tasks.json to .vscode directory (default: true)",
        default_value_t = true
    )]
    add_tasks: bool,
}

// Add VSCode settings
impl Strategy for VscodeOpts {
    fn execute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始添加VSCode配置");
        let target_dir = ".vscode";
        let target_file_settings = format!("{}/settings.json", target_dir);
        let target_file_tasks = format!("{}/tasks.json", target_dir);

        fs::create_dir_all(target_dir)?; // Ensure .vscode directory exists

        let setting = tera.render(".vscode/settings.json", context)?;
        fs::write(&target_file_settings, setting.as_bytes())?;
        println!("Created {}", target_file_settings);

        let tasks = tera.render(".vscode/tasks.json", context)?;
        fs::write(&target_file_tasks, tasks.as_bytes())?;
        println!("Created {}", target_file_tasks);

        tracing::info!("VSCode配置添加成功");
        Ok(())
    }
}
