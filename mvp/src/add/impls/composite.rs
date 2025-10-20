use super::super::prelude::*;
use crate::add::impls::{
    fmt_strategy::FmtStrategy, git_ignore_strategy, md_strategy::MdStrategy,
    vscode_strategy::VscodeStrategy,
};

pub struct Composite {
    strategies: Vec<Box<dyn AddStrategy>>,
}

impl Default for Composite {
    fn default() -> Self {
        Self {
            strategies: vec![
                Box::new(VscodeStrategy),
                Box::new(FmtStrategy),
                Box::new(MdStrategy),
                Box::new(git_ignore_strategy::GitIgnoreStrategy),
            ],
        }
    }
}

impl Composite {
    pub fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        for strat in &self.strategies {
            println!("Running strategy: {}", strat.name());
            strat.handle(tera, context)?;
        }
        Ok(())
    }
}
