use std::{collections::HashMap, sync::LazyLock};

use super::super::strategy::*;
pub struct AddStrategyFactory {
    handlers: HashMap<String, Box<dyn AddStrategy>>,
}
impl Default for AddStrategyFactory {
    fn default() -> Self {
        Self::new()
    }
}
impl AddStrategyFactory {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    fn register(&mut self, v: Box<dyn AddStrategy>) {
        let k = v.name().to_owned();
        self.handlers.insert(k, v);
    }

    pub fn get(&self, k: &str) -> Option<&dyn AddStrategy> {
        self.handlers.get(k).map(|b| b.as_ref())
    }

    pub fn get_add_strategy_factory() -> &'static AddStrategyFactory {
        static FACTORY: LazyLock<AddStrategyFactory> = LazyLock::new(|| {
            let mut factory = AddStrategyFactory::new();
            factory.register(Box::new(InitStrategy));
            factory.register(Box::new(MdStrategy));
            factory.register(Box::new(LicStrategy));
            factory.register(Box::new(VscodeStrategy));
            factory.register(Box::new(FmtStrategy));
            factory
        });
        &FACTORY
    }
}
