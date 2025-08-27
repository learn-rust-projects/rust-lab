use super::AddStrategy;
pub struct InitStrategy;

impl AddStrategy for InitStrategy {
    fn handle(&self) {
        unimplemented!()
    }
    fn str(&self) -> &str {
        "init"
    }
}
