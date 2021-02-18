use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use super::resource::Resource;

pub type GetResourceData = Rc<RefCell<Resource>>;

pub struct ResourcesData<K> {
    resources: HashMap<K, Rc<RefCell<Resource>>>,
}

impl<K> ResourcesData<K>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: K, resource: Resource) {
        self.resources.insert(name, Rc::new(RefCell::new(resource)));
    }

    /// Get a reference to the resource given the key. If we cannot find the resource we will panic and crash.
    pub fn get(&self, name: &K) -> &GetResourceData {
        self.resources.get(name).unwrap()
    }
}

impl<K> Default for ResourcesData<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}
