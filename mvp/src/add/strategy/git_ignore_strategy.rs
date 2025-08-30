use std::fs;

use super::prelude::*;

pub struct GitIgnoreStrategy;

// Add .gitignore
impl AddStrategy for GitIgnoreStrategy {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        let content = tera.render(".gitignore", context)?;
        fs::write(".gitignore", content.as_bytes())?;
        println!("Created .gitignore");
        Ok(())
    }
    fn name(&self) -> &str {
        "gi"
    }
}
