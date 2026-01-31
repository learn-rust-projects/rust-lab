pub mod ci_strategy;
pub mod fmt_strategy;
pub mod git_ignore_strategy;
pub mod init_strategy;
pub mod lic_strategy;
pub mod md_strategy;
pub mod vscode_strategy;

pub mod composite;

pub use ci_strategy::*;
pub use composite::*;
pub use fmt_strategy::*;
pub use git_ignore_strategy::*;
pub use init_strategy::*;
pub use lic_strategy::*;
pub use md_strategy::*;
pub use vscode_strategy::*;
