use std::{env, path::PathBuf};

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

    /// vec参数
    #[arg(short, long, value_name = "FILE", num_args = 0..)] // 0个或多个
    config: Vec<PathBuf>,

    // use long arg to receive String argument: not required
    #[arg(long)]
    // 可能为空的才需要Option
    arg: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, default_value_t = 0)]
    // 有默认值可以不加Option
    debug: u8,

    // 核心参数设置为位置参数，其他都设置为可选参数更友好
    // 没有 #[arg] 标记，因此它会被解析为位置参数，即必须直接在命令行中输入
    /// 端口号
    port: u16,

    /// 可选位置参数
    name: Option<String>,
    /// subcommand
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    #[command(about = "命令结构体")]
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// 测试2
    #[command(about = "命令元组结构体")]
    Test2(Test2Opts),
    /// 测试3
    #[command(subcommand, about = "子命令枚举结构体")]
    Test3(Test3),
}
#[derive(Parser, Debug)]
struct Test2Opts {
    #[arg(short, long)]
    list: bool,
}

#[derive(Parser, Debug)]
enum Test3 {
    /// 测试3
    #[command(about = "命令结构体")]
    TestSub,
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    for config_path in &cli.config {
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
        Some(Commands::Test2(opts)) => {
            if opts.list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Test3(Test3::TestSub)) => {
            println!("Printing testing lists...");
        }
        None => {}
    }

    // Continued program logic goes here...

    if let Some(arg) = cli.arg.as_deref() {
        println!("Value for arg: {arg}");
    }
}
#[cfg(test)]
mod test {
    use super::*;
    // 添加测试
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert(); // 关键：触发参数定义检查
    }

    #[test]
    fn test_full_cli_with_subcommand() {
        let _ = dotenv::from_path("./.env");
        let test_key = env::var("test_key").unwrap();
        assert_eq!(test_key, "test_value");
        let test_key = env::var_os("test_key").unwrap();
        println!("test_key: {:?}", test_key);
        assert_eq!(test_key.into_string().unwrap(), "test_value");

        // 顺序
        let cli = Cli::parse_from([
            "bin",
            "-b",
            "-c",
            "./config.toml",
            "--arg",
            "hello",
            "-d",
            "3",
            // 位置参数
            "8080",
            // 可选位置参数
            "name",
            // 子命令
            "test",
            "--list",
        ]);

        assert!(cli.bool);
        assert_eq!(cli.config, vec![PathBuf::from("./config.toml")]);
        assert_eq!(cli.arg.as_deref(), Some("hello"));
        assert_eq!(cli.debug, 3);
        assert_eq!(cli.port, 8080);

        assert_eq!(cli.name.as_deref(), Some("name"));

        match cli.command {
            Some(Commands::Test { list }) => assert!(list),
            _ => panic!("Expected Commands::Test with list = true"),
        }
    }
    #[test]
    fn test_full_cli_with_subcommand2() {
        let _ = dotenv::from_path("./.env");
        let test_key = env::var("test_key").unwrap();
        assert_eq!(test_key, "test_value");
        let test_key = env::var_os("test_key").unwrap();
        println!("test_key: {:?}", test_key);
        assert_eq!(test_key.into_string().unwrap(), "test_value");

        // 顺序
        let cli = Cli::parse_from([
            "bin",
            "-b",
            "-c",
            "./config.toml",
            "--arg",
            "hello",
            "-d",
            "3",
            // 位置参数
            "8080",
            // 可选位置参数
            "name",
            // 子命令
            "test2",
            "--list",
        ]);

        assert!(cli.bool);
        assert_eq!(cli.config, vec![PathBuf::from("./config.toml")]);
        assert_eq!(cli.arg.as_deref(), Some("hello"));
        assert_eq!(cli.debug, 3);
        assert_eq!(cli.port, 8080);

        assert_eq!(cli.name.as_deref(), Some("name"));

        match cli.command {
            Some(Commands::Test2(opts)) => assert!(opts.list),
            _ => panic!("Expected Commands::Test2 with list = true"),
        }
    }
    #[test]
    fn test_full_cli_with_subcommand3() {
        let _ = dotenv::from_path("./.env");
        let test_key = env::var("test_key").unwrap();
        assert_eq!(test_key, "test_value");
        let test_key = env::var_os("test_key").unwrap();
        println!("test_key: {:?}", test_key);
        assert_eq!(test_key.into_string().unwrap(), "test_value");

        // 顺序
        let cli = Cli::parse_from([
            "bin",
            "-b",
            "-c",
            "./config.toml",
            "--arg",
            "hello",
            "-d",
            "3",
            // 位置参数
            "8080",
            // 可选位置参数
            "name",
            // 子命令
            "test3",
            "test-sub",
        ]);

        assert!(cli.bool);
        assert_eq!(cli.config, vec![PathBuf::from("./config.toml")]);
        assert_eq!(cli.arg.as_deref(), Some("hello"));
        assert_eq!(cli.debug, 3);
        assert_eq!(cli.port, 8080);

        assert_eq!(cli.name.as_deref(), Some("name"));

        match cli.command {
            Some(Commands::Test3(Test3::TestSub)) => {}
            _ => panic!("Expected Commands::Test3(Test3::Test31)"),
        }
    }
}
