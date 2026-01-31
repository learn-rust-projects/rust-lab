use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::{sub_add::command::AddCommand, sub_template::TemplateCommand};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Short description here",
    long_about = "This is a longer description of your CLI tool.\nIt can span multiple lines, and provides more details in the help output."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// Optional list of values
    pub values: Option<Vec<String>>,
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(CommandStrategy)]
pub enum Commands {
    /// Add a new component
    #[command(subcommand)]
    Add(AddCommand),
    /// Generate project templates
    #[command(subcommand)]
    Template(TemplateCommand),
}
