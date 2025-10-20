use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name ="myapp", //--version will show name
    version ,
    about = "Short description here",
    long_about = "This is a longer description of your CLI tool.\nIt can span multiple lines, and provides more details in the help output."
)]
struct Cli {
    /// bool 布尔开关
    #[arg(short, long)]
    bool: bool,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    // use long arg to receive String argument: not required
    #[arg(long)]
    arg: Option<String>,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: u8,

    // 没有 #[arg] 标记，因此它会被解析为位置参数，即必须直接在命令行中输入
    port: u16,

    /// Optional name to operate on
    name: Option<String>,
    /// subcommand
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    println!("port: {:?}", cli.port);

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...

    if let Some(arg) = cli.arg.as_deref() {
        println!("Value for arg: {arg}");
    }
}

// 添加测试
#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert(); // 关键：触发参数定义检查
}

#[test]
fn test_full_cli_with_subcommand() {
    let cli = Cli::parse_from([
        "bin",
        "-b",
        "-c",
        "./config.toml",
        "--arg",
        "hello",
        "-d",
        "3",
        "8080",
        "name",
        "test",
        "--list",
    ]);

    assert!(cli.bool);
    assert_eq!(cli.config, Some(PathBuf::from("./config.toml")));
    assert_eq!(cli.arg.as_deref(), Some("hello"));
    assert_eq!(cli.debug, 3);
    assert_eq!(cli.port, 8080);

    assert_eq!(cli.name.as_deref(), Some("name"));

    match cli.command {
        Some(Commands::Test { list }) => assert!(list),
        _ => panic!("Expected Commands::Test with list = true"),
    }
}
