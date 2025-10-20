use std::fs::write;

use super::super::prelude::*;

pub struct FmtStrategy;

// Add rustfmt.toml
impl AddStrategy for FmtStrategy {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        let content = tera.render("rustfmt.toml", context)?;
        write("rustfmt.toml", content.as_bytes())?;
        println!("Created rustfmt.toml");
        Ok(())
    }
    fn name(&self) -> &str {
        "fmt"
    }
}
