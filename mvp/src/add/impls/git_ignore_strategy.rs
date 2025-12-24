use std::fs;

use super::super::prelude::*;

pub struct GitIgnoreStrategy;

// Add .gitignore
impl AddStrategy for GitIgnoreStrategy {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        let content = tera.render(".gitignore", context)?;
        fs::write(".gitignore", content.as_bytes())?;
        println!("Created .gitignore");
        let content = tera.render(".gitattributes", context)?;
        fs::write(".gitattributes", content.as_bytes())?;
        println!("Created .gitattributes");
        Ok(())
    }
    fn name(&self) -> &str {
        "git"
    }
}
