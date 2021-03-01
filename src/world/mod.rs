mod entity_data;

use std::hash::Hash;

use entity_data::EntityData;

use crate::resources::resources_data::ResourcesData;

pub struct World {
    entity_data: EntityData,
    resources: ResourcesData,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}

// todo implement the following methods
// register component
// insert spawn entity
// with component
// query one
// insert resource
// get resource

impl Default for World {
    fn default() -> Self {
        Self {
            entity_data: EntityData::new(),
            resources: ResourcesData::new(),
        }
    }
}
