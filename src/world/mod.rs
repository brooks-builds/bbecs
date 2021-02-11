use std::rc::Rc;
use std::{any::Any, fmt::Debug};

use data::Data;
use query::Query;

mod data;
pub mod query;

pub struct World {
    data: Data,
}

impl World {
    pub fn new() -> Self {
        let data = Data::new();
        Self { data }
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
    /// Each component needs to be a unique type, otherwise they will be stored together in
    /// the [Data](data.rs) component.
    /// ```
    /// let mut world = World::new();
    /// world.spawn_entity()
    ///     .with_component(32.0_f32)
    ///     .with_component(16_i32);
    /// ```
    pub fn with_component<C: 'static>(&mut self, component: C) -> &mut Self {
        self.data.insert(component);
        self
    }

    pub fn query(&self, query: Query) -> Vec<&Vec<Rc<dyn Any>>> {
        self.data.query(query)
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::{data::Data, World};

    #[test]
    fn create_new_world() {
        let world = World::new();
        assert_eq!(world.data.data.len(), 0);
    }

    #[test]
    fn spawn_entity() {
        let mut world = World::new();
        let location = TestLocation { x: 0.0, y: 0.0 };
        let velocity = (32, 34);
        world
            .spawn_entity()
            .with_component(location)
            .with_component(velocity);
        let location_data = world.data.data.get(&location.type_id()).unwrap()[0]
            .clone()
            .downcast::<TestLocation>()
            .unwrap();
        assert_eq!(*location_data, location);

        let velocity_data = world.data.data.get(&velocity.type_id()).unwrap()[0]
            .clone()
            .downcast::<(i32, i32)>()
            .unwrap();
        assert_eq!(*velocity_data, velocity);
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct TestLocation {
        x: f32,
        y: f32,
    }
}
