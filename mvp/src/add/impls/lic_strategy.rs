use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use clap::Parser;

use super::super::prelude::*;

pub struct LicStrategy;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct LicOpts {
    /// 是否添加 MIT 许可证
    #[arg(
        short,
        long,
        help = "Add MIT license (default: true)",
        default_value_t = false
    )]
    mit: bool,

    /// 是否添加 Apache 许可证
    #[arg(
        short,
        long,
        help = "Add Apache 2.0 license (default: true)",
        default_value_t = false
    )]
    apache: bool,
}

// Add License
impl ExcuteStrategy for LicOpts {
    fn excute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始添加许可证");
        if self.apache {
            // Render and write LICENSE-APACHE
            write_license_md(tera, context, "LICENSE-APACHE")?;
            write_md(tera, context, "LICENSE-APACHE", true)?;
        }

        if self.mit {
            // Render and write LICENSE-MIT
            write_license_md(tera, context, "LICENSE-MIT")?;
            write_md(tera, context, "LICENSE-MIT", true)?;
        }
        if self.apache || !self.mit {
            write_license_md(tera, context, "LICENSE-APACHE")?;
            write_license_md(tera, context, "LICENSE-MIT")?;
            // Append LICENSE info to README.md
            write_md(tera, context, "LICENSE-APACHE && LICENSE-MIT", false)?;
        }
        tracing::info!("许可证添加成功");
        Ok(())
    }
}

fn write_license_md(tera: &Tera, context: &mut Context, license: &str) -> Result<(), MvpError> {
    tracing::info!("开始添加许可证 {}", license);

    let license_mit = tera.render(license, context)?;
    fs::write(license, license_mit.as_bytes())?;
    println!("Created {license}");

    Ok(())
}
fn write_md(
    tera: &Tera,
    context: &mut Context,
    license: &str,
    singel: bool,
) -> Result<(), MvpError> {
    tracing::info!("开始添加许可证 {} 到 README.md", license);

    let template = if singel {
        "SINGLE-LIC.md"
    } else {
        "LICENSE.md"
    };
    let license_md = tera.render(template, context)?;
    let mut readme = OpenOptions::new()
        .create(true)
        .append(true)
        .open("README.md")?;
    readme.write_all(license_md.as_bytes())?;
    println!("Added {license} info to README.md");
    Ok(())
}
