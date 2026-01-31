mod preludes;
mod sub;
use clap::Parser;
use enum_dispatch::enum_dispatch;
pub use sub::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}

#[derive(Debug, Parser)]
#[command(name = "{{ project_name }}", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "sub", about = "A subcommand example")]
    Sub(SubOpts),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Opts::command().debug_assert();
    }

    #[tokio::test]
    async fn test_main_function() {
        let opts = Opts::parse_from(["{{ project_name }}", "sub"]);
        let result = opts.cmd.execute().await;
        assert!(result.is_ok());
    }
}
