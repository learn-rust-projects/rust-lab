pub use tera::{Context, Tera};

use crate::error::MvpError;

pub trait AddStrategy: Sync + Send {
    fn handle(&self, tera: &Tera, context: &mut Context) -> Result<(), MvpError>;
    fn name(&self) -> &str;
}
