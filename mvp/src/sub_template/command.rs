use clap::Subcommand;
use enum_dispatch::enum_dispatch;
use strum::{Display, EnumIter, IntoStaticStr};
use tera::{Context, Tera};

use crate::{error::MvpError, strategy::Strategy, sub_template::impls::cli_strategy::CliOpts};

#[derive(Subcommand, Debug, Clone, IntoStaticStr, Display, EnumIter)]
#[strum(serialize_all = "lowercase")]
#[derive(PartialEq, Eq, Hash)]
#[enum_dispatch(Strategy)]
pub enum TemplateCommand {
    #[command(name = "cli")]
    Cli(CliOpts),
}
impl crate::strategy::CommandStrategy for TemplateCommand {
    fn execute(
        &self,
        tera: &tera::Tera,
        context: &mut tera::Context,
    ) -> Result<(), crate::error::MvpError> {
        crate::strategy::Strategy::execute(self, tera, context)
    }
}
