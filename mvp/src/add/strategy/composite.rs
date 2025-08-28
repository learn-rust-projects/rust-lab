use super::prelude::*;
use crate::add::strategy::{
    fmt_strategy::FmtStrategy, md_strategy::MdStrategy, vscode_strategy::VscodeStrategy,
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
