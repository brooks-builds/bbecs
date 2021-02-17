mod data;

use std::cell::Cell;

use crate::components::Component;
use crate::resources::{Resources, ResourcesData};
use data::Data;

pub struct World {
    data: Data,
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        let data = Data::new();
        let resources = Resources::new();
        Self { data, resources }
    }

    /// We want to begin spawing an entity with all of its components into the ECS data
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
    pub fn with_component(&mut self, name: &str, component: Component) -> &mut Self {
        self.data.insert(name, component);
        self
    }

    pub fn query(&mut self, component_name: &str) -> Option<&mut Cell<Vec<Component>>> {
        self.data.query_one_mut(component_name)
    }

    pub fn insert_resource(&mut self, name: &str, resource: ResourcesData) {
        self.resources.insert(name, resource);
    }

    pub fn get_resource(&self, name: &str) -> Option<&ResourcesData> {
        self.resources.get(name)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Component;

    use super::World;

    #[test]
    fn create_new_world() {
        let world = World::new();
        assert_eq!(world.data.data.len(), 0);
    }

    #[test]
    fn spawn_entity() {
        let mut world = World::new();
        world
            .spawn_entity()
            .with_component("location", Component::create_vector_2(10.0, 10.0))
            .with_component("velocity", Component::create_vector_2(0.5, 0.3));

        todo!("query for locations and / or velocities");
    }
}
