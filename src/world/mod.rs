mod entity_data;

use std::hash::Hash;

use entity_data::{Components, EntityData};

use crate::components::Component;
use crate::resources::resource::Resource;
use crate::resources::resources_data::{GetResourceData, ResourcesData};

pub struct World<T, K> {
    entity_data: EntityData<T>,
    resources: ResourcesData<K>,
}

impl<T, K> World<T, K>
where
    T: Eq + std::hash::Hash,
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self::default()
    }

    /// We want to begin spawning an entity with all of its components into the ECS data
    /// we can't do that all in one go unfortunately so we are using a builder style
    /// pattern to enter the components one-by-one. This is meant to be used with the
    /// `with_component` function.
    pub fn spawn_entity(&mut self) -> &mut Self {
        self
    }

    /// The second part of the spawning entity builder pattern functions. To use this call
    /// `spawn_entity` first and then you can chain as many `with_component`s as you want.
    /// Each component is given a name which will determine how it is stored in the data. The components
    /// are made of the enum Component.
    /// ```
    /// let mut world = World::new();
    /// world.spawn_entity()
    ///     .with_component(32.0_f32)
    ///     .with_component(16_i32);
    /// ```
    pub fn with_component(&mut self, name: T, component: Component) -> &mut Self {
        self.entity_data.insert(name, component);
        self
    }

    pub fn query_one(&self, name: &T) -> &Components {
        self.entity_data.query_one(name)
    }

    pub fn add_resource(&mut self, name: K, resource: Resource) {
        self.resources.insert(name, resource);
    }

    pub fn get_resource(&self, name: &K) -> &GetResourceData {
        self.resources.get(name)
    }
}

impl<T, K> Default for World<T, K>
where
    T: Eq + Hash,
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            entity_data: EntityData::<T>::new(),
            resources: ResourcesData::<K>::new(),
        }
    }
}
