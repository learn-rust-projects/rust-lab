use std::{collections::HashMap, sync::LazyLock};

use chrono::Datelike;
use clap::{Parser, Subcommand};
#[cfg(test)]
use mvp::add::impls::VscodeOpts;
use mvp::{
    add::{
        impls::InitOpts,
        prelude::{AddCommand, ExcuteStrategy},
    },
    error::MvpError,
};
use tera::{Context, Result as TeraResult, Tera, Value};

// Custom filter: does nothing
fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    Ok(value.clone())
}
// include! 会在编译期把文件内容插入这里
include!("templates.rs");

// Global template singleton
pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![".html", ".sql"]);
    // 可选配置
    tera.register_filter("do_nothing", do_nothing_filter);
    // 循环注册 build.rs 生成的模板
    for (name, content) in TEMPLATE_MAP {
        tera.add_raw_template(name, content).unwrap();
    }

    tera
});

#[derive(Parser, Debug)]
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

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new component
    #[command(subcommand)]
    Add(AddCommand),
    /// Initialize a new project
    Init(InitOpts),
}

fn fill_context_with_year_and_author(context: &mut Context) {
    let current_year = chrono::Utc::now().year();
    context.insert("year", &current_year);
    context.insert("author", "Levio-Z");
}

fn main() -> Result<(), MvpError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    tracing::info!("Starting mvp");
    let cli = Cli::parse();
    run(&cli)
}

fn run(cli: &Cli) -> Result<(), MvpError> {
    for name in TEMPLATES.get_template_names() {
        println!("Loaded template: {name}");
    }
    let mut context = Context::new();
    fill_context_with_year_and_author(&mut context);
    match &cli.command {
        Some(Commands::Add(cmd)) => {
            println!("Add command: {}", Into::<&'static str>::into(cmd.clone()));
            cmd.excute(&TEMPLATES, &mut context)?;
        }
        Some(Commands::Init(cmd)) => {
            context.insert("init_values", &cli.values);
            // Default to init
            cmd.excute(&TEMPLATES, &mut context)?;
        }
        None => {
            return Err(MvpError::NoCommand);
        }
    }
    // 这里可以根据解析到的命令行参数执行相应的逻辑
    Ok(())
}

#[cfg(test)]
mod tests {

    use tempfile::tempdir;

    use super::*;
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
    // 如果 TEMPLATES 或全局资源在测试中会被修改，需要用 Mutex 或 LazyLock + reset
    // 方法保证环境还原

    #[test]
    fn test_add_strategy_in_temp_dir() {
        // 创建一个临时目录
        let dir = tempdir().expect("Failed to create temp dir");
        let temp_path = dir.path();

        // 保存当前工作目录
        let original_dir = std::env::current_dir().expect("Failed to get current dir");

        // 切换到临时目录
        std::env::set_current_dir(&temp_path).expect("Failed to change dir");

        // 构造 CLI
        let cli = Cli {
            command: Some(Commands::Add(AddCommand::Vscode(VscodeOpts::default()))),
            values: None,
        };

        // 调用核心逻辑
        let result = run(&cli);

        // 验证执行成功
        assert!(result.is_ok(), "Add strategy should succeed");

        // 如果测试中会生成 `.vscode` 文件夹，可以在这里验证它存在
        let vscode_path = temp_path.join(".vscode");
        assert!(vscode_path.exists(), ".vscode folder should be created");

        // 测试结束，临时目录会自动删除
        drop(dir);

        // 恢复工作目录
        std::env::set_current_dir(&original_dir).expect("Failed to restore original dir");
    }
}
