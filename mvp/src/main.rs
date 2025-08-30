use std::{collections::HashMap, sync::LazyLock};

use chrono::Datelike;
use clap::{Parser, Subcommand};
use mvp::{add::*, error::MvpError};
use tera::{Context, Result as TeraResult, Tera, Value};

// Custom filter: does nothing
fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    Ok(value.clone())
}

// Global template singleton
pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = Tera::default();
    use mvp::templates::*;
    tera.add_raw_template("LICENSE-APACHE", LICENSE_APACHE)
        .unwrap();
    tera.add_raw_template("LICENSE-MIT", LICENSE_MIT).unwrap();
    tera.add_raw_template("LICENSE.md", LICENSE).unwrap();
    tera.add_raw_template("README.md", README).unwrap();
    tera.add_raw_template("rustfmt.toml", RUSTFMT).unwrap();
    tera.add_raw_template("vscode/settings.json", SETTINGS)
        .unwrap();
    tera.autoescape_on(vec![".html", ".sql"]);
    tera.register_filter("do_nothing", do_nothing_filter);
    tera
});

#[derive(Parser)]
#[command(
    version,
    about = "Short description here",
    long_about = "This is a longer description of your CLI tool.\nIt can span multiple lines, and provides more details in the help output."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Optional list of values
    values: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new component
    Add {
        /// Name of the component to add
        name: String,
    },
}

fn fill_context_with_year_and_author(context: &mut Context) {
    let current_year = chrono::Utc::now().year();
    context.insert("year", &current_year);
    context.insert("author", "Levio-Z");
}

fn main() -> Result<(), MvpError> {
    let mut context = Context::new();
    fill_context_with_year_and_author(&mut context);

    for name in TEMPLATES.get_template_names() {
        println!("Loaded template: {name}");
    }

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { name }) => {
            if let Some(handler) = AddStrategyFactory::get_add_strategy_factory().get(name) {
                println!("Add {}", name);
                handler.handle(&TEMPLATES, &mut context)?;
            } else {
                eprintln!("No strategy found for '{}'.", name);
            }
        }
        None => {
            context.insert("init_values", &cli.values);
            if let Some(handler) = AddStrategyFactory::get_add_strategy_factory().get("init") {
                handler.handle(&TEMPLATES, &mut context)?;
            } else {
                eprintln!("No strategy found for 'init'.");
            }
        }
    }
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
