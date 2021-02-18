use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::components::Component;

pub type Components = Rc<RefCell<Vec<Component>>>;

pub struct EntityData<K> {
    components: HashMap<K, Rc<RefCell<Vec<Component>>>>,
}

impl<K> EntityData<K>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: K, component: Component) {
        self.components
            .entry(name)
            .or_default()
            .borrow_mut()
            .push(component);
    }

    /// Query for a single vector of components in the entity data hashmap. If we cannot find the components then we
    /// will panic.
    pub fn query_one(&self, name: &K) -> &Components {
        self.components.get(name).unwrap()
    }
}
