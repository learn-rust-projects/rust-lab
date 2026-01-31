use std::fs;

use clap::Parser;

use crate::strategy::prelude::*;

pub struct MdStrategy;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MdOpts {
    /// 文件名
    #[arg(
        long,
        help = "Name of the README file (default: README.md)",
        default_value = "README.md"
    )]
    filename: String,
}
// Provide a default implementation for MdOpts 来源是策略的时候会用到
impl Default for MdOpts {
    fn default() -> Self {
        MdOpts {
            filename: "README.md".to_string(),
        }
    }
}
// Add README.md
impl Strategy for MdOpts {
    fn execute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始添加Markdown文件");
        let content = tera.render("README.md", context)?;
        fs::write("README.md", content.as_bytes())?;
        println!("Created README.md");
        tracing::info!("Markdown文件添加成功");
        Ok(())
    }
}
