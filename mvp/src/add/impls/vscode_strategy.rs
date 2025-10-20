use std::fs;

use super::super::prelude::*;

pub struct VscodeStrategy;

// Add VSCode settings
impl AddStrategy for VscodeStrategy {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        let target_dir = ".vscode";
        let target_file_settings = format!("{}/settings.json", target_dir);
        let target_file_tasks = format!("{}/tasks.json", target_dir);

        fs::create_dir_all(target_dir)?; // Ensure .vscode directory exists

        let setting = tera.render("vscode/settings.json", context)?;
        fs::write(&target_file_settings, setting.as_bytes())?;
        println!("Created {}", target_file_settings);

        let tasks = tera.render("vscode/tasks.json", context)?;
        fs::write(&target_file_tasks, tasks.as_bytes())?;
        println!("Created {}", target_file_tasks);

        Ok(())
    }
    fn name(&self) -> &str {
        "vscode"
    }
}
