use std::fs;

use super::prelude::*;

pub struct MdStrategy;

// Add README.md
impl AddStrategy for MdStrategy {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        let content = tera.render("README.md", context)?;
        fs::write("README.md", content.as_bytes())?;
        println!("Created README.md");
        Ok(())
    }
    fn name(&self) -> &str {
        "md"
    }
}
