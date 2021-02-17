mod entity_data;

use entity_data::{Components, EntityData};

use crate::components::Component;

pub struct World {
    entity_data: EntityData,
}

impl World {
    pub fn new() -> Self {
        let entity_data = EntityData::new();
        Self { entity_data }
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
        self.entity_data.insert(name, component);
        self
    }

    pub fn query_one(&self, name: &str) -> &Components {
        self.entity_data.query_one(name)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
