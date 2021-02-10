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

    pub fn insert_entity(&mut self, entity_data: Vec<impl Any>) {
        for entity_part in entity_data {
            self.data.insert(entity_part);
        }
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
    fn insert_entity_into_world() {
        let mut world = World::new();
        let location = TestLocation { x: 0.0, y: 0.0 };
        world.insert_entity(vec![location]);
        let data = world.data.data.get(&location.type_id()).unwrap()[0]
            .clone()
            .downcast::<TestLocation>()
            .unwrap();
        assert_eq!(*data, location);
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct TestLocation {
        x: f32,
        y: f32,
    }
}
