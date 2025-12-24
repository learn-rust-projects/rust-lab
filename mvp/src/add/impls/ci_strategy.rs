use std::fs;

use super::super::prelude::*;

pub struct CiStrategy;

// Add .gitignore
impl AddStrategy for CiStrategy {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        let target_dir = ".github/workflows";
        let target_file_ci = format!("{}/ci.yml", target_dir);

        fs::create_dir_all(target_dir)?; // Ensure .github/workflows directory exists

        let ci = tera.render(".github/workflows/ci.yml", context)?;
        fs::write(&target_file_ci, ci.as_bytes())?;

        println!("Created {}", target_file_ci);

        Ok(())
    }
    fn name(&self) -> &str {
        "ci"
    }
}
