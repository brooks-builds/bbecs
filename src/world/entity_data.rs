use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::components::Component;

pub type Components = Rc<RefCell<Vec<Component>>>;

#[derive(Default)]
pub struct EntityData {
    components: HashMap<String, Rc<RefCell<Vec<Component>>>>,
}

impl EntityData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: &str, component: Component) {
        self.components
            .entry(name.to_owned())
            .or_default()
            .borrow_mut()
            .push(component);
    }

    /// Query for a single vector of components in the entity data hashmap. If we cannot find the components then we
    /// will panic.
    pub fn query_one(&self, name: &str) -> &Components {
        self.components.get(name).unwrap()
    }
}
