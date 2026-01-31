use crate::{
    sub_add::impls::{CiOpts, FmtOpts, GitOpts, LicOpts, MdOpts, VscodeOpts},
    strategy::prelude::*,
};

#[derive(Debug)]
pub struct Composite {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Default for Composite {
    fn default() -> Self {
        Self {
            strategies: vec![
                Box::new(VscodeOpts::default()),
                Box::new(FmtOpts::default()),
                Box::new(MdOpts::default()),
                Box::new(GitOpts::default()),
                Box::new(CiOpts::default()),
                Box::new(LicOpts::default()),
            ],
        }
    }
}

impl Composite {
    pub fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        tracing::info!("开始添加Composite配置");
        for strat in &self.strategies {
            strat.execute(tera, context)?;
        }
        tracing::info!("Composite配置添加成功");
        Ok(())
    }
}
