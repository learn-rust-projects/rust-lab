use std::fs;

use clap::Parser;

use super::super::prelude::*;

pub struct GitIgnoreStrategy;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GitOpts {
    /// 是否添加 .gitattributes
    #[arg(
        long,
        help = "Add .gitattributes file (default: true)",
        default_value_t = true
    )]
    add_attributes: bool,
}

// Add .gitignore
impl ExcuteStrategy for GitOpts {
    fn excute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始添加git相关配置: .gitignore .gitattributes");
        let content = tera.render(".gitignore", context)?;
        fs::write(".gitignore", content.as_bytes())?;
        println!("Created .gitignore");

        let content = tera.render(".gitattributes", context)?;
        fs::write(".gitattributes", content.as_bytes())?;
        println!("Created .gitattributes");

        tracing::info!("GitIgnore配置添加成功");
        Ok(())
    }
}
