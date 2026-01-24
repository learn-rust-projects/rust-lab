mod ci_strategy;
mod fmt_strategy;
mod git_ignore_strategy;
mod init_strategy;
mod lic_strategy;
mod md_strategy;
mod vscode_strategy;

mod composite;

pub use ci_strategy::*;
pub use fmt_strategy::*;
pub use git_ignore_strategy::*;
pub use init_strategy::*;
pub use lic_strategy::*;
pub use md_strategy::*;
pub use vscode_strategy::*;
