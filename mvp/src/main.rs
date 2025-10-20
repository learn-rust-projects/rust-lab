use std::{collections::HashMap, sync::LazyLock};

use chrono::Datelike;
use clap::{Parser, Subcommand};
use mvp::{add::context::AddStrategyFactory, error::MvpError};
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

    // 循环注册 build.rs 生成的模板
    for (name, content) in TEMPLATE_MAP {
        tera.add_raw_template(name, content).unwrap();
    }

    // 可选配置
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
    // 这里可以根据解析到的命令行参数执行相应的逻辑
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::time::Duration;

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
            command: Some(Commands::Add {
                name: "vscode".to_string(),
            }),
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
