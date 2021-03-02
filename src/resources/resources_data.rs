use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

use super::resource::{Resource, ResourceCast};

pub type GetResourceData = Rc<RefCell<Resource>>;

pub trait ResourceDataLens<T> {
    fn get(&self, name: &str) -> &T;
}

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
}

impl Default for ResourcesData {
    fn default() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}
