use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use eyre::Result;

use crate::errors::BbEcsError;

use super::resource::Resource;

pub struct ResourcesData {
    resources: HashMap<String, Rc<RefCell<Resource>>>,
}

impl ResourcesData {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a resource into storage, overwriting any resource that had the same name
    pub fn insert(&mut self, name: String, resource: Resource) {
        self.resources.insert(name, Rc::new(RefCell::new(resource)));
    }

    pub fn get(&self, name: &str) -> Result<&Rc<RefCell<Resource>>> {
        if let Some(resource) = self.resources.get(name) {
            Ok(resource)
        } else {
            Err(BbEcsError::ResourceNotFound(name.to_owned()).into())
        }
    }
}

impl Default for ResourcesData {
    fn default() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}
