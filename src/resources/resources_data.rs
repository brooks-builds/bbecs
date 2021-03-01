use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
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

impl ResourceDataLens<u32> for ResourcesData {
    fn get(&self, name: &str) -> &u32 {
        let wrapped_resource = self.resources.get(name).unwrap().borrow();
        wrapped_resource.cast()
        // referencing https://www.youtube.com/redirect?event=comments&redir_token=QUFFLUhqa2hMRTlfTmNHQUN1N0ZPZ2N4dUwyN3RUN0RpQXxBQ3Jtc0trcnE2TWp5WGZSazV5eDhNazF1cU9RRUNfRlFzZWpxTHR0bzY5d0dPZEpSZWlMcnZhaVMwNlFkekctcnJkcDNwdnRhQlNmZUV3NDNBQW5NX3EyOFo5aTFvR0NUaW41SFRZWTU5WllfQ2JyNWk0RElMUQ&q=https%3A%2F%2Fplay.rust-lang.org%2F%3Fversion%3Dstable%26mode%3Ddebug%26edition%3D2018%26gist%3D333ce298801245684b26acbd8f0b6cb7&stzid=Ugx29ERFRLqkICw2_ZF4AaABAg.9Jx18LbsCFq9Jy0svrkSMe
        // get and cast at the same time to see if that works
    }
}
