use super::preludes::*;

#[derive(Debug, Parser)]
pub struct SubOpts {}

impl CmdExector for SubOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("Hello from sub command!");
        Ok(())
    }
}
