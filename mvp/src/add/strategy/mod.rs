mod add_strategy;

mod fmt_strategy;
mod git_ignore_strategy;
mod init_strategy;
mod lic_strategy;
mod md_strategy;
mod vscode_strategy;

mod composite;
mod prelude;

pub use add_strategy::AddStrategy;
pub use fmt_strategy::FmtStrategy;
pub use git_ignore_strategy::GitIgnoreStrategy;
pub use init_strategy::InitStrategy;
pub use lic_strategy::LicStrategy;
pub use md_strategy::MdStrategy;
pub use vscode_strategy::VscodeStrategy;
