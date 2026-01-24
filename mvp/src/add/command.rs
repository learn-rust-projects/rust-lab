//! AddCommand 枚举 - 定义所有可用的 add 子命令
//!
//! 枚举顺序决定了 Composite 策略的执行顺序：
//! vscode -> fmt -> md -> git -> ci -> lic (许可证在最后)

use clap::Subcommand;
use enum_dispatch::enum_dispatch;
use strum::{Display, EnumIter, IntoEnumIterator, IntoStaticStr};

use crate::{
    add::{
        impls::{CiOpts, FmtOpts, GitOpts, InitOpts, LicOpts, MdOpts, VscodeOpts},
        prelude::{Context, ExcuteStrategy, Tera},
    },
    error::MvpError,
};

/// Add 子命令枚举
///
/// 每个变体对应一个策略实现。枚举的定义顺序即为批量执行时的顺序。
#[derive(Subcommand, Debug, Clone, IntoStaticStr, Display, EnumIter)]
#[strum(serialize_all = "lowercase")]
#[derive(PartialEq, Eq, Hash)]
#[enum_dispatch(ExcuteStrategy)]
pub enum AddCommand {
    /// 初始化新项目
    #[command(name = "init")]
    Init(InitOpts),

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

impl AddCommand {
    /// 返回所有命令的迭代器（按枚举定义顺序）
    pub fn all() -> impl Iterator<Item = AddCommand> {
        AddCommand::iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_order() {
        let commands: Vec<AddCommand> = AddCommand::all().collect();
        assert_eq!(
            commands,
            vec![
                AddCommand::Init(InitOpts::default()),
                AddCommand::Vscode(VscodeOpts::default()),
                AddCommand::Fmt(FmtOpts::default()),
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
