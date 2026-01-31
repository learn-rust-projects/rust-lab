use std::fs;

use clap::Parser;

use crate::strategy::prelude::*;

pub struct CiStrategy;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CiOpts {
    /// CI 工作流文件名
    #[arg(
        long,
        help = "Name of the CI workflow file (default: ci.yml)",
        default_value = "ci.yml"
    )]
    workflow_name: String,
}

// Add CI configuration
impl Strategy for CiOpts {
    fn execute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始添加CI配置 {}", "ci.yml");
        let target_dir = ".github/workflows";
        let target_file_ci = format!("{}/{}", target_dir, "ci.yml");

        fs::create_dir_all(target_dir)?; // Ensure .github/workflows directory exists

        let ci = tera.render(target_file_ci.as_str(), context)?;
        fs::write(&target_file_ci, ci.as_bytes())?;

        println!("Created {}", target_file_ci);
        tracing::info!("CI配置添加成功");
        Ok(())
    }
}
