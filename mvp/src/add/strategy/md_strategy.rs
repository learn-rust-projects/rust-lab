use super::AddStrategy;
pub struct MdStrategy;
// add md
impl AddStrategy for MdStrategy {
    fn handle(&self) {
        unimplemented!()
    }
    fn str(&self) -> &str {
        "md"
    }
}
