use std::fs::write;

use clap::Parser;

use super::super::prelude::*;

pub struct FmtStrategy;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FmtOpts;

// Add rustfmt.toml
impl ExcuteStrategy for FmtOpts {
    fn excute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始添加格式化配置 {}", "rustfmt.toml");
        let content = tera.render("rustfmt.toml", context)?;
        write("rustfmt.toml", content.as_bytes())?;
        println!("Created rustfmt.toml");
        tracing::info!("格式化配置添加成功");
        Ok(())
    }
}
