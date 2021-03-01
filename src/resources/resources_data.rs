use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use super::resource::Resource;

pub type GetResourceData = Rc<RefCell<Resource>>;

pub struct ResourcesData {
    resources: HashMap<String, GetResourceData>,
}

impl ResourcesData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: String, resource: Resource) {
        self.resources.insert(name, Rc::new(RefCell::new(resource)));
    }

    /// Get a reference to the resource given the key. If we cannot find the resource we will panic and crash.
    pub fn get(&self, name: &str) -> &GetResourceData {
        self.resources.get(name).unwrap()
    }
}

impl Default for ResourcesData {
    fn default() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}
