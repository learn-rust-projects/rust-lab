use clap::{Parser, Subcommand};
use mvp::add::*;

#[derive(Parser)]
#[command(
    //name ="myapp", --version will show name
    version ,
    about = "Short description here",
    long_about = "This is a longer description of your CLI tool.\nIt can span multiple lines, and provides more details in the help output."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Add {
        /// lists test values
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let add = cli.command;
    if let Some(subcommand) = add {
        match subcommand {
            Commands::Add { name } => {
                if let Some(hadler) = AddStrategyFactory::get_add_strategy_factory().get(&name) {
                    hadler.handle();
                }
            }
        }
    }
}

// 添加测试
#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert(); // 关键：触发参数定义检查
}
