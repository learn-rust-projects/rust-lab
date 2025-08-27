pub trait AddStrategy: Sync + Send {
    fn handle(&self);
    fn str(&self) -> &str;
}
