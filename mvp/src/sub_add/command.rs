//! AddCommand 枚举 - 定义所有可用的 add 子命令
//!
//! 枚举顺序决定了 Composite 策略的执行顺序：
//! vscode -> fmt -> md -> git -> ci -> lic (许可证在最后)

use clap::Subcommand;
use enum_dispatch::enum_dispatch;
use strum::{Display, EnumIter, IntoStaticStr};

use crate::{
    strategy::prelude::*,
    sub_add::impls::{CiOpts, FmtOpts, GitOpts, InitOpts, LicOpts, MdOpts, VscodeOpts},
};
#[derive(Subcommand, Debug, Clone, IntoStaticStr, Display, EnumIter)]
#[strum(serialize_all = "lowercase")]
#[derive(PartialEq, Eq, Hash)]
#[enum_dispatch(Strategy)]
pub enum AddCommand {
    /// 添加 VSCode 配置 (.vscode/settings.json, tasks.json)
    #[command(name = "vscode")]
    Vscode(VscodeOpts),

    /// 添加 rustfmt 配置 (rustfmt.toml)
    #[command(name = "fmt")]
    Fmt(FmtOpts),

    /// 添加 README.md
    #[command(name = "md")]
    Md(MdOpts),

    /// 添加 Git 配置 (.gitignore, .gitattributes)
    #[command(name = "git")]
    Git(GitOpts),

    /// 添加 CI 配置 (.github/workflows/ci.yml)
    #[command(name = "ci")]
    Ci(CiOpts),

    /// 添加许可证文件 (LICENSE-APACHE, LICENSE-MIT)
    #[command(name = "lic")]
    Lic(LicOpts),
}
impl crate::strategy::CommandStrategy for AddCommand {
    fn execute(
        &self,
        tera: &tera::Tera,
        context: &mut tera::Context,
    ) -> Result<(), crate::error::MvpError> {
        println!("Add command: {}", self);
        crate::strategy::Strategy::execute(self, tera, context)
    }
}


#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_command_order() {
        let commands: Vec<AddCommand> = AddCommand::iter().collect();
        assert_eq!(
            commands,
            vec![
                AddCommand::Vscode(VscodeOpts::default()),
                AddCommand::Fmt(FmtOpts),
                AddCommand::Md(MdOpts::default()),
                AddCommand::Git(GitOpts::default()),
                AddCommand::Ci(CiOpts::default()),
                AddCommand::Lic(LicOpts::default()), // lic 在最后
            ]
        );
    }

    #[test]
    fn test_command_as_str() {
        assert_eq!(
            Into::<&'static str>::into(AddCommand::Vscode(VscodeOpts::default())),
            "vscode"
        );
        assert_eq!(
            Into::<&'static str>::into(AddCommand::Lic(LicOpts::default())),
            "lic"
        );
    }
}
