use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
pub mod prelude;
use tera::{Context, Tera};

use crate::{
    cli::Commands,
    error::MvpError,
    sub_add::{command::AddCommand, impls::InitOpts},
    sub_template::TemplateCommand,
};
#[enum_dispatch]
pub trait Strategy: Debug + Send + Sync {
    fn execute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError>;
}
#[enum_dispatch]
pub trait CommandStrategy: Debug + Send + Sync {
    fn execute(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError>;
}
