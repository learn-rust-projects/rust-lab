use std::fs;

use clap::Parser;

use super::super::prelude::*;

pub struct MdStrategy;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct MdOpts {
    /// 文件名
    #[arg(
        long,
        help = "Name of the README file (default: README.md)",
        default_value = "README.md"
    )]
    filename: String,
}

// Add README.md
impl ExcuteStrategy for MdOpts {
    fn excute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始添加Markdown文件");
        let content = tera.render("README.md", context)?;
        fs::write("README.md", content.as_bytes())?;
        println!("Created README.md");
        tracing::info!("Markdown文件添加成功");
        Ok(())
    }
}
