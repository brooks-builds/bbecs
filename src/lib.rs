use std::any::Any;

use entities::Entities;
use query_builder::Query;
use resources::Resources;

pub mod entities;
pub mod query_builder;
pub mod resources;

#[derive(Debug, Default)]
pub struct World {
    resources: Resources,
    pub entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add(resource);
    }

    pub fn get_resource<T: Any + 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    pub fn get_resource_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn register_entity(&mut self) -> &mut Entities {
        self.entities.register_entity()
    }

    pub fn register_component(&mut self, component: impl Any) {
        self.entities.register_component(component);
    }

    pub fn query(&self) -> Query {
        self.entities.new_query()
    }
}
