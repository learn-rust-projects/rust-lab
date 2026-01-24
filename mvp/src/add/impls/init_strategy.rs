use std::process::Command;

use clap::Parser;

use super::{super::prelude::*, composite::Composite};

pub struct InitStrategy;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct InitOpts {
    /// 项目名称
    #[arg(
        help = "Name of the project (default: current directory name)",
        default_value = "init"
    )]
    name: String,

    /// 是否禁用 VCS (git)
    #[arg(
        short = 'n',
        long,
        help = "Disable VCS initialization (default: false)",
        default_value_t = false
    )]
    no_vcs: bool,
}

impl ExcuteStrategy for InitOpts {
    fn excute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始初始化项目是否禁用 VCS {}", self.no_vcs);
        let project_name = self.name.as_ref();

        let disable_vcs = self.no_vcs;

        create_project(project_name, disable_vcs)?;
        let composite = Composite::default();
        println!("Adding init files...");
        composite.handle(tera, context)?;
        println!("Init files added.");
        tracing::info!("项目初始化成功");
        Ok(())
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
