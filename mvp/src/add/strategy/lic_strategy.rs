use super::AddStrategy;
pub struct LicStrategy;
// add License
impl AddStrategy for LicStrategy {
    fn handle(&self) {
        unimplemented!()
    }
    fn str(&self) -> &str {
        "lic"
    }
}
