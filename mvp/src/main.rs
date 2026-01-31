use std::{collections::HashMap, sync::LazyLock};

use chrono::Datelike;
use clap::Parser;
use mvp::{cli::Cli, error::MvpError, strategy::CommandStrategy};
use tera::{Context, Result as TeraResult, Tera, Value};

fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    Ok(value.clone())
}

include!("templates.rs");

pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![".html", ".sql"]);
    tera.register_filter("do_nothing", do_nothing_filter);
    for (name, content) in TEMPLATE_MAP {
        tera.add_raw_template(name, content).unwrap();
    }
    tera
});

fn init_context(cli: &Cli) -> Context {
    let mut context = Context::new();
    let current_year = chrono::Utc::now().year();
    context.insert("year", &current_year);
    context.insert("author", "Levio-Z");
    context.insert("init_values", &cli.values);
    context
}

fn main() -> Result<(), MvpError> {
    let cli = Cli::parse();
    run(cli)
}

fn run(cli: Cli) -> Result<(), MvpError> {
    let mut context = init_context(&cli);
    match cli.command {
        Some(cmd) => {
            cmd.execute(&TEMPLATES, &mut context)?;
        }
        None => {
            return Err(MvpError::NoCommand);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use mvp::{cli::Commands, sub_add::impls::VscodeOpts};
    use tempfile::tempdir;

    use super::*;
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_add_strategy_in_temp_dir() {
        let dir = tempdir().expect("Failed to create temp dir");
        let temp_path = dir.path();

        let original_dir = std::env::current_dir().expect("Failed to get current dir");

        std::env::set_current_dir(temp_path).expect("Failed to change dir");

        let cli = Cli {
            command: Some(Commands::Add(mvp::sub_add::command::AddCommand::Vscode(
                VscodeOpts::default(),
            ))),
            values: None,
        };

        let result = run(cli);

        assert!(result.is_ok(), "Add strategy should succeed");

        let vscode_path = temp_path.join(".vscode");
        assert!(vscode_path.exists(), ".vscode folder should be created");

        drop(dir);

        std::env::set_current_dir(&original_dir).expect("Failed to restore original dir");
    }
}
