use std::process::Command;

use super::{composite::Composite, prelude::*};

pub struct InitStrategy;

impl AddStrategy for InitStrategy {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        let values = context
            .get("init_values")
            .and_then(|v| v.as_array())
            .ok_or_else(|| MvpError::Custom("init_values must be an array".to_owned()))?;

        let project_name = values
            .first()
            .and_then(|v| v.as_str())
            .ok_or_else(|| MvpError::Custom("Project name is required".to_owned()))?
            .trim_matches('"')
            .to_string();

        let disable_vcs = values
            .get(1)
            .and_then(|v| v.as_str())
            .map(|s| s.trim_matches('"') == "n")
            .unwrap_or(false);

        create_project(&project_name, disable_vcs)?;
        if disable_vcs {
            return Ok(());
        }
        let composite = Composite::default();
        println!("Adding init files...");
        composite.handle(tera, context)?;
        println!("Init files added.");
        Ok(())
    }
    fn name(&self) -> &str {
        "init"
    }
}

fn create_project(project_name: &str, disable_vcs: bool) -> Result<(), MvpError> {
    println!("Creating project: {}", project_name);
    let mut cmd = Command::new("cargo");
    cmd.arg("new").arg(project_name);

    if disable_vcs {
        cmd.arg("--vcs").arg("none");
    }

    let status = cmd.status()?;

    if status.success() {
        println!("Project '{}' created successfully!", project_name);
        // 新增cd into the project directory
        std::env::set_current_dir(project_name)?;
        Ok(())
    } else {
        Err(MvpError::Custom(format!(
            "Failed to create project '{}'",
            project_name
        )))
    }
}
