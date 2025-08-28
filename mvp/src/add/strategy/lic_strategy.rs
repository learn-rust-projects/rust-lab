use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use super::prelude::*;

pub struct LicStrategy;

// Add License
impl AddStrategy for LicStrategy {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError> {
        // Render and write LICENSE-APACHE
        let license_apache = tera.render("LICENSE-APACHE", context)?;
        fs::write("LICENSE-APACHE", license_apache.as_bytes())?;
        println!("Created LICENSE-APACHE");

        // Render and write LICENSE-MIT
        let license_mit = tera.render("LICENSE-MIT", context)?;
        fs::write("LICENSE-MIT", license_mit.as_bytes())?;
        println!("Created LICENSE-MIT");

        // Append LICENSE info to README.md
        let license_md = tera.render("LICENSE.md", context)?;
        let mut readme = OpenOptions::new()
            .create(true)
            .append(true)
            .open("README.md")?;
        readme.write_all(license_md.as_bytes())?;
        println!("Added LICENSE info to README.md");

        Ok(())
    }
    fn name(&self) -> &str {
        "lic"
    }
}
